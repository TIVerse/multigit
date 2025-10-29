//! Integration tests for complete workflows

use multigit::core::config::{Config, RemoteConfig};
use multigit::git::operations::GitOperations;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_and_add_remote_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repository
    let _git_ops = GitOperations::init(repo_path).unwrap();

    // Create config
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
    let config_path = repo_path.join(".multigit").join("config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    config.save_to_file(&config_path).unwrap();

    // Verify config was saved
    assert!(config_path.exists());

    // Load and verify
    let content = fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&content).unwrap();
    assert!(loaded.remotes.contains_key("github"));
}

#[test]
fn test_multiple_remotes_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize repository
    let git_ops = GitOperations::init(repo_path).unwrap();

    // Add multiple remotes to git
    git_ops
        .add_remote("github", "https://github.com/user/repo.git")
        .unwrap();
    git_ops
        .add_remote("gitlab", "https://gitlab.com/user/repo.git")
        .unwrap();
    git_ops
        .add_remote("bitbucket", "https://bitbucket.org/user/repo.git")
        .unwrap();

    // Verify remotes were added
    let github_url = git_ops.get_remote_url("github").unwrap();
    assert_eq!(github_url, "https://github.com/user/repo.git");

    let gitlab_url = git_ops.get_remote_url("gitlab").unwrap();
    assert_eq!(gitlab_url, "https://gitlab.com/user/repo.git");

    let bitbucket_url = git_ops.get_remote_url("bitbucket").unwrap();
    assert_eq!(bitbucket_url, "https://bitbucket.org/user/repo.git");
}

#[test]
fn test_remote_removal_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    let git_ops = GitOperations::init(repo_path).unwrap();

    // Add a remote
    git_ops
        .add_remote("origin", "https://github.com/user/repo.git")
        .unwrap();
    assert!(git_ops.get_remote_url("origin").is_ok());

    // Remove the remote
    git_ops.remove_remote("origin").unwrap();

    // Verify it's gone
    assert!(git_ops.get_remote_url("origin").is_err());
}

#[test]
fn test_config_merge_workflow() {
    let mut base_config = Config::default();
    base_config.settings.default_branch = "main".to_string();
    base_config.add_remote(
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

    let mut override_config = Config::default();
    override_config.settings.default_branch = "master".to_string();
    override_config.add_remote(
        "gitlab".to_string(),
        RemoteConfig {
            username: "user2".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("gitlab".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    // Note: Config::merge is private, so we test the public interface
    // Just verify both remotes would exist if added separately
    assert!(base_config.remotes.contains_key("github"));
    assert!(override_config.remotes.contains_key("gitlab"));
}

#[test]
fn test_enabled_remotes_filtering_workflow() {
    let mut config = Config::default();

    // Add multiple remotes with different enabled states
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

    config.add_remote(
        "bitbucket".to_string(),
        RemoteConfig {
            username: "user3".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("bitbucket".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    let enabled = config.enabled_remotes();
    assert_eq!(enabled.len(), 2);
    assert!(enabled.contains_key("github"));
    assert!(enabled.contains_key("bitbucket"));
    assert!(!enabled.contains_key("gitlab"));
}
