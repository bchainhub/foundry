use crate::{
    debug::Instruction::OpCode,
    executor::inspector::utils::{energy_used, get_create_address},
    trace::{
        CallTrace, CallTraceArena, CallTraceStep, LogCallOrder, RawOrDecodedCall, RawOrDecodedLog,
        RawOrDecodedReturnData,
    },
    utils::{b176_to_h176, b256_to_h256, ru256_to_u256},
    CallKind,
};
use bytes::Bytes;
use corebc::{
    abi::RawLog,
    types::{Address, Network, U256},
};
use revm::{
    inspectors::EnergyInspector,
    interpreter::{
        opcode, return_ok, CallInputs, CallScheme, CreateInputs, Energy, InstructionResult,
        Interpreter,
    },
    primitives::{B176, B256},
    Database, EVMData, Inspector, JournalEntry,
};
use std::{cell::RefCell, rc::Rc};

/// An inspector that collects call traces.
#[derive(Default, Debug, Clone)]
pub struct Tracer {
    record_steps: bool,

    pub traces: CallTraceArena,
    trace_stack: Vec<usize>,
    step_stack: Vec<(usize, usize)>, // (trace_idx, step_idx)

    energy_inspector: Rc<RefCell<EnergyInspector>>,
}

impl Tracer {
    /// Enables step recording and uses [revm::EnergyInspector] to report energy costs for each
    /// step.
    ///
    /// Energy Inspector should be called externally **before** [Tracer], this is why we need it as
    /// `Rc<RefCell<_>>` here.
    pub fn with_steps_recording(mut self, energy_inspector: Rc<RefCell<EnergyInspector>>) -> Self {
        self.record_steps = true;
        self.energy_inspector = energy_inspector;
        self
    }

    fn start_trace(
        &mut self,
        depth: usize,
        address: Address,
        data: Vec<u8>,
        value: U256,
        kind: CallKind,
        caller: Address,
    ) {
        self.trace_stack.push(self.traces.push_trace(
            0,
            CallTrace {
                depth,
                address,
                kind,
                data: RawOrDecodedCall::Raw(data.into()),
                value,
                status: InstructionResult::Continue,
                caller,
                ..Default::default()
            },
        ));
    }

    fn fill_trace(
        &mut self,
        status: InstructionResult,
        cost: u64,
        output: Vec<u8>,
        address: Option<Address>,
    ) {
        let success = matches!(status, return_ok!());
        let trace = &mut self.traces.arena
            [self.trace_stack.pop().expect("more traces were filled than started")]
        .trace;
        trace.status = status;
        trace.success = success;
        trace.energy_cost = cost;
        trace.output = RawOrDecodedReturnData::Raw(output.into());

        if let Some(address) = address {
            trace.address = address;
        }
    }

    fn start_step<DB: Database>(&mut self, interp: &mut Interpreter, data: &mut EVMData<'_, DB>) {
        let trace_idx =
            *self.trace_stack.last().expect("can't start step without starting a trace first");
        let trace = &mut self.traces.arena[trace_idx];

        self.step_stack.push((trace_idx, trace.trace.steps.len()));

        let pc = interp.program_counter();

        trace.trace.steps.push(CallTraceStep {
            depth: data.journaled_state.depth(),
            pc,
            op: OpCode(interp.contract.bytecode.bytecode()[pc]),
            contract: b176_to_h176(interp.contract.address),
            stack: interp.stack.clone(),
            memory: interp.memory.clone(),
            energy: self.energy_inspector.borrow().energy_remaining(),
            energy_refund_counter: interp.energy.refunded() as u64,
            energy_cost: 0,
            state_diff: None,
            error: None,
        });
    }

    fn fill_step<DB: Database>(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        status: InstructionResult,
    ) {
        let (trace_idx, step_idx) =
            self.step_stack.pop().expect("can't fill step without starting a step first");
        let step = &mut self.traces.arena[trace_idx].trace.steps[step_idx];

        if let Some(pc) = interp.program_counter().checked_sub(1) {
            let op = interp.contract.bytecode.bytecode()[pc];

            let journal_entry = data
                .journaled_state
                .journal
                .last()
                // This should always work because revm initializes it as `vec![vec![]]`
                .unwrap()
                .last();

            step.state_diff = match (op, journal_entry) {
                (
                    opcode::SLOAD | opcode::SSTORE,
                    Some(JournalEntry::StorageChange { address, key, .. }),
                ) => {
                    let value = data.journaled_state.state[address].storage[key].present_value();
                    Some((ru256_to_u256(*key), ru256_to_u256(value)))
                }
                _ => None,
            };

            step.energy_cost = step.energy - self.energy_inspector.borrow().energy_remaining();
        }

        // Error codes only
        if status as u8 > InstructionResult::OutOfEnergy as u8 {
            step.error = Some(format!("{status:?}"));
        }
    }
}

impl<DB> Inspector<DB> for Tracer
where
    DB: Database,
{
    fn step(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        _is_static: bool,
    ) -> InstructionResult {
        if !self.record_steps {
            return InstructionResult::Continue
        }

        self.start_step(interp, data);

        InstructionResult::Continue
    }

    fn log(&mut self, _: &mut EVMData<'_, DB>, _: &B176, topics: &[B256], data: &Bytes) {
        let node = &mut self.traces.arena[*self.trace_stack.last().expect("no ongoing trace")];
        let topics: Vec<_> = topics.iter().copied().map(b256_to_h256).collect();
        node.ordering.push(LogCallOrder::Log(node.logs.len()));
        node.logs.push(RawOrDecodedLog::Raw(RawLog { topics, data: data.to_vec() }));
    }

    fn step_end(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        _is_static: bool,
        status: InstructionResult,
    ) -> InstructionResult {
        if !self.record_steps {
            return InstructionResult::Continue
        }

        self.fill_step(interp, data, status);

        status
    }

    fn call(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &mut CallInputs,
        _is_static: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        let (from, to) = match inputs.context.scheme {
            CallScheme::DelegateCall | CallScheme::CallCode => {
                (inputs.context.address, inputs.context.code_address)
            }
            _ => (inputs.context.caller, inputs.context.address),
        };

        self.start_trace(
            data.journaled_state.depth() as usize,
            b176_to_h176(to),
            inputs.input.to_vec(),
            ru256_to_u256(inputs.transfer.value),
            inputs.context.scheme.into(),
            b176_to_h176(from),
        );

        (InstructionResult::Continue, Energy::new(inputs.energy_limit), Bytes::new())
    }

    fn call_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        _inputs: &CallInputs,
        energy: Energy,
        status: InstructionResult,
        retdata: Bytes,
        _is_static: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        self.fill_trace(
            status,
            energy_used(data.env.cfg.spec_id, energy.spend(), energy.refunded() as u64),
            retdata.to_vec(),
            None,
        );

        (status, energy, retdata)
    }

    fn create(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &mut CreateInputs,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        // TODO: Does this increase energy cost?
        let _ = data.journaled_state.load_account(inputs.caller, data.db);
        let nonce = data.journaled_state.account(inputs.caller).info.nonce;
        self.start_trace(
            data.journaled_state.depth() as usize,
            get_create_address(inputs, nonce, &Network::from(data.env.cfg.network_id)),
            inputs.init_code.to_vec(),
            ru256_to_u256(inputs.value),
            inputs.scheme.into(),
            b176_to_h176(inputs.caller),
        );

        (InstructionResult::Continue, None, Energy::new(inputs.energy_limit), Bytes::new())
    }

    fn create_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        _inputs: &CreateInputs,
        status: InstructionResult,
        address: Option<B176>,
        energy: Energy,
        retdata: Bytes,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        let code = match address {
            Some(address) => data
                .journaled_state
                .account(address)
                .info
                .code
                .as_ref()
                .map_or(vec![], |code| code.bytes()[..code.len()].to_vec()),
            None => vec![],
        };
        self.fill_trace(
            status,
            energy_used(data.env.cfg.spec_id, energy.spend(), energy.refunded() as u64),
            code,
            address.map(b176_to_h176),
        );

        (status, address, energy, retdata)
    }
}
