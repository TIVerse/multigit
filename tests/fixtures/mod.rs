//! Test fixtures and mock data

use multigit::core::config::{Config, RemoteConfig};
use multigit::models::Repository;

/// Create a default test config with multiple remotes
pub fn create_test_config() -> Config {
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

    config.add_remote(
        "gitlab".to_string(),
        RemoteConfig {
            username: "testuser".to_string(),
            api_url: Some("https://gitlab.com".to_string()),
            enabled: true,
            provider: Some("gitlab".to_string()),
            use_ssh: false,
            priority: 1,
        },
    );

    config
}

/// Create a test config with disabled remotes
pub fn create_test_config_with_disabled_remotes() -> Config {
    let mut config = Config::default();

    config.add_remote(
        "enabled".to_string(),
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
        "disabled".to_string(),
        RemoteConfig {
            username: "user2".to_string(),
            api_url: None,
            enabled: false,
            provider: Some("gitlab".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    config
}

/// Create a mock repository
pub fn create_mock_repository(name: &str) -> Repository {
    Repository::new(
        name,
        format!("https://github.com/testuser/{}.git", name),
        format!("git@github.com:testuser/{}.git", name),
        false,
        "main",
    )
    .with_full_name(format!("testuser/{}", name))
    .with_description(format!("Test repository {}", name))
    .with_html_url(format!("https://github.com/testuser/{}", name))
}

/// Create multiple mock repositories
pub fn create_mock_repositories(count: usize) -> Vec<Repository> {
    (0..count)
        .map(|i| create_mock_repository(&format!("repo{}", i)))
        .collect()
}

/// Create a test TOML config string
pub fn create_test_config_toml() -> String {
    r#"
[settings]
default_branch = "main"
parallel_push = true
max_parallel = 4

[sync]
auto_sync = false
strategy = "fast-forward"

[security]
auth_backend = "keyring"
audit_log = false

[remotes.github]
username = "testuser"
enabled = true
provider = "github"
use_ssh = false
priority = 0

[remotes.gitlab]
username = "testuser"
api_url = "https://gitlab.com"
enabled = true
provider = "gitlab"
use_ssh = false
priority = 1
"#
    .to_string()
}

/// Create a test repository structure in a temporary directory
pub fn setup_test_repo(temp_dir: &std::path::Path) -> multigit::utils::error::Result<()> {
    use multigit::git::operations::GitOperations;
    use std::fs;

    // Initialize git repo
    let _git_ops = GitOperations::init(temp_dir)?;

    // Create multigit config directory
    let config_dir = temp_dir.join(".multigit");
    fs::create_dir_all(&config_dir)?;

    // Write config
    let config_path = config_dir.join("config.toml");
    fs::write(config_path, create_test_config_toml())?;

    // Create a test file
    fs::write(temp_dir.join("README.md"), "# Test Repository\n")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_config() {
        let config = create_test_config();
        assert_eq!(config.remotes.len(), 2);
        assert!(config.remotes.contains_key("github"));
        assert!(config.remotes.contains_key("gitlab"));
    }

    #[test]
    fn test_create_test_config_with_disabled() {
        let config = create_test_config_with_disabled_remotes();
        assert_eq!(config.remotes.len(), 2);

        let enabled = config.enabled_remotes();
        assert_eq!(enabled.len(), 1);
        assert!(enabled.contains_key("enabled"));
    }

    #[test]
    fn test_create_mock_repository() {
        let repo = create_mock_repository("test");
        assert_eq!(repo.name, "test");
        assert_eq!(repo.full_name, Some("testuser/test".to_string()));
        assert_eq!(repo.default_branch, "main");
    }

    #[test]
    fn test_create_mock_repositories() {
        let repos = create_mock_repositories(5);
        assert_eq!(repos.len(), 5);
        assert_eq!(repos[0].name, "repo0");
        assert_eq!(repos[4].name, "repo4");
    }

    #[test]
    fn test_create_test_config_toml() {
        let toml = create_test_config_toml();
        assert!(toml.contains("[settings]"));
        assert!(toml.contains("[remotes.github]"));
        assert!(toml.contains("testuser"));
    }
}
