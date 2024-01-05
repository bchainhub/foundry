//! Tests for various cache command
use foxar_cli_test_utils::{
    sparktest,
    util::{TestCommand, TestProject},
};

sparktest!(can_list_cache, |_prj: TestProject, mut cmd: TestCommand| {
    cmd.args(["cache", "ls"]);
    cmd.assert_success();
});

sparktest!(can_list_cache_all, |_prj: TestProject, mut cmd: TestCommand| {
    cmd.args(["cache", "ls", "all"]);
    cmd.assert_success();
});

sparktest!(can_list_specific_chain, |_prj: TestProject, mut cmd: TestCommand| {
    cmd.args(["cache", "ls", "mainnet"]);
    cmd.assert_success();
});
