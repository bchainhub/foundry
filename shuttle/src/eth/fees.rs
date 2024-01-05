use crate::eth::{
    backend::{info::StorageInfo, notifications::NewBlockNotifications},
    error::BlockchainError,
};
use corebc::types::{H256, U256};
use foxar_evm::revm::primitives::SpecId;
use futures::StreamExt;
use parking_lot::{Mutex, RwLock};
use shuttle_core::eth::transaction::TypedTransaction;
use std::{
    collections::BTreeMap,
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tracing::trace;

/// Maximum number of entries in the fee history cache
pub const MAX_FEE_HISTORY_CACHE_SIZE: u64 = 2048u64;

/// Initial default energy price for the first block
pub const INITIAL_ENERGY_PRICE: u64 = 1_875_000_000;

/// Bounds the amount the base fee can change between blocks.
pub const BASE_FEE_CHANGE_DENOMINATOR: u64 = 8;

/// Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_ELASTICITY_MULTIPLIER: u64 = 2;

pub fn default_elasticity() -> f64 {
    1f64 / BASE_FEE_CHANGE_DENOMINATOR as f64
}

/// Stores the fee related information
#[derive(Debug, Clone)]
pub struct FeeManager {
    /// Hardfork identifier
    #[allow(dead_code)]
    spec_id: SpecId,
    /// The base price to use Pre London
    ///
    /// This will be constant value unless changed manually
    energy_price: Arc<RwLock<U256>>,
    elasticity: Arc<RwLock<f64>>,
}

// === impl FeeManager ===

impl FeeManager {
    pub fn new(spec_id: SpecId, energy_price: U256) -> Self {
        Self {
            spec_id,
            energy_price: Arc::new(RwLock::new(energy_price)),
            elasticity: Arc::new(RwLock::new(default_elasticity())),
        }
    }

    pub fn elasticity(&self) -> f64 {
        *self.elasticity.read()
    }

    /// Calculates the current energy price
    pub fn energy_price(&self) -> U256 {
        *self.energy_price.read()
    }

    /// Returns the current energy price
    pub fn set_energy_price(&self, price: U256) {
        let mut energy = self.energy_price.write();
        *energy = price;
    }
}

/// An async service that takes care of the `FeeHistory` cache
pub struct FeeHistoryService {
    /// incoming notifications about new blocks
    new_blocks: NewBlockNotifications,
    /// contains all fee history related entries
    cache: FeeHistoryCache,
    /// number of items to consider
    fee_history_limit: u64,
    // current fee info
    #[allow(dead_code)]
    fees: FeeManager,
    /// a type that can fetch ethereum-storage data
    storage_info: StorageInfo,
}

// === impl FeeHistoryService ===

impl FeeHistoryService {
    pub fn new(
        new_blocks: NewBlockNotifications,
        cache: FeeHistoryCache,
        fees: FeeManager,
        storage_info: StorageInfo,
    ) -> Self {
        Self {
            new_blocks,
            cache,
            fee_history_limit: MAX_FEE_HISTORY_CACHE_SIZE,
            fees,
            storage_info,
        }
    }

    /// Returns the configured history limit
    pub fn fee_history_limit(&self) -> u64 {
        self.fee_history_limit
    }

    /// Create a new history entry for the block
    fn create_cache_entry(
        &self,
        hash: H256,
        elasticity: f64,
    ) -> (FeeHistoryCacheItem, Option<u64>) {
        // percentile list from 0.0 to 100.0 with a 0.5 resolution.
        // this will create 200 percentile points
        let reward_percentiles: Vec<f64> = {
            let mut percentile: f64 = 0.0;
            (0..=200)
                .map(|_| {
                    let val = percentile;
                    percentile += 0.5;
                    val
                })
                .collect()
        };

        let mut block_number: Option<u64> = None;
        let mut item = FeeHistoryCacheItem { energy_used_ratio: 0f64, rewards: Vec::new() };

        let current_block = self.storage_info.block(hash);
        let current_receipts = self.storage_info.receipts(hash);

        if let (Some(block), Some(receipts)) = (current_block, current_receipts) {
            block_number = Some(block.header.number.as_u64());

            let energy_used = block.header.energy_used.as_u64() as f64;
            let energy_limit = block.header.energy_limit.as_u64() as f64;

            let energy_target = energy_limit / elasticity;
            item.energy_used_ratio = energy_used / (energy_target * elasticity);

            // extract useful tx info (energy_used, effective_reward)
            let mut transactions: Vec<(u64, u64)> = receipts
                .iter()
                .enumerate()
                .map(|(i, receipt)| {
                    let energy_used = receipt.energy_used().as_u64();
                    let effective_reward = match block.transactions.get(i).map(|tx| &tx.transaction)
                    {
                        Some(TypedTransaction::Legacy(t)) => t.energy_price.as_u64(),
                        None => 0,
                    };

                    (energy_used, effective_reward)
                })
                .collect();

            // sort by effective reward asc
            transactions.sort_by(|(_, a), (_, b)| a.cmp(b));

            // calculate percentile rewards
            item.rewards = reward_percentiles
                .into_iter()
                .filter_map(|p| {
                    let target_energy = (p * energy_used / 100f64) as u64;
                    let mut sum_energy = 0;
                    for (energy_used, effective_reward) in transactions.iter().cloned() {
                        sum_energy += energy_used;
                        if target_energy <= sum_energy {
                            return Some(effective_reward);
                        }
                    }
                    None
                })
                .collect();
        } else {
            item.rewards = reward_percentiles.iter().map(|_| 0).collect();
        }
        (item, block_number)
    }

    fn insert_cache_entry(&self, item: FeeHistoryCacheItem, block_number: Option<u64>) {
        if let Some(block_number) = block_number {
            trace!(target: "fees", "insert new history item={:?} for {}", item, block_number);
            let mut cache = self.cache.lock();
            cache.insert(block_number, item);

            // adhere to cache limit
            let pop_next = block_number.saturating_sub(self.fee_history_limit);

            let num_remove = (cache.len() as u64).saturating_sub(self.fee_history_limit);
            for num in 0..num_remove {
                let key = pop_next - num;
                cache.remove(&key);
            }
        }
    }
}

// An endless future that listens for new blocks and updates the cache
impl Future for FeeHistoryService {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pin = self.get_mut();

        while let Poll::Ready(Some(notification)) = pin.new_blocks.poll_next_unpin(cx) {
            let hash = notification.hash;
            let elasticity = default_elasticity();

            // add the imported block.
            let (result, block_number) = pin.create_cache_entry(hash, elasticity);
            pin.insert_cache_entry(result, block_number)
        }

        Poll::Pending
    }
}

pub type FeeHistoryCache = Arc<Mutex<BTreeMap<u64, FeeHistoryCacheItem>>>;

/// A single item in the whole fee history cache
#[derive(Debug, Clone)]
pub struct FeeHistoryCacheItem {
    pub energy_used_ratio: f64,
    pub rewards: Vec<u64>,
}

#[derive(Default, Clone)]
pub struct FeeDetails {
    pub energy_price: Option<U256>,
}

impl FeeDetails {
    /// All values zero
    pub fn zero() -> Self {
        Self { energy_price: Some(U256::zero()) }
    }

    /// If neither `energy_price` nor `max_fee_per_energy` is `Some`, this will set both to `0`
    pub fn or_zero_fees(self) -> Self {
        let FeeDetails { energy_price } = self;

        let no_fees = energy_price.is_none();
        let energy_price = if no_fees { Some(U256::zero()) } else { energy_price };

        Self { energy_price }
    }

    /// Turns this type into a tuple
    pub fn split(self) -> Option<U256> {
        let Self { energy_price } = self;
        energy_price
    }

    pub fn energy_price(&self) -> Option<U256> {
        self.energy_price
    }

    /// Creates a new instance from the request's energy related values
    pub fn new(request_energy_price: Option<U256>) -> Result<FeeDetails, BlockchainError> {
        let energy_price = request_energy_price;
        Ok(FeeDetails { energy_price })
    }
}

impl fmt::Debug for FeeDetails {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Fees {{ ")?;
        write!(fmt, "gaPrice: {:?}, ", self.energy_price)?;
        write!(fmt, "}}")?;
        Ok(())
    }
}
