//! Contains various tests for checking spark commands related to config values
use crate::spark_utils;
use corebc::{
    prelude::artifacts::YulDetails,
    types::{Address, H256, U256},
    ylem::{artifacts::RevertStrings, CvmVersion},
};
use foxar_cli_test_utils::{
    corebc_ylem::{remappings::Remapping, YlemVersion},
    pretty_eq, sparktest, sparktest_init,
    util::{pretty_err, OutputExt, TestCommand, TestProject},
};
use foxar_config::{
    cache::{CachedEndpoints, CachedNetworks, StorageCachingConfig},
    Config, FuzzConfig, InvariantConfig, OptimizerDetails, YlemReq,
};
use path_slash::PathBufExt;
use spark::executor::opts::EvmOpts;
use std::{fs, path::PathBuf, str::FromStr};

// tests all config values that are in use
sparktest!(can_extract_config_values, |prj: TestProject, mut cmd: TestCommand| {
    // explicitly set all values
    let input = Config {
        profile: Config::DEFAULT_PROFILE,
        __root: Default::default(),
        src: "test-src".into(),
        test: "test-test".into(),
        script: "test-script".into(),
        out: "out-test".into(),
        libs: vec!["lib-test".into()],
        cache: true,
        cache_path: "test-cache".into(),
        broadcast: "broadcast".into(),
        force: true,
        cvm_version: CvmVersion::Nucleus,
        energy_reports: vec!["Contract".to_string()],
        energy_reports_ignore: vec![],
        ylem: Some(YlemReq::Local(PathBuf::from("custom-ylem"))),
        auto_detect_ylem: false,
        auto_detect_remappings: true,
        offline: true,
        optimizer: false,
        optimizer_runs: 1000,
        optimizer_details: Some(OptimizerDetails {
            yul: Some(false),
            yul_details: Some(YulDetails { stack_allocation: Some(true), ..Default::default() }),
            ..Default::default()
        }),
        model_checker: None,
        extra_output: Default::default(),
        extra_output_files: Default::default(),
        names: true,
        sizes: true,
        test_pattern: None,
        test_pattern_inverse: None,
        contract_pattern: None,
        contract_pattern_inverse: None,
        path_pattern: None,
        path_pattern_inverse: None,
        fuzz: FuzzConfig {
            runs: 1000,
            max_test_rejects: 100203,
            seed: Some(1000.into()),
            ..Default::default()
        },
        invariant: InvariantConfig { runs: 256, ..Default::default() },
        ffi: true,
        sender: "cb5400a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap(),
        tx_origin: "cb5400a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap(),
        initial_balance: U256::from(0xffffffffffffffffffffffffu128),
        block_number: 10,
        fork_block_number: Some(200),
        network_id: Some(corebc::types::Network::Mainnet),
        energy_limit: 99_000_000u64.into(),
        code_size_limit: Some(100000),
        energy_price: Some(999),
        block_coinbase: Address::random(),
        block_timestamp: 10,
        block_difficulty: 10,
        block_energy_limit: Some(100u64.into()),
        memory_limit: 2u64.pow(25),
        eth_rpc_url: Some("localhost".to_string()),
        etherscan_api_key: None,
        etherscan: Default::default(),
        verbosity: 4,
        remappings: vec![Remapping::from_str("forge-std=lib/forge-std/").unwrap().into()],
        libraries: vec![
            "src/DssSpell.sol:DssExecLib:0x8De6DDbCd5053d32292AAA0D2105A32d108484a6".to_string()
        ],
        ignored_error_codes: vec![],
        deny_warnings: false,
        via_ir: true,
        rpc_storage_caching: StorageCachingConfig {
            networks: CachedNetworks::None,
            endpoints: CachedEndpoints::Remote,
        },
        no_storage_caching: true,
        no_rpc_rate_limit: true,
        use_literal_content: false,
        bytecode_hash: Default::default(),
        cbor_metadata: true,
        revert_strings: Some(RevertStrings::Strip),
        sparse_mode: true,
        allow_paths: vec![],
        include_paths: vec![],
        rpc_endpoints: Default::default(),
        build_info: false,
        build_info_path: None,
        fmt: Default::default(),
        doc: Default::default(),
        fs_permissions: Default::default(),
        __non_exhaustive: (),
        __warnings: vec![],
    };
    prj.write_config(input.clone());
    let config = cmd.config();
    pretty_assertions::assert_eq!(input, config);
});

// tests config gets printed to std out
sparktest!(
    #[serial_test::serial]
    can_show_config,
    |prj: TestProject, mut cmd: TestCommand| {
        cmd.arg("config");
        let expected =
            Config::load_with_root(prj.root()).to_string_pretty().unwrap().trim().to_string();
        assert_eq!(expected, cmd.stdout().trim().to_string());
    }
);

// checks that config works
// - foxar.toml is properly generated
// - paths are resolved properly
// - config supports overrides from env, and cli
sparktest_init!(
    #[serial_test::serial]
    can_override_config,
    |prj: TestProject, mut cmd: TestCommand| {
        cmd.set_current_dir(prj.root());
        let foxar_toml = prj.root().join(Config::FILE_NAME);
        assert!(foxar_toml.exists());

        let profile = Config::load_with_root(prj.root());

        // ensure remappings contain test
        assert_eq!(profile.remappings.len(), 2);
        assert_eq!("ds-test/=lib/forge-std/lib/ds-test/src/", profile.remappings[0].to_string());
        // the loaded config has resolved, absolute paths
        assert_eq!(
            "ds-test/=lib/forge-std/lib/ds-test/src/",
            Remapping::from(profile.remappings[0].clone()).to_string()
        );

        cmd.arg("config");
        let expected = profile.to_string_pretty().unwrap();
        assert_eq!(expected.trim().to_string(), cmd.stdout().trim().to_string());

        // remappings work
        let remappings_txt =
            prj.create_file("remappings.txt", "ds-test/=lib/forge-std/lib/ds-test/from-file/");
        let config = spark_utils::load_config_with_root(Some(prj.root().into()));
        assert_eq!(
            format!(
                "ds-test/={}/",
                prj.root().join("lib/forge-std/lib/ds-test/from-file").to_slash_lossy()
            ),
            Remapping::from(config.remappings[0].clone()).to_string()
        );

        // env vars work
        std::env::set_var("DAPP_REMAPPINGS", "ds-test/=lib/forge-std/lib/ds-test/from-env/");
        let config = spark_utils::load_config_with_root(Some(prj.root().into()));
        assert_eq!(
            format!(
                "ds-test/={}/",
                prj.root().join("lib/forge-std/lib/ds-test/from-env").to_slash_lossy()
            ),
            Remapping::from(config.remappings[0].clone()).to_string()
        );

        let config =
            prj.config_from_output(["--remappings", "ds-test/=lib/forge-std/lib/ds-test/from-cli"]);
        assert_eq!(
            format!(
                "ds-test/={}/",
                prj.root().join("lib/forge-std/lib/ds-test/from-cli").to_slash_lossy()
            ),
            Remapping::from(config.remappings[0].clone()).to_string()
        );

        let config = prj.config_from_output(["--remappings", "other-key/=lib/other/"]);
        assert_eq!(config.remappings.len(), 3);
        assert_eq!(
            format!("other-key/={}/", prj.root().join("lib/other").to_slash_lossy()),
            Remapping::from(config.remappings[2].clone()).to_string()
        );

        std::env::remove_var("DAPP_REMAPPINGS");
        pretty_err(&remappings_txt, fs::remove_file(&remappings_txt));

        cmd.set_cmd(prj.spark_bin()).args(["config", "--basic"]);
        let expected = profile.into_basic().to_string_pretty().unwrap();
        pretty_eq!(expected.trim().to_string(), cmd.stdout().trim().to_string());
    }
);

sparktest_init!(
    #[serial_test::serial]
    can_detect_config_vals,
    |prj: TestProject, _cmd: TestCommand| {
        let url = "http://127.0.0.1:8545";
        let config = prj.config_from_output(["--no-auto-detect", "--rpc-url", url]);
        assert!(!config.auto_detect_ylem);
        assert_eq!(config.eth_rpc_url, Some(url.to_string()));

        let mut config = Config::load_with_root(prj.root());
        config.eth_rpc_url = Some("http://127.0.0.1:8545".to_string());
        config.auto_detect_ylem = false;
        // write to `foxar.toml`
        prj.create_file(
            Config::FILE_NAME,
            &config.to_string_pretty().unwrap().replace("eth_rpc_url", "eth-rpc-url"),
        );
        let config = prj.config_from_output(["--force"]);
        assert!(!config.auto_detect_ylem);
        assert_eq!(config.eth_rpc_url, Some(url.to_string()));
    }
);

// checks that `clean` removes dapptools style paths
sparktest_init!(
    #[serial_test::serial]
    can_get_evm_opts,
    |prj: TestProject, _cmd: TestCommand| {
        let url = "http://127.0.0.1:8545";
        let config = prj.config_from_output(["--rpc-url", url, "--ffi"]);
        assert_eq!(config.eth_rpc_url, Some(url.to_string()));
        assert!(config.ffi);

        std::env::set_var("FOXAR_ETH_RPC_URL", url);
        let figment = Config::figment_with_root(prj.root()).merge(("debug", false));
        let evm_opts: EvmOpts = figment.extract().unwrap();
        assert_eq!(evm_opts.fork_url, Some(url.to_string()));
        std::env::remove_var("FOXAR_ETH_RPC_URL");
    }
);

// checks that we can set various config values
sparktest_init!(can_set_config_values, |prj: TestProject, _cmd: TestCommand| {
    let config = prj.config_from_output(["--via-ir"]);
    assert!(config.via_ir);
});

// tests that ylem can be explicitly set
sparktest!(can_set_ylem_explicitly, |prj: TestProject, mut cmd: TestCommand| {
    prj.inner()
        .add_source(
            "Foo",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >1.0.0;
contract Greeter {}
   "#,
        )
        .unwrap();

    // explicitly set to run with 1.1.0
    let config = Config { ylem: Some("1.1.0".into()), ..Default::default() };
    prj.write_config(config);

    cmd.arg("build");

    assert!(cmd.stdout_lossy().ends_with(
        "
Compiler run successful
",
    ));
});

// tests that `--use <ylem>` works
sparktest!(can_use_ylem, |prj: TestProject, mut cmd: TestCommand| {
    prj.inner()
        .add_source(
            "Foo",
            r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >=1.1.0;
contract Foo {}
   "#,
        )
        .unwrap();

    cmd.args(["build", "--use", "1.1.0"]);

    let stdout = cmd.stdout_lossy();
    assert!(stdout.contains("Compiler run successful"));

    cmd.spark_fuse().args(["build", "--force", "--use", "ylem:1.1.0"]).root_arg();

    assert!(stdout.contains("Compiler run successful"));

    // fails to use ylem that does not exist
    cmd.spark_fuse().args(["build", "--use", "this/ylem/does/not/exist"]);
    assert!(cmd.stderr_lossy().contains("this/ylem/does/not/exist does not exist"));

    // 0.7.1 was installed in previous step, so we can use the path to this directly
    let local_ylem = corebc::ylem::Ylem::find_yvm_installed_version("1.1.0")
        .unwrap()
        .expect("ylem 1.1.0 is installed");
    cmd.spark_fuse().args(["build", "--force", "--use"]).arg(local_ylem.ylem).root_arg();
    assert!(stdout.contains("Compiler run successful"));
});

// tests that the lib triple can be parsed
sparktest_init!(can_parse_dapp_libraries, |_prj: TestProject, mut cmd: TestCommand| {
    cmd.set_env(
        "DAPP_LIBRARIES",
        "src/DssSpell.sol:DssExecLib:0x8De6DDbCd5053d32292AAA0D2105A32d108484a6",
    );
    let config = cmd.config();
    assert_eq!(
        config.libraries,
        vec!["src/DssSpell.sol:DssExecLib:0x8De6DDbCd5053d32292AAA0D2105A32d108484a6".to_string(),]
    );
});

// test that optimizer runs works
sparktest!(can_set_optimizer_runs, |prj: TestProject, mut cmd: TestCommand| {
    // explicitly set optimizer runs
    let config = Config { optimizer_runs: 1337, ..Default::default() };
    prj.write_config(config);

    let config = cmd.config();
    assert_eq!(config.optimizer_runs, 1337);

    let config = prj.config_from_output(["--optimizer-runs", "300"]);
    assert_eq!(config.optimizer_runs, 300);
});

// test that gas_price can be set
sparktest!(can_set_gas_price, |prj: TestProject, mut cmd: TestCommand| {
    // explicitly set gas_price
    let config = Config { energy_price: Some(1337), ..Default::default() };
    prj.write_config(config);

    let config = cmd.config();
    assert_eq!(config.energy_price, Some(1337));

    let config = prj.config_from_output(["--energy-price", "300"]);
    assert_eq!(config.energy_price, Some(300));
});

// test that optimizer runs works
sparktest_init!(can_detect_lib_foxar_toml, |prj: TestProject, mut cmd: TestCommand| {
    let config = cmd.config();
    let remappings = config.remappings.iter().cloned().map(Remapping::from).collect::<Vec<_>>();
    pretty_assertions::assert_eq!(
        remappings,
        vec![
            "ds-test/=lib/forge-std/lib/ds-test/src/".parse().unwrap(),
            "forge-std/=lib/forge-std/src/".parse().unwrap(),
        ]
    );
    // create a new lib directly in the `lib` folder
    let mut config = config;
    config.remappings = vec![Remapping::from_str("nested/=lib/nested").unwrap().into()];
    let nested = prj.paths().libraries[0].join("nested-lib");
    pretty_err(&nested, fs::create_dir_all(&nested));
    let toml_file = nested.join("foxar.toml");
    pretty_err(&toml_file, fs::write(&toml_file, config.to_string_pretty().unwrap()));

    let config = cmd.config();
    let remappings = config.remappings.iter().cloned().map(Remapping::from).collect::<Vec<_>>();
    pretty_assertions::assert_eq!(
        remappings,
        vec![
            "ds-test/=lib/forge-std/lib/ds-test/src/".parse().unwrap(),
            "forge-std/=lib/forge-std/src/".parse().unwrap(),
            "nested-lib/=lib/nested-lib/src/".parse().unwrap(),
            "nested/=lib/nested-lib/lib/nested/".parse().unwrap(),
        ]
    );

    // nest another lib under the already nested lib
    let mut config = config;
    config.remappings = vec![Remapping::from_str("nested-twice/=lib/nested-twice").unwrap().into()];
    let nested = nested.join("lib/another-lib");
    pretty_err(&nested, fs::create_dir_all(&nested));
    let toml_file = nested.join("foxar.toml");
    pretty_err(&toml_file, fs::write(&toml_file, config.to_string_pretty().unwrap()));

    let another_config = cmd.config();
    let remappings =
        another_config.remappings.iter().cloned().map(Remapping::from).collect::<Vec<_>>();
    pretty_assertions::assert_eq!(
        remappings,
        vec![
            "another-lib/=lib/nested-lib/lib/another-lib/src/".parse().unwrap(),
            "ds-test/=lib/forge-std/lib/ds-test/src/".parse().unwrap(),
            "forge-std/=lib/forge-std/src/".parse().unwrap(),
            "nested-lib/=lib/nested-lib/src/".parse().unwrap(),
            "nested-twice/=lib/nested-lib/lib/another-lib/lib/nested-twice/".parse().unwrap(),
            "nested/=lib/nested-lib/lib/nested/".parse().unwrap(),
        ]
    );

    config.src = "custom-source-dir".into();
    pretty_err(&toml_file, fs::write(&toml_file, config.to_string_pretty().unwrap()));
    let config = cmd.config();
    let remappings = config.remappings.iter().cloned().map(Remapping::from).collect::<Vec<_>>();
    pretty_assertions::assert_eq!(
        remappings,
        vec![
            "another-lib/=lib/nested-lib/lib/another-lib/custom-source-dir/".parse().unwrap(),
            "ds-test/=lib/forge-std/lib/ds-test/src/".parse().unwrap(),
            "forge-std/=lib/forge-std/src/".parse().unwrap(),
            "nested-lib/=lib/nested-lib/src/".parse().unwrap(),
            "nested-twice/=lib/nested-lib/lib/another-lib/lib/nested-twice/".parse().unwrap(),
            "nested/=lib/nested-lib/lib/nested/".parse().unwrap(),
        ]
    );
});

// test remappings with closer paths are prioritised
// so that `dep/=lib/a/src` will take precedent over  `dep/=lib/a/lib/b/src`
sparktest_init!(
    #[serial_test::serial]
    can_prioritise_closer_lib_remappings,
    |prj: TestProject, mut cmd: TestCommand| {
        let config = cmd.config();

        // create a new lib directly in the `lib` folder with conflicting remapping `forge-std/`
        let mut config = config;
        config.remappings =
            vec![Remapping::from_str("forge-std/=lib/forge-std/src/").unwrap().into()];
        let nested = prj.paths().libraries[0].join("dep1");
        pretty_err(&nested, fs::create_dir_all(&nested));
        let toml_file = nested.join("foxar.toml");
        pretty_err(&toml_file, fs::write(&toml_file, config.to_string_pretty().unwrap()));

        let config = cmd.config();
        let remappings = config.get_all_remappings();
        pretty_assertions::assert_eq!(
            remappings,
            vec![
                "dep1/=lib/dep1/src/".parse().unwrap(),
                "ds-test/=lib/forge-std/lib/ds-test/src/".parse().unwrap(),
                "forge-std/=lib/forge-std/src/".parse().unwrap()
            ]
        );
    }
);

// test to check that foxar.toml libs section updates on install
sparktest!(can_update_libs_section, |prj: TestProject, mut cmd: TestCommand| {
    cmd.git_init();

    // explicitly set gas_price
    let init = Config { libs: vec!["node_modules".into()], ..Default::default() };
    prj.write_config(init);

    cmd.args(["install", "bchainhub/forge-std", "--no-commit"]);
    cmd.assert_non_empty_stdout();

    let config = cmd.spark_fuse().config();
    // `lib` was added automatically
    let expected = vec![PathBuf::from("node_modules"), PathBuf::from("lib")];
    assert_eq!(config.libs, expected);

    // additional install don't edit `libs`
    cmd.spark_fuse().args(["install", "bchainhub/ds-test", "--no-commit"]);
    cmd.assert_non_empty_stdout();

    let config = cmd.spark_fuse().config();
    assert_eq!(config.libs, expected);
});

// test to check that loading the config emits warnings on the root foxar.toml and
// is silent for any libs
sparktest!(config_emit_warnings, |prj: TestProject, mut cmd: TestCommand| {
    cmd.git_init();

    cmd.args(["install", "bchainhub/forge-std", "--no-commit"]);
    cmd.assert_non_empty_stdout();

    let faulty_toml = r#"[default]
    src = 'src'
    out = 'out'
    libs = ['lib']"#;

    fs::write(prj.root().join("foxar.toml"), faulty_toml).unwrap();
    fs::write(prj.root().join("lib").join("forge-std").join("foxar.toml"), faulty_toml).unwrap();

    cmd.spark_fuse().args(["config"]);
    let output = cmd.execute();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .filter(|line| { line.contains("Unknown section [default]") })
            .count(),
        1
    )
});

sparktest_init!(can_skip_remappings_auto_detection, |prj: TestProject, mut cmd: TestCommand| {
    // explicitly set remapping and libraries
    let config = Config {
        remappings: vec![Remapping::from_str("remapping/=lib/remapping/").unwrap().into()],
        auto_detect_remappings: false,
        ..Default::default()
    };
    prj.write_config(config);

    let config = cmd.config();

    // only loads remappings from foxar.toml
    assert_eq!(config.remappings.len(), 1);
    assert_eq!("remapping/=lib/remapping/", config.remappings[0].to_string());
});
