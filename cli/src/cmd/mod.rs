//! Subcommands
//!
//! All subcommands should respect the `foxar_config::Config`.
//! If a subcommand accepts values that are supported by the `Config`, then the subcommand should
//! implement `figment::Provider` which allows the subcommand to override the config's defaults, see
//! [`foxar_config::Config`].

pub mod probe;
pub mod spark;

// Re-export our shared utilities
mod retry;
mod utils;

pub use utils::*;
