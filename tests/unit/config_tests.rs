//! Unit tests for configuration management

use multigit::core::config::{Config, RemoteConfig};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.settings.default_branch, "main");
    assert!(config.settings.parallel_push);
    assert_eq!(config.settings.max_parallel, 4);
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let mut config = Config::default();
    config.add_remote(
        "github".to_string(),
        RemoteConfig {
            username: "testuser".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    // Save config
    config.save_to_file(&config_path).unwrap();

    // Load config
    let content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: Config = toml::from_str(&content).unwrap();

    assert!(loaded_config.remotes.contains_key("github"));
    assert_eq!(
        loaded_config.remotes.get("github").unwrap().username,
        "testuser"
    );
}

#[test]
fn test_remote_config_enabled_filtering() {
    let mut config = Config::default();

    config.add_remote(
        "github".to_string(),
        RemoteConfig {
            username: "user1".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    config.add_remote(
        "gitlab".to_string(),
        RemoteConfig {
            username: "user2".to_string(),
            api_url: None,
            enabled: false,
            provider: Some("gitlab".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    let enabled = config.enabled_remotes();
    assert_eq!(enabled.len(), 1);
    assert!(enabled.contains_key("github"));
    assert!(!enabled.contains_key("gitlab"));
}

#[test]
fn test_config_add_and_remove_remote() {
    let mut config = Config::default();

    // Add remote
    config.add_remote(
        "test".to_string(),
        RemoteConfig {
            username: "testuser".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    assert!(config.remotes.contains_key("test"));

    // Remove remote
    let removed = config.remove_remote("test");
    assert!(removed.is_some());
    assert!(!config.remotes.contains_key("test"));
}

#[test]
fn test_config_get_remote() {
    let mut config = Config::default();

    config.add_remote(
        "github".to_string(),
        RemoteConfig {
            username: "testuser".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    let remote = config.get_remote("github");
    assert!(remote.is_some());
    assert_eq!(remote.unwrap().username, "testuser");

    let nonexistent = config.get_remote("nonexistent");
    assert!(nonexistent.is_none());
}
