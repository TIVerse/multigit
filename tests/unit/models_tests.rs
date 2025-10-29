//! Models tests

use chrono::Utc;
use multigit::models::config::{AuthBackend, Settings, SyncConfig, SyncStrategy};
use multigit::models::remote::{ProviderType, RateLimit, Remote};
use multigit::models::repository::Repository;

#[test]
fn test_remote_builder() {
    let remote = Remote::builder("github", "testuser")
        .with_ssh()
        .with_priority(10)
        .build();

    assert_eq!(remote.name, "github");
    assert_eq!(remote.username, "testuser");
    assert!(remote.use_ssh);
    assert_eq!(remote.priority, 10);
}

#[test]
fn test_remote_disabled() {
    let remote = Remote::builder("gitlab", "user").disabled().build();

    assert!(!remote.enabled);
}

#[test]
fn test_git_remote_name() {
    let remote = Remote::builder("github", "user").build();
    assert_eq!(remote.git_remote_name(), "multigit-github");
}

#[test]
fn test_provider_type_parsing() {
    assert!(matches!(ProviderType::from("github"), ProviderType::GitHub));
    assert!(matches!(ProviderType::from("gitlab"), ProviderType::GitLab));
    assert!(matches!(
        ProviderType::from("bitbucket"),
        ProviderType::Bitbucket
    ));
}

#[test]
fn test_provider_display_names() {
    assert_eq!(ProviderType::GitHub.display_name(), "GitHub");
    assert_eq!(ProviderType::GitLab.display_name(), "GitLab");
    assert_eq!(ProviderType::Bitbucket.display_name(), "Bitbucket");
    assert_eq!(ProviderType::Gitea.display_name(), "Gitea");
}

#[test]
fn test_provider_api_urls() {
    assert_eq!(
        ProviderType::GitHub.default_api_url(),
        "https://api.github.com"
    );
    assert_eq!(
        ProviderType::GitLab.default_api_url(),
        "https://gitlab.com/api/v4"
    );
}

#[test]
fn test_self_hosted_detection() {
    assert!(!ProviderType::GitHub.is_self_hosted());
    assert!(!ProviderType::GitLab.is_self_hosted());
    assert!(ProviderType::Gitea.is_self_hosted());
    assert!(ProviderType::Forgejo.is_self_hosted());
}

#[test]
fn test_https_url_templates() {
    let url = ProviderType::GitHub.https_url_template("user", "repo", None);
    assert_eq!(url, "https://github.com/user/repo.git");
}

#[test]
fn test_ssh_url_templates() {
    let url = ProviderType::GitHub.ssh_url_template("user", "repo", None);
    assert_eq!(url, "git@github.com:user/repo.git");
}

#[test]
fn test_rate_limit_is_low() {
    let limit = RateLimit {
        limit: 5000,
        remaining: 100,
        reset_at: Utc::now() + chrono::Duration::hours(1),
    };

    assert!(limit.is_low());
}

#[test]
fn test_rate_limit_is_exceeded() {
    let limit = RateLimit {
        limit: 5000,
        remaining: 0,
        reset_at: Utc::now() + chrono::Duration::hours(1),
    };

    assert!(limit.is_exceeded());
}

#[test]
fn test_rate_limit_time_until_reset() {
    let limit = RateLimit {
        limit: 5000,
        remaining: 1000,
        reset_at: Utc::now() + chrono::Duration::hours(1),
    };

    let duration = limit.time_until_reset();
    assert!(duration.num_seconds() > 0);
}

#[test]
fn test_repository_local() {
    let repo = Repository::local("/path/to/repo");
    assert_eq!(repo.path, "/path/to/repo");
    assert!(repo.remotes.is_empty());
}

#[test]
fn test_repository_builder() {
    let repo = Repository::builder("/path/to/repo")
        .with_remote(Remote::builder("origin", "user").build())
        .build();

    assert_eq!(repo.path, "/path/to/repo");
    assert_eq!(repo.remotes.len(), 1);
}

#[test]
fn test_settings_default() {
    let settings = Settings::default();
    assert_eq!(settings.default_branch, "main");
    assert!(settings.parallel_push);
    assert_eq!(settings.max_parallel, 4);
}

#[test]
fn test_sync_config_default() {
    let config = SyncConfig::default();
    assert!(matches!(config.strategy, SyncStrategy::FastForward));
    assert!(config.auto_fetch);
}

#[test]
fn test_sync_strategy_display() {
    assert_eq!(SyncStrategy::FastForward.to_string(), "fast-forward");
    assert_eq!(SyncStrategy::Merge.to_string(), "merge");
    assert_eq!(SyncStrategy::Rebase.to_string(), "rebase");
}

#[test]
fn test_auth_backend_variants() {
    // Verify AuthBackend enum variants exist
    let _keyring = AuthBackend::Keyring;
    let _file = AuthBackend::EncryptedFile;
    let _env = AuthBackend::Environment;
}

#[test]
fn test_remote_creation() {
    let remote = Remote {
        name: "origin".to_string(),
        provider: ProviderType::GitHub,
        username: "user".to_string(),
        api_url: None,
        enabled: true,
        use_ssh: false,
        priority: 0,
    };

    assert_eq!(remote.name, "origin");
    assert!(remote.enabled);
}
