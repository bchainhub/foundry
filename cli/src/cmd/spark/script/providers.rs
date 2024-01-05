use corebc::{
    prelude::{Http, Middleware, Provider, RetryClient, U256},
    types::Network,
};
use eyre::WrapErr;
use foxar_common::{get_http_provider, RpcUrl};

use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Deref,
    sync::Arc,
};

/// Contains a map of RPC urls to single instances of [`ProviderInfo`].
#[derive(Default)]
pub struct ProvidersManager {
    pub inner: HashMap<RpcUrl, ProviderInfo>,
}

impl ProvidersManager {
    /// Get or initialize the RPC provider.
    pub async fn get_or_init_provider(&mut self, rpc: &str) -> eyre::Result<&ProviderInfo> {
        Ok(match self.inner.entry(rpc.to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let info = ProviderInfo::new(rpc).await?;
                entry.insert(info)
            }
        })
    }
}

impl Deref for ProvidersManager {
    type Target = HashMap<RpcUrl, ProviderInfo>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Holds related metadata to each provider RPC.
#[derive(Debug)]
pub struct ProviderInfo {
    pub provider: Arc<Provider<RetryClient<Http>>>,
    pub network: Network,
    pub energy_price: EnergyPrice,
}

/// Represents the outcome of a energy price request
#[derive(Debug)]
pub enum EnergyPrice {
    Legacy(eyre::Result<U256>),
}

impl ProviderInfo {
    pub async fn new(rpc: &str) -> eyre::Result<ProviderInfo> {
        let provider = Arc::new(get_http_provider(rpc));
        let network = Network::from(provider.get_networkid().await?.as_u64());

        let energy_price =
            provider.get_energy_price().await.wrap_err("Failed to get legacy energy price");
        if energy_price.is_ok() {
            let energy_price = EnergyPrice::Legacy(energy_price);
            return Ok(ProviderInfo { provider, network, energy_price });
        }
        Err(eyre::eyre!("{}", energy_price.unwrap_err()))
    }

    /// Returns the energy price to use
    pub fn energy_price(&self) -> eyre::Result<U256> {
        let res = match &self.energy_price {
            EnergyPrice::Legacy(res) => res.as_ref(),
        };
        match res {
            Ok(val) => Ok(*val),
            Err(err) => Err(eyre::eyre!("{}", err)),
        }
    }
}
