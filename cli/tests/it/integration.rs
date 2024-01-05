use foxar_cli_test_utils::{sparktest_external, util::setup_spark_remote};

// sparktest_external!(solmate, "transmissions11/solmate");
// sparktest_external!(prb_math, "PaulRBerg/prb-math");
// sparktest_external!(prb_proxy, "PaulRBerg/prb-proxy");
// sparktest_external!(solady, "Vectorized/solady");
// sparktest_external!(
//     geb,
//     "reflexer-labs/geb",
//     &["--chain-id", "99", "--sender", "0xcb5400a329c0648769a73afac7f9381e08fb43dbea72"]
// );
// sparktest_external!(stringutils, "Arachnid/solidity-stringutils");
// sparktest_external!(lootloose, "gakonst/lootloose");
// sparktest_external!(lil_web3, "m1guelpf/lil-web3");

/// clone + build in one step
#[test]
#[ignore]
fn can_checkout_build() {
    let (_prj, _cmd) = setup_spark_remote("transmissions11/solmate");
}

/// Forking tests
mod fork_integration {
    // use foxar_cli_test_utils::sparktest_external;

    // sparktest_external!(multicall, "makerdao/multicall", &["--block-number", "1"]);
    // sparktest_external!(
    //     drai,
    //     "mds1/drai",
    //     13633752,
    //     &["--chain-id", "99", "--sender", "0xcb5400a329c0648769a73afac7f9381e08fb43dbea72"]
    // );
    // sparktest_external!(gunilev, "hexonaut/guni-lev", 13633752);
    // sparktest_external!(convex, "mds1/convex-shutdown-simulation", 14445961);
}
