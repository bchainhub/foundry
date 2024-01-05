//! tests for `eth_getProof`

use corebc::{
    abi::ethereum_types::BigEndianHash,
    types::{Address, H256, U256},
};
use shuttle::{spawn, NodeConfig};

use shuttle_core::eth::proof::{AccountProof, BasicAccount};

use crate::proof::eip1186::verify_proof;
use corebc::utils::{rlp, sha3};
use foxar_evm::revm::primitives::SHA3_EMPTY;
use shuttle_core::eth::trie::ExtensionLayout;

mod eip1186;

#[tokio::test(flavor = "multi_thread")]
#[ignore = "Fix later"]
async fn can_get_proof() {
    let (api, _handle) = spawn(NodeConfig::test()).await;

    let acc: Address = "0xcb66aaaf5374fce5edbc8e2a8697c15331677e6ebaaa".parse().unwrap();

    let key = U256::zero();
    let value = U256::one();

    api.shuttle_set_storage_at(acc, key, H256::from_uint(&value)).await.unwrap();

    let proof: AccountProof = api.get_proof(acc, vec![H256::from_uint(&key)], None).await.unwrap();

    let account = BasicAccount {
        nonce: 0.into(),
        balance: 0.into(),
        storage_root: proof.storage_hash,
        code_hash: SHA3_EMPTY.into(),
    };

    let rlp_account = rlp::encode(&account);

    let root: H256 = api.state_root().await.unwrap();
    let acc_proof: Vec<Vec<u8>> = proof.account_proof.into_iter().map(|b| b.to_vec()).collect();

    verify_proof::<ExtensionLayout>(
        &root.0,
        &acc_proof,
        &sha3(acc.as_bytes())[..],
        Some(rlp_account.as_ref()),
    )
    .unwrap();

    assert_eq!(proof.storage_proof.len(), 1);
    let expected_value = rlp::encode(&value);
    let proof = proof.storage_proof[0].clone();
    let storage_proof: Vec<Vec<u8>> = proof.proof.into_iter().map(|b| b.to_vec()).collect();
    verify_proof::<ExtensionLayout>(
        &account.storage_root.0,
        &storage_proof,
        proof.key.as_bytes(),
        Some(expected_value.as_ref()),
    )
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn can_get_random_account_proofs() {
    let (api, _handle) = spawn(NodeConfig::test()).await;

    for acc in std::iter::repeat_with(Address::random).take(10) {
        let _ = api
            .get_proof(acc, Vec::new(), None)
            .await
            .unwrap_or_else(|_| panic!("Failed to get proof for {acc:?}"));
    }
}
