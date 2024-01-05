//! Cast
//!
//! Contains core function implementation for `probe`

use crate::rlp_converter::Item;
use base::{Base, NumberWithBase, ToBase};
use chrono::NaiveDateTime;
use corebc_blockindex::{errors::BlockindexError, Client};
use corebc_core::{
    abi::{
        token::{LenientTokenizer, Tokenizer},
        Function, HumanReadableParser, ParamType, RawAbi, /* RawAbi, */ Token,
    },
    types::{Network, *},
    utils::{
        format_bytes32_string, format_units, get_contract_address, parse_bytes32_string,
        parse_units, rlp, sha3, Units,
    },
};
use corebc_providers::{Middleware, PendingTransaction};
use evm_disassembler::{disassemble_bytes, disassemble_str, format_operations};
use eyre::{Context, Result};
use foxar_common::{abi::encode_args, fmt::*, TransactionReceiptWithRevertReason};
pub use foxar_evm::*;
use rayon::prelude::*;
pub use rusoto_core::{
    credential::ChainProvider as AwsChainProvider, region::Region as AwsRegion,
    request::HttpClient as AwsHttpClient, Client as AwsClient,
};
pub use rusoto_kms::KmsClient;
use rustc_hex::{FromHexIter, ToHex};
use std::{
    path::PathBuf,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};
pub use tx::TxBuilder;
use tx::{TxBuilderOutput, TxBuilderPeekOutput};

pub mod base;
pub mod errors;
mod rlp_converter;
mod tx;

// TODO: CastContract with common contract initializers? Same for CastProviders?

pub struct Cast<M> {
    provider: M,
}

impl<M: Middleware> Cast<M>
where
    M::Error: 'static,
{
    /// Creates a new Cast instance from the provided client
    ///
    /// # Example
    ///
    /// ```
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(provider: M) -> Self {
        Self { provider }
    }

    /// Makes a read-only call to the specified address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::{Cast, TxBuilder};
    /// use corebc_core::types::{Address, Network};
    /// use corebc_providers::{Provider, Http};
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let to = Address::from_str("0xB3C95ff08316fb2F2e3E52Ee82F8e7b605Aa1304")?;
    /// let sig = "function greeting(uint256 i) public returns (string)";
    /// let args = vec!["5".to_owned()];
    /// let mut builder = TxBuilder::new(&provider, Address::zero(), Some(to), Network::Mainnet).await?;
    /// builder
    ///     .set_args(sig, args).await?;
    /// let builder_output = builder.build();
    /// let probe = Cast::new(provider);
    /// let data = probe.call(builder_output, None).await?;
    /// println!("{}", data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call<'a>(
        &self,
        builder_output: TxBuilderOutput,
        block: Option<BlockId>,
    ) -> Result<String> {
        let (tx, func) = builder_output;
        let res = self.provider.call(&tx, block).await?;

        let mut decoded = vec![];

        if let Some(func) = func {
            // decode args into tokens
            decoded = match func.decode_output(res.as_ref()) {
                Ok(decoded) => decoded,
                Err(err) => {
                    // ensure the address is a contract
                    if res.is_empty() {
                        // check that the recipient is a contract that can be called
                        if let Some(NameOrAddress::Address(addr)) = tx.to() {
                            let code = self.provider.get_code(*addr, block).await?;
                            if code.is_empty() {
                                eyre::bail!("Contract {:?} does not exist", addr)
                            }
                        }
                    }
                    return Err(err).wrap_err(
                        "could not decode output. did you specify the wrong function return data type perhaps?"
                    );
                }
            };
        }
        // handle case when return type is not specified
        Ok(if decoded.is_empty() {
            format!("{res}\n")
        } else {
            // seth compatible user-friendly return type conversions
            decoded
                .iter()
                .map(TokenDisplay)
                .map(|token| token.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        })
    }

    pub async fn balance<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        who: T,
        block: Option<BlockId>,
    ) -> Result<U256> {
        Ok(self.provider.get_balance(who, block).await?)
    }

    /// Sends a transaction to the specified address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::{Cast, TxBuilder};
    /// use corebc_core::types::{Address, Network, U256};
    /// use corebc_providers::{Provider, Http};
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let from = "vitalik.eth";
    /// let to = Address::from_str("0x0000B3C95ff08316fb2F2e3E52Ee82F8e7b605Aa1304")?;
    /// let sig = "greet(string)()";
    /// let args = vec!["hello".to_owned()];
    /// let gas = U256::from_str("200000").unwrap();
    /// let value = U256::from_str("1").unwrap();
    /// let nonce = U256::from_str("1").unwrap();
    /// let mut builder = TxBuilder::new(&provider, Address::zero(), Some(to), Network::Mainnet).await?;
    /// builder
    ///     .set_args(sig, args).await?
    ///     .set_energy(gas)
    ///     .set_value(value)
    ///     .set_nonce(nonce);
    /// let builder_output = builder.build();
    /// let probe = Cast::new(provider);
    /// let data = probe.send(builder_output).await?;
    /// println!("{}", *data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send<'a>(
        &self,
        builder_output: TxBuilderOutput,
    ) -> Result<PendingTransaction<'_, M::Provider>> {
        let (tx, _) = builder_output;
        let res = self.provider.send_transaction(tx, None).await?;

        Ok::<_, eyre::Error>(res)
    }

    /// Publishes a raw transaction to the network
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let res = probe.publish("0x1234".to_string()).await?;
    /// println!("{:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn publish(&self, mut raw_tx: String) -> Result<PendingTransaction<'_, M::Provider>> {
        raw_tx = match raw_tx.strip_prefix("0x") {
            Some(s) => s.to_string(),
            None => raw_tx,
        };
        let tx = Bytes::from(hex::decode(raw_tx)?);
        let res = self.provider.send_raw_transaction(tx).await?;

        Ok::<_, eyre::Error>(res)
    }

    /// Estimates the gas cost of a transaction
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::{Cast, TxBuilder};
    /// use corebc_core::types::{Address, Network, U256};
    /// use corebc_providers::{Provider, Http};
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let from = "vitalik.eth";
    /// let to = Address::from_str("0xB3C95ff08316fb2F2e3E52Ee82F8e7b605Aa1304")?;
    /// let sig = "greet(string)()";
    /// let args = vec!["5".to_owned()];
    /// let value = U256::from_str("1").unwrap();
    /// let mut builder = TxBuilder::new(&provider, from, Some(to), Network::Mainnet).await?;
    /// builder
    ///     .set_value(value)
    ///     .set_args(sig, args).await?;
    /// let builder_output = builder.peek();
    /// let probe = Cast::new(&provider);
    /// let data = probe.estimate(builder_output).await?;
    /// println!("{}", data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn estimate(&self, builder_output: TxBuilderPeekOutput<'_>) -> Result<U256> {
        let (tx, _) = builder_output;

        let res = self.provider.estimate_energy(tx, None).await?;

        Ok::<_, eyre::Error>(res)
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let block = probe.block(5, true, None, false).await?;
    /// println!("{}", block);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn block<T: Into<BlockId>>(
        &self,
        block: T,
        full: bool,
        field: Option<String>,
        to_json: bool,
    ) -> Result<String> {
        let block = block.into();
        let block = if full {
            let block = self
                .provider
                .get_block_with_txs(block)
                .await?
                .ok_or_else(|| eyre::eyre!("block {:?} not found", block))?;
            if let Some(ref field) = field {
                get_pretty_block_attr(&block, field)
                    .unwrap_or_else(|| format!("{field} is not a valid block field"))
            } else if to_json {
                serde_json::to_value(&block).unwrap().to_string()
            } else {
                block.pretty()
            }
        } else {
            let block = self
                .provider
                .get_block(block)
                .await?
                .ok_or_else(|| eyre::eyre!("block {:?} not found", block))?;

            if let Some(ref field) = field {
                if field == "transactions" {
                    "use --full to view transactions".to_string()
                } else {
                    get_pretty_block_attr(&block, field)
                        .unwrap_or_else(|| format!("{field} is not a valid block field"))
                }
            } else if to_json {
                serde_json::to_value(&block).unwrap().to_string()
            } else {
                block.pretty()
            }
        };

        Ok(block)
    }

    async fn block_field_as_num<T: Into<BlockId>>(&self, block: T, field: String) -> Result<U256> {
        let block = block.into();
        let block_field = Cast::block(
            self,
            block,
            false,
            // Select only select field
            Some(field),
            false,
        )
        .await?;

        let ret = if block_field.starts_with("0x") {
            U256::from_str_radix(strip_0x(&block_field), 16).expect("Unable to convert hex to U256")
        } else {
            U256::from_str_radix(&block_field, 10).expect("Unable to convert decimal to U256")
        };
        Ok(ret)
    }

    pub async fn base_fee<T: Into<BlockId>>(&self, block: T) -> Result<U256> {
        Cast::block_field_as_num(self, block, String::from("baseFeePerGas")).await
    }

    pub async fn age<T: Into<BlockId>>(&self, block: T) -> Result<String> {
        let timestamp_str =
            Cast::block_field_as_num(self, block, String::from("timestamp")).await?.to_string();
        let datetime =
            NaiveDateTime::from_timestamp_opt(timestamp_str.parse::<i64>().unwrap(), 0).unwrap();
        Ok(datetime.format("%a %b %e %H:%M:%S %Y").to_string())
    }

    pub async fn timestamp<T: Into<BlockId>>(&self, block: T) -> Result<U256> {
        Cast::block_field_as_num(self, block, "timestamp".to_string()).await
    }

    pub async fn network(&self) -> Result<&str> {
        let genesis_hash = Cast::block(
            self,
            0,
            false,
            // Select only block hash
            Some(String::from("hash")),
            false,
        )
        .await?;

        Ok(match &genesis_hash[..] {
            "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3" => {
                match &(Cast::block(self, 1920000, false, Some("hash".to_string()), false).await?)[..]
                {
                    "0x94365e3a8c0b35089c1d1195081fe7489b528a84b22199c916180db8b28ade7f" => {
                        "etclive"
                    }
                    _ => "ethlive",
                }
            }
            "0xa3c565fc15c7478862d50ccd6561e3c06b24cc509bf388941c25ea985ce32cb9" => "kovan",
            "0x41941023680923e0fe4d74a34bdac8141f2540e3ae90623718e47d66d1ca4a2d" => "ropsten",
            "0x7ca38a1916c42007829c55e69d3e9a73265554b586a499015373241b8a3fa48b" => {
                "optimism-mainnet"
            }
            "0xc1fc15cd51159b1f1e5cbc4b82e85c1447ddfa33c52cf1d98d14fba0d6354be1" => {
                "optimism-goerli"
            }
            "0x02adc9b449ff5f2467b8c674ece7ff9b21319d76c4ad62a67a70d552655927e5" => {
                "optimism-kovan"
            }
            "0x7ee576b35482195fc49205cec9af72ce14f003b9ae69f6ba0faef4514be8b442" => {
                "arbitrum-mainnet"
            }
            "0x0cd786a2425d16f152c658316c423e6ce1181e15c3295826d7c9904cba9ce303" => "morden",
            "0x6341fd3daf94b748c72ced5a5b26028f2474f5f00d824504e4fa37a75767e177" => "rinkeby",
            "0xbf7e331f7f7c1dd2e05159666b3bf8bc7a8a3a9eb1d518969eab529dd9b88c1a" => "goerli",
            "0x14c2283285a88fe5fce9bf5c573ab03d6616695d717b12a127188bcacfc743c4" => "kotti",
            "0xa9c28ce2141b56c474f1dc504bee9b01eb1bd7d1a507580d5519d4437a97de1b" => "polygon",
            "0x7b66506a9ebdbf30d32b43c5f15a3b1216269a1ec3a75aa3182b86176a2b1ca7" => {
                "polygon-mumbai"
            }
            "0x4f1dd23188aab3a76b463e4af801b52b1248ef073c648cbdc4c9333d3da79756" => "gnosis",
            "0xada44fd8d2ecab8b08f256af07ad3e777f17fb434f8f8e678b312f576212ba9a" => "chiado",
            "0x6d3c66c5357ec91d5c43af47e234a939b22557cbb552dc45bebbceeed90fbe34" => "bsctest",
            "0x0d21840abff46b96c84b2ac9e10e4f5cdaeb5693cb665db62a2f3b02d2d57b5b" => "bsc",
            "0x31ced5b9beb7f8782b014660da0cb18cc409f121f408186886e1ca3e8eeca96b" => {
                match &(Cast::block(self, 1, false, Some(String::from("hash")), false).await?)[..] {
                    "0x738639479dc82d199365626f90caa82f7eafcfe9ed354b456fb3d294597ceb53" => {
                        "avalanche-fuji"
                    }
                    _ => "avalanche",
                }
            }
            _ => "unknown",
        })
    }

    pub async fn network_id(&self) -> Result<U256> {
        Ok(self.provider.get_networkid().await?)
    }

    pub async fn block_number(&self) -> Result<U64> {
        Ok(self.provider.get_block_number().await?)
    }

    pub async fn energy_price(&self) -> Result<U256> {
        Ok(self.provider.get_energy_price().await?)
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::Address;
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x7eD52863829AB99354F3a0503A622e82AcD5F7d3")?;
    /// let nonce = probe.nonce(addr, None).await?;
    /// println!("{}", nonce);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn nonce<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        who: T,
        block: Option<BlockId>,
    ) -> Result<U256> {
        Ok(self.provider.get_transaction_count(who, block).await?)
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::Address;
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x7eD52863829AB99354F3a0503A622e82AcD5F7d3")?;
    /// let implementation = probe.implementation(addr, None).await?;
    /// println!("{}", implementation);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn implementation<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        who: T,
        block: Option<BlockId>,
    ) -> Result<String> {
        let slot =
            H256::from_str("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")?;
        let value = self.provider.get_storage_at(who, slot, block).await?;
        let addr: H176 = value.into();
        Ok(format!("{addr:?}"))
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::Address;
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x7eD52863829AB99354F3a0503A622e82AcD5F7d3")?;
    /// let admin = probe.admin(addr, None).await?;
    /// println!("{}", admin);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn admin<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        who: T,
        block: Option<BlockId>,
    ) -> Result<String> {
        let slot =
            H256::from_str("0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103")?;
        let value = self.provider.get_storage_at(who, slot, block).await?;
        let addr: H176 = value.into();
        Ok(format!("{addr:?}"))
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::{Address, Network};
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x7eD52863829AB99354F3a0503A622e82AcD5F7d3")?;
    /// let nonce = probe.nonce(addr, None).await? + 5;
    /// let computed_address = probe.compute_address(addr, Some(nonce), &Network::Mainnet).await?;
    /// println!("Computed address for address {} with nonce {}: {}", addr, nonce, computed_address);
    /// let computed_address_no_nonce = probe.compute_address(addr, None, &Network::Mainnet).await?;
    /// println!("Computed address for address {} with nonce {}: {}", addr, nonce, computed_address_no_nonce);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compute_address<T: Into<Address> + Copy + Send + Sync>(
        &self,
        address: T,
        nonce: Option<U256>,
        network: &Network,
    ) -> Result<Address> {
        let unpacked = if let Some(n) = nonce {
            n
        } else {
            self.provider.get_transaction_count(address.into(), None).await?
        };

        Ok(get_contract_address(address, unpacked, network))
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::Address;
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa")?;
    /// let code = probe.code(addr, None, false).await?;
    /// println!("{}", code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn code<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        who: T,
        block: Option<BlockId>,
        disassemble: bool,
    ) -> Result<String> {
        if disassemble {
            let code = self.provider.get_code(who, block).await?.to_vec();
            Ok(format_operations(disassemble_bytes(code)?)?)
        } else {
            Ok(format!("{}", self.provider.get_code(who, block).await?))
        }
    }

    /// Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::Address;
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa")?;
    /// let codesize = probe.codesize(addr, None).await?;
    /// println!("{}", codesize);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn codesize<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        who: T,
        block: Option<BlockId>,
    ) -> Result<String> {
        let code = self.provider.get_code(who, block).await?.to_vec();
        Ok(format!("{}", code.len()))
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let tx_hash = "0xf8d1713ea15a81482958fb7ddf884baee8d3bcc478c5f2f604e008dc788ee4fc";
    /// let tx = probe.transaction(tx_hash.to_string(), None, false).await?;
    /// println!("{}", tx);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn transaction(
        &self,
        tx_hash: String,
        field: Option<String>,
        to_json: bool,
    ) -> Result<String> {
        let tx_hash = H256::from_str(&tx_hash).wrap_err("invalid tx hash")?;
        let tx = self
            .provider
            .get_transaction(tx_hash)
            .await?
            .ok_or_else(|| eyre::eyre!("tx not found: {:?}", tx_hash))?;

        Ok(if let Some(ref field) = field {
            get_pretty_tx_attr(&tx, field)
                .ok_or_else(|| eyre::eyre!("invalid tx field: {}", field))?
        } else if to_json {
            // to_value first to sort json object keys
            serde_json::to_value(&tx)?.to_string()
        } else {
            tx.pretty()
        })
    }

    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let tx_hash = "0xf8d1713ea15a81482958fb7ddf884baee8d3bcc478c5f2f604e008dc788ee4fc";
    /// let receipt = probe.receipt(tx_hash.to_string(), None, 1, false, false).await?;
    /// println!("{}", receipt);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn receipt(
        &self,
        tx_hash: String,
        field: Option<String>,
        confs: usize,
        probe_async: bool,
        to_json: bool,
    ) -> Result<String> {
        let tx_hash = H256::from_str(&tx_hash).wrap_err("invalid tx hash")?;

        let mut receipt: TransactionReceiptWithRevertReason =
            match self.provider.get_transaction_receipt(tx_hash).await? {
                Some(r) => r,
                None => {
                    // if the async flag is provided, immediately exit if no tx is found, otherwise
                    // try to poll for it
                    if probe_async {
                        eyre::bail!("tx not found: {:?}", tx_hash)
                    } else {
                        let tx = PendingTransaction::new(tx_hash, self.provider.provider());
                        tx.confirmations(confs).await?.ok_or_else(|| {
                            eyre::eyre!(
                                "tx not found, might have been dropped from mempool: {:?}",
                                tx_hash
                            )
                        })?
                    }
                }
            }
            .into();

        // Allow to fail silently
        let _ = receipt.update_revert_reason(&self.provider).await;

        Ok(if let Some(ref field) = field {
            get_pretty_tx_receipt_attr(&receipt, field)
                .ok_or_else(|| eyre::eyre!("invalid receipt field: {}", field))?
        } else if to_json {
            // to_value first to sort json object keys
            serde_json::to_value(&receipt)?.to_string()
        } else {
            receipt.pretty()
        })
    }

    /// Perform a raw JSON-RPC request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let result = probe.rpc("eth_getBalance", &["0xc94770007dda54cF92009BFF0dE90c06F603a09f", "latest"])
    ///     .await?;
    /// println!("{}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rpc<T>(&self, method: &str, params: T) -> Result<String>
    where
        T: std::fmt::Debug + serde::Serialize + Send + Sync,
    {
        let res = self.provider.provider().request::<T, serde_json::Value>(method, params).await?;
        Ok(serde_json::to_string(&res)?)
    }

    /// Returns the slot
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::Cast;
    /// use corebc_providers::{Provider, Http};
    /// use corebc_core::types::{Address, H256};
    /// use std::{str::FromStr, convert::TryFrom};
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    /// let probe = Cast::new(provider);
    /// let addr = Address::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581")?;
    /// let slot = H256::zero();
    /// let storage = probe.storage(addr, slot, None).await?;
    /// println!("{}", storage);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn storage<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        slot: H256,
        block: Option<BlockId>,
    ) -> Result<String> {
        Ok(format!("{:?}", self.provider.get_storage_at(from, slot, block).await?))
    }

    pub async fn filter_logs(&self, filter: Filter, to_json: bool) -> Result<String> {
        let logs = self.provider.get_logs(&filter).await?;

        let res = if to_json {
            serde_json::to_string(&logs)?
        } else {
            let mut s = vec![];
            for log in logs {
                let pretty = log
                    .pretty()
                    .replacen('\n', "- ", 1) // Remove empty first line
                    .replace('\n', "\n  "); // Indent
                s.push(pretty);
            }
            s.join("\n")
        };
        Ok(res)
    }
}

pub struct InterfaceSource {
    pub name: String,
    pub source: String,
}

// Local is a path to the directory containing the ABI files
// In case of etherscan, ABI is fetched from the address on the chain
pub enum AbiPath {
    Local { path: String, name: Option<String> },
    Etherscan { address: Address, network: Network /* api_key: String */ },
}

pub struct SimpleCast;

impl SimpleCast {
    /// Returns the maximum value of the given integer type
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast;
    /// # use corebc_core::types::{I256, U256};
    /// assert_eq!(SimpleCast::max_int("uint256")?, format!("{}", U256::MAX));
    /// assert_eq!(SimpleCast::max_int("int256")?, format!("{}", I256::MAX));
    /// assert_eq!(SimpleCast::max_int("int32")?, format!("{}", i32::MAX));
    /// # Ok::<(), eyre::Report>(())
    /// ```
    pub fn max_int(s: &str) -> Result<String> {
        Self::max_min_int::<true>(s)
    }

    /// Returns the maximum value of the given integer type
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast;
    /// # use corebc_core::types::{I256, U256};
    /// assert_eq!(SimpleCast::min_int("uint256")?, "0");
    /// assert_eq!(SimpleCast::min_int("int256")?, format!("{}", I256::MIN));
    /// assert_eq!(SimpleCast::min_int("int32")?, format!("{}", i32::MIN));
    /// # Ok::<(), eyre::Report>(())
    /// ```
    pub fn min_int(s: &str) -> Result<String> {
        Self::max_min_int::<false>(s)
    }

    fn max_min_int<const MAX: bool>(s: &str) -> Result<String> {
        let ty = HumanReadableParser::parse_type(s)
            .wrap_err("Invalid type, expected `(u)int<bit size>`")?;
        match ty {
            ParamType::Int(n) => {
                let mask = U256::one() << U256::from(n - 1);
                let max = (U256::MAX & mask) - 1;
                if MAX {
                    Ok(max.to_string())
                } else {
                    let min = I256::from_raw(max).wrapping_neg() + I256::minus_one();
                    Ok(min.to_string())
                }
            }
            ParamType::Uint(n) => {
                if MAX {
                    let mut max = U256::MAX;
                    if n < 255 {
                        max &= U256::one() << U256::from(n);
                    }
                    Ok(max.to_string())
                } else {
                    Ok("0".to_string())
                }
            }
            _ => Err(eyre::eyre!("Type is not int/uint: {s}")),
        }
    }

    /// Converts UTF-8 text input to hex
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::from_utf8("yo"), "0x796f");
    ///     assert_eq!(Cast::from_utf8("Hello, World!"), "0x48656c6c6f2c20576f726c6421");
    ///     assert_eq!(Cast::from_utf8("TurboDappTools"), "0x547572626f44617070546f6f6c73");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_utf8(s: &str) -> String {
        let s: String = s.as_bytes().to_hex();
        format!("0x{s}")
    }

    /// Converts hex data into text data
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_ascii("0x796f")?, "yo");
    ///     assert_eq!(Cast::to_ascii("48656c6c6f2c20576f726c6421")?, "Hello, World!");
    ///     assert_eq!(Cast::to_ascii("0x547572626f44617070546f6f6c73")?, "TurboDappTools");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_ascii(hex: &str) -> Result<String> {
        let hex_trimmed = hex.trim_start_matches("0x");
        let iter = FromHexIter::new(hex_trimmed);
        let mut ascii = String::new();
        for letter in iter.collect::<Vec<_>>() {
            ascii.push(letter? as char);
        }
        Ok(ascii)
    }

    /// Converts fixed point number into specified number of decimals
    /// ```
    /// use probe::SimpleCast as Cast;
    /// use corebc_core::types::U256;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::from_fixed_point("10", "0")?, "10");
    ///     assert_eq!(Cast::from_fixed_point("1.0", "1")?, "10");
    ///     assert_eq!(Cast::from_fixed_point("0.10", "2")?, "10");
    ///     assert_eq!(Cast::from_fixed_point("0.010", "3")?, "10");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_fixed_point(value: &str, decimals: &str) -> Result<String> {
        // first try u32 as Units assumes a string can only be "ether", "gwei"... and not a number
        let units = match decimals.parse::<u32>() {
            Ok(d) => Units::Other(d),
            Err(_) => Units::try_from(decimals)?,
        };
        let n: NumberWithBase = parse_units(value, units.as_num())?.into();
        Ok(format!("{n}"))
    }

    /// Converts integers with specified decimals into fixed point numbers
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    /// use corebc_core::types::U256;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_fixed_point("10", "0")?, "10.");
    ///     assert_eq!(Cast::to_fixed_point("10", "1")?, "1.0");
    ///     assert_eq!(Cast::to_fixed_point("10", "2")?, "0.10");
    ///     assert_eq!(Cast::to_fixed_point("10", "3")?, "0.010");
    ///
    ///     assert_eq!(Cast::to_fixed_point("-10", "0")?, "-10.");
    ///     assert_eq!(Cast::to_fixed_point("-10", "1")?, "-1.0");
    ///     assert_eq!(Cast::to_fixed_point("-10", "2")?, "-0.10");
    ///     assert_eq!(Cast::to_fixed_point("-10", "3")?, "-0.010");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_fixed_point(value: &str, decimals: &str) -> Result<String> {
        let (sign, mut value, value_len) = {
            let number = NumberWithBase::parse_int(value, None)?;
            let sign = if number.is_nonnegative() { "" } else { "-" };
            let value = format!("{number:#}");
            let value_stripped = value.strip_prefix('-').unwrap_or(&value).to_string();
            let value_len = value_stripped.len();
            (sign, value_stripped, value_len)
        };
        let decimals = NumberWithBase::parse_uint(decimals, None)?.number().low_u64() as usize;

        let value = if decimals >= value_len {
            // Add "0." and pad with 0s
            format!("0.{value:0>decimals$}")
        } else {
            // Insert decimal at -idx (i.e 1 => decimal idx = -1)
            value.insert(value_len - decimals, '.');
            value
        };

        Ok(format!("{sign}{value}"))
    }

    /// Concatencates hex strings
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::concat_hex(["0x00", "0x01"]), "0x0001");
    ///     assert_eq!(Cast::concat_hex(["1", "2"]), "0x12");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn concat_hex<T: AsRef<str>>(values: impl IntoIterator<Item = T>) -> String {
        let mut out = String::new();
        for s in values {
            let s = s.as_ref();
            out.push_str(s.strip_prefix("0x").unwrap_or(s))
        }
        format!("0x{out}")
    }

    /// Converts a number into uint256 hex string with 0x prefix
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_uint256("100")?, "0x0000000000000000000000000000000000000000000000000000000000000064");
    ///     assert_eq!(Cast::to_uint256("192038293923")?, "0x0000000000000000000000000000000000000000000000000000002cb65fd1a3");
    ///     assert_eq!(
    ///         Cast::to_uint256("115792089237316195423570985008687907853269984665640564039457584007913129639935")?,
    ///         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_uint256(value: &str) -> Result<String> {
        let n = NumberWithBase::parse_uint(value, None)?;
        Ok(format!("{n:#066x}"))
    }

    /// Converts a number into int256 hex string with 0x prefix
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_int256("0")?, "0x0000000000000000000000000000000000000000000000000000000000000000");
    ///     assert_eq!(Cast::to_int256("100")?, "0x0000000000000000000000000000000000000000000000000000000000000064");
    ///     assert_eq!(Cast::to_int256("-100")?, "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9c");
    ///     assert_eq!(Cast::to_int256("192038293923")?, "0x0000000000000000000000000000000000000000000000000000002cb65fd1a3");
    ///     assert_eq!(Cast::to_int256("-192038293923")?, "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffd349a02e5d");
    ///     assert_eq!(
    ///         Cast::to_int256("57896044618658097711785492504343953926634992332820282019728792003956564819967")?,
    ///         "0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    ///     );
    ///     assert_eq!(
    ///         Cast::to_int256("-57896044618658097711785492504343953926634992332820282019728792003956564819968")?,
    ///         "0x8000000000000000000000000000000000000000000000000000000000000000"
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_int256(value: &str) -> Result<String> {
        let n = NumberWithBase::parse_int(value, None)?;
        Ok(format!("{n:#066x}"))
    }

    /// Converts an eth amount into a specified unit
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_unit("1 wei", "wei")?, "1");
    ///     assert_eq!(Cast::to_unit("1", "wei")?, "1");
    ///     assert_eq!(Cast::to_unit("1ether", "wei")?, "1000000000000000000");
    ///     assert_eq!(Cast::to_unit("100 gwei", "gwei")?, "100");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_unit(value: &str, unit: &str) -> Result<String> {
        let value = U256::from(LenientTokenizer::tokenize_uint(value)?);

        Ok(match unit {
            "eth" | "ether" => corebc_core::utils::format_units(value, 18)?
                .trim_end_matches(".000000000000000000")
                .to_string(),
            "gwei" | "nano" | "nanoether" => corebc_core::utils::format_units(value, 9)?
                .trim_end_matches(".000000000")
                .to_string(),
            "wei" => corebc_core::utils::format_units(value, 0)?.trim_end_matches(".0").to_string(),
            _ => eyre::bail!("invalid unit: \"{}\"", unit),
        })
    }

    /// Converts wei into an eth amount
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::from_wei("1", "gwei")?, "0.000000001");
    ///     assert_eq!(Cast::from_wei("12340000005", "gwei")?, "12.340000005");
    ///     assert_eq!(Cast::from_wei("10", "ether")?, "0.000000000000000010");
    ///     assert_eq!(Cast::from_wei("100", "eth")?, "0.000000000000000100");
    ///     assert_eq!(Cast::from_wei("17", "")?, "0.000000000000000017");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_wei(value: &str, unit: &str) -> Result<String> {
        let value = NumberWithBase::parse_int(value, None)?.number();

        Ok(match unit {
            "gwei" => format_units(value, 9),
            _ => format_units(value, 18),
        }?)
    }

    /// Converts an eth amount into wei
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_wei("1", "")?, "1000000000000000000");
    ///     assert_eq!(Cast::to_wei("100", "gwei")?, "100000000000");
    ///     assert_eq!(Cast::to_wei("100", "eth")?, "100000000000000000000");
    ///     assert_eq!(Cast::to_wei("1000", "ether")?, "1000000000000000000000");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_wei(value: &str, unit: &str) -> Result<String> {
        let wei = match unit {
            "gwei" => parse_units(value, 9),
            _ => parse_units(value, 18),
        }?;
        Ok(wei.to_string())
    }

    /// Decodes rlp encoded list with hex data
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::from_rlp("0xc0".to_string()).unwrap(), "[]");
    ///     assert_eq!(Cast::from_rlp("0x0f".to_string()).unwrap(), "\"0x0f\"");
    ///     assert_eq!(Cast::from_rlp("0x33".to_string()).unwrap(), "\"0x33\"");
    ///     assert_eq!(Cast::from_rlp("0xc161".to_string()).unwrap(), "[\"0x61\"]");
    ///     assert_eq!(Cast::from_rlp("0xc26162".to_string()).unwrap(), "[\"0x61\",\"0x62\"]");
    ///     Ok(())
    /// }
    /// ```
    pub fn from_rlp(value: impl AsRef<str>) -> Result<String> {
        let data = strip_0x(value.as_ref());
        let bytes = hex::decode(data).wrap_err("Could not decode hex")?;
        let item = rlp::decode::<Item>(&bytes).wrap_err("Could not decode rlp")?;
        Ok(item.to_string())
    }

    /// Encodes hex data or list of hex data to hexadecimal rlp
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_rlp("[]").unwrap(),"0xc0".to_string());
    ///     assert_eq!(Cast::to_rlp("0x22").unwrap(),"0x22".to_string());
    ///     assert_eq!(Cast::to_rlp("[\"0x61\"]",).unwrap(), "0xc161".to_string());
    ///     assert_eq!(Cast::to_rlp( "[\"0xf1\",\"f2\"]").unwrap(), "0xc481f181f2".to_string());
    ///     Ok(())
    /// }
    /// ```
    pub fn to_rlp(value: &str) -> Result<String> {
        let val = serde_json::from_str(value)
            .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));
        let item = Item::value_to_item(&val)?;
        Ok(format!("0x{}", hex::encode(rlp::encode(&item))))
    }

    /// Converts a number of one base to another
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    /// use corebc_core::types::{I256, U256};
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::to_base("100", Some("10".to_string()), "16")?, "0x64");
    ///     assert_eq!(Cast::to_base("100", Some("10".to_string()), "oct")?, "0o144");
    ///     assert_eq!(Cast::to_base("100", Some("10".to_string()), "binary")?, "0b1100100");
    ///
    ///     assert_eq!(Cast::to_base("0xffffffffffffffff", None, "10")?, u64::MAX.to_string());
    ///     assert_eq!(Cast::to_base("0xffffffffffffffffffffffffffffffff", None, "dec")?, u128::MAX.to_string());
    ///     // U256::MAX overflows as internally it is being parsed as I256
    ///     assert_eq!(Cast::to_base("0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", None, "decimal")?, I256::MAX.to_string());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_base(value: &str, base_in: Option<String>, base_out: &str) -> Result<String> {
        let base_in = Base::unwrap_or_detect(base_in, value)?;
        let base_out: Base = base_out.parse()?;
        if base_in == base_out {
            return Ok(value.to_string());
        }

        let mut n = NumberWithBase::parse_int(value, Some(base_in.to_string()))?;
        n.set_base(base_out);

        // Use Debug fmt
        Ok(format!("{n:#?}"))
    }

    /// Converts hexdata into bytes32 value
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// # fn main() -> eyre::Result<()> {
    /// let bytes = Cast::to_bytes32("1234")?;
    /// assert_eq!(bytes, "0x1234000000000000000000000000000000000000000000000000000000000000");
    ///
    /// let bytes = Cast::to_bytes32("0x1234")?;
    /// assert_eq!(bytes, "0x1234000000000000000000000000000000000000000000000000000000000000");
    ///
    /// let err = Cast::to_bytes32("0x123400000000000000000000000000000000000000000000000000000000000011").unwrap_err();
    /// assert_eq!(err.to_string(), "string >32 bytes");
    ///
    /// # Ok(())
    /// # }
    pub fn to_bytes32(s: &str) -> Result<String> {
        let s = strip_0x(s);
        if s.len() > 64 {
            eyre::bail!("string >32 bytes");
        }

        let padded = format!("{s:0<64}");
        // need to use the Debug implementation
        Ok(format!("{:?}", H256::from_str(&padded)?))
    }

    /// Encodes string into bytes32 value
    pub fn format_bytes32_string(s: &str) -> Result<String> {
        let formatted = format_bytes32_string(s)?;
        Ok(format!("0x{}", hex::encode(formatted)))
    }

    /// Decodes string from bytes32 value
    pub fn parse_bytes32_string(s: &str) -> Result<String> {
        let s = strip_0x(s);
        if s.len() != 64 {
            eyre::bail!("expected 64 byte hex-string, got {s}");
        }

        let bytes = hex::decode(s)?;
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&bytes);

        Ok(parse_bytes32_string(&buffer)?.to_owned())
    }

    /// Decodes checksummed address from bytes32 value
    pub fn parse_bytes32_address(s: &str) -> Result<String> {
        let s = strip_0x(s);
        if s.len() != 64 {
            eyre::bail!("expected 64 byte hex-string, got {s}");
        }

        let s = if let Some(stripped) = s.strip_prefix("00000000000000000000") {
            stripped
        } else {
            return Err(eyre::eyre!("Not convertible to address, there are non-zero bytes"));
        };
        Ok(s.to_string())
    }

    /// Decodes abi-encoded hex input or output
    ///
    /// When `input=true`, `calldata` string MUST not be prefixed with function selector
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     // Passing `input = false` will decode the data as the output type.
    ///     // The input data types and the full function sig are ignored, i.e.
    ///     // you could also pass `balanceOf()(uint256)` and it'd still work.
    ///     let data = "0x0000000000000000000000000000000000000000000000000000000000000001";
    ///     let sig = "balanceOf(address, uint256)(uint256)";
    ///     let decoded = Cast::abi_decode(sig, data, false)?[0].to_string();
    ///     assert_eq!(decoded, "1");
    ///
    ///     // Passing `input = true` will decode the data with the input function signature.
    ///     // We exclude the "prefixed" function selector from the data field (the first 4 bytes).
    ///     let data = "0x0000000000000000000000008dbd1b711dc621e1404633da156fcc779e1c6f3e000000000000000000000000d9f3c9cc99548bf3b44a43e0a2d07399eb918adc000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
    ///     let sig = "safeTransferFrom(address, address, uint256, uint256, bytes)";
    ///     let decoded = Cast::abi_decode(sig, data, true)?;
    ///     let decoded = decoded.iter().map(ToString::to_string).collect::<Vec<_>>();
    ///     assert_eq!(
    ///         decoded,
    ///         vec!["00008dbd1b711dc621e1404633da156fcc779e1c6f3e", "0000d9f3c9cc99548bf3b44a43e0a2d07399eb918adc", "2a", "1", ""]
    ///     );
    ///
    ///
    ///     # Ok(())
    /// }
    /// ```
    pub fn abi_decode(sig: &str, calldata: &str, input: bool) -> Result<Vec<Token>> {
        foxar_common::abi::abi_decode(sig, calldata, input, false)
    }

    /// Decodes calldata-encoded hex input or output
    ///
    /// Similar to `abi_decode`, but `calldata` string MUST be prefixed with function selector
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     // Passing `input = false` will decode the data as the output type.
    ///     // The input data types and the full function sig are ignored, i.e.
    ///     // you could also pass `balanceOf()(uint256)` and it'd still work.
    ///     let data = "0x0000000000000000000000000000000000000000000000000000000000000001";
    ///     let sig = "balanceOf(address, uint256)(uint256)";
    ///     let decoded = Cast::calldata_decode(sig, data, false)?[0].to_string();
    ///     assert_eq!(decoded, "1");
    ///
    ///     // Passing `input = true` will decode the data with the input function signature.
    ///     let data = "0xabfbcbd50000000000000000000000008dbd1b711dc621e1404633da156fcc779e1c6f3e000000000000000000000000d9f3c9cc99548bf3b44a43e0a2d07399eb918adc000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
    ///     let sig = "safeTransferFrom(address, address, uint256, uint256, bytes)";
    ///     let decoded = Cast::calldata_decode(sig, data, true)?;
    ///     let decoded = decoded.iter().map(ToString::to_string).collect::<Vec<_>>();
    ///     assert_eq!(
    ///         decoded,
    ///         vec!["00008dbd1b711dc621e1404633da156fcc779e1c6f3e", "0000d9f3c9cc99548bf3b44a43e0a2d07399eb918adc", "2a", "1", ""]
    ///     );
    ///
    ///
    ///     # Ok(())
    /// }
    /// ```
    pub fn calldata_decode(sig: &str, calldata: &str, input: bool) -> Result<Vec<Token>> {
        foxar_common::abi::abi_decode(sig, calldata, input, true)
    }

    /// Performs ABI encoding based off of the function signature. Does not include
    /// the function selector in the result.
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast as Cast;
    ///
    /// # fn main() -> eyre::Result<()> {
    ///     assert_eq!(
    ///         "0x0000000000000000000000000000000000000000000000000000000000000001",
    ///         Cast::abi_encode("f(uint a)", &["1"]).unwrap().as_str()
    ///     );
    ///     assert_eq!(
    ///         "0x0000000000000000000000000000000000000000000000000000000000000001",
    ///         Cast::abi_encode("constructor(uint a)", &["1"]).unwrap().as_str()
    ///     );
    /// #    Ok(())
    /// # }
    /// ```
    pub fn abi_encode(sig: &str, args: &[impl AsRef<str>]) -> Result<String> {
        let func = match HumanReadableParser::parse_function(sig) {
            Ok(func) => func,
            Err(err) => {
                if let Ok(constructor) = HumanReadableParser::parse_constructor(sig) {
                    #[allow(deprecated)]
                    Function {
                        name: "constructor".to_string(),
                        inputs: constructor.inputs,
                        outputs: vec![],
                        constant: None,
                        state_mutability: Default::default(),
                    }
                } else {
                    // we return the `Function` parse error as this case is more likely
                    eyre::bail!("Could not process human-readable ABI. Please check if you've left the parenthesis unclosed or if some type is incomplete.\nError:\n{}", err)
                    // return Err(err.into()).wrap_err("Could not process human-readable ABI. Please
                    // check if you've left the parenthesis unclosed or if some type is
                    // incomplete.")
                }
            }
        };
        let calldata = match encode_args(&func, args) {
            Ok(res) => res.to_hex::<String>(),
            Err(e) => eyre::bail!("Could not ABI encode the function and arguments. Did you pass in the right types?\nError\n{}", e),
        };
        let encoded = &calldata[8..];
        Ok(format!("0x{encoded}"))
    }

    /// Performs ABI encoding to produce the hexadecimal calldata with the given arguments.
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast as Cast;
    ///
    /// # fn main() -> eyre::Result<()> {
    ///     assert_eq!(
    ///         "0x6a6f4c680000000000000000000000000000000000000000000000000000000000000001",
    ///         Cast::calldata_encode("f(uint a)", &["1"]).unwrap().as_str()
    ///     );
    /// #    Ok(())
    /// # }
    /// ```
    pub fn calldata_encode(sig: impl AsRef<str>, args: &[impl AsRef<str>]) -> Result<String> {
        let func = HumanReadableParser::parse_function(sig.as_ref())?;
        let calldata = encode_args(&func, args)?;
        Ok(format!("0x{}", calldata.to_hex::<String>()))
    }

    /// Generates an interface in solidity from either a local file ABI or a verified contract on
    /// Etherscan. It returns a vector of [`InterfaceSource`] structs that contain the source of the
    /// interface and their name.
    /// ```no_run
    /// use probe::SimpleCast as Cast;
    /// use probe::AbiPath;
    /// # async fn foo() -> eyre::Result<()> {
    /// let path = AbiPath::Local {
    ///     path: "utils/testdata/interfaceTestABI.json".to_owned(),
    ///     name: None,
    /// };
    /// let interfaces= Cast::generate_interface(path).await?;
    /// println!("interface {} {{\n {}\n}}", interfaces[0].name, interfaces[0].source);
    /// # Ok(())
    /// # }
    /// ```
    // todo::error2215 - fix functionality
    pub async fn generate_interface(address_or_path: AbiPath) -> Result<Vec<InterfaceSource>> {
        let (contract_abis, contract_names): (Vec<RawAbi>, Vec<String>) = match address_or_path {
            AbiPath::Local { path, name } => {
                let file = std::fs::read_to_string(path).wrap_err("unable to read abi file")?;
                let mut json: serde_json::Value = serde_json::from_str(&file)?;
                let json = if !json["abi"].is_null() { json["abi"].take() } else { json };
                let abi: RawAbi =
                    serde_json::from_value(json).wrap_err("unable to parse json ABI from file")?;

                (vec![abi], vec![name.unwrap_or_else(|| "Interface".to_owned())])
            }
            AbiPath::Etherscan { address, network } => {
                // BLOCKINDEX TODO: Uncomment api_key
                let client = Client::new(network /* , api_key */)?;

                // get the source
                let source = match client.contract_source_code(address).await {
                    Ok(source) => source,
                    Err(BlockindexError::ContractCodeNotVerified(address)) => {
                        eyre::bail!(
                            "Contract source code at {:?} on {} not verified. Maybe you
    have selected the wrong network?",
                            address,
                            network
                        )
                    }
                    Err(err) => {
                        eyre::bail!(err)
                    }
                };

                let names = source
                    .items
                    .iter()
                    .map(|item| item.contract_name.clone())
                    .collect::<Vec<String>>();

                let abis = source.raw_abis()?;

                (abis, names)
            }
        };
        contract_abis
            .iter()
            .zip(contract_names)
            .map(|(contract_abi, name)| {
                let source = foxar_utils::abi::abi_to_solidity(contract_abi, &name)?;
                Ok(InterfaceSource { name, source })
            })
            .collect::<Result<Vec<InterfaceSource>>>()
    }

    /// Prints the slot number for the specified mapping type and input data
    /// Uses abi_encode to pad the data to 32 bytes.
    /// For value types v, slot number of v is sha3(concat(h(v) , p)) where h is the padding
    /// function and p is slot number of the mapping.
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast as Cast;
    ///
    /// # fn main() -> eyre::Result<()> {
    ///
    ///    assert_eq!(Cast::index("address", "0x0000D0074F4E6490ae3f888d1d4f7E3E43326bD3f0f5" ,"2").unwrap().as_str(),"0x0a334937675f50ee43f99d31cbd28697db98472cee27165471589c95989131d6");
    ///    assert_eq!(Cast::index("uint256","42" ,"6").unwrap().as_str(),"0xef68ff66695636cf6e6d2e15f3949d71b20be722147c7ea2c53478d504623c49");
    /// #    Ok(())
    /// # }
    /// ```
    pub fn index(from_type: &str, from_value: &str, slot_number: &str) -> Result<String> {
        let sig = format!("x({from_type},uint256)");
        let encoded = Self::abi_encode(&sig, &[from_value, slot_number])?;
        let location: String = Self::sha3(&encoded)?;
        Ok(location)
    }

    /// Converts ENS names to their namehash representation
    /// [Namehash reference](https://docs.ens.domains/contract-api-reference/name-processing#hashing-names)
    /// [namehash-rust reference](https://github.com/InstateDev/namehash-rust/blob/master/src/lib.rs)
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::namehash("")?, "0x0000000000000000000000000000000000000000000000000000000000000000");
    ///     assert_eq!(Cast::namehash("eth")?, "0x2f76c451b22a76caba77f6102bc541ef54f443340fea5924b8ba3d6f2e84e0c7");
    ///     assert_eq!(Cast::namehash("foo.eth")?, "0x9321c247b82b8e06384ae3d6f01d489e9d03982a6df01f20578b939677a531fd");
    ///     assert_eq!(Cast::namehash("sub.foo.eth")?, "0x734979202b3615e02b4d68e3d9a52c8fc8d6f5d3ecaa89a45b23aab1a8c2cfe0");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn namehash(ens: &str) -> Result<String> {
        let mut node = vec![0u8; 32];

        if !ens.is_empty() {
            let ens_lower = ens.to_lowercase();
            let mut labels: Vec<&str> = ens_lower.split('.').collect();
            labels.reverse();

            for label in labels {
                let mut label_hash = sha3(label.as_bytes());
                node.append(&mut label_hash.to_vec());

                label_hash = sha3(node.as_slice());
                node = label_hash.to_vec();
            }
        }

        let namehash: String = node.to_hex();
        Ok(format!("0x{namehash}"))
    }

    /// sha3-256 hashes arbitrary data
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::sha3("foo")?, "0x76d3bc41c9f588f7fcd0d5bf4718f8f84b1c41b20882703100b9eb9413807c01");
    ///     assert_eq!(Cast::sha3("123abc")?, "0xb1b6a3de29ab907153614683d357b2db943a317d036ff25f7022d4707109005a");
    ///     assert_eq!(Cast::sha3("0x12")?, "0xbf931c9eed1d7d81c3ab815ea4150d5f9efe357f32dbece862c15cf4ed92ed67");
    ///     assert_eq!(Cast::sha3("12")?, "0x1a9a118cb653759c3fcb3bd5060e6f9910c8c27008dd11fe4315f4635c9caa98");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn sha3(data: &str) -> Result<String> {
        let hash = match data.as_bytes() {
            // 0x prefix => read as hex data
            [b'0', b'x', rest @ ..] => sha3(hex::decode(rest)?),
            // No 0x prefix => read as text
            _ => sha3(data),
        };

        Ok(format!("{:?}", H256(hash)))
    }

    /// Performs the left shift operation (<<) on a number
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::left_shift("16", "10", Some("10".to_string()), "hex")?, "0x4000");
    ///     assert_eq!(Cast::left_shift("255", "16", Some("dec".to_string()), "hex")?, "0xff0000");
    ///     assert_eq!(Cast::left_shift("0xff", "16", None, "hex")?, "0xff0000");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn left_shift(
        value: &str,
        bits: &str,
        base_in: Option<String>,
        base_out: &str,
    ) -> Result<String> {
        let base_out: Base = base_out.parse()?;
        let value = NumberWithBase::parse_uint(value, base_in)?;
        let bits = NumberWithBase::parse_uint(bits, None)?;

        let res = value.number() << bits.number();

        Ok(res.to_base(base_out, true)?)
    }

    /// Performs the right shift operation (>>) on a number
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::right_shift("0x4000", "10", None, "dec")?, "16");
    ///     assert_eq!(Cast::right_shift("16711680", "16", Some("10".to_string()), "hex")?, "0xff");
    ///     assert_eq!(Cast::right_shift("0xff0000", "16", None, "hex")?, "0xff");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn right_shift(
        value: &str,
        bits: &str,
        base_in: Option<String>,
        base_out: &str,
    ) -> Result<String> {
        let base_out: Base = base_out.parse()?;
        let value = NumberWithBase::parse_uint(value, base_in)?;
        let bits = NumberWithBase::parse_uint(bits, None)?;

        let res = value.number() >> bits.number();

        Ok(res.to_base(base_out, true)?)
    }

    /// Fetches source code of verified contracts from etherscan.
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast as Cast;
    /// # use corebc_core::types::Network;
    ///
    /// # async fn foo() -> eyre::Result<()> {
    ///     assert_eq!(
    ///             "/*
    ///             - Bytecode Verification performed was compared on second iteration -
    ///             This file is part of the DAO.....",
    ///         Cast::etherscan_source(Network::Mainnet, "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".to_string(), "<etherscan_api_key>".to_string()).await.unwrap().as_str()
    ///     );
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn etherscan_source(
        chain: Network,
        contract_address: String,
        _etherscan_api_key: String,
    ) -> Result<String> {
        // BLOCKINDEX TODO
        let client = Client::new(chain /* , etherscan_api_key */)?;
        let metadata = client.contract_source_code(contract_address.parse()?).await?;
        Ok(metadata.source_code())
    }

    /// Fetches the source code of verified contracts from etherscan and expands the resulting
    /// files to a directory for easy perusal.
    ///
    /// # Example
    ///
    /// ```
    /// # use probe::SimpleCast as Cast;
    /// # use corebc_core::types::Network;
    /// # use std::path::PathBuf;
    ///
    /// # async fn expand() -> eyre::Result<()> {
    ///      Cast::expand_etherscan_source_to_directory(Network::Mainnet, "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".to_string(), "<etherscan_api_key>".to_string(), PathBuf::from("output_dir")).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn expand_etherscan_source_to_directory(
        chain: Network,
        contract_address: String,
        _etherscan_api_key: String,
        output_directory: PathBuf,
    ) -> eyre::Result<()> {
        // BLOCKINDEX TODO
        let client = Client::new(chain /* , etherscan_api_key */)?;
        let meta = client.contract_source_code(contract_address.parse()?).await?;
        let source_tree = meta.source_tree();
        source_tree.write_to(&output_directory)?;
        Ok(())
    }

    /// Disassembles hex encoded bytecode into individual / human readable opcodes
    ///
    /// # Example
    ///
    /// ```no_run
    /// use probe::SimpleCast as Cast;
    ///
    /// # async fn foo() -> eyre::Result<()> {
    /// let bytecode = "0x608060405260043610603f57600035";
    /// let opcodes = Cast::disassemble(bytecode)?;
    /// println!("{}", opcodes);
    /// # Ok(())
    /// # }
    /// ```
    pub fn disassemble(bytecode: &str) -> Result<String> {
        format_operations(disassemble_str(bytecode)?)
    }

    /// Gets the selector for a given function signature
    /// Optimizes if the `optimize` parameter is set to a number of leading zeroes
    ///
    /// # Example
    ///
    /// ```
    /// use probe::SimpleCast as Cast;
    ///
    /// fn main() -> eyre::Result<()> {
    ///     assert_eq!(Cast::get_selector("foo(address,uint256)", None)?.0, String::from("0xe041c8d8"));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_selector(signature: &str, optimize: Option<usize>) -> Result<(String, String)> {
        let optimize = optimize.unwrap_or(0);
        if optimize > 4 {
            eyre::bail!("Number of leading zeroes must not be greater than 4");
        }
        if optimize == 0 {
            let selector = HumanReadableParser::parse_function(signature)?.short_signature();
            return Ok((format!("0x{}", hex::encode(selector)), String::from(signature)));
        }
        let Some((name, params)) = signature.split_once('(') else {
            eyre::bail!("Invalid signature");
        };

        let num_threads = num_cpus::get();
        let found = AtomicBool::new(false);

        let result: Option<(u32, String, String)> =
            (0..num_threads).into_par_iter().find_map_any(|i| {
                let nonce_start = i as u32;
                let nonce_step = num_threads as u32;

                let mut nonce = nonce_start;
                while nonce < u32::MAX && !found.load(Ordering::Relaxed) {
                    let input = format!("{}{}({}", name, nonce, params);
                    let hash = sha3(input.as_bytes());
                    let selector = &hash[..4];

                    if selector.iter().take_while(|&&byte| byte == 0).count() == optimize {
                        found.store(true, Ordering::Relaxed);
                        return Some((nonce, format!("0x{}", hex::encode(selector)), input));
                    }

                    nonce += nonce_step;
                }
                None
            });

        match result {
            Some((_nonce, selector, signature)) => Ok((selector, signature)),
            None => eyre::bail!("No selector found"),
        }
    }
}

fn strip_0x(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::SimpleCast as Cast;

    #[test]
    fn calldata_uint() {
        assert_eq!(
            "0x6a6f4c680000000000000000000000000000000000000000000000000000000000000001",
            Cast::calldata_encode("f(uint a)", &["1"]).unwrap().as_str()
        );
    }

    // <https://github.com/foxar-rs/foxar/issues/2681>
    #[test]
    fn calldata_array() {
        assert_eq!(
            "0xdfbd84700000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000",
            Cast::calldata_encode("propose(string[])", &["[\"\"]"]).unwrap().as_str()
        );
    }

    #[test]
    fn calldata_bool() {
        assert_eq!(
            "0xf02e41eb0000000000000000000000000000000000000000000000000000000000000000",
            Cast::calldata_encode("bar(bool)", &["false"]).unwrap().as_str()
        );
    }

    #[test]
    fn abi_decode() {
        let data = "0x0000000000000000000000000000000000000000000000000000000000000001";
        let sig = "balanceOf(address, uint256)(uint256)";
        assert_eq!("1", Cast::abi_decode(sig, data, false).unwrap()[0].to_string());

        let data = "0x00000000000000000000cb668dbd1b711dc621e1404633da156fcc779e1c6f3e00000000000000000000cb66d9f3c9cc99548bf3b44a43e0a2d07399eb918adc000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
        let sig = "safeTransferFrom(address,address,uint256,uint256,bytes)";
        let decoded = Cast::abi_decode(sig, data, true).unwrap();
        let decoded = decoded.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert_eq!(
            decoded,
            vec![
                "cb668dbd1b711dc621e1404633da156fcc779e1c6f3e",
                "cb66d9f3c9cc99548bf3b44a43e0a2d07399eb918adc",
                "2a",
                "1",
                ""
            ]
        );
    }

    #[test]
    fn calldata_decode() {
        let data = "0x0000000000000000000000000000000000000000000000000000000000000001";
        let sig = "balanceOf(address, uint256)(uint256)";
        let decoded = Cast::calldata_decode(sig, data, false).unwrap()[0].to_string();
        assert_eq!(decoded, "1");

        // Passing `input = true` will decode the data with the input function signature.
        // We exclude the "prefixed" function selector from the data field (the first 4 bytes).
        let data = "0xf242432a00000000000000000000cb668dbd1b711dc621e1404633da156fcc779e1c6f3e00000000000000000000cb66d9f3c9cc99548bf3b44a43e0a2d07399eb918adc000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
        let sig = "safeTransferFrom(address, address, uint256, uint256, bytes)";
        let decoded = Cast::calldata_decode(sig, data, true).unwrap();
        let decoded = decoded.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert_eq!(
            decoded,
            vec![
                "cb668dbd1b711dc621e1404633da156fcc779e1c6f3e",
                "cb66d9f3c9cc99548bf3b44a43e0a2d07399eb918adc",
                "2a",
                "1",
                ""
            ]
        );
    }

    #[test]
    fn concat_hex() {
        assert_eq!(Cast::concat_hex(["0x00", "0x01"]), "0x0001");
        assert_eq!(Cast::concat_hex(["1", "2"]), "0x12");
    }

    #[test]
    fn from_rlp() {
        let rlp = "0xf8b1a02b5df5f0757397573e8ff34a8b987b21680357de1f6c8d10273aa528a851eaca8080a02838ac1d2d2721ba883169179b48480b2ba4f43d70fcf806956746bd9e83f90380a0e46fff283b0ab96a32a7cc375cecc3ed7b6303a43d64e0a12eceb0bc6bd8754980a01d818c1c414c665a9c9a0e0c0ef1ef87cacb380b8c1f6223cb2a68a4b2d023f5808080a0236e8f61ecde6abfebc6c529441f782f62469d8a2cc47b7aace2c136bd3b1ff08080808080";
        let item = Cast::from_rlp(rlp).unwrap();
        assert_eq!(
            item,
            r#"["0x2b5df5f0757397573e8ff34a8b987b21680357de1f6c8d10273aa528a851eaca","0x","0x","0x2838ac1d2d2721ba883169179b48480b2ba4f43d70fcf806956746bd9e83f903","0x","0xe46fff283b0ab96a32a7cc375cecc3ed7b6303a43d64e0a12eceb0bc6bd87549","0x","0x1d818c1c414c665a9c9a0e0c0ef1ef87cacb380b8c1f6223cb2a68a4b2d023f5","0x","0x","0x","0x236e8f61ecde6abfebc6c529441f782f62469d8a2cc47b7aace2c136bd3b1ff0","0x","0x","0x","0x","0x"]"#
        )
    }
}
