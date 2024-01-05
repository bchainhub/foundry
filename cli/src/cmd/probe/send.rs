// probe send subcommands
use crate::{
    opts::{EthereumOpts, TransactionOpts},
    utils,
};
use clap::Parser;
use corebc::{
    core::types::Network, prelude::MiddlewareBuilder, providers::Middleware, signers::Signer,
    types::NameOrAddress,
};
use foxar_common::cli_warn;
use foxar_config::Config;
use probe::{Cast, TxBuilder};
use std::str::FromStr;

/// CLI arguments for `probe send`.
#[derive(Debug, Parser)]
pub struct SendTxArgs {
    /// The destination of the transaction.
    ///
    /// If not provided, you must use probe send --create.
    #[clap(value_parser = NameOrAddress::from_str)]
    to: Option<NameOrAddress>,

    /// The signature of the function to call.
    sig: Option<String>,

    /// The arguments of the function to call.
    args: Vec<String>,

    /// Only print the transaction hash and exit immediately.
    #[clap(name = "async", long = "async", alias = "probe-async", env = "CAST_ASYNC")]
    probe_async: bool,

    #[clap(flatten)]
    tx: TransactionOpts,

    #[clap(flatten)]
    eth: EthereumOpts,

    /// The number of confirmations until the receipt is fetched.
    #[clap(long, default_value = "1")]
    confirmations: usize,

    /// Print the transaction receipt as JSON.
    #[clap(long, short, help_heading = "Display options")]
    json: bool,

    /// Reuse the latest nonce for the sender account.
    #[clap(long, conflicts_with = "nonce")]
    resend: bool,

    #[clap(subcommand)]
    command: Option<SendTxSubcommands>,

    /// Send via `eth_sendTransaction using the `--from` argument or $ETH_FROM as sender
    #[clap(long, requires = "from")]
    unlocked: bool,
}

#[derive(Debug, Parser)]
pub enum SendTxSubcommands {
    /// Use to deploy raw contract bytecode.
    #[clap(name = "--create")]
    Create {
        /// The bytecode of the contract to deploy.
        code: String,

        /// The signature of the function to call.
        sig: Option<String>,

        /// The arguments of the function to call.
        args: Vec<String>,
    },
}

impl SendTxArgs {
    pub async fn run(self) -> eyre::Result<()> {
        let SendTxArgs {
            eth,
            to,
            sig,
            probe_async,
            mut args,
            mut tx,
            confirmations,
            json: to_json,
            resend,
            command,
            unlocked,
        } = self;
        let config = Config::from(&eth);
        let provider = utils::get_provider(&config)?;
        let network = utils::get_network(config.network_id, &provider).await?;
        // let api_key = config.get_etherscan_api_key(Some(network));
        let mut sig = sig.unwrap_or_default();

        let code = if let Some(SendTxSubcommands::Create {
            code,
            sig: constructor_sig,
            args: constructor_args,
        }) = command
        {
            sig = constructor_sig.unwrap_or_default();
            args = constructor_args;
            Some(code)
        } else {
            None
        };

        // Case 1:
        // Default to sending via eth_sendTransaction if the --unlocked flag is passed.
        // This should be the only way this RPC method is used as it requires a local node
        // or remote RPC with unlocked accounts.
        if unlocked {
            // only check current chain id if it was specified in the config
            if let Some(config_chain) = config.network_id {
                let current_network_id = provider.get_networkid().await?.as_u64();
                let config_network_id = current_network_id;
                // switch chain if current chain id is not the same as the one specified in the
                // config
                if config_network_id != current_network_id {
                    cli_warn!("Switching to chain {}", config_chain);
                    provider
                        .request(
                            "wallet_switchEthereumChain",
                            [serde_json::json!({
                                "chainId": format!("0x{:x}", config_network_id),
                            })],
                        )
                        .await?;
                }
            }

            if resend {
                tx.nonce = Some(provider.get_transaction_count(config.sender, None).await?);
            }

            probe_send(
                provider,
                config.sender,
                to,
                code,
                (sig, args),
                tx,
                network,
                // api_key,
                probe_async,
                confirmations,
                to_json,
            )
            .await
        // Case 2:
        // An option to use a local signer was provided.
        // If we cannot successfully instanciate a local signer, then we will assume we don't have
        // enough information to sign and we must bail.
        } else {
            // Retrieve the signer, and bail if it can't be constructed.
            let signer = eth.wallet.signer(u64::from(network)).await?;
            let from = signer.address();

            // prevent misconfigured hwlib from sending a transaction that defies
            // user-specified --from
            if let Some(specified_from) = eth.wallet.from {
                if specified_from != from {
                    eyre::bail!(
                        "\
The specified sender via CLI/env vars does not match the sender configured via
the hardware wallet's HD Path.
Please use the `--hd-path <PATH>` parameter to specify the BIP32 Path which
corresponds to the sender, or let foxar automatically detect it by not specifying any sender address."
                    )
                }
            }

            if resend {
                tx.nonce = Some(provider.get_transaction_count(from, None).await?);
            }

            let provider = provider.with_signer(signer);

            probe_send(
                provider,
                from,
                to,
                code,
                (sig, args),
                tx,
                network,
                // api_key,
                probe_async,
                confirmations,
                to_json,
            )
            .await
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn probe_send<M: Middleware, F: Into<NameOrAddress>, T: Into<NameOrAddress>>(
    provider: M,
    from: F,
    to: Option<T>,
    code: Option<String>,
    args: (String, Vec<String>),
    tx: TransactionOpts,
    chain: Network,
    // etherscan_api_key: Option<String>,
    probe_async: bool,
    confs: usize,
    to_json: bool,
) -> eyre::Result<()>
where
    M::Error: 'static,
{
    let (sig, params) = args;
    let params = if !sig.is_empty() { Some((&sig[..], params)) } else { None };
    let mut builder = TxBuilder::new(&provider, from, to, chain).await?;
    builder
        // .etherscan_api_key(etherscan_api_key)
        .energy(tx.energy_limit)
        .energy_price(tx.energy_price)
        .value(tx.value)
        .nonce(tx.nonce);

    if let Some(code) = code {
        let mut data = hex::decode(code.strip_prefix("0x").unwrap_or(&code))?;

        if let Some((sig, args)) = params {
            let (mut sigdata, _) = builder.create_args(sig, args).await?;
            data.append(&mut sigdata);
        }

        builder.set_data(data);
    } else {
        builder.args(params).await?;
    };
    let builder_output = builder.build();

    let probe = Cast::new(provider);

    let pending_tx = probe.send(builder_output).await?;
    let tx_hash = *pending_tx;

    if probe_async {
        println!("{tx_hash:#x}");
    } else {
        let receipt = probe.receipt(format!("{tx_hash:#x}"), None, confs, false, to_json).await?;
        println!("{receipt}");
    }

    Ok(())
}
