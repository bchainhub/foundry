//! Subcommands for spark
//!
//! All subcommands should respect the `foxar_config::Config`.
//! If a subcommand accepts values that are supported by the `Config`, then the subcommand should
//! implement `figment::Provider` which allows the subcommand to override the config's defaults, see
//! [`foxar_config::Config`].
//!
//! See [`BuildArgs`] for a reference implementation.
//! And [`DebugArgs`] for how to merge `Providers`.
//!
//! # Example
//!
//! create a `clap` subcommand into a `figment::Provider` and integrate it in the
//! `foxar_config::Config`:
//!
//! ```
//! use clap::Parser;
//! use spark::executor::opts::EvmOpts;
//! use foxar_cli::cmd::spark::build::BuildArgs;
//! use foxar_common::evm::EvmArgs;
//! use foxar_config::{figment::Figment, *};
//!
//! // A new clap subcommand that accepts both `EvmArgs` and `BuildArgs`
//! #[derive(Debug, Clone, Parser)]
//! pub struct MyArgs {
//!     #[clap(flatten)]
//!     evm_opts: EvmArgs,
//!     #[clap(flatten)]
//!     opts: BuildArgs,
//! }
//!
//! // add `Figment` and `Config` converters
//! foxar_config::impl_figment_convert!(MyArgs, opts, evm_opts);
//! let args = MyArgs::parse_from(["build"]);
//!
//! let figment: Figment = From::from(&args);
//! let evm_opts = figment.extract::<EvmOpts>().unwrap();
//!
//! let config: Config = From::from(&args);
//! ```

pub mod bind;
pub mod build;
pub mod cache;
pub mod config;
pub mod coverage;
pub mod create;
pub mod debug;
pub mod doc;
pub mod flatten;
pub mod fmt;
pub mod fourbyte;
pub mod geiger;
pub mod init;
pub mod inspect;
pub mod install;
pub mod remappings;
pub mod remove;
pub mod script;
pub mod selectors;
pub mod snapshot;
pub mod test;
pub mod tree;
pub mod update;
pub mod verify;
pub mod watch;
