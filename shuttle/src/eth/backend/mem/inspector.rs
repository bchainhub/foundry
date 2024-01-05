//! Anvil specific [`revm::Inspector`] implementation

use crate::{eth::macros::node_info, revm::Database};
use bytes::Bytes;
use corebc::types::Log;
use foxar_evm::{
    call_inspectors,
    decode::decode_console_logs,
    executor::inspector::{LogCollector, Tracer},
    revm,
    revm::{
        inspectors::EnergyInspector,
        interpreter::{CallInputs, CreateInputs, Energy, InstructionResult, Interpreter},
        EVMData,
    },
};
use spark::revm::primitives::{B176, B256};
use std::{cell::RefCell, rc::Rc};

/// The [`revm::Inspector`] used when transacting in the evm
#[derive(Debug, Clone, Default)]
pub struct Inspector {
    pub energy: Option<Rc<RefCell<EnergyInspector>>>,
    pub tracer: Option<Tracer>,
    /// collects all `console.sol` logs
    pub logs: LogCollector,
}

// === impl Inspector ===

impl Inspector {
    /// Called after the inspecting the evm
    ///
    /// This will log all `console.sol` logs
    pub fn print_logs(&self) {
        print_logs(&self.logs.logs)
    }

    /// Configures the `Tracer` [`revm::Inspector`]
    pub fn with_tracing(mut self) -> Self {
        self.tracer = Some(Default::default());
        self
    }

    /// Enables steps recording for `Tracer` and attaches `GasInspector` to it
    /// If `Tracer` wasn't configured before, configures it automatically
    pub fn with_steps_tracing(mut self) -> Self {
        if self.tracer.is_none() {
            self = self.with_tracing()
        }
        let energy_inspector = Rc::new(RefCell::new(EnergyInspector::default()));
        self.energy = Some(energy_inspector.clone());
        self.tracer = self.tracer.map(|tracer| tracer.with_steps_recording(energy_inspector));

        self
    }
}

impl<DB: Database> revm::Inspector<DB> for Inspector {
    fn initialize_interp(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        is_static: bool,
    ) -> InstructionResult {
        call_inspectors!(
            inspector,
            [&mut self.energy.as_deref().map(|energy| energy.borrow_mut()), &mut self.tracer],
            { inspector.initialize_interp(interp, data, is_static) }
        );
        InstructionResult::Continue
    }

    fn step(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        is_static: bool,
    ) -> InstructionResult {
        call_inspectors!(
            inspector,
            [&mut self.energy.as_deref().map(|energy| energy.borrow_mut()), &mut self.tracer],
            {
                inspector.step(interp, data, is_static);
            }
        );
        InstructionResult::Continue
    }

    fn log(
        &mut self,
        evm_data: &mut EVMData<'_, DB>,
        address: &B176,
        topics: &[B256],
        data: &Bytes,
    ) {
        call_inspectors!(
            inspector,
            [
                &mut self.energy.as_deref().map(|energy| energy.borrow_mut()),
                &mut self.tracer,
                Some(&mut self.logs)
            ],
            {
                inspector.log(evm_data, address, topics, data);
            }
        );
    }

    fn step_end(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        is_static: bool,
        eval: InstructionResult,
    ) -> InstructionResult {
        call_inspectors!(
            inspector,
            [&mut self.energy.as_deref().map(|energy| energy.borrow_mut()), &mut self.tracer],
            {
                inspector.step_end(interp, data, is_static, eval);
            }
        );
        eval
    }

    fn call(
        &mut self,
        data: &mut EVMData<'_, DB>,
        call: &mut CallInputs,
        is_static: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        call_inspectors!(
            inspector,
            [
                &mut self.energy.as_deref().map(|energy| energy.borrow_mut()),
                &mut self.tracer,
                Some(&mut self.logs)
            ],
            {
                inspector.call(data, call, is_static);
            }
        );

        (InstructionResult::Continue, Energy::new(call.energy_limit), Bytes::new())
    }

    fn call_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &CallInputs,
        remaining_energy: Energy,
        ret: InstructionResult,
        out: Bytes,
        is_static: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        call_inspectors!(
            inspector,
            [&mut self.energy.as_deref().map(|energy| energy.borrow_mut()), &mut self.tracer],
            {
                inspector.call_end(data, inputs, remaining_energy, ret, out.clone(), is_static);
            }
        );
        (ret, remaining_energy, out)
    }

    fn create(
        &mut self,
        data: &mut EVMData<'_, DB>,
        call: &mut CreateInputs,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        call_inspectors!(
            inspector,
            [&mut self.energy.as_deref().map(|energy| energy.borrow_mut()), &mut self.tracer],
            {
                inspector.create(data, call);
            }
        );

        (InstructionResult::Continue, None, Energy::new(call.energy_limit), Bytes::new())
    }

    fn create_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &CreateInputs,
        status: InstructionResult,
        address: Option<B176>,
        energy: Energy,
        retdata: Bytes,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        call_inspectors!(
            inspector,
            [&mut self.energy.as_deref().map(|energy| energy.borrow_mut()), &mut self.tracer],
            {
                inspector.create_end(data, inputs, status, address, energy, retdata.clone());
            }
        );
        (status, address, energy, retdata)
    }
}

/// Prints all the logs
pub fn print_logs(logs: &[Log]) {
    for log in decode_console_logs(logs) {
        node_info!("{}", log);
    }
}
