use crate::opts::EtherscanOpts;
use clap::{Parser, ValueHint};
use corebc::prelude::{errors::BlockindexError, Abigen, Client, MultiAbigen};
use eyre::Result;
use foxar_config::Config;
use std::path::{Path, PathBuf};

static DEFAULT_CRATE_NAME: &str = "foxar-contracts";
static DEFAULT_CRATE_VERSION: &str = "0.0.1";

/// CLI arguments for `probe bind`.
#[derive(Debug, Clone, Parser)]
pub struct BindArgs {
    /// The contract address, or the path to an ABI Directory
    ///
    /// If an address is specified, then the ABI is fetched from Etherscan.
    path_or_address: String,

    /// Path to where bindings will be stored
    #[clap(
        short,
        long,
        value_hint = ValueHint::DirPath,
        value_name = "PATH"
    )]
    pub output_dir: Option<PathBuf>,

    /// The name of the Rust crate to generate.
    ///
    /// This should be a valid crates.io crate name. However, this is currently not validated by
    /// this command.
    #[clap(
        long,
        default_value = DEFAULT_CRATE_NAME,
        value_name = "NAME"
    )]
    crate_name: String,

    /// The version of the Rust crate to generate.
    ///
    /// This should be a standard semver version string. However, it is not currently validated by
    /// this command.
    #[clap(
        long,
        default_value = DEFAULT_CRATE_VERSION,
        value_name = "VERSION"
    )]
    crate_version: String,

    /// Generate bindings as separate files.
    #[clap(long)]
    separate_files: bool,

    #[clap(flatten)]
    etherscan: EtherscanOpts,
}

impl BindArgs {
    pub async fn run(self) -> Result<()> {
        let path = Path::new(&self.path_or_address);
        let multi = if path.exists() {
            MultiAbigen::from_json_files(path)
        } else {
            self.abigen_etherscan().await
        }?;

        println!("Generating bindings for {} contracts", multi.len());
        let bindings = multi.build()?;

        let out = self
            .output_dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap().join("bindings"));
        bindings.write_to_crate(self.crate_name, self.crate_version, out, !self.separate_files)?;
        Ok(())
    }

    async fn abigen_etherscan(&self) -> Result<MultiAbigen> {
        let config = Config::from(&self.etherscan);

        let chain = config.network_id.unwrap_or_default();

        let client = Client::new(chain)?;
        let address = self.path_or_address.parse()?;
        let source = match client.contract_source_code(address).await {
            Ok(source) => source,
            // Err(BlockindexError::InvalidApiKey) => {
            //     eyre::bail!("Invalid Etherscan API key. Did you set it correctly? You may be
            // using an API key for another Etherscan API chain (e.g. Etherscan API key for
            // Polygonscan).") }
            Err(BlockindexError::ContractCodeNotVerified(address)) => {
                eyre::bail!("Contract source code at {:?} on {} not verified. Maybe you have selected the wrong chain?", address, chain)
            }
            Err(err) => {
                eyre::bail!(err)
            }
        };
        let abigens = source
            .items
            .into_iter()
            .map(|item| Abigen::new(item.contract_name, item.abi).unwrap())
            .collect::<Vec<Abigen>>();

        Ok(MultiAbigen::from_abigens(abigens))
    }
}
