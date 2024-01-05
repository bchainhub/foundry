use crate::{
    executor::{patch_hardhat_console_selector, HardhatConsoleCalls, HARDHAT_CONSOLE_ADDRESS},
    utils::{b176_to_h176, b256_to_h256, h176_to_b176},
};
use bytes::Bytes;
use corebc::{
    abi::{AbiDecode, Token},
    types::{Log, H256},
};
use foxar_macros::ConsoleFmt;
use revm::{
    interpreter::{CallInputs, Energy, InstructionResult},
    primitives::{B176, B256},
    Database, EVMData, Inspector,
};

/// An inspector that collects logs during execution.
///
/// The inspector collects logs from the LOG opcodes as well as Hardhat-style logs.
#[derive(Debug, Clone, Default)]
pub struct LogCollector {
    pub logs: Vec<Log>,
}

impl LogCollector {
    fn hardhat_log(&mut self, mut input: Vec<u8>) -> (InstructionResult, Bytes) {
        // Patch the Hardhat-style selectors
        patch_hardhat_console_selector(&mut input);
        let decoded = match HardhatConsoleCalls::decode(input) {
            Ok(inner) => inner,
            Err(err) => {
                return (
                    InstructionResult::Revert,
                    corebc::abi::encode(&[Token::String(err.to_string())]).into(),
                )
            }
        };

        // Convert it to a DS-style `emit log(string)` event
        self.logs.push(convert_hh_log_to_event(decoded));

        (InstructionResult::Continue, Bytes::new())
    }
}

impl<DB> Inspector<DB> for LogCollector
where
    DB: Database,
{
    fn log(&mut self, _: &mut EVMData<'_, DB>, address: &B176, topics: &[B256], data: &Bytes) {
        self.logs.push(Log {
            address: b176_to_h176(*address),
            topics: topics.iter().copied().map(b256_to_h256).collect(),
            data: data.clone().into(),
            ..Default::default()
        });
    }

    fn call(
        &mut self,
        _: &mut EVMData<'_, DB>,
        call: &mut CallInputs,
        _: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        if call.contract == h176_to_b176(HARDHAT_CONSOLE_ADDRESS) {
            let (status, reason) = self.hardhat_log(call.input.to_vec());
            (status, Energy::new(call.energy_limit), reason)
        } else {
            (InstructionResult::Continue, Energy::new(call.energy_limit), Bytes::new())
        }
    }
}

/// Topic 0 of DSTest's `log(string)`.
///
/// `0x5d8464571af6a643447e20a7170a35efed478750e3627c556e4f503d70b20a6a`
const TOPIC: H256 = H256([
    0x5d, 0x84, 0x64, 0x57, 0x1a, 0xf6, 0xa6, 0x43, 0x44, 0x7e, 0x20, 0xa7, 0x17, 0x0a, 0x35, 0xef,
    0xed, 0x47, 0x87, 0x50, 0xe3, 0x62, 0x7c, 0x55, 0x6e, 0x4f, 0x50, 0x3d, 0x70, 0xb2, 0x0a, 0x6a,
]);

/// Converts a call to Hardhat's `console.log` to a DSTest `log(string)` event.
fn convert_hh_log_to_event(call: HardhatConsoleCalls) -> Log {
    // Convert the parameters of the call to their string representation using `ConsoleFmt`.
    let fmt = call.fmt(Default::default());
    let token = Token::String(fmt);
    let data = corebc::abi::encode(&[token]).into();
    Log { topics: vec![TOPIC], data, ..Default::default() }
}
