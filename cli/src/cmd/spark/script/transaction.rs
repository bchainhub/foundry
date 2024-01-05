use crate::cmd::spark::script::{artifacts::ArtifactInfo, ScriptResult};
use corebc::{
    abi,
    abi::Address,
    prelude::{NameOrAddress, H256 as TxHash},
    types::{transaction::eip2718::TypedTransaction, Network, H160},
    utils::to_ican,
};
use eyre::{ContextCompat, WrapErr};
use foxar_common::{abi::format_token_raw, RpcUrl, SELECTOR_LEN};
use probe::{executor::inspector::DEFAULT_CREATE2_DEPLOYER, trace::CallTraceDecoder, CallKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::error;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalContract {
    #[serde(rename = "transactionType")]
    pub opcode: CallKind,
    #[serde(serialize_with = "wrapper::serialize_addr")]
    pub address: Address,
    #[serde(with = "hex")]
    pub init_code: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionWithMetadata {
    pub hash: Option<TxHash>,
    #[serde(rename = "transactionType")]
    pub opcode: CallKind,
    #[serde(default = "default_string")]
    pub contract_name: Option<String>,
    #[serde(default = "default_address", serialize_with = "wrapper::serialize_opt_addr")]
    pub contract_address: Option<Address>,
    #[serde(default = "default_string")]
    pub function: Option<String>,
    #[serde(default = "default_vec_of_strings")]
    pub arguments: Option<Vec<String>>,
    #[serde(skip)]
    pub rpc: Option<RpcUrl>,
    pub transaction: TypedTransaction,
    pub additional_contracts: Vec<AdditionalContract>,
    pub is_fixed_energy_limit: bool,
}

fn default_string() -> Option<String> {
    Some("".to_string())
}

fn default_address() -> Option<Address> {
    Some(Address::zero())
}

fn default_vec_of_strings() -> Option<Vec<String>> {
    Some(vec![])
}

impl TransactionWithMetadata {
    pub fn from_typed_transaction(transaction: TypedTransaction) -> Self {
        Self { transaction, ..Default::default() }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: TypedTransaction,
        rpc: Option<RpcUrl>,
        result: &ScriptResult,
        local_contracts: &BTreeMap<Address, ArtifactInfo>,
        decoder: &CallTraceDecoder,
        additional_contracts: Vec<AdditionalContract>,
        is_fixed_energy_limit: bool,
        network: &Network,
    ) -> eyre::Result<Self> {
        let mut metadata = Self { transaction, rpc, is_fixed_energy_limit, ..Default::default() };

        // Specify if any contract was directly created with this transaction
        if let Some(NameOrAddress::Address(to)) = metadata.transaction.to().cloned() {
            if to == DEFAULT_CREATE2_DEPLOYER {
                let address = to_ican(&H160::from_slice(&result.returned), network);
                metadata.set_create(true, address, local_contracts, decoder)?;
            } else {
                metadata
                    .set_call(to, local_contracts, decoder)
                    .wrap_err("Could not decode transaction type.")?;
            }
        } else if metadata.transaction.to().is_none() {
            metadata.set_create(
                false,
                result.address.wrap_err("There should be a contract address from CREATE.")?,
                local_contracts,
                decoder,
            )?;
        }

        // Add the additional contracts created in this transaction, so we can verify them later.
        if let Some(tx_address) = metadata.contract_address {
            metadata.additional_contracts = additional_contracts
                .into_iter()
                .filter_map(|contract| {
                    // Filter out the transaction contract repeated init_code.
                    if contract.address != tx_address {
                        Some(contract)
                    } else {
                        None
                    }
                })
                .collect();
        }

        Ok(metadata)
    }

    /// Populate the transaction as CREATE tx
    ///
    /// If this is a CREATE2 transaction this attempt to decode the arguments from the CREATE2
    /// deployer's function
    fn set_create(
        &mut self,
        is_create2: bool,
        address: Address,
        contracts: &BTreeMap<Address, ArtifactInfo>,
        decoder: &CallTraceDecoder,
    ) -> eyre::Result<()> {
        if is_create2 {
            self.opcode = CallKind::Create2;
        } else {
            self.opcode = CallKind::Create;
        }

        self.contract_name = contracts.get(&address).map(|info| info.contract_name.clone());
        self.contract_address = Some(address);

        if let Some(data) = self.transaction.data() {
            // a CREATE2 transaction is a CALL to the CREATE2 deployer function, so we need to
            // decode the arguments  from the function call
            if is_create2 {
                // this will be the `deploy()` function of the CREATE2 call, we assume the contract
                // is already deployed and will try to fetch it via its signature
                if let Some(Some(function)) = decoder
                    .functions
                    .get(&data.0[..SELECTOR_LEN])
                    .map(|functions| functions.first())
                {
                    self.function = Some(function.signature());
                    self.arguments = Some(
                        function
                            .decode_input(&data.0[SELECTOR_LEN..])
                            .map(|tokens| tokens.iter().map(format_token_raw).collect())
                            .map_err(|_| eyre::eyre!("Failed to decode CREATE2 call arguments"))?,
                    );
                }
            } else {
                // This is a regular CREATE via the constructor, in which case we expect the
                // contract has a constructor and if so decode its input params
                if let Some(info) = contracts.get(&address) {
                    // set constructor arguments
                    if data.len() > info.code.len() {
                        if let Some(constructor) = info.abi.constructor() {
                            let on_err = || {
                                let inputs = constructor
                                    .inputs
                                    .iter()
                                    .map(|p| p.kind.to_string())
                                    .collect::<Vec<_>>()
                                    .join(",");
                                let signature = format!("constructor({inputs})");
                                let bytecode = hex::encode(&data.0);
                                (signature, bytecode)
                            };

                            let params = constructor
                                .inputs
                                .iter()
                                .map(|p| p.kind.clone())
                                .collect::<Vec<_>>();

                            // the constructor args start after bytecode
                            let constructor_args = &data.0[info.code.len()..];

                            if let Ok(arguments) = abi::decode(&params, constructor_args) {
                                self.arguments =
                                    Some(arguments.iter().map(format_token_raw).collect());
                            } else {
                                let (signature, bytecode) = on_err();
                                error!(constructor=?signature, contract=?self.contract_name, bytecode, "Failed to decode constructor arguments")
                            };
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Populate the transaction as CALL tx
    fn set_call(
        &mut self,
        target: Address,
        local_contracts: &BTreeMap<Address, ArtifactInfo>,
        decoder: &CallTraceDecoder,
    ) -> eyre::Result<()> {
        self.opcode = CallKind::Call;

        if let Some(data) = self.transaction.data() {
            if data.0.len() >= SELECTOR_LEN {
                if let Some(info) = local_contracts.get(&target) {
                    // This CALL is made to a local contract.

                    self.contract_name = Some(info.contract_name.clone());
                    if let Some(function) = info
                        .abi
                        .functions()
                        .find(|function| function.short_signature() == data.0[..SELECTOR_LEN])
                    {
                        self.function = Some(function.signature());
                        self.arguments = Some(
                            function
                                .decode_input(&data.0[SELECTOR_LEN..])
                                .map(|tokens| tokens.iter().map(format_token_raw).collect())?,
                        );
                    }
                } else {
                    // This CALL is made to an external contract. We can only decode it, if it has
                    // been verified and identified by etherscan.

                    if let Some(Some(function)) = decoder
                        .functions
                        .get(&data.0[..SELECTOR_LEN])
                        .map(|functions| functions.first())
                    {
                        self.contract_name = decoder.contracts.get(&target).cloned();

                        self.function = Some(function.signature());
                        self.arguments = Some(
                            function
                                .decode_input(&data.0[SELECTOR_LEN..])
                                .map(|tokens| tokens.iter().map(format_token_raw).collect())?,
                        );
                    }
                }
                self.contract_address = Some(target);
            }
        }
        Ok(())
    }

    pub fn set_tx(&mut self, tx: TypedTransaction) {
        self.transaction = tx;
    }

    pub fn typed_tx(&self) -> &TypedTransaction {
        &self.transaction
    }

    pub fn typed_tx_mut(&mut self) -> &mut TypedTransaction {
        &mut self.transaction
    }

    pub fn is_create2(&self) -> bool {
        self.opcode == CallKind::Create2
    }
}

// wrapper for modifying ethers-rs type serialization
pub mod wrapper {
    pub use super::*;

    use corebc::{
        types::{Bloom, Bytes, Log, TransactionReceipt, H256, U256, U64},
        utils::to_checksum,
    };

    pub fn serialize_addr<S>(addr: &Address, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&to_checksum(addr, None))
    }

    pub fn serialize_opt_addr<S>(opt: &Option<Address>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match opt {
            Some(addr) => serialize_addr(addr, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn serialize_vec_with_wrapped<S, T, WrappedType>(
        vec: &[T],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: Clone,
        WrappedType: serde::Serialize + From<T>,
    {
        serializer.collect_seq(vec.iter().cloned().map(WrappedType::from))
    }

    // copied from https://github.com/gakonst/ethers-rs
    #[derive(Serialize, Deserialize)]
    struct WrappedLog {
        /// H160. the contract that emitted the log
        #[serde(serialize_with = "serialize_addr")]
        pub address: Address,

        /// topics: Array of 0 to 4 32 Bytes of indexed log arguments.
        /// (In solidity: The first topic is the hash of the signature of the event
        /// (e.g. `Deposit(address,bytes32,uint256)`), except you declared the event
        /// with the anonymous specifier.)
        pub topics: Vec<H256>,

        /// Data
        pub data: Bytes,

        /// Block Hash
        #[serde(rename = "blockHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_hash: Option<H256>,

        /// Block Number
        #[serde(rename = "blockNumber")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_number: Option<U64>,

        /// Transaction Hash
        #[serde(rename = "transactionHash")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<H256>,

        /// Transaction Index
        #[serde(rename = "transactionIndex")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_index: Option<U64>,

        /// Integer of the log index position in the block. None if it's a pending log.
        #[serde(rename = "logIndex")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub log_index: Option<U256>,

        /// Integer of the transactions index position log was created from.
        /// None when it's a pending log.
        #[serde(rename = "transactionLogIndex")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_log_index: Option<U256>,

        /// Log Type
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub log_type: Option<String>,

        /// True when the log was removed, due to a chain reorganization.
        /// false if it's a valid log.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub removed: Option<bool>,
    }
    impl From<Log> for WrappedLog {
        fn from(log: Log) -> Self {
            Self {
                address: log.address,
                topics: log.topics,
                data: log.data,
                block_hash: log.block_hash,
                block_number: log.block_number,
                transaction_hash: log.transaction_hash,
                transaction_index: log.transaction_index,
                log_index: log.log_index,
                transaction_log_index: log.transaction_log_index,
                log_type: log.log_type,
                removed: log.removed,
            }
        }
    }

    fn serialize_logs<S: serde::Serializer>(
        logs: &[Log],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_vec_with_wrapped::<S, Log, WrappedLog>(logs, serializer)
    }

    // "Receipt" of an executed transaction: details of its execution.
    // copied from https://github.com/gakonst/ethers-rs
    #[derive(Default, Clone, Serialize, Deserialize)]
    pub struct WrappedTransactionReceipt {
        /// Transaction hash.
        #[serde(rename = "transactionHash")]
        pub transaction_hash: H256,
        /// Index within the block.
        #[serde(rename = "transactionIndex")]
        pub transaction_index: U64,
        /// Hash of the block this transaction was included within.
        #[serde(rename = "blockHash")]
        pub block_hash: Option<H256>,
        /// Number of the block this transaction was included within.
        #[serde(rename = "blockNumber")]
        pub block_number: Option<U64>,
        /// The address of the sender.
        #[serde(serialize_with = "serialize_addr")]
        pub from: Address,
        // The address of the receiver. `None` when its a contract creation transaction.
        #[serde(serialize_with = "serialize_opt_addr")]
        pub to: Option<Address>,
        /// Cumulative energy used within the block after this was executed.
        #[serde(rename = "cumulativeEnergyUsed")]
        pub cumulative_energy_used: U256,
        /// Energy used by this transaction alone.
        ///
        /// Energy used is `None` if the the client is running in light client mode.
        #[serde(rename = "energyUsed")]
        pub energy_used: Option<U256>,
        /// Contract address created, or `None` if not a deployment.
        #[serde(rename = "contractAddress", serialize_with = "serialize_opt_addr")]
        pub contract_address: Option<Address>,
        /// Logs generated within this transaction.
        #[serde(serialize_with = "serialize_logs")]
        pub logs: Vec<Log>,
        /// Status: either 1 (success) or 0 (failure). Only present after activation of [EIP-658](https://eips.ethereum.org/EIPS/eip-658)
        pub status: Option<U64>,
        /// State root. Only present before activation of [EIP-658](https://eips.ethereum.org/EIPS/eip-658)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root: Option<H256>,
        /// Logs bloom
        #[serde(rename = "logsBloom")]
        pub logs_bloom: Bloom,
    }
    impl From<TransactionReceipt> for WrappedTransactionReceipt {
        fn from(receipt: TransactionReceipt) -> Self {
            Self {
                transaction_hash: receipt.transaction_hash,
                transaction_index: receipt.transaction_index,
                block_hash: receipt.block_hash,
                block_number: receipt.block_number,
                from: receipt.from,
                to: receipt.to,
                cumulative_energy_used: receipt.cumulative_energy_used,
                energy_used: receipt.energy_used,
                contract_address: receipt.contract_address,
                logs: receipt.logs,
                status: receipt.status,
                root: receipt.root,
                logs_bloom: receipt.logs_bloom,
            }
        }
    }

    pub fn serialize_receipts<S: serde::Serializer>(
        receipts: &[TransactionReceipt],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_vec_with_wrapped::<S, TransactionReceipt, WrappedTransactionReceipt>(
            receipts, serializer,
        )
    }
}
