use crate::cmd::{
    spark::build::CoreBuildArgs,
    utils::{Cmd, LoadConfig},
};
use clap::{Parser, ValueHint};
use corebc::contract::{Abigen, ContractFilter, ExcludeContracts, MultiAbigen, SelectContracts};
use foxar_common::{compile, fs::json_files};
use foxar_config::impl_figment_convert;
use std::{
    fs,
    path::{Path, PathBuf},
};

impl_figment_convert!(BindArgs, build_args);

const DEFAULT_CRATE_NAME: &str = "foxar-contracts";
const DEFAULT_CRATE_VERSION: &str = "0.1.0";

/// CLI arguments for `spark bind`.
#[derive(Debug, Clone, Parser)]
pub struct BindArgs {
    /// Path to where the contract artifacts are stored.
    #[clap(
        long = "bindings-path",
        short,
        value_hint = ValueHint::DirPath,
        value_name = "PATH"
    )]
    pub bindings: Option<PathBuf>,

    /// Create bindings only for contracts whose names match the specified filter(s)
    #[clap(long)]
    pub select: Vec<regex::Regex>,

    /// Create bindings only for contracts whose names do not match the specified filter(s)
    #[clap(long, conflicts_with = "select")]
    pub skip: Vec<regex::Regex>,

    /// Explicitly generate bindings for all contracts
    ///
    /// By default all contracts ending with `Test` or `Script` are excluded.
    #[clap(long, conflicts_with_all = &["select", "skip"])]
    pub select_all: bool,

    /// The name of the Rust crate to generate.
    ///
    /// This should be a valid crates.io crate name,
    /// however, this is not currently validated by this command.
    #[clap(long, default_value = DEFAULT_CRATE_NAME, value_name = "NAME")]
    crate_name: String,

    /// The version of the Rust crate to generate.
    ///
    /// This should be a standard semver version string,
    /// however, this is not currently validated by this command.
    #[clap(long, default_value = DEFAULT_CRATE_VERSION, value_name = "VERSION")]
    crate_version: String,

    /// Generate the bindings as a module instead of a crate.
    #[clap(long)]
    module: bool,

    /// Overwrite existing generated bindings.
    ///
    /// By default, the command will check that the bindings are correct, and then exit. If
    /// --overwrite is passed, it will instead delete and overwrite the bindings.
    #[clap(long)]
    overwrite: bool,

    /// Generate bindings as a single file.
    #[clap(long)]
    single_file: bool,

    /// Skip Cargo.toml consistency checks.
    #[clap(long)]
    skip_cargo_toml: bool,

    /// Skips running spark build before generating binding
    #[clap(long)]
    skip_build: bool,

    #[clap(flatten)]
    build_args: CoreBuildArgs,
}

impl BindArgs {
    /// Get the path to the root of the autogenerated crate
    fn bindings_root(&self, artifacts: impl AsRef<Path>) -> PathBuf {
        self.bindings.clone().unwrap_or_else(|| artifacts.as_ref().join("bindings"))
    }

    /// `true` if the bindings root already exists
    fn bindings_exist(&self, artifacts: impl AsRef<Path>) -> bool {
        self.bindings_root(artifacts).is_dir()
    }

    /// Returns the filter to use for `MultiAbigen`
    fn get_filter(&self) -> ContractFilter {
        if self.select_all {
            return ContractFilter::All;
        }
        if !self.select.is_empty() {
            return SelectContracts::default().extend_regex(self.select.clone()).into();
        }
        if !self.skip.is_empty() {
            return ExcludeContracts::default().extend_regex(self.skip.clone()).into();
        }
        // This excludes all Test/Script and forge-std contracts
        ExcludeContracts::default()
            .extend_pattern([
                ".*Test.*",
                ".*Script",
                "console[2]?",
                "CommonBase",
                "Components",
                "[Ss]td(Chains|Math|Error|Json|Utils|Cheats|Assertions|Storage(Safe)?)",
                "[Vv]m.*",
            ])
            .extend_names(["IMulticall3"])
            .into()
    }

    /// Instantiate the multi-abigen
    fn get_multi(&self, artifacts: impl AsRef<Path>) -> eyre::Result<MultiAbigen> {
        let abigens = json_files(artifacts.as_ref())
            .into_iter()
            .filter_map(|path| {
                // we don't want `.metadata.json files
                let stem = path.file_stem()?;
                if stem.to_str()?.ends_with(".metadata") {
                    None
                } else {
                    Some(path)
                }
            })
            .map(Abigen::from_file)
            .collect::<Result<Vec<_>, _>>()?;
        let multi = MultiAbigen::from_abigens(abigens).with_filter(self.get_filter());

        eyre::ensure!(
            !multi.is_empty(),
            r#"
No contract artifacts found. Hint: Have you built your contracts yet? `spark bind` does not currently invoke `spark build`, although this is planned for future versions.
            "#
        );
        Ok(multi)
    }

    /// Check that the existing bindings match the expected abigen output
    fn check_existing_bindings(&self, artifacts: impl AsRef<Path>) -> eyre::Result<()> {
        let bindings = self.get_multi(&artifacts)?.build()?;
        println!("Checking bindings for {} contracts.", bindings.len());
        if !self.module {
            bindings.ensure_consistent_crate(
                &self.crate_name,
                &self.crate_version,
                self.bindings_root(&artifacts),
                self.single_file,
                !self.skip_cargo_toml,
            )?;
        } else {
            bindings.ensure_consistent_module(self.bindings_root(&artifacts), self.single_file)?;
        }
        println!("OK.");
        Ok(())
    }

    /// Generate the bindings
    fn generate_bindings(&self, artifacts: impl AsRef<Path>) -> eyre::Result<()> {
        let bindings = self.get_multi(&artifacts)?.build()?;
        println!("Generating bindings for {} contracts", bindings.len());
        if !self.module {
            bindings.write_to_crate(
                &self.crate_name,
                &self.crate_version,
                self.bindings_root(&artifacts),
                self.single_file,
            )?;
        } else {
            bindings.write_to_module(self.bindings_root(&artifacts), self.single_file)?;
        }
        Ok(())
    }
}

impl Cmd for BindArgs {
    type Output = ();

    fn run(self) -> eyre::Result<Self::Output> {
        if !self.skip_build {
            // run `spark build`
            let project = self.build_args.project()?;
            compile::compile(&project, false, false)?;
        }

        let artifacts = self.try_load_config_emit_warnings()?.out;

        if !self.overwrite && self.bindings_exist(&artifacts) {
            println!("Bindings found. Checking for consistency.");
            return self.check_existing_bindings(&artifacts);
        }

        if self.overwrite && self.bindings_exist(&artifacts) {
            fs::remove_dir_all(self.bindings_root(&artifacts))?;
        }

        self.generate_bindings(&artifacts)?;

        println!(
            "Bindings have been output to {}",
            self.bindings_root(&artifacts).to_str().unwrap()
        );
        Ok(())
    }
}
