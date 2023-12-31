//! Helpers for formatting ethereum types

use crate::TransactionReceiptWithRevertReason;

pub use foxar_macros::fmt::*;

impl UIfmt for TransactionReceiptWithRevertReason {
    fn pretty(&self) -> String {
        if let Some(revert_reason) = &self.revert_reason {
            format!(
                "{}
revertReason            {}",
                self.receipt.pretty(),
                revert_reason
            )
        } else {
            self.receipt.pretty()
        }
    }
}

/// Returns the ``UiFmt::pretty()` formatted attribute of the transaction receipt
pub fn get_pretty_tx_receipt_attr(
    receipt: &TransactionReceiptWithRevertReason,
    attr: &str,
) -> Option<String> {
    match attr {
        "blockHash" | "block_hash" => Some(receipt.receipt.block_hash.pretty()),
        "blockNumber" | "block_number" => Some(receipt.receipt.block_number.pretty()),
        "contractAddress" | "contract_address" => Some(receipt.receipt.contract_address.pretty()),
        "cumulativeEnergyUsed" | "cumulative_energy_used" => {
            Some(receipt.receipt.cumulative_energy_used.pretty())
        }
        "energyUsed" | "energy_used" => Some(receipt.receipt.energy_used.pretty()),
        "logs" => Some(receipt.receipt.logs.pretty()),
        "logsBloom" | "logs_bloom" => Some(receipt.receipt.logs_bloom.pretty()),
        "root" => Some(receipt.receipt.root.pretty()),
        "status" => Some(receipt.receipt.status.pretty()),
        "transactionHash" | "transaction_hash" => Some(receipt.receipt.transaction_hash.pretty()),
        "transactionIndex" | "transaction_index" => {
            Some(receipt.receipt.transaction_index.pretty())
        }
        "revertReason" | "revert_reason" => Some(receipt.revert_reason.pretty()),
        _ => None,
    }
}
