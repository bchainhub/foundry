/// A macro to generate a new integration test case
///
/// The `sparktest!` macro's first argument is the name of the test, the second argument is a
/// closure to configure and execute the test. The `TestProject` provides utility functions to setup
/// the project's workspace. The `TestCommand` is a wrapper around the actual `spark` executable
/// that this then executed with the configured command arguments.
///
/// # Example
///
/// run `spark init`
///
/// ```no_run
/// use foxar_cli_test_utils::*;
/// sparktest!(my_test, |prj: TestProject, mut cmd: TestCommand| {
///     // adds `init` to spark's command arguments
///     cmd.arg("init");
///     // executes spark <args> and panics if the command failed or output is empty
///     cmd.assert_non_empty_stdout();
/// });
/// ```
///
/// Configure a hardhat project layout by adding a `PathStyle::HardHat` argument
///
/// ```no_run
/// use foxar_cli_test_utils::*;
/// use foxar_cli_test_utils::corebc_ylem::PathStyle;
/// sparktest!(can_clean_hardhat, PathStyle::HardHat, |prj: TestProject, mut cmd: TestCommand| {
///     prj.assert_create_dirs_exists();
///     prj.assert_style_paths_exist(PathStyle::HardHat);
///     cmd.arg("clean");
///     cmd.assert_empty_stdout();
///     prj.assert_cleaned();
/// });
#[macro_export]
macro_rules! sparktest {
    ($(#[$meta:meta])* $test:ident, $fun:expr) => {
        $crate::sparktest!($(#[$meta])* $test, $crate::corebc_ylem::PathStyle::Dapptools, $fun);
    };
    ($(#[$meta:meta])* $test:ident, $style:expr, $fun:expr) => {
        #[test]
        $(#[$meta])*
        fn $test() {
            let (prj, cmd) = $crate::util::setup_spark(stringify!($test), $style);
            let f = $fun;
            f(prj, cmd);
        }
    };
}

#[macro_export]
macro_rules! sparktest_async {
    ($(#[$meta:meta])* $test:ident, $fun:expr) => {
        $crate::sparktest_async!($(#[$meta])* $test, $crate::corebc_ylem::PathStyle::Dapptools, $fun);
    };
    ($(#[$meta:meta])* $test:ident, $style:expr, $fun:expr) => {
        #[tokio::test(flavor = "multi_thread")]
        $(#[$meta])*
        async fn $test() {
            let (prj, cmd) = $crate::util::setup_spark(stringify!($test), $style);
            let f = $fun;
            f(prj, cmd).await;
        }
    };
}

#[macro_export]
macro_rules! probetest {
    ($(#[$meta:meta])* $test:ident, $fun:expr) => {
        $crate::probetest!($(#[$meta])* $test, $crate::corebc_ylem::PathStyle::Dapptools, $fun);
    };
    ($(#[$meta:meta])* $test:ident, $style:expr, $fun:expr) => {
        #[test]
        $(#[$meta])*
        fn $test() {
            let (prj, cmd) = $crate::util::setup_probe(stringify!($test), $style);
            let f = $fun;
            f(prj, cmd);
        }
    };
}

/// Same as `sparktest` but returns an already initialized project workspace (`spark init`)
#[macro_export]
macro_rules! sparktest_init {
    ($(#[$meta:meta])* $test:ident, $fun:expr) => {
        $crate::sparktest_init!($(#[$meta])* $test, $crate::corebc_ylem::PathStyle::Dapptools, $fun);
    };
    ($(#[$meta:meta])* $test:ident, $style:expr, $fun:expr) => {
        #[test]
        $(#[$meta])*
        fn $test() {
            let (prj, cmd) = $crate::util::setup_spark(stringify!($test), $style);
            $crate::util::initialize(prj.root());
            let f = $fun;
            f(prj, cmd);
        }
    };
}

/// Clones an external repository and makes sure the tests pass.
/// Can optionally enable fork mode as well if a fork block is passed.
/// The fork block needs to be greater than 0.
#[macro_export]
macro_rules! sparktest_external {
    // sparktest_external!(test_name, "owner/repo");
    ($(#[$meta:meta])* $test:ident, $repo:literal) => {
        $crate::sparktest_external!($(#[$meta])* $test, $repo, 0, Vec::<String>::new());
    };
    // sparktest_external!(test_name, "owner/repo", 1234);
    ($(#[$meta:meta])* $test:ident, $repo:literal, $fork_block:literal) => {
        $crate::sparktest_external!(
            $(#[$meta])*
            $test,
            $repo,
            $crate::corebc_ylem::PathStyle::Dapptools,
            $fork_block,
            Vec::<String>::new()
        );
    };
    // sparktest_external!(test_name, "owner/repo", &["--extra-opt", "val"]);
    ($(#[$meta:meta])* $test:ident, $repo:literal, $spark_opts:expr) => {
        $crate::sparktest_external!($(#[$meta])* $test, $repo, 0, $spark_opts);
    };
    // sparktest_external!(test_name, "owner/repo", 1234, &["--extra-opt", "val"]);
    ($(#[$meta:meta])* $test:ident, $repo:literal, $fork_block:literal, $spark_opts:expr) => {
        $crate::sparktest_external!(
            $(#[$meta])*
            $test,
            $repo,
            $crate::corebc_ylem::PathStyle::Dapptools,
            $fork_block,
            $spark_opts
        );
    };
    // sparktest_external!(test_name, "owner/repo", PathStyle::Dapptools, 123);
    ($(#[$meta:meta])* $test:ident, $repo:literal, $style:expr, $fork_block:literal, $spark_opts:expr) => {
        #[test]
        $(#[$meta])*
        fn $test() {
            use std::process::{Command, Stdio};
            use $crate::corebc_addressbook;

            // Skip fork tests if the RPC url is not set.
            if $fork_block > 0 && std::env::var("ETH_RPC_URL").is_err() {
                eprintln!("Skipping test {}. ETH_RPC_URL is not set.", $repo);
                return
            };

            let (prj, mut cmd) = $crate::util::setup_spark(stringify!($test), $style);

            // Wipe the default structure
            prj.wipe();

            // Clone the external repository
            let git_clone =
                $crate::util::clone_remote(&format!("https://github.com/{}", $repo), prj.root())
                    .expect("Could not clone repository. Is git installed?");
            assert!(
                git_clone.status.success(),
                "could not clone repository:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&git_clone.stdout),
                String::from_utf8_lossy(&git_clone.stderr)
            );

            // We just run make install, but we do not care if it worked or not,
            // since some repositories do not have that target
            let make_install = Command::new("make")
                .arg("install")
                .current_dir(prj.root())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();

            // Run the tests
            cmd.arg("test").args($spark_opts).args([
                "--optimize",
                "--optimizer-runs",
                "20000",
                "--ffi",
            ]);
            cmd.set_env("FOXAR_FUZZ_RUNS", "1");

            let next_eth_rpc_url = foxar_utils::rpc::next_http_archive_rpc_endpoint(corebc_addressbook::Network::Mainnet);
            if $fork_block > 0 {
                cmd.set_env("FOXAR_ETH_RPC_URL", next_eth_rpc_url);
                cmd.set_env("FOXAR_FORK_BLOCK_NUMBER", stringify!($fork_block));
            }
            cmd.assert_non_empty_stdout();
        }
    };
}

/// A macro to compare outputs
#[macro_export]
macro_rules! pretty_eq {
    ($expected:expr, $got:expr) => {
        let expected = &*$expected;
        let got = &*$got;
        if expected != got {
            panic!(
                "
outputs differ!

expected:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

got:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
",
                expected, got
            );
        }
    };
}
