//! transaction related data

use crate::eth::receipt::Log;
use corebc_core::{
    types::{
        Address, Bloom, Bytes, Network, Signature, SignatureError, TxHash, H1368, H256, U256, U64,
    },
    utils::{
        rlp,
        rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream},
        sha3,
    },
};
use foxar_evm::trace::CallTraceArena;
use foxar_utils::types::ToRuint;
use revm::{
    interpreter::InstructionResult,
    primitives::{CreateScheme, TransactTo, TxEnv},
};
use std::ops::Deref;

/// compatibility with `corebc-rs` types
mod ethers_compat;

/// The signature used to bypass signing via the `eth_sendUnsignedTransaction` cheat RPC
#[cfg(feature = "impersonated-tx")]
pub const IMPERSONATED_SIGNATURE: Signature = Signature { sig: H1368::zero() };

/// Container type for various Ethereum transaction requests
///
/// Its variants correspond to specific allowed transactions:
/// 1. Legacy (pre-EIP2718) [`LegacyTransactionRequest`]
/// 2. EIP2930 (state access lists) [`EIP2930TransactionRequest`]
/// 3. EIP1559 [`EIP1559TransactionRequest`]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypedTransactionRequest {
    Legacy(LegacyTransactionRequest),
}

/// Represents _all_ transaction requests received from RPC
#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct EthTransactionRequest {
    /// from address
    pub from: Option<Address>,
    /// to address
    pub to: Option<Address>,
    /// legacy, energy Price
    #[cfg_attr(feature = "serde", serde(default))]
    pub energy_price: Option<U256>,
    ///energy
    pub energy: Option<U256>,
    /// value of th tx in wei
    pub value: Option<U256>,
    /// Any additional data sent
    pub data: Option<Bytes>,
    /// Transaction nonce
    pub nonce: Option<U256>,
    /// chain id
    #[cfg_attr(feature = "serde", serde(default = "default_network_id"))]
    pub network_id: U64,
}

pub fn default_network_id() -> U64 {
    U64::from(1)
}

// == impl EthTransactionRequest ==

impl EthTransactionRequest {
    /// Converts the request into a [TypedTransactionRequest]
    pub fn into_typed_request(self) -> Option<TypedTransactionRequest> {
        let EthTransactionRequest {
            to, energy_price, energy, value, data, nonce, network_id, ..
        } = self;
        match energy_price {
            Some(_) => Some(TypedTransactionRequest::Legacy(LegacyTransactionRequest {
                nonce: nonce.unwrap_or(U256::zero()),
                energy_price: energy_price.unwrap_or_default(),
                energy_limit: energy.unwrap_or_default(),
                value: value.unwrap_or(U256::zero()),
                input: data.unwrap_or_default(),
                kind: match to {
                    Some(to) => TransactionKind::Call(to),
                    None => TransactionKind::Create,
                },
                network_id: network_id.as_u64(),
            })),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransactionKind {
    Call(Address),
    Create,
}

// == impl TransactionKind ==

impl TransactionKind {
    /// If this transaction is a call this returns the address of the callee
    pub fn as_call(&self) -> Option<&Address> {
        match self {
            TransactionKind::Call(to) => Some(to),
            TransactionKind::Create => None,
        }
    }
}

impl Encodable for TransactionKind {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            TransactionKind::Call(address) => {
                s.encoder().encode_value(&address[..]);
            }
            TransactionKind::Create => s.encoder().encode_value(&[]),
        }
    }
}

impl Decodable for TransactionKind {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            if rlp.is_data() {
                Ok(TransactionKind::Create)
            } else {
                Err(DecoderError::RlpExpectedToBeData)
            }
        } else {
            Ok(TransactionKind::Call(rlp.as_val()?))
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for TransactionKind {
    fn length(&self) -> usize {
        match self {
            TransactionKind::Call(to) => to.length(),
            TransactionKind::Create => ([]).length(),
        }
    }
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        match self {
            TransactionKind::Call(to) => to.encode(out),
            TransactionKind::Create => ([]).encode(out),
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for TransactionKind {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        use bytes::Buf;

        if let Some(&first) = buf.first() {
            if first == 0x80 {
                buf.advance(1);
                Ok(TransactionKind::Create)
            } else {
                let addr = <Address as open_fastrlp::Decodable>::decode(buf)?;
                Ok(TransactionKind::Call(addr))
            }
        } else {
            Err(open_fastrlp::DecodeError::InputTooShort)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyTransactionRequest {
    pub nonce: U256,
    pub energy_price: U256,
    pub energy_limit: U256,
    pub kind: TransactionKind,
    pub value: U256,
    pub input: Bytes,
    pub network_id: u64,
}

// == impl LegacyTransactionRequest ==

impl LegacyTransactionRequest {
    pub fn hash(&self) -> H256 {
        H256::from_slice(sha3(&rlp::encode(self)).as_slice())
    }
}

impl From<LegacyTransaction> for LegacyTransactionRequest {
    fn from(tx: LegacyTransaction) -> Self {
        let network_id = tx.network_id();
        Self {
            nonce: tx.nonce,
            energy_price: tx.energy_price,
            energy_limit: tx.energy_limit,
            kind: tx.kind,
            value: tx.value,
            input: tx.input,
            network_id,
        }
    }
}

impl Encodable for LegacyTransactionRequest {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.nonce);
        s.append(&self.energy_price);
        s.append(&self.energy_limit);
        s.append(&self.kind);
        s.append(&self.value);
        s.append(&self.input.as_ref());
        s.append(&self.network_id);
    }
}

/// A wrapper for `TypedTransaction` that allows impersonating accounts.
///
/// This is a helper that carries the `impersonated` sender so that the right hash
/// [TypedTransaction::impersonated_hash] can be created.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaybeImpersonatedTransaction {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub transaction: TypedTransaction,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub impersonated_sender: Option<Address>,
}

// === impl MaybeImpersonatedTransaction ===

impl MaybeImpersonatedTransaction {
    /// Creates a new wrapper for the given transaction
    pub fn new(transaction: TypedTransaction) -> Self {
        Self { transaction, impersonated_sender: None }
    }

    /// Creates a new impersonated transaction wrapper using the given sender
    pub fn impersonated(transaction: TypedTransaction, impersonated_sender: Address) -> Self {
        Self { transaction, impersonated_sender: Some(impersonated_sender) }
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    ///
    /// Note: this is feature gated so it does not conflict with the `Deref`ed
    /// [TypedTransaction::recover] function by default.
    #[cfg(feature = "impersonated-tx")]
    pub fn recover(&self) -> Result<Address, SignatureError> {
        if let Some(sender) = self.impersonated_sender {
            return Ok(sender);
        }
        self.transaction.recover()
    }

    /// Returns the hash of the transaction
    ///
    /// Note: this is feature gated so it does not conflict with the `Deref`ed
    /// [TypedTransaction::hash] function by default.
    #[cfg(feature = "impersonated-tx")]
    pub fn hash(&self) -> H256 {
        if self.transaction.is_impersonated() {
            if let Some(sender) = self.impersonated_sender {
                return self.transaction.impersonated_hash(sender);
            }
        }
        self.transaction.hash()
    }
}

impl Encodable for MaybeImpersonatedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.transaction.rlp_append(s)
    }
}

impl From<MaybeImpersonatedTransaction> for TypedTransaction {
    fn from(value: MaybeImpersonatedTransaction) -> Self {
        value.transaction
    }
}

impl From<TypedTransaction> for MaybeImpersonatedTransaction {
    fn from(value: TypedTransaction) -> Self {
        MaybeImpersonatedTransaction::new(value)
    }
}

impl Decodable for MaybeImpersonatedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let transaction = TypedTransaction::decode(rlp)?;
        Ok(Self { transaction, impersonated_sender: None })
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for MaybeImpersonatedTransaction {
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        self.transaction.encode(out)
    }
    fn length(&self) -> usize {
        self.transaction.length()
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for MaybeImpersonatedTransaction {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        Ok(Self { transaction: open_fastrlp::Decodable::decode(buf)?, impersonated_sender: None })
    }
}

impl AsRef<TypedTransaction> for MaybeImpersonatedTransaction {
    fn as_ref(&self) -> &TypedTransaction {
        &self.transaction
    }
}

impl Deref for MaybeImpersonatedTransaction {
    type Target = TypedTransaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypedTransaction {
    /// Legacy transaction type
    Legacy(LegacyTransaction),
}

// == impl TypedTransaction ==

impl TypedTransaction {
    pub fn energy_price(&self) -> U256 {
        match self {
            TypedTransaction::Legacy(tx) => tx.energy_price,
        }
    }

    pub fn energy_limit(&self) -> U256 {
        match self {
            TypedTransaction::Legacy(tx) => tx.energy_limit,
        }
    }

    pub fn value(&self) -> U256 {
        match self {
            TypedTransaction::Legacy(tx) => tx.value,
        }
    }

    pub fn data(&self) -> &Bytes {
        match self {
            TypedTransaction::Legacy(tx) => &tx.input,
        }
    }

    /// Returns the transaction type
    pub fn r#type(&self) -> Option<u8> {
        match self {
            TypedTransaction::Legacy(_) => None,
        }
    }

    /// Max cost of the transaction
    pub fn max_cost(&self) -> U256 {
        self.energy_limit().saturating_mul(self.energy_price())
    }

    /// Returns a helper type that contains commonly used values as fields
    pub fn essentials(&self) -> TransactionEssentials {
        match self {
            TypedTransaction::Legacy(t) => TransactionEssentials {
                kind: t.kind,
                input: t.input.clone(),
                nonce: t.nonce,
                energy_limit: t.energy_limit,
                energy_price: Some(t.energy_price),
                value: t.value,
                network_id: t.network_id(),
            },
        }
    }

    pub fn nonce(&self) -> &U256 {
        match self {
            TypedTransaction::Legacy(t) => t.nonce(),
        }
    }

    pub fn network_id(&self) -> u64 {
        match self {
            TypedTransaction::Legacy(t) => t.network_id(),
        }
    }

    pub fn as_legacy(&self) -> Option<&LegacyTransaction> {
        match self {
            TypedTransaction::Legacy(tx) => Some(tx),
        }
    }

    /// Returns true whether this tx is a legacy transaction
    pub fn is_legacy(&self) -> bool {
        matches!(self, TypedTransaction::Legacy(_))
    }

    /// Returns the hash of the transaction.
    ///
    /// Note: If this transaction has the Impersonated signature then this returns a modified unique
    /// hash. This allows us to treat impersonated transactions as unique.
    pub fn hash(&self) -> H256 {
        match self {
            TypedTransaction::Legacy(t) => t.hash(),
        }
    }

    /// Returns true if the transaction was impersonated (using the impersonate Signature)
    #[cfg(feature = "impersonated-tx")]
    pub fn is_impersonated(&self) -> bool {
        self.signature() == IMPERSONATED_SIGNATURE
    }

    /// Returns the hash if the transaction is impersonated (using a fake signature)
    ///
    /// This appends the `address` before hashing it
    #[cfg(feature = "impersonated-tx")]
    pub fn impersonated_hash(&self, sender: Address) -> H256 {
        let mut bytes = rlp::encode(self);
        bytes.extend_from_slice(sender.as_ref());
        H256::from_slice(sha3(&bytes).as_slice())
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    pub fn recover(&self) -> Result<Address, SignatureError> {
        match self {
            TypedTransaction::Legacy(tx) => tx.recover(),
        }
    }

    /// Returns what kind of transaction this is
    pub fn kind(&self) -> &TransactionKind {
        match self {
            TypedTransaction::Legacy(tx) => &tx.kind,
        }
    }

    /// Returns the callee if this transaction is a call
    pub fn to(&self) -> Option<&Address> {
        self.kind().as_call()
    }

    /// Returns the Signature of the transaction
    pub fn signature(&self) -> Signature {
        match self {
            TypedTransaction::Legacy(tx) => tx.signature,
        }
    }
}

impl Encodable for TypedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            TypedTransaction::Legacy(tx) => tx.rlp_append(s),
        }
    }
}

impl Decodable for TypedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(TypedTransaction::Legacy(rlp.as_val()?))
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for TypedTransaction {
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        match self {
            TypedTransaction::Legacy(tx) => tx.encode(out),
        }
    }
    fn length(&self) -> usize {
        match self {
            TypedTransaction::Legacy(tx) => tx.length(),
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for TypedTransaction {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        <LegacyTransaction as open_fastrlp::Decodable>::decode(buf).map(TypedTransaction::Legacy)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fastrlp", derive(open_fastrlp::RlpEncodable, open_fastrlp::RlpDecodable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegacyTransaction {
    pub nonce: U256,
    pub energy_price: U256,
    pub energy_limit: U256,
    pub network_id: u64,
    pub kind: TransactionKind,
    pub value: U256,
    pub input: Bytes,
    pub signature: Signature,
}

impl LegacyTransaction {
    pub fn nonce(&self) -> &U256 {
        &self.nonce
    }

    pub fn hash(&self) -> H256 {
        H256::from_slice(sha3(&rlp::encode(self)).as_slice())
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    pub fn recover(&self) -> Result<Address, SignatureError> {
        self.signature.recover(
            LegacyTransactionRequest::from(self.clone()).hash(),
            &Network::from(self.network_id),
        )
    }

    pub fn network_id(&self) -> u64 {
        self.network_id
    }

    /// See <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md>
    /// > If you do, then the v of the signature MUST be set to {0,1} + CHAIN_ID * 2 + 35 where
    /// > {0,1} is the parity of the y value of the curve point for which r is the x-value in the
    /// > secp256k1 signing process.
    pub fn meets_eip155(&self, _chain_id: u64) -> bool {
        true
        // let double_chain_id = chain_id.saturating_mul(2);
        // let v = self.signature.v;
        // v == double_chain_id + 35 || v == double_chain_id + 36;
    }
}

impl Encodable for LegacyTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(8);
        s.append(&self.nonce);
        s.append(&self.energy_price);
        s.append(&self.energy_limit);
        s.append(&self.network_id);
        s.append(&self.kind);
        s.append(&self.value);
        s.append(&self.input.as_ref());
        s.append(&self.signature.sig);
    }
}

impl Decodable for LegacyTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.item_count()? != 8 {
            return Err(DecoderError::RlpIncorrectListLen);
        }

        Ok(Self {
            nonce: rlp.val_at(0)?,
            energy_price: rlp.val_at(1)?,
            energy_limit: rlp.val_at(2)?,
            network_id: rlp.val_at(3)?,
            kind: rlp.val_at(4)?,
            value: rlp.val_at(5)?,
            input: rlp.val_at::<Vec<u8>>(6)?.into(),
            signature: Signature { sig: rlp.val_at(7)? },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionEssentials {
    pub kind: TransactionKind,
    pub input: Bytes,
    pub nonce: U256,
    pub energy_limit: U256,
    pub energy_price: Option<U256>,
    pub value: U256,
    pub network_id: u64,
}

/// Queued transaction
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingTransaction {
    /// The actual transaction
    pub transaction: MaybeImpersonatedTransaction,
    /// the recovered sender of this transaction
    sender: Address,
    /// hash of `transaction`, so it can easily be reused with encoding and hashing agan
    hash: TxHash,
}

// == impl PendingTransaction ==

impl PendingTransaction {
    /// Creates a new pending transaction and tries to verify transaction and recover sender.
    pub fn new(transaction: TypedTransaction) -> Result<Self, SignatureError> {
        let sender = transaction.recover()?;
        Ok(Self { hash: transaction.hash(), transaction: transaction.into(), sender })
    }

    /// Creates a new transaction with the given sender.
    ///
    /// In order to prevent collisions from multiple different impersonated accounts, we update the
    /// transaction's hash with the address to make it unique.
    ///
    /// See: <https://github.com/foxar-rs/foxar/issues/3759>
    #[cfg(feature = "impersonated-tx")]
    pub fn with_impersonated(transaction: TypedTransaction, sender: Address) -> Self {
        let hash = transaction.impersonated_hash(sender);
        let transaction = MaybeImpersonatedTransaction::impersonated(transaction, sender);
        Self { hash, transaction, sender }
    }

    pub fn nonce(&self) -> &U256 {
        self.transaction.nonce()
    }

    pub fn hash(&self) -> &TxHash {
        &self.hash
    }

    pub fn sender(&self) -> &Address {
        &self.sender
    }

    /// Converts the [PendingTransaction] into the [TxEnv] context that [`revm`](foxar_evm)
    /// expects.
    pub fn to_revm_tx_env(&self) -> TxEnv {
        fn transact_to(kind: &TransactionKind) -> TransactTo {
            match kind {
                TransactionKind::Call(c) => TransactTo::Call((*c).into()),
                TransactionKind::Create => TransactTo::Create(CreateScheme::Create),
            }
        }

        let caller = *self.sender();
        match &self.transaction.transaction {
            TypedTransaction::Legacy(tx) => {
                let LegacyTransaction {
                    nonce,
                    energy_price,
                    energy_limit,
                    value,
                    kind,
                    input,
                    network_id,
                    ..
                } = tx;
                TxEnv {
                    caller: caller.into(),
                    transact_to: transact_to(kind),
                    data: input.0.clone(),
                    network_id: Some(*network_id),
                    nonce: Some(nonce.as_u64()),
                    value: (*value).to_ruint(),
                    energy_price: (*energy_price).to_ruint(),
                    energy_limit: energy_limit.as_u64(),
                }
            }
        }
    }
}

/// Represents all relevant information of an executed transaction
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransactionInfo {
    pub transaction_hash: H256,
    pub transaction_index: u32,
    pub from: Address,
    pub to: Option<Address>,
    pub contract_address: Option<Address>,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
    pub traces: CallTraceArena,
    pub exit: InstructionResult,
    pub out: Option<Bytes>,
}

// === impl TransactionInfo ===

impl TransactionInfo {
    /// Returns the `traceAddress` of the node in the arena
    ///
    /// The `traceAddress` field of all returned traces, gives the exact location in the call trace
    /// [index in root, index in first CALL, index in second CALL, …].
    ///
    /// # Panics
    ///
    /// if the `idx` does not belong to a node
    pub fn trace_address(&self, idx: usize) -> Vec<usize> {
        if idx == 0 {
            // root call has empty traceAddress
            return vec![];
        }
        let mut graph = vec![];
        let mut node = &self.traces.arena[idx];
        while let Some(parent) = node.parent {
            // the index of the child call in the arena
            let child_idx = node.idx;
            node = &self.traces.arena[parent];
            // find the index of the child call in the parent node
            let call_idx = node
                .children
                .iter()
                .position(|child| *child == child_idx)
                .expect("child exists in parent");
            graph.push(call_idx);
        }
        graph.reverse();
        graph
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use corebc_core::utils::hex;

    #[test]
    fn can_recover_sender() {
        let bytes = hex::decode("f90171030a82c3500196cb08095e7baea6a6c7c4c2dfeb977efac326af552d870ab8a427dc297e800332e506f28f49a13c1edf087bdd6482d6cb3abdf2a4c455642aef1e98fc240000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002d7b22444149223a313439332e37342c2254555344223a313438392e36362c2255534443223a313439322e34387d00000000000000000000000000000000000000b8ab0fbac47922e6e0649343400231a15e26f4f5ab1490fa5e243470de6ca26fd3583b7fa03170600a37b29d214fa618a32d6c2a121552f556578097176bf2ccb9dee0f37e8547d8f5981b6b998f99bf24c92e08b61ca5a7da5ab3da43986881356af9ad55e9b9481432cb1194a7c1302bc72500ba277941fcb9ac8063a9b6ed64fbc86c51dd5ae6cf1f01f7bcf533cf0b0cfc5dc3fdc5bc7eaa99366ada5e7127331b862586a46c12a85f9580").unwrap();

        let tx: TypedTransaction = rlp::decode(&bytes).expect("decoding TypedTransaction failed");
        let tx = match tx {
            TypedTransaction::Legacy(tx) => tx,
        };

        assert_eq!(tx.input, Bytes::from_str("0x27dc297e800332e506f28f49a13c1edf087bdd6482d6cb3abdf2a4c455642aef1e98fc240000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002d7b22444149223a313439332e37342c2254555344223a313438392e36362c2255534443223a313439322e34387d00000000000000000000000000000000000000").unwrap());
        assert_eq!(tx.energy_price, U256::from(0xa));
        assert_eq!(tx.energy_limit, U256::from(0xc350));
        assert_eq!(tx.nonce, U256::from(0x3));
        if let TransactionKind::Call(ref to) = tx.kind {
            assert_eq!(*to, "cb08095e7baea6a6c7c4c2dfeb977efac326af552d87".parse().unwrap());
        } else {
            panic!();
        }
        assert_eq!(tx.value, U256::from(0xa));
        assert_eq!(
            tx.recover().unwrap(),
            "cb8238748ee459bc0c1d86eab1d3f6d83bb433cdad9c".parse().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn test_decode_fastrlp_create_goerli() {
        // test that an example create tx from goerli decodes properly
        let tx_bytes =
              hex::decode("f90171030a82c3500196cb08095e7baea6a6c7c4c2dfeb977efac326af552d870ab8a427dc297e800332e506f28f49a13c1edf087bdd6482d6cb3abdf2a4c455642aef1e98fc240000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002d7b22444149223a313439332e37342c2254555344223a313438392e36362c2255534443223a313439322e34387d00000000000000000000000000000000000000b8ab0fbac47922e6e0649343400231a15e26f4f5ab1490fa5e243470de6ca26fd3583b7fa03170600a37b29d214fa618a32d6c2a121552f556578097176bf2ccb9dee0f37e8547d8f5981b6b998f99bf24c92e08b61ca5a7da5ab3da43986881356af9ad55e9b9481432cb1194a7c1302bc72500ba277941fcb9ac8063a9b6ed64fbc86c51dd5ae6cf1f01f7bcf533cf0b0cfc5dc3fdc5bc7eaa99366ada5e7127331b862586a46c12a85f9580")
                  .unwrap();
        let _decoded =
            <TypedTransaction as open_fastrlp::Decodable>::decode(&mut &tx_bytes[..]).unwrap();
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn decode_transaction_consumes_buffer() {
        let bytes = &mut &hex::decode("f90171030a82c3500196cb08095e7baea6a6c7c4c2dfeb977efac326af552d870ab8a427dc297e800332e506f28f49a13c1edf087bdd6482d6cb3abdf2a4c455642aef1e98fc240000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002d7b22444149223a313439332e37342c2254555344223a313438392e36362c2255534443223a313439322e34387d00000000000000000000000000000000000000b8ab0fbac47922e6e0649343400231a15e26f4f5ab1490fa5e243470de6ca26fd3583b7fa03170600a37b29d214fa618a32d6c2a121552f556578097176bf2ccb9dee0f37e8547d8f5981b6b998f99bf24c92e08b61ca5a7da5ab3da43986881356af9ad55e9b9481432cb1194a7c1302bc72500ba277941fcb9ac8063a9b6ed64fbc86c51dd5ae6cf1f01f7bcf533cf0b0cfc5dc3fdc5bc7eaa99366ada5e7127331b862586a46c12a85f9580").unwrap()[..];
        let _transaction_res =
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes).unwrap();
        assert_eq!(
            bytes.len(),
            0,
            "did not consume all bytes in the buffer, {:?} remaining",
            bytes.len()
        );
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn decode_multiple_network_txs() {
        use std::str::FromStr;

        use open_fastrlp::Encodable;

        let bytes_first = &mut &hex::decode("f8d802843b9aca00830186a001960000c26ad91f4e7a0cad84c4b9315f420ca9217e315d87038d7ea4c6800080b8abeeb96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835b96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 2u64.into(),
            energy_price: 1000000000u64.into(),
            energy_limit: 100000u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("0000c26ad91f4e7a0cad84c4b9315f420ca9217e315d").unwrap()[..],
            )),
            value: 1000000000000000u64.into(),
            input: Bytes::default(),
            signature: Signature {
                sig: H1368::from_str("eeb96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835b96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835").unwrap()
            },
        });
        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_first).unwrap()
        );

        let bytes_second = &mut &hex::decode("f8d801843b9aca00830186a001960000d3e8763675e4c425df46cc3b5c0f6cbdac3960468702769bb01b2a0080b8abeeb96ca19e84444444444444444444444444444444444444444444444444444444444444896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835b96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 1u64.into(),
            energy_price: 1000000000u64.into(),
            energy_limit: 100000u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("0000d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 693361000000000u64.into(),
            input: Bytes::default(),
            signature: Signature {
                sig: H1368::from_str("eeb96ca19e84444444444444444444444444444444444444444444444444444444444444896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835b96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e83350220ca188350220ca1850220ca188350220ca1350220ca18835").unwrap()
            },
        });
        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_second).unwrap()
        );

        let bytes_third = &mut &hex::decode("f8d8038477359400839896800196cb44d3e8763675e4c425df46cc3b5c0f6cbdac39604687038d7ea4c6800080b8abeeb96ca19e84444444444444444444444444444444444444444444444444444444444444896b1b6055311536bf00a718568c744d8c1f9df59fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff35").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 3u64.into(),
            energy_price: 2000000000u64.into(),
            energy_limit: 10000000u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("cb44d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 1000000000000000u64.into(),
            input: Bytes::default(),
            signature: Signature {
                sig: H1368::from_str("eeb96ca19e84444444444444444444444444444444444444444444444444444444444444896b1b6055311536bf00a718568c744d8c1f9df59fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff35").unwrap(),
            },
        });

        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_third).unwrap()
        );

        let bytes_fifth = &mut &hex::decode("f8d20f84832156008287fb03960000cf7f9e66af820a19257a2108375b180b0ec491678204d280b8abeeb96ca19e84444444444444444444444444444444444444444444444444444444444444896b1b6055311536bf00a718568c744d8c1f9df59fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff35").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 15u64.into(),
            energy_price: 2200000000u64.into(),
            energy_limit: 34811u64.into(),
            network_id: 3,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("0000cf7f9e66af820a19257a2108375b180b0ec49167").unwrap()[..],
            )),
            value: 1234u64.into(),
            input: Bytes::default(),
            signature: Signature {
                sig: H1368::from_str("eeb96ca19e84444444444444444444444444444444444444444444444444444444444444896b1b6055311536bf00a718568c744d8c1f9df59fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff35").unwrap(),
            },
        });

        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_fifth).unwrap()
        );
    }

    // CORETODO: Add real data from go-core to test here
    // <https://github.com/gakonst/corebc-rs/issues/1732>
    #[test]
    fn test_recover_legacy_tx() {
        let raw_tx = "f90171030a82c3500196cb08095e7baea6a6c7c4c2dfeb977efac326af552d870ab8a427dc297e800332e506f28f49a13c1edf087bdd6482d6cb3abdf2a4c455642aef1e98fc240000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002d7b22444149223a313439332e37342c2254555344223a313438392e36362c2255534443223a313439322e34387d00000000000000000000000000000000000000b8ab0fbac47922e6e0649343400231a15e26f4f5ab1490fa5e243470de6ca26fd3583b7fa03170600a37b29d214fa618a32d6c2a121552f556578097176bf2ccb9dee0f37e8547d8f5981b6b998f99bf24c92e08b61ca5a7da5ab3da43986881356af9ad55e9b9481432cb1194a7c1302bc72500ba277941fcb9ac8063a9b6ed64fbc86c51dd5ae6cf1f01f7bcf533cf0b0cfc5dc3fdc5bc7eaa99366ada5e7127331b862586a46c12a85f9580";
        let tx: TypedTransaction = rlp::decode(&hex::decode(raw_tx).unwrap()).unwrap();
        let recovered = tx.recover().unwrap();
        let expected: Address = "cb8238748ee459bc0c1d86eab1d3f6d83bb433cdad9c".parse().unwrap();
        assert_eq!(expected, recovered);
    }
}
