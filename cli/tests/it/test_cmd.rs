//! Contains various tests for checking `spark test`
use corebc::types::Network;
use foxar_cli_test_utils::{
    sparktest, sparktest_init,
    util::{OutputExt, TestCommand, TestProject},
};
use foxar_config::Config;
use foxar_utils::rpc;
use std::{path::PathBuf, str::FromStr};

// tests that test filters are handled correctly
sparktest!(can_set_filter_values, |prj: TestProject, mut cmd: TestCommand| {
    let patt = regex::Regex::new("test*").unwrap();
    let glob = globset::Glob::from_str("foo/bar/baz*").unwrap();

    // explicitly set patterns
    let config = Config {
        test_pattern: Some(patt.clone().into()),
        test_pattern_inverse: None,
        contract_pattern: Some(patt.clone().into()),
        contract_pattern_inverse: None,
        path_pattern: Some(glob.clone()),
        path_pattern_inverse: None,
        ..Default::default()
    };
    prj.write_config(config);

    let config = cmd.config();

    assert_eq!(config.test_pattern.unwrap().as_str(), patt.as_str());
    assert_eq!(config.test_pattern_inverse, None);
    assert_eq!(config.contract_pattern.unwrap().as_str(), patt.as_str());
    assert_eq!(config.contract_pattern_inverse, None);
    assert_eq!(config.path_pattern.unwrap(), glob);
    assert_eq!(config.path_pattern_inverse, None);
});

// tests that warning is displayed when there are no tests in project
sparktest!(warn_no_tests, |prj: TestProject, mut cmd: TestCommand| {
    prj.inner()
        .add_source(
            "dummy",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity =1.1.0;

contract Dummy {}
"#,
        )
        .unwrap();
    // set up command
    cmd.args(["test"]);

    // run command and assert
    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/warn_no_tests.stdout"),
    );
});

// tests that warning is displayed with pattern when no tests match
sparktest!(warn_no_tests_match, |prj: TestProject, mut cmd: TestCommand| {
    prj.inner()
        .add_source(
            "dummy",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity =1.1.0;

contract Dummy {}
"#,
        )
        .unwrap();

    // set up command
    cmd.args(["test", "--match-test", "testA.*", "--no-match-test", "testB.*"]);
    cmd.args(["--match-contract", "TestC.*", "--no-match-contract", "TestD.*"]);
    cmd.args(["--match-path", "*TestE*", "--no-match-path", "*TestF*"]);

    // run command and assert
    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/warn_no_tests_match.stdout"),
    );
});

// tests that suggestion is provided with pattern when no tests match
sparktest!(suggest_when_no_tests_match, |prj: TestProject, mut cmd: TestCommand| {
    // set up project
    prj.inner()
        .add_source(
            "TestE.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity 1.1.0;

contract TestC {
    function test1() public {
    }
}
   "#,
        )
        .unwrap();

    // set up command
    cmd.args(["test", "--match-test", "testA.*", "--no-match-test", "testB.*"]);
    cmd.args(["--match-contract", "TestC.*", "--no-match-contract", "TestD.*"]);
    cmd.args(["--match-path", "*TestE*", "--no-match-path", "*TestF*"]);

    // run command and assert
    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/suggest_when_no_tests_match.stdout"),
    );
});

// tests that direct import paths are handled correctly
sparktest!(can_fuzz_array_params, |prj: TestProject, mut cmd: TestCommand| {
    prj.insert_ds_test();

    prj.inner()
        .add_source(
            "ATest.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity 1.1.0;
import "./test.sol";
contract ATest is DSTest {
    function testArray(uint64[2] calldata values) external {
        assertTrue(true);
    }
}
   "#,
        )
        .unwrap();

    cmd.arg("test");
    cmd.stdout().contains("[PASS]")
});

// tests that `bytecode_hash` will be sanitized
sparktest!(can_test_pre_bytecode_hash, |prj: TestProject, mut cmd: TestCommand| {
    prj.insert_ds_test();

    prj.inner()
        .add_source(
            "ATest.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
// pre bytecode hash version, was introduced in 0.6.0
pragma solidity 1.1.0;
import "./test.sol";
contract ATest is DSTest {
    function testArray(uint64[2] calldata values) external {
        assertTrue(true);
    }
}
   "#,
        )
        .unwrap();

    cmd.arg("test");
    cmd.stdout().contains("[PASS]")
});

// tests that using the --match-path option only runs files matching the path
sparktest!(can_test_with_match_path, |prj: TestProject, mut cmd: TestCommand| {
    prj.insert_ds_test();

    prj.inner()
        .add_source(
            "ATest.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity 1.1.0;
import "./test.sol";
contract ATest is DSTest {
    function testArray(uint64[2] calldata values) external {
        assertTrue(true);
    }
}
   "#,
        )
        .unwrap();

    prj.inner()
        .add_source(
            "FailTest.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity 1.1.0;
import "./test.sol";
contract FailTest is DSTest {
    function testNothing() external {
        assertTrue(false);
    }
}
   "#,
        )
        .unwrap();

    cmd.args(["test", "--match-path", "*src/ATest.t.sol"]);
    cmd.stdout().contains("[PASS]") && !cmd.stdout().contains("[FAIL]")
});

// tests that `spark test` will pick up tests that are stored in the `test = <path>` config value
sparktest!(can_run_test_in_custom_test_folder, |prj: TestProject, mut cmd: TestCommand| {
    prj.insert_ds_test();

    // explicitly set the test folder
    let config = Config { test: "nested/spark-tests".into(), ..Default::default() };
    prj.write_config(config);
    let config = cmd.config();
    assert_eq!(config.test, PathBuf::from("nested/spark-tests"));

    prj.inner()
        .add_source(
            "nested/spark-tests/MyTest.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity 1.1.0;
import "../../test.sol";
contract MyTest is DSTest {
    function testTrue() public {
        assertTrue(true);
    }
}
   "#,
        )
        .unwrap();

    cmd.arg("test");
    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/can_run_test_in_custom_test_folder.stdout"),
    );
});

// checks that spark test repeatedly produces the same output
sparktest_init!(can_test_repeatedly, |_prj: TestProject, mut cmd: TestCommand| {
    cmd.arg("test");
    cmd.assert_non_empty_stdout();

    for _ in 0..10 {
        cmd.unchecked_output().stdout_matches_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/can_test_repeatedly.stdout"),
        );
    }
});

// tests that `spark test` will run a test only once after changing the version
sparktest!(
    //Todo:error2215 - we have only one version of ylem for now so  test is unvalid
    #[ignore]
    runs_tests_exactly_once_with_changed_versions,
    |prj: TestProject, mut cmd: TestCommand| {
        prj.insert_ds_test();

        prj.inner()
            .add_source(
                "Contract.t.sol",
                r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >=1.1.0;
import "./test.sol";
contract ContractTest is DSTest {
    function setUp() public {}

    function testExample() public {
        assertTrue(true);
    }
}
   "#,
            )
            .unwrap();

        // pin version
        let config = Config { ylem: Some("1.1.0".into()), ..Default::default() };
        prj.write_config(config);

        cmd.arg("test");
        cmd.unchecked_output().stdout_matches_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/runs_tests_exactly_once_with_changed_versions.1.1.0.stdout"),
        );

        // pin version
        let config = Config { ylem: Some("1.1.0".into()), ..Default::default() };
        prj.write_config(config);

        cmd.unchecked_output().stdout_matches_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/runs_tests_exactly_once_with_changed_versions.1.1.0.stdout"),
        );
    }
);

// checks that we can test spark std successfully
// `sparktest_init!` will install with `forge-std` under `lib/forge-std`

sparktest_init!(
    #[serial_test::serial]
    // todo:error2215 - fix tests in forge-std repo
    #[ignore]
    can_test_spark_std,
    |prj: TestProject, mut cmd: TestCommand| {
        let spark_std_dir = prj.root().join("lib/forge-std");
        // execute in subdir
        cmd.cmd().current_dir(spark_std_dir);
        cmd.args(["test", "--root", "."]);
        let stdout = cmd.stdout();
        assert!(stdout.contains("[PASS]"), "No tests passed:\n{stdout}");
        assert!(!stdout.contains("[FAIL]"), "Tests failed :\n{stdout}");
    }
);

// tests that libraries are handled correctly in multiforking mode
sparktest_init!(can_use_libs_in_multi_fork, |prj: TestProject, mut cmd: TestCommand| {
    prj.wipe_contracts();
    prj.inner()
        .add_source(
            "Contract.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity =1.1.0;

library Library {
    function f(uint256 a, uint256 b) public pure returns (uint256) {
        return a + b;
    }
}

contract Contract {
    uint256 c;

    constructor() {
        c = Library.f(1, 2);
    }
}
   "#,
        )
        .unwrap();

    let endpoint = rpc::next_http_archive_rpc_endpoint(Network::Mainnet);

    prj.inner()
        .add_test(
            "Contract.t.sol",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity =1.1.0;

import "forge-std/Test.sol";
import "src/Contract.sol";

contract ContractTest is Test {
    function setUp() public {
        vm.createSelectFork("<url>");
    }

    function test() public {
        new Contract();
    }
}
   "#
            .replace("<url>", &endpoint),
        )
        .unwrap();

    cmd.arg("test");
    cmd.unchecked_output().stdout_matches_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/can_use_libs_in_multi_fork.stdout"),
    );
});

static FAILING_TEST: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^1.1.0;

import "forge-std/Test.sol";

contract FailingTest is Test {
    function testShouldFail() public {
        assertTrue(false);
    }
}
"#;

sparktest_init!(exit_code_error_on_fail_fast, |prj: TestProject, mut cmd: TestCommand| {
    prj.wipe_contracts();
    prj.inner().add_source("failing_test", FAILING_TEST).unwrap();

    // set up command
    cmd.args(["test", "--fail-fast"]);

    // run command and assert error exit code
    cmd.assert_err();
});

sparktest_init!(
    exit_code_error_on_fail_fast_with_json,
    |prj: TestProject, mut cmd: TestCommand| {
        prj.wipe_contracts();

        prj.inner().add_source("failing_test", FAILING_TEST).unwrap();
        // set up command
        cmd.args(["test", "--fail-fast", "--json"]);

        // run command and assert error exit code
        cmd.assert_err();
    }
);
