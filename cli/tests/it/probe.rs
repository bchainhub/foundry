//! Contains various tests for checking probe commands

use clap::CommandFactory;
use foxar_cli::opts::probe::Opts;
use foxar_cli_test_utils::{
    probetest,
    util::{OutputExt, TestCommand, TestProject},
};
use foxar_utils::rpc::http_rpc_endpoint;
use std::{io::Write, path::PathBuf};

// tests `--help` is printed to std out
probetest!(print_help, |_: TestProject, mut cmd: TestCommand| {
    cmd.arg("--help");
    cmd.assert_non_empty_stdout();
});

// tests `--help` for all subcommand
probetest!(print_probe_subcommand_help, |_: TestProject, mut cmd: TestCommand| {
    let probe = Opts::command();
    for sub_command in probe.get_subcommands() {
        cmd.probe_fuse().args([sub_command.get_name(), "--help"]);
        cmd.assert_non_empty_stdout();
    }
});

// tests that the `probe block` command works correctly
probetest!(latest_block, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();

    // Call `probe find-block`
    cmd.args(["block", "latest", "--rpc-url", eth_rpc_url.as_str()]);
    let output = cmd.stdout_lossy();
    assert!(output.contains("transactions:"));
    assert!(output.contains("energyUsed"));

    // <https://etherscan.io/block/15007840>
    cmd.probe_fuse().args(["block", "6820110", "-f", "hash", "--rpc-url", eth_rpc_url.as_str()]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim(), "0x71bcf7f7df0ac8dde1baec6c13fc70b4438c3c6e5655d3d62893be164f4377cb")
});

// tests that the `probe find-block` command works correctly
probetest!(finds_block, |_: TestProject, mut cmd: TestCommand| {
    // Construct args
    let timestamp = "1700574718".to_string();
    let eth_rpc_url = http_rpc_endpoint();

    // Call `probe find-block`
    cmd.args(["find-block", "--rpc-url", eth_rpc_url.as_str(), &timestamp]);
    let output = cmd.stdout_lossy();
    println!("{output}");

    // Expect successful block query
    // Query: 1647843609, Mar 21 2022 06:20:09 UTC
    // Output block: https://etherscan.io/block/14428082
    // Output block time: Mar 21 2022 06:20:09 UTC
    assert!(output.contains("6820109"), "{}", output);
});

// tests that we can create a new wallet with keystore
probetest!(new_wallet_keystore_with_password, |_: TestProject, mut cmd: TestCommand| {
    cmd.args(["wallet", "new", ".", "--unsafe-password", "test", "--network", "1"]);
    let out = cmd.stdout_lossy();
    assert!(out.contains("Created new encrypted keystore file"));
    assert!(out.contains("Address"));
});

// tests that we can get the address of a keystore file
probetest!(wallet_address_keystore_with_password_file, |_: TestProject, mut cmd: TestCommand| {
    let keystore_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/keystore");

    cmd.args([
        "wallet",
        "address",
        "--wallet-network",
        "1",
        "--keystore",
        keystore_dir
            .join(
                "UTC--2023-11-17T08-49-29.100000000Z--cb65e49851f010cd7d81b5b4969f3b0e8325c415359d",
            )
            .to_str()
            .unwrap(),
        "--password-file",
        keystore_dir.join("password-ec554").to_str().unwrap(),
    ]);
    let out = cmd.stdout_lossy();
    assert!(out.contains("cb65e49851f010cd7d81b5b4969f3b0e8325c415359d"));
});

// tests that `probe wallet sign message` outputs the expected signature
probetest!(probe_wallet_sign_message_utf8_data, |_: TestProject, mut cmd: TestCommand| {
    cmd.args([
        "wallet",
        "sign",
        "--wallet-network",
        "1",
        "--private-key",
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
        "test",
    ]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim(), "0x1611b04e357853a6f7c71f11e8171402a6617eb9408c8050ea6fc1fae399d13065b839c5a6b910259761ee9cd357f703416e90e1700ff20000e393a963a36853db504b229ae5a99d04fb3aba06d62d74c62a792283d0d51c0a31ff95dcfd6d22edae35f5c6955b0a7e3bf0f270b3b1011500cce9b032abc93b3607188cb68fb3630ef23b9dabf1a8860baa1420532d2285bca721427d5075a4678bdc61d6e380b4998ac48172805ad7c580");
});

// tests that `probe wallet sign message` outputs the expected signature, given a 0x-prefixed data
probetest!(probe_wallet_sign_message_hex_data, |_: TestProject, mut cmd: TestCommand| {
    cmd.args([
        "wallet",
        "sign",
        "--wallet-network",
        "1",
        "--private-key",
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    ]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim(), "0x5d6ccf8ed3d26506106132fe9fb59893065d41e27222e9bfa673607f072adb95d55640bd2d5e4afc40e6d7b88a4ea64374e92000baae65c9801441ab6d5152019940fb334e24607c633d66d1a7286b3c7f8c1f28ab5dda65f6e869c41b1abb2ee3b4fc3aed77e500a794bcd51558ece30c00cce9b032abc93b3607188cb68fb3630ef23b9dabf1a8860baa1420532d2285bca721427d5075a4678bdc61d6e380b4998ac48172805ad7c580");
});

// tests that `probe wallet sign typed-data` outputs the expected signature, given a JSON string
probetest!(probe_wallet_sign_typed_data_string, |_: TestProject, mut cmd: TestCommand| {
    cmd.args([
        "wallet",
        "sign",
        "--wallet-network",
        "1",
        "--private-key",
        "168956d1b5cc0ff15c8705e25b93a64e81b195f4fb8fa959d91c8d4d9045486af00e24cc2498c450178642517d7b27333c137e434353f1f02f",
        "--data",
        "{\"types\": {\"CIP712Domain\": [{\"name\": \"name\",\"type\": \"string\"},{\"name\": \"version\",\"type\": \"string\"},{\"name\": \"networkId\",\"type\": \"uint256\"},{\"name\": \"verifyingContract\",\"type\": \"address\"}],\"Message\": [{\"name\": \"data\",\"type\": \"string\"}]},\"primaryType\": \"Message\",\"domain\": {\"name\": \"example.metamask.io\",\"version\": \"1\",\"networkId\": \"1\",\"verifyingContract\": \"cb375a538daf54f2e568bb4237357b1cee1aa3cb7eba\"},\"message\": {\"data\": \"Hello!\"}}",
    ]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim(), "0xf6ed02416347c64926700e2ad5dcbfc5633cf4d1a665b4944ce6e8f42e74428f297d94960679f5edd01693946fa0e4ced2e69622dcf8d95f809893141848170572742df9cfce20c6bf48837285a7ee58615d9bae2c3b6faf768ffd33bf35524b58ad89761f2b2228b45f04f12eb707db220029b84bcbeddddb185ca7e6be50e8295d6dd9a27024a199a8b25ceb24bc32b1a815c7c7b61f88a29c4af9a3cd89d84051218bdb518dd851c700");
});

// tests that `probe wallet sign typed-data` outputs the expected signature, given a JSON file
probetest!(probe_wallet_sign_typed_data_file, |_: TestProject, mut cmd: TestCommand| {
    cmd.args([
        "wallet",
        "sign",
        "--wallet-network",
        "1",
        "--private-key",
        "168956d1b5cc0ff15c8705e25b93a64e81b195f4fb8fa959d91c8d4d9045486af00e24cc2498c450178642517d7b27333c137e434353f1f02f",
        "--data",
        "--from-file",
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/sign_typed_data.json")
            .into_os_string()
            .into_string()
            .unwrap()
            .as_str(),
    ]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim(), "0xf6ed02416347c64926700e2ad5dcbfc5633cf4d1a665b4944ce6e8f42e74428f297d94960679f5edd01693946fa0e4ced2e69622dcf8d95f809893141848170572742df9cfce20c6bf48837285a7ee58615d9bae2c3b6faf768ffd33bf35524b58ad89761f2b2228b45f04f12eb707db220029b84bcbeddddb185ca7e6be50e8295d6dd9a27024a199a8b25ceb24bc32b1a815c7c7b61f88a29c4af9a3cd89d84051218bdb518dd851c700");
});

// tests that `probe estimate` is working correctly.
probetest!(estimate_function_gas, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();
    cmd.args([
        "estimate",
        "0xcb656dadee521bea601692312454a655a0f49051ddc9",
        "--value",
        "100",
        "deposit()",
        "--rpc-url",
        eth_rpc_url.as_str(),
    ]);
    let out: u32 = cmd.stdout_lossy().trim().parse().unwrap();
    // ensure we get a positive non-error value for gas estimate
    assert!(out.ge(&0));
});

// tests that `probe estimate --create` is working correctly.
probetest!(estimate_contract_deploy_gas, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();
    // sample contract code bytecode. Wouldn't run but is valid bytecode that the estimate method
    // accepts and could be deployed.
    cmd.args([
        "estimate",
        "--rpc-url",
        eth_rpc_url.as_str(),
        "--create",
        "0000",
        "CBC20(uint256,string,string)",
        "100",
        "Test",
        "TST",
    ]);

    let gas: u32 = cmd.stdout_lossy().trim().parse().unwrap();
    // ensure we get a positive non-error value for gas estimate
    assert!(gas > 0);
});

//TODO:error2215 fix this test after function signature uploading will be fixed
// tests that the `probe upload-signatures` command works correctly
// probetest!(upload_signatures, |_: TestProject, mut cmd: TestCommand| {
//     // test no prefix is accepted as function
//     cmd.args(["upload-signature", "transfer(address,uint256)"]);
//     let output = cmd.stdout_lossy();

//     assert!(output.contains("Function transfer(address,uint256): 0xa9059cbb"), "{}", output);

//     // test event prefix
//     cmd.args(["upload-signature", "event Transfer(address,uint256)"]);
//     let output = cmd.stdout_lossy();

//     assert!(output.contains("Event Transfer(address,uint256):
// 0x69ca02dd4edd7bf0a4abb9ed3b7af3f14778db5d61921c7dc7cd545266326de2"), "{}", output);

//     // test multiple sigs
//     cmd.args([
//         "upload-signature",
//         "event Transfer(address,uint256)",
//         "transfer(address,uint256)",
//         "approve(address,uint256)",
//     ]);
//     let output = cmd.stdout_lossy();

//     assert!(output.contains("Event Transfer(address,uint256):
// 0x69ca02dd4edd7bf0a4abb9ed3b7af3f14778db5d61921c7dc7cd545266326de2"), "{}", output);     assert!
// (output.contains("Function transfer(address,uint256): 0xa9059cbb"), "{}", output);     assert!
// (output.contains("Function approve(address,uint256): 0x095ea7b3"), "{}", output);

//     // test abi
//     cmd.args([
//         "upload-signature",
//         "event Transfer(address,uint256)",
//         "transfer(address,uint256)",
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("tests/fixtures/ERC20Artifact.json")
//             .into_os_string()
//             .into_string()
//             .unwrap()
//             .as_str(),
//     ]);
//     let output = cmd.stdout_lossy();

//     assert!(output.contains("Event Transfer(address,uint256):
// 0x69ca02dd4edd7bf0a4abb9ed3b7af3f14778db5d61921c7dc7cd545266326de2"), "{}", output);     assert!
// (output.contains("Function transfer(address,uint256): 0xa9059cbb"), "{}", output);     assert!
// (output.contains("Function approve(address,uint256): 0x095ea7b3"), "{}", output);     assert!
// (output.contains("Function decimals(): 0x313ce567"), "{}", output);     assert!(output.contains("
// Function allowance(address,address): 0xdd62ed3e"), "{}", output); });

// tests that the `probe to-rlp` and `probe from-rlp` commands work correctly
probetest!(probe_rlp, |_: TestProject, mut cmd: TestCommand| {
    cmd.args(["--to-rlp", "[\"0xaa\", [[\"bb\"]], \"0xcc\"]"]);
    let out = cmd.stdout_lossy();
    assert!(out.contains("0xc881aac3c281bb81cc"), "{}", out);

    cmd.probe_fuse();
    cmd.args(["--from-rlp", "0xcbc58455556666c0c0c2c1c0"]);
    let out = cmd.stdout_lossy();
    assert!(out.contains("[[\"0x55556666\"],[],[],[[[]]]]"), "{}", out);
});

// test for probe_rpc without arguments
probetest!(probe_rpc_no_args, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();

    // Call `probe rpc xcb_networkId`
    cmd.args(["rpc", "--rpc-url", eth_rpc_url.as_str(), "xcb_networkId"]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim_end(), r#""0x1""#);
});

// test for probe_rpc with arguments
probetest!(probe_rpc_with_args, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();

    // Call `probe rpc eth_getBlockByNumber 0x123 false`
    cmd.args(["rpc", "--rpc-url", eth_rpc_url.as_str(), "xcb_getBlockByNumber", "0x123", "false"]);
    let output = cmd.stdout_lossy();
    assert!(output.contains(r#""number":"0x123""#), "{}", output);
});

// test for probe_rpc with raw params
probetest!(probe_rpc_raw_params, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();

    // Call `probe rpc xcb_getBlockByNumber --raw '["0x123", false]'`
    cmd.args([
        "rpc",
        "--rpc-url",
        eth_rpc_url.as_str(),
        "xcb_getBlockByNumber",
        "--raw",
        r#"["0x123", false]"#,
    ]);
    let output = cmd.stdout_lossy();
    assert!(output.contains(r#""number":"0x123""#), "{}", output);
});

// test for probe_rpc with direct params
probetest!(probe_rpc_raw_params_stdin, |_: TestProject, mut cmd: TestCommand| {
    let eth_rpc_url = http_rpc_endpoint();

    // Call `echo "\n[\n\"0x123\",\nfalse\n]\n" | probe rpc  xcb_getBlockByNumber --raw
    cmd.args(["rpc", "--rpc-url", eth_rpc_url.as_str(), "xcb_getBlockByNumber", "--raw"]).stdin(
        |mut stdin| {
            stdin.write_all(b"\n[\n\"0x123\",\nfalse\n]\n").unwrap();
        },
    );
    let output = cmd.stdout_lossy();
    assert!(output.contains(r#""number":"0x123""#), "{}", output);
});

// checks `probe calldata` can handle arrays
probetest!(calldata_array, |_: TestProject, mut cmd: TestCommand| {
    cmd.args(["calldata", "propose(string[])", "[\"\"]"]);
    let out = cmd.stdout_lossy();
    assert_eq!(out.trim(),"0xdfbd84700000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000"
    );
});

probetest!(
    //todo:error2215 need an archive node
    #[ignore]
    probe_run_succeeds,
    |_: TestProject, mut cmd: TestCommand| {
        let rpc = http_rpc_endpoint();
        cmd.args([
            "run",
            "-v",
            "0xcedb08f67df469cbb947390a4aef71b950d192d98c68005abb23f0ce8652c37d",
            "--quick",
            "--rpc-url",
            rpc.as_str(),
        ]);
        let output = cmd.stdout_lossy();
        assert!(output.contains("Transaction successfully executed"));
        assert!(!output.contains("Revert"));
    }
);

// tests that `probe --to-base` commands are working correctly.
probetest!(probe_to_base, |_: TestProject, mut cmd: TestCommand| {
    let values = [
        "1",
        "100",
        "100000",
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "-1",
        "-100",
        "-100000",
        "-57896044618658097711785492504343953926634992332820282019728792003956564819968",
    ];
    for value in values {
        for subcmd in ["--to-base", "--to-hex", "--to-dec"] {
            if subcmd == "--to-base" {
                for base in ["bin", "oct", "dec", "hex"] {
                    cmd.probe_fuse().args([subcmd, value, base]);
                    assert!(!cmd.stdout_lossy().trim().is_empty());
                }
            } else {
                cmd.probe_fuse().args([subcmd, value]);
                assert!(!cmd.stdout_lossy().trim().is_empty());
            }
        }
    }
});

// tests that revert reason is only present if transaction has reverted.
probetest!(probe_receipt_revert_reason, |_: TestProject, mut cmd: TestCommand| {
    let rpc = http_rpc_endpoint();

    // <https://etherscan.io/tx/0x44f2aaa351460c074f2cb1e5a9e28cbc7d83f33e425101d2de14331c7b7ec31e>
    cmd.probe_fuse().args([
        "receipt",
        "0x9a39153ee009885cd83ea19651fb245a9e8424178bd34c02e63102c4bafabd24",
        "--rpc-url",
        rpc.as_str(),
    ]);
    let output = cmd.stdout_lossy();
    assert!(!output.contains("revertReason"));

    //TODO:error2215 - find some tx with revert reason
    // <https://etherscan.io/tx/0x0e07d8b53ed3d91314c80e53cf25bcde02084939395845cbb625b029d568135c>
    // cmd.probe_fuse().args([
    //     "receipt",
    //     "0x5ebf392a2804570f5eff84de3f476c1052dc2e189fcd4276cd4a525d90fb8db3",
    //     "--rpc-url",
    //     rpc.as_str(),
    // ]);
    // let output = cmd.stdout_lossy();
    // assert!(output.contains("revertReason"));
    // assert!(output.contains("Transaction too old"));
});

// tests that `probe --parse-bytes32-address` command is working correctly.
probetest!(parse_bytes32_address, |_: TestProject, mut cmd: TestCommand| {
    cmd.args([
        "--parse-bytes32-address",
        "0x000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045",
    ]);
    let output = cmd.stdout_lossy();
    assert_eq!(output.trim(), "0000d8da6bf26964af9d7eed9e03e53415d37aa96045")
});

probetest!(probe_logs_topics, |_: TestProject, mut cmd: TestCommand| {
    let rpc = http_rpc_endpoint();
    cmd.args([
        "logs",
        "--rpc-url",
        rpc.as_str(),
        "--from-block",
        "6797829",
        "--to-block",
        "6797830",
        "0xc17a9d92b89f27cb79cc390f23a1a5d302fefab8c7911075ede952ac2b5607a1",
        "0x00000000000000000000cb12f515d8d1bb1f18994719602afb6617d182d722a8",
    ]);

    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/probe_logs.stdout"),
    );
});

probetest!(probe_logs_topic_2, |_: TestProject, mut cmd: TestCommand| {
    let rpc = http_rpc_endpoint();
    cmd.args([
        "logs",
        "--rpc-url",
        rpc.as_str(),
        "--from-block",
        "6797829",
        "--to-block",
        "6797830",
        "0xc17a9d92b89f27cb79cc390f23a1a5d302fefab8c7911075ede952ac2b5607a1",
        "",
        "0x00000000000000000000cb83decf9503684237ef6a37695a0bb8c1dad6552eed", /* Filter on the
                                                                               * `to` address */
    ]);

    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/probe_logs.stdout"),
    );
});

probetest!(probe_logs_sig, |_: TestProject, mut cmd: TestCommand| {
    let rpc = http_rpc_endpoint();
    cmd.args([
        "logs",
        "--rpc-url",
        rpc.as_str(),
        "--from-block",
        "6797829",
        "--to-block",
        "6797830",
        "Transfer(address,address,uint256)",
        "cb12f515d8d1bb1f18994719602afb6617d182d722a8",
    ]);

    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/probe_logs.stdout"),
    );
});

probetest!(probe_logs_sig_2, |_: TestProject, mut cmd: TestCommand| {
    let rpc = http_rpc_endpoint();
    cmd.args([
        "logs",
        "--rpc-url",
        rpc.as_str(),
        "--from-block",
        "6797829",
        "--to-block",
        "6797830",
        "Transfer(address,address,uint256)",
        "",
        "cb83decf9503684237ef6a37695a0bb8c1dad6552eed",
    ]);

    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/probe_logs.stdout"),
    );
});
