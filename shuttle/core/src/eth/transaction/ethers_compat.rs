//! corebc compatibility, this is mainly necessary so we can use all of `corebc` signers

use super::EthTransactionRequest;
use crate::eth::transaction::{
    LegacyTransactionRequest, MaybeImpersonatedTransaction, TypedTransaction,
    TypedTransactionRequest,
};
// use corebc_core::types::{
//     Address, NameOrAddress, Transaction as EthersTransaction,
//     TransactionRequest as EthersLegacyTransactionRequest, TransactionRequest, H256, U256, U64,
// };

use corebc_core::types::{
    transaction::eip2718::TypedTransaction as EthersTypedTransactionRequest, Address,
    NameOrAddress, Transaction as EthersTransaction,
    TransactionRequest as EthersLegacyTransactionRequest, TransactionRequest, H256,
};

impl From<TypedTransactionRequest> for EthersTypedTransactionRequest {
    fn from(tx: TypedTransactionRequest) -> Self {
        match tx {
            TypedTransactionRequest::Legacy(tx) => {
                let LegacyTransactionRequest {
                    nonce,
                    energy_price,
                    energy_limit,
                    kind,
                    value,
                    input,
                    network_id,
                } = tx;
                EthersTypedTransactionRequest::Legacy(EthersLegacyTransactionRequest {
                    from: None,
                    to: kind.as_call().cloned().map(Into::into),
                    energy: Some(energy_limit),
                    energy_price: Some(energy_price),
                    value: Some(value),
                    data: Some(input),
                    nonce: Some(nonce),
                    network_id: Some(network_id.into()),
                })
            }
        }
    }
}

fn to_ethers_transaction_with_hash_and_sender(
    transaction: TypedTransaction,
    hash: H256,
    from: Address,
) -> EthersTransaction {
    match transaction {
        TypedTransaction::Legacy(t) => EthersTransaction {
            hash,
            nonce: t.nonce,
            block_hash: None,
            block_number: None,
            transaction_index: None,
            from,
            to: None,
            value: t.value,
            energy_price: t.energy_price,
            energy: t.energy_limit,
            input: t.input.clone(),
            network_id: Some(t.network_id().into()),
            sig: t.signature.sig,
            // v: t.signature.v.into(),
            // r: t.signature.r,
            // s: t.signature.s,
        },
    }
}

impl From<TypedTransaction> for EthersTransaction {
    fn from(transaction: TypedTransaction) -> Self {
        let hash = transaction.hash();
        let sender = transaction.recover().unwrap_or_default();
        to_ethers_transaction_with_hash_and_sender(transaction, hash, sender)
    }
}

impl From<MaybeImpersonatedTransaction> for EthersTransaction {
    fn from(transaction: MaybeImpersonatedTransaction) -> Self {
        let hash = transaction.hash();
        let sender = transaction.recover().unwrap_or_default();
        to_ethers_transaction_with_hash_and_sender(transaction.into(), hash, sender)
    }
}

impl From<TransactionRequest> for EthTransactionRequest {
    fn from(req: TransactionRequest) -> Self {
        let TransactionRequest {
            from,
            to,
            energy,
            energy_price,
            value,
            data,
            nonce,
            network_id,
            ..
        } = req;
        EthTransactionRequest {
            from,
            to: to.and_then(|to| match to {
                NameOrAddress::Name(_) => None,
                NameOrAddress::Address(to) => Some(to),
            }),
            energy_price,
            energy,
            value,
            data,
            nonce,
            network_id: network_id.unwrap_or(1.into()),
        }
    }
}

impl From<EthTransactionRequest> for TransactionRequest {
    fn from(req: EthTransactionRequest) -> Self {
        let EthTransactionRequest { from, to, energy_price, energy, value, data, nonce, .. } = req;
        TransactionRequest {
            from,
            to: to.map(NameOrAddress::Address),
            energy,
            energy_price,
            value,
            data,
            nonce,
            network_id: None,
        }
    }
}
