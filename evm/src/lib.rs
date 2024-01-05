#[macro_use]
extern crate tracing;

/// Decoding helpers
pub mod decode;

/// Call tracing
/// Contains a call trace arena, decoding and formatting utilities
pub mod trace;

/// Debugger data structures
pub mod debug;

/// Coverage data structures
pub mod coverage;

/// Forge test execution backends
pub mod executor;

use corebc::types::{ActionType, CallType, H176};
pub use executor::abi;

/// Fuzzing wrapper for executors
pub mod fuzz;

/// utils for working with revm
pub mod utils;

// Re-exports
pub use corebc::types::Address;
pub use hashbrown::{self, HashMap};
pub use revm;
use revm::interpreter::{CallScheme, CreateScheme};
use serde::{Deserialize, Serialize};

//TODO:error2215 change addresses for each network
/// Stores the caller address to be used as _sender_ account for:
///     - deploying Test contracts
///     - deploying Script contracts
///
/// The address was derived from `address(uint160(uint256(sha3("foxar default caller"))))`
/// and is equal to cb681804c8ab1f12e6bbf3894d4083f33e07309d1f38.
pub const CALLER: Address = H176([
    0xcb, 0x68, 0x18, 0x04, 0xc8, 0xAB, 0x1F, 0x12, 0xE6, 0xbb, 0xF3, 0x89, 0x4D, 0x40, 0x83, 0xF3,
    0x3E, 0x07, 0x30, 0x9D, 0x1F, 0x38,
]);

/// Stores the default test contract address: cb04b4c79dab8f259c7aee6e5b2aa729821864227e84
pub const TEST_CONTRACT_ADDRESS: Address = H176([
    203, 4, 180, 199, 157, 171, 143, 37, 156, 122, 238, 110, 91, 42, 167, 41, 130, 24, 100, 34,
    126, 132,
]);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Default)]
pub enum CallKind {
    #[default]
    Call,
    StaticCall,
    CallCode,
    DelegateCall,
    Create,
    Create2,
}

impl From<CallScheme> for CallKind {
    fn from(scheme: CallScheme) -> Self {
        match scheme {
            CallScheme::Call => CallKind::Call,
            CallScheme::StaticCall => CallKind::StaticCall,
            CallScheme::CallCode => CallKind::CallCode,
            CallScheme::DelegateCall => CallKind::DelegateCall,
        }
    }
}

impl From<CreateScheme> for CallKind {
    fn from(create: CreateScheme) -> Self {
        match create {
            CreateScheme::Create => CallKind::Create,
            CreateScheme::Create2 { .. } => CallKind::Create2,
        }
    }
}

impl From<CallKind> for ActionType {
    fn from(kind: CallKind) -> Self {
        match kind {
            CallKind::Call | CallKind::StaticCall | CallKind::DelegateCall | CallKind::CallCode => {
                ActionType::Call
            }
            CallKind::Create => ActionType::Create,
            CallKind::Create2 => ActionType::Create,
        }
    }
}

impl From<CallKind> for CallType {
    fn from(ty: CallKind) -> Self {
        match ty {
            CallKind::Call => CallType::Call,
            CallKind::StaticCall => CallType::StaticCall,
            CallKind::CallCode => CallType::CallCode,
            CallKind::DelegateCall => CallType::DelegateCall,
            CallKind::Create => CallType::None,
            CallKind::Create2 => CallType::None,
        }
    }
}
