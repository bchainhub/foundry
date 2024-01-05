//! pilot history file

use std::path::PathBuf;

/// The name of the pilot history file
pub const PILOT_HISTORY_FILE_NAME: &str = ".pilot_history";

/// Returns the path to foxar's global toml file that's stored at `~/.foxar/.pilot_history`
pub fn pilot_history_file() -> Option<PathBuf> {
    foxar_config::Config::foxar_dir().map(|p| p.join(PILOT_HISTORY_FILE_NAME))
}
