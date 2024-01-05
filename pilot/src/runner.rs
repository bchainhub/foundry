//! PilotRunner
//!
//! This module contains the `PilotRunner` struct, which assists with deploying
//! and calling the REPL contract on a in-memory REVM instance.

use corebc::{
    prelude::{types::U256, Address},
    types::{Bytes, Log},
};
use eyre::Result;
use revm::interpreter::{return_ok, InstructionResult};
use spark::{
    executor::{DeployResult, Executor, RawCallResult},
    trace::{CallTraceArena, TraceKind},
};
use std::collections::BTreeMap;

/// The function selector of the REPL contract's entrypoint, the `run()` function.
static RUN_SELECTOR: [u8; 4] = [0x3b, 0x21, 0xbc, 0x14];

/// The Pilot Runner
///
/// Based off of foxar's spark cli runner for scripting.
/// See: [runner](cli::cmd::spark::script::runner.rs)
#[derive(Debug)]
pub struct PilotRunner {
    /// The Executor
    pub executor: Executor,
    /// An initial balance
    pub initial_balance: U256,
    /// The sender
    pub sender: Address,
    /// Input calldata appended to `RUN_SELECTOR`
    pub input: Option<Vec<u8>>,
}

/// Represents the result of a Pilot REPL run
#[derive(Debug, Default)]
pub struct PilotResult {
    /// Was the run a success?
    pub success: bool,
    /// Transaction logs
    pub logs: Vec<Log>,
    /// Call traces
    pub traces: Vec<(TraceKind, CallTraceArena)>,
    /// Amount of energy used in the transaction
    pub energy_used: u64,
    /// Map of addresses to their labels
    pub labeled_addresses: BTreeMap<Address, String>,
    /// Return data
    pub returned: bytes::Bytes,
    /// Called address
    pub address: Option<Address>,
    /// EVM State at the final instruction of the `run()` function
    pub state: Option<(
        revm::interpreter::Stack,
        revm::interpreter::Memory,
        revm::interpreter::InstructionResult,
    )>,
}

/// PilotRunner implementation
impl PilotRunner {
    /// Create a new [PilotRunner]
    ///
    /// ### Takes
    ///
    /// An [Executor], the initial balance of the sender, and the sender's [Address].
    ///
    /// ### Returns
    ///
    /// A new [PilotRunner]
    pub fn new(
        executor: Executor,
        initial_balance: U256,
        sender: Address,
        input: Option<Vec<u8>>,
    ) -> Self {
        Self { executor, initial_balance, sender, input }
    }

    /// Run a contract as a REPL session
    ///
    /// ### Takes
    ///
    /// The creation bytecode of the REPL contract
    ///
    /// ### Returns
    ///
    /// Optionally, a tuple containing the deployed address of the bytecode as well as a
    /// [PilotResult] containing information about the result of the call to the deployed REPL
    /// contract.
    pub fn run(&mut self, bytecode: Bytes) -> Result<(Address, PilotResult)> {
        // Set the sender's balance to [U256::MAX] for deployment of the REPL contract.
        self.executor.set_balance(self.sender, U256::MAX)?;

        // Deploy an instance of the REPL contract
        // We don't care about deployment traces / logs here
        let DeployResult { address, .. } = self
            .executor
            .deploy(self.sender, bytecode.0, 0.into(), None)
            .map_err(|err| eyre::eyre!("Failed to deploy REPL contract:\n{}", err))?;

        // Reset the sender's balance to the initial balance for calls.
        self.executor.set_balance(self.sender, self.initial_balance)?;

        // Append the input to the `RUN_SELECTOR` to form the calldata
        let mut calldata = RUN_SELECTOR.to_vec();
        if let Some(mut input) = self.input.clone() {
            calldata.append(&mut input);
        }

        // Call the "run()" function of the REPL contract
        let call_res = self.call(self.sender, address, Bytes::from(calldata), 0.into(), true);

        call_res.map(|res| (address, res))
    }

    /// Executes the call
    ///
    /// This will commit the changes if `commit` is true.
    ///
    /// This will return _estimated_ energy instead of the precise energy the call would consume, so
    /// it can be used as `energy_limit`.
    ///
    /// Taken from [Forge's Script Runner](https://github.com/foxar-rs/foxar/blob/master/cli/src/cmd/spark/script/runner.rs)
    fn call(
        &mut self,
        from: Address,
        to: Address,
        calldata: Bytes,
        value: U256,
        commit: bool,
    ) -> eyre::Result<PilotResult> {
        let fs_commit_changed =
            if let Some(ref mut cheatcodes) = self.executor.inspector_config_mut().cheatcodes {
                let original_fs_commit = cheatcodes.fs_commit;
                cheatcodes.fs_commit = false;
                original_fs_commit != cheatcodes.fs_commit
            } else {
                false
            };

        let mut res = self.executor.call_raw(from, to, calldata.0.clone(), value)?;
        let mut energy_used = res.energy_used;
        if matches!(res.exit_reason, return_ok!()) {
            // store the current energy limit and reset it later
            let init_energy_limit = self.executor.env_mut().tx.energy_limit;

            // the executor will return the _exact_ energy value this transaction consumed, setting
            // this value as energy limit will result in `OutOfGas` so to come up with a
            // better estimate we search over a possible range we pick a higher energy
            // limit 3x of a succeeded call should be safe
            let mut highest_energy_limit = energy_used * 3;
            let mut lowest_energy_limit = energy_used;
            let mut last_highest_energy_limit = highest_energy_limit;
            while (highest_energy_limit - lowest_energy_limit) > 1 {
                let mid_energy_limit = (highest_energy_limit + lowest_energy_limit) / 2;
                self.executor.env_mut().tx.energy_limit = mid_energy_limit;
                let res = self.executor.call_raw(from, to, calldata.0.clone(), value)?;
                match res.exit_reason {
                    InstructionResult::Revert |
                    InstructionResult::OutOfEnergy |
                    InstructionResult::OutOfFund => {
                        lowest_energy_limit = mid_energy_limit;
                    }
                    _ => {
                        highest_energy_limit = mid_energy_limit;
                        // if last two successful estimations only vary by 10%, we consider this to
                        // sufficiently accurate
                        const ACCURACY: u64 = 10;
                        if (last_highest_energy_limit - highest_energy_limit) * ACCURACY /
                            last_highest_energy_limit <
                            1
                        {
                            // update the energy
                            energy_used = highest_energy_limit;
                            break;
                        }
                        last_highest_energy_limit = highest_energy_limit;
                    }
                }
            }
            // reset energy limit in the
            self.executor.env_mut().tx.energy_limit = init_energy_limit;
        }

        // if we changed `fs_commit` during energy limit search, re-execute the call with original
        // value
        if fs_commit_changed {
            if let Some(ref mut cheatcodes) = self.executor.inspector_config_mut().cheatcodes {
                cheatcodes.fs_commit = !cheatcodes.fs_commit;
            }

            res = self.executor.call_raw(from, to, calldata.0.clone(), value)?;
        }

        if commit {
            // if explicitly requested we can now commit the call
            res = self.executor.call_raw_committing(from, to, calldata.0, value)?;
        }

        let RawCallResult { result, reverted, logs, traces, labels, pilot_state, .. } = res;

        Ok(PilotResult {
            returned: result,
            success: !reverted,
            energy_used,
            logs,
            traces: traces
                .map(|traces| {
                    // Manually adjust energy for the trace to add back the stipend/real used energy
                    // TODO: For pilot, we may not want to perform this adjustment.
                    // traces.arena[0].trace.energy_cost = energy_used;
                    vec![(TraceKind::Execution, traces)]
                })
                .unwrap_or_default(),
            labeled_addresses: labels,
            address: None,
            state: pilot_state,
        })
    }
}
