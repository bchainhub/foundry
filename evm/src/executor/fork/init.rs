use crate::utils::{h176_to_b176, u256_to_ru256};
use corebc::{
    providers::Middleware,
    types::{Address, Block, TxHash, U256},
};
use eyre::WrapErr;
use foxar_common::NON_ARCHIVE_NODE_WARNING;
use futures::TryFutureExt;
use revm::primitives::{BlockEnv, CfgEnv, Env, TxEnv};

/// Initializes a REVM block environment based on a forked
/// ethereum provider.
pub async fn environment<M: Middleware>(
    provider: &M,
    memory_limit: u64,
    energy_price: Option<u64>,
    override_network_id: Option<u64>,
    pin_block: Option<u64>,
    origin: Address,
) -> eyre::Result<(Env, Block<TxHash>)>
where
    M::Error: 'static,
{
    let block_number = if let Some(pin_block) = pin_block {
        pin_block
    } else {
        provider.get_block_number().await.wrap_err("Failed to get latest block number")?.as_u64()
    };
    let (fork_energy_price, rpc_network_id, block) = tokio::try_join!(
        provider
            .get_energy_price()
            .map_err(|err| { eyre::Error::new(err).wrap_err("Failed to get energy price") }),
        provider
            .get_networkid()
            .map_err(|err| { eyre::Error::new(err).wrap_err("Failed to get network id") }),
        provider.get_block(block_number).map_err(|err| {
            eyre::Error::new(err).wrap_err(format!("Failed to get block {block_number}"))
        })
    )?;
    let block = if let Some(block) = block {
        block
    } else {
        if let Ok(latest_block) = provider.get_block_number().await {
            // If the `eth_getBlockByNumber` call succeeds, but returns null instead of
            // the block, and the block number is less than equal the latest block, then
            // the user is forking from a non-archive node with an older block number.
            if block_number <= latest_block.as_u64() {
                error!("{NON_ARCHIVE_NODE_WARNING}");
            }
            eyre::bail!(
                "Failed to get block for block number: {}\nlatest block number: {}",
                block_number,
                latest_block
            );
        }
        eyre::bail!("Failed to get block for block number: {}", block_number)
    };

    let env = Env {
        cfg: CfgEnv {
            network_id: override_network_id.unwrap_or(rpc_network_id.as_u64()),
            memory_limit,
            limit_contract_code_size: Some(usize::MAX),
            ..Default::default()
        },
        block: BlockEnv {
            number: u256_to_ru256(block.number.expect("block number not found").as_u64().into()),
            timestamp: u256_to_ru256(block.timestamp),
            coinbase: h176_to_b176(block.author.unwrap_or_default()),
            difficulty: u256_to_ru256(block.difficulty),
            energy_limit: u256_to_ru256(block.energy_limit),
        },
        tx: TxEnv {
            caller: h176_to_b176(origin),
            energy_price: u256_to_ru256(energy_price.map(U256::from).unwrap_or(fork_energy_price)),
            network_id: Some(override_network_id.unwrap_or(rpc_network_id.as_u64())),
            energy_limit: block.energy_limit.as_u64(),
            ..Default::default()
        },
    };

    Ok((env, block))
}
