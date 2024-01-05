use std::path::Path;

use corebc_ylem::CvmVersion;
use foxar_config::Config;
use pilot::session::PilotSession;
use serial_test::serial;
use spark::executor::opts::EvmOpts;

#[test]
#[serial]
fn test_cache_directory() {
    // Get the cache dir
    // Should be ~/.foxar/cache/pilot
    let cache_dir = PilotSession::cache_dir().unwrap();

    // Validate the cache directory
    let home_dir = dirs::home_dir().unwrap();
    assert_eq!(cache_dir, format!("{}/.foxar/cache/pilot/", home_dir.to_str().unwrap()));
}

#[test]
#[serial]
fn test_create_cache_directory() {
    // Get the cache dir
    let cache_dir = PilotSession::cache_dir().unwrap();

    // Create the cache directory
    PilotSession::create_cache_dir().unwrap();

    // Validate the cache directory
    assert!(Path::new(&cache_dir).exists());
}

#[test]
#[serial]
fn test_write_session() {
    // Create the cache directory if it doesn't exist
    let cache_dir = PilotSession::cache_dir().unwrap();
    PilotSession::create_cache_dir().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    // Create a new session
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        evm_opts: EvmOpts::default(),
        backend: None,
        traces: false,
        calldata: None,
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession!, {}", e));

    // Write the session
    let cached_session_name = env.write().unwrap();

    // Count the number of items in the cache_dir directory
    let mut num_items = std::fs::read_dir(&cache_dir).unwrap().count();
    num_items = if num_items > 0 { num_items - 1 } else { 0 };

    // Validate the session
    assert_eq!(cached_session_name, format!("{cache_dir}pilot-{num_items}.json"));
}

#[test]
#[serial]
fn test_write_session_with_name() {
    // Create the cache directory if it doesn't exist
    let cache_dir = PilotSession::cache_dir().unwrap();
    PilotSession::create_cache_dir().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    // Create a new session
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        ..Default::default()
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession! {}", e));
    env.id = Some(String::from("test"));

    // Write the session
    let cached_session_name = env.write().unwrap();

    // Validate the session
    assert_eq!(cached_session_name, format!("{cache_dir}pilot-test.json"));
}

#[test]
#[serial]
fn test_clear_cache() {
    // Create a session to validate clearing a non-empty cache directory
    let cache_dir = PilotSession::cache_dir().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    PilotSession::create_cache_dir().unwrap();
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        ..Default::default()
    })
    .unwrap_or_else(|_| panic!("Failed to create PilotSession!"));
    env.write().unwrap();

    // Clear the cache
    PilotSession::clear_cache().unwrap();

    // Validate there are no items in the cache dir
    let num_items = std::fs::read_dir(cache_dir).unwrap().count();
    assert_eq!(num_items, 0);
}

#[test]
#[serial]
fn test_list_sessions() {
    // Create and clear the cache directory
    PilotSession::create_cache_dir().unwrap();
    PilotSession::clear_cache().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    // Create a new session
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        ..Default::default()
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession! {}", e));

    env.write().unwrap();

    // List the sessions
    let sessions = PilotSession::list_sessions().unwrap();

    // Validate the sessions
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].1, "pilot-0.json");
}

#[test]
#[serial]
fn test_load_cache() {
    // Create and clear the cache directory
    PilotSession::create_cache_dir().unwrap();
    PilotSession::clear_cache().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    // Create a new session
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        ..Default::default()
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession! {}", e));
    env.write().unwrap();

    // Load the session
    let new_env = PilotSession::load("0");

    // Validate the session
    assert!(new_env.is_ok());
    let new_env = new_env.unwrap();
    assert_eq!(new_env.id.unwrap(), String::from("0"));
    assert_eq!(
        new_env.session_source.unwrap().to_repl_source(),
        env.session_source.unwrap().to_repl_source()
    );
}

#[test]
#[serial]
fn test_write_same_session_multiple_times() {
    // Create and clear the cache directory
    PilotSession::create_cache_dir().unwrap();
    PilotSession::clear_cache().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    // Create a new session
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        ..Default::default()
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession! {}", e));
    env.write().unwrap();
    env.write().unwrap();
    env.write().unwrap();
    env.write().unwrap();
    assert_eq!(PilotSession::list_sessions().unwrap().len(), 1);
}

#[test]
#[serial]
fn test_load_latest_cache() {
    // Create and clear the cache directory
    PilotSession::create_cache_dir().unwrap();
    PilotSession::clear_cache().unwrap();

    // Force the ylem version to be 1.1.0
    let foxar_config = Config { cvm_version: CvmVersion::Nucleus, ..Default::default() };

    // Create sessions
    let mut env = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config: foxar_config.clone(),
        ..Default::default()
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession! {}", e));
    env.write().unwrap();

    let wait_time = std::time::Duration::from_millis(100);
    std::thread::sleep(wait_time);

    let mut env2 = PilotSession::new(pilot::session_source::SessionSourceConfig {
        foxar_config,
        ..Default::default()
    })
    .unwrap_or_else(|e| panic!("Failed to create PilotSession! {}", e));
    env2.write().unwrap();

    // Load the latest session
    let new_env = PilotSession::latest().unwrap();

    // Validate the session
    assert_eq!(new_env.id.unwrap(), "1");
    assert_eq!(
        new_env.session_source.unwrap().to_repl_source(),
        env.session_source.unwrap().to_repl_source()
    );
}
