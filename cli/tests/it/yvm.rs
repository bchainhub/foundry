//! yvm sanity checks

use foxar_cli_test_utils::{sparktest_init, TestCommand, TestProject};
use semver::Version;
use yvm::{self, Platform};

/// The latest solc release
///
/// solc to foxar release process:
///     1. new solc release
///     2. yvm updated with all build info
///     3. yvm bumped in ethers-rs
///     4. ethers bumped in foxar + update the `LATEST_SOLC`
const LATEST_SOLC: Version = Version::new(1, 1, 0);

macro_rules! ensure_yvm_releases {
    ($($test:ident => $platform:ident),*) => {
        $(
        #[tokio::test(flavor = "multi_thread")]
        async fn $test() {
            ensure_latest_release(Platform::$platform).await
        }
        )*
    };
}

async fn ensure_latest_release(platform: Platform) {
    let releases = yvm::all_releases(platform)
        .unwrap_or_else(|err| panic!("Could not fetch releases for {platform}: {err:?}"));
    assert!(
        releases.releases.contains_key(&LATEST_SOLC),
        "platform {platform:?} is missing solc info {LATEST_SOLC}"
    );
}

// ensures all platform have the latest solc release version
ensure_yvm_releases!(
    test_yvm_releases_linux_amd64 => LinuxAmd64,
    test_yvm_releases_linux_aarch64 => LinuxAarch64,
    // todo:error2215 add support for macos amd64
    // test_yvm_releases_macos_amd64 => MacOsAmd64,
    test_yvm_releases_macos_aarch64 => MacOsAarch64,
    test_yvm_releases_windows_amd64 => WindowsAmd64
);

// Ensures we can always test with the latest solc build
sparktest_init!(can_test_with_latest_solc, |prj: TestProject, mut cmd: TestCommand| {
    prj.inner()
        .add_test(
            "Counter",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity =<VERSION>;

import "forge-std/Test.sol";

contract CounterTest is Test {

    function testAssert() public {
       assert(true);
    }
}
   "#
            .replace("<VERSION>", &LATEST_SOLC.to_string()),
        )
        .unwrap();

    cmd.args(["test"]);
    cmd.stdout().contains("[PASS]")
});
