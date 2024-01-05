use crate::{
    debug::{DebugArena, DebugNode, DebugStep, Instruction},
    executor::{
        backend::DatabaseExt,
        inspector::utils::{energy_used, get_create_address},
        CHEATCODE_ADDRESS,
    },
    utils::{b176_to_h176, ru256_to_u256},
    CallKind,
};
use bytes::Bytes;
use corebc::types::{Address, Network};
use foxar_utils::error::SolError;
use revm::{
    inspectors::EnergyInspector,
    interpreter::{
        opcode::{self, spec_opcode_energy},
        CallInputs, CreateInputs, Energy, InstructionResult, Interpreter, Memory,
    },
    primitives::B176,
    EVMData, Inspector,
};
use std::{cell::RefCell, rc::Rc};

/// An inspector that collects debug nodes on every step of the interpreter.
#[derive(Debug)]
pub struct Debugger {
    /// The arena of [DebugNode]s
    pub arena: DebugArena,
    /// The ID of the current [DebugNode].
    pub head: usize,
    /// The current execution address.
    pub context: Address,

    energy_inspector: Rc<RefCell<EnergyInspector>>,
}

impl Debugger {
    pub fn new(energy_inspector: Rc<RefCell<EnergyInspector>>) -> Self {
        Self {
            arena: Default::default(),
            head: Default::default(),
            context: Default::default(),
            energy_inspector,
        }
    }

    /// Enters a new execution context.
    pub fn enter(&mut self, depth: usize, address: Address, kind: CallKind) {
        self.context = address;
        self.head = self.arena.push_node(DebugNode { depth, address, kind, ..Default::default() });
    }

    /// Exits the current execution context, replacing it with the previous one.
    pub fn exit(&mut self) {
        if let Some(parent_id) = self.arena.arena[self.head].parent {
            let DebugNode { depth, address, kind, .. } = self.arena.arena[parent_id];
            self.context = address;
            self.head =
                self.arena.push_node(DebugNode { depth, address, kind, ..Default::default() });
        }
    }
}

impl<DB> Inspector<DB> for Debugger
where
    DB: DatabaseExt,
{
    fn step(
        &mut self,
        interpreter: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        _is_static: bool,
    ) -> InstructionResult {
        let pc = interpreter.program_counter();
        let op = interpreter.contract.bytecode.bytecode()[pc];

        // Get opcode information
        let opcode_infos = spec_opcode_energy(data.env.cfg.spec_id);
        let opcode_info = &opcode_infos[op as usize];

        // Extract the push bytes
        let push_size = if opcode_info.is_push() { (op - opcode::PUSH1 + 1) as usize } else { 0 };
        let push_bytes = match push_size {
            0 => None,
            n => {
                let start = pc + 1;
                let end = start + n;
                Some(interpreter.contract.bytecode.bytecode()[start..end].to_vec())
            }
        };

        let total_energy_used = energy_used(
            data.env.cfg.spec_id,
            interpreter
                .energy
                .limit()
                .saturating_sub(self.energy_inspector.borrow().energy_remaining()),
            interpreter.energy.refunded() as u64,
        );

        self.arena.arena[self.head].steps.push(DebugStep {
            pc,
            stack: interpreter.stack().data().iter().copied().map(ru256_to_u256).collect(),
            memory: interpreter.memory.clone(),
            instruction: Instruction::OpCode(op),
            push_bytes,
            total_energy_used,
        });

        InstructionResult::Continue
    }

    fn call(
        &mut self,
        data: &mut EVMData<'_, DB>,
        call: &mut CallInputs,
        _: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        self.enter(
            data.journaled_state.depth() as usize,
            b176_to_h176(call.context.code_address),
            call.context.scheme.into(),
        );
        if CHEATCODE_ADDRESS == b176_to_h176(call.contract) {
            self.arena.arena[self.head].steps.push(DebugStep {
                memory: Memory::new(),
                instruction: Instruction::Cheatcode(
                    call.input[0..4].try_into().expect("malformed cheatcode call"),
                ),
                ..Default::default()
            });
        }

        (InstructionResult::Continue, Energy::new(call.energy_limit), Bytes::new())
    }

    fn call_end(
        &mut self,
        _: &mut EVMData<'_, DB>,
        _: &CallInputs,
        energy: Energy,
        status: InstructionResult,
        retdata: Bytes,
        _: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        self.exit();

        (status, energy, retdata)
    }

    fn create(
        &mut self,
        data: &mut EVMData<'_, DB>,
        call: &mut CreateInputs,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        // TODO: Does this increase energy cost?
        if let Err(err) = data.journaled_state.load_account(call.caller, data.db) {
            let energy = Energy::new(call.energy_limit);
            return (InstructionResult::Revert, None, energy, err.encode_string().0);
        }

        let nonce = data.journaled_state.account(call.caller).info.nonce;
        self.enter(
            data.journaled_state.depth() as usize,
            get_create_address(call, nonce, &Network::from(data.env.cfg.network_id)),
            CallKind::Create,
        );

        (InstructionResult::Continue, None, Energy::new(call.energy_limit), Bytes::new())
    }

    fn create_end(
        &mut self,
        _: &mut EVMData<'_, DB>,
        _: &CreateInputs,
        status: InstructionResult,
        address: Option<B176>,
        energy: Energy,
        retdata: Bytes,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        self.exit();

        (status, address, energy, retdata)
    }
}
