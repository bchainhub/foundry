//! genesis.json tests

use corebc::{abi::Address, prelude::Middleware, types::U256};
use shuttle::{genesis::Genesis, spawn, NodeConfig};

#[tokio::test(flavor = "multi_thread")]
async fn can_apply_genesis() {
    let genesis = r#"{
  "config": {
    "networkId": 19763,
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "ethash": {}
  },
  "nonce": "0xdeadbeefdeadbeef",
  "timestamp": "0x0",
  "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "energyLimit": "0x80000000",
  "difficulty": "0x20000",
  "coinbase": "0x00000000000000000000000000000000000000000000",
  "alloc": {
    "cb6671562b71999873db5b286df957af199ec94617f7": {
      "balance": "0xffffffffffffffffffffffffff"
    }
  },
  "number": "0x0",
  "gasUsed": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}
"#;
    let genesis: Genesis = serde_json::from_str(genesis).unwrap();
    let (_api, handle) = spawn(NodeConfig::test().with_genesis(Some(genesis))).await;

    let provider = handle.http_provider();

    // assert_eq!(provider.get_networkid().await.unwrap(), 19763u64.into());

    let addr: Address = "cb6671562b71999873db5b286df957af199ec94617f7".parse().unwrap();
    let balance = provider.get_balance(addr, None).await.unwrap();

    let expected: U256 = "ffffffffffffffffffffffffff".parse().unwrap();
    assert_eq!(balance, expected);
}
