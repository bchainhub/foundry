//! probe wallet subcommand

pub mod vanity;

use crate::{
    cmd::{probe::wallet::vanity::VanityArgs, Cmd},
    opts::Wallet,
};
use clap::Parser;
use corebc::{
    core::rand::thread_rng,
    signers::{LocalWallet, Signer},
    types::{transaction::cip712::TypedData, Address, Network, Signature},
};
use eyre::Context;

/// CLI arguments for `probe wallet`.
#[derive(Debug, Parser)]
pub enum WalletSubcommands {
    /// Create a new random keypair.
    #[clap(visible_alias = "n")]
    New {
        /// If provided, then keypair will be written to an encrypted JSON keystore.
        path: Option<String>,

        /// Triggers a hidden password prompt for the JSON keystore.
        ///
        /// Deprecated: prompting for a hidden password is now the default.
        #[clap(long, short, requires = "path", conflicts_with = "unsafe_password")]
        password: bool,

        /// Password for the JSON keystore in cleartext.
        ///
        /// This is UNSAFE to use and we recommend using the --password.
        #[clap(long, requires = "path", env = "CAST_PASSWORD", value_name = "PASSWORD")]
        unsafe_password: Option<String>,

        /// Network to use for address prefix validation.
        #[clap(short, long)]
        network: Network,
    },

    /// Generate a vanity address.
    #[clap(visible_alias = "va")]
    Vanity(VanityArgs),

    /// Convert a private key to an address.
    #[clap(visible_aliases = &["a", "addr"])]
    Address {
        /// If provided, the address will be derived from the specified private key.
        #[clap(
            value_name = "PRIVATE_KEY",
            value_parser = foxar_common::clap_helpers::strip_0x_prefix,
        )]
        private_key_override: Option<String>,

        #[clap(flatten)]
        wallet: Wallet,
    },

    /// Sign a message or typed data.
    #[clap(visible_alias = "s")]
    Sign {
        /// The message or typed data to sign.
        ///
        /// Messages starting with 0x are expected to be hex encoded,
        /// which get decoded before being signed.
        /// The message will be prefixed with the Ethereum Signed Message header and hashed before
        /// signing.
        ///
        /// Typed data can be provided as a json string or a file name.
        /// Use --data flag to denote the message is a string of typed data.
        /// Use --data --from-file to denote the message is a file name containing typed data.
        /// The data will be combined and hashed using the EIP712 specification before signing.
        /// The data should be formatted as JSON.
        message: String,

        /// If provided, the message will be treated as typed data.
        #[clap(long)]
        data: bool,

        /// If provided, the message will be treated as a file name containing typed data. Requires
        /// --data.
        #[clap(long, requires = "data")]
        from_file: bool,

        #[clap(flatten)]
        wallet: Wallet,
    },

    /// Verify the signature of a message.
    #[clap(visible_alias = "v")]
    Verify {
        /// The original message.
        message: String,

        /// The signature to verify.
        signature: Signature,

        /// The address of the message signer.
        #[clap(long, short)]
        address: Address,

        /// Network to use for address prefix validation.
        #[clap(short, long)]
        network: Network,
    },
}

impl WalletSubcommands {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            WalletSubcommands::New { path, unsafe_password, network, .. } => {
                let mut rng = thread_rng();

                if let Some(path) = path {
                    let path = dunce::canonicalize(path)?;
                    if !path.is_dir() {
                        // we require path to be an existing directory
                        eyre::bail!("`{}` is not a directory", path.display());
                    }

                    let password = if let Some(password) = unsafe_password {
                        password
                    } else {
                        // if no --unsafe-password was provided read via stdin
                        rpassword::prompt_password("Enter secret: ")?
                    };

                    let (wallet, uuid) =
                        LocalWallet::new_keystore(&path, &mut rng, password, None, network)?;

                    println!("Created new encrypted keystore file: {}", path.join(uuid).display());
                    println!("Address: {}", wallet.address());
                } else {
                    let wallet = LocalWallet::new(&mut rng, network);
                    println!("Successfully created new keypair.");
                    println!("Address:     {}", wallet.address());
                    println!("Private key: 0x{}", hex::encode(wallet.signer().to_bytes()));
                }
            }
            WalletSubcommands::Vanity(cmd) => {
                cmd.run()?;
            }
            WalletSubcommands::Address { wallet, private_key_override } => {
                if wallet.wallet_network.is_none() {
                    eyre::bail!("--wallet.network is required");
                }
                let wallet = private_key_override
                    .map(|pk| Wallet { private_key: Some(pk), ..Default::default() })
                    .unwrap_or(wallet.clone())
                    .signer(u64::from(wallet.wallet_network.unwrap()))
                    .await?;
                let addr = wallet.address();
                println!("{}", addr);
            }
            WalletSubcommands::Sign { message, data, from_file, wallet } => {
                if wallet.wallet_network.is_none() {
                    eyre::bail!("--wallet.network is required");
                }
                let wallet = wallet.signer(u64::from(wallet.wallet_network.unwrap())).await?;
                let sig = if data {
                    let typed_data: TypedData = if from_file {
                        // data is a file name, read json from file
                        foxar_common::fs::read_json_file(message.as_ref())?
                    } else {
                        // data is a json string
                        serde_json::from_str(&message)?
                    };
                    wallet.sign_typed_data(&typed_data).await?
                } else {
                    wallet.sign_message(Self::hex_str_to_bytes(&message)?).await?
                };
                println!("0x{sig}");
            }
            WalletSubcommands::Verify { message, signature, address, network } => {
                match signature.verify(Self::hex_str_to_bytes(&message)?, &network, address) {
                    Ok(_) => {
                        println!("Validation succeeded. Address {address} signed this message.")
                    }
                    Err(_) => {
                        println!("Validation failed. Address {address} did not sign this message.")
                    }
                }
            }
        };

        Ok(())
    }

    fn hex_str_to_bytes(s: &str) -> eyre::Result<Vec<u8>> {
        Ok(match s.strip_prefix("0x") {
            Some(data) => hex::decode(data).wrap_err("Could not decode 0x-prefixed string.")?,
            None => s.as_bytes().to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_wallet_sign_message() {
        let args = WalletSubcommands::parse_from(["foxar-cli", "sign", "deadbeef"]);
        match args {
            WalletSubcommands::Sign { message, data, from_file, .. } => {
                assert_eq!(message, "deadbeef".to_string());
                assert!(!data);
                assert!(!from_file);
            }
            _ => panic!("expected WalletSubcommands::Sign"),
        }
    }

    #[test]
    fn can_parse_wallet_sign_hex_message() {
        let args = WalletSubcommands::parse_from(["foxar-cli", "sign", "0xdeadbeef"]);
        match args {
            WalletSubcommands::Sign { message, data, from_file, .. } => {
                assert_eq!(message, "0xdeadbeef".to_string());
                assert!(!data);
                assert!(!from_file);
            }
            _ => panic!("expected WalletSubcommands::Sign"),
        }
    }

    #[test]
    fn can_parse_wallet_sign_data() {
        let args = WalletSubcommands::parse_from(["foxar-cli", "sign", "--data", "{ ... }"]);
        match args {
            WalletSubcommands::Sign { message, data, from_file, .. } => {
                assert_eq!(message, "{ ... }".to_string());
                assert!(data);
                assert!(!from_file);
            }
            _ => panic!("expected WalletSubcommands::Sign"),
        }
    }

    #[test]
    fn can_parse_wallet_sign_data_file() {
        let args = WalletSubcommands::parse_from([
            "foxar-cli",
            "sign",
            "--data",
            "--from-file",
            "tests/data/typed_data.json",
        ]);
        match args {
            WalletSubcommands::Sign { message, data, from_file, .. } => {
                assert_eq!(message, "tests/data/typed_data.json".to_string());
                assert!(data);
                assert!(from_file);
            }
            _ => panic!("expected WalletSubcommands::Sign"),
        }
    }
}
