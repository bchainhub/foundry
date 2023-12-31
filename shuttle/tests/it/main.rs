mod abi;
mod api;
mod fork;
mod genesis;
mod geth;
mod ipc;
mod logs;
mod proof;
mod pubsub;
mod shuttle;
mod shuttle_api;
// mod revert; // TODO uncomment <https://github.com/gakonst/ethers-rs/issues/2186>
mod sign;
mod traces;
mod transaction;
mod txpool;
pub mod utils;
mod wsapi;

#[allow(unused)]
pub(crate) fn init_tracing() {
    let _ = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}

fn main() {}
