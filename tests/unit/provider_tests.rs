//! Provider module tests

use multigit::models::remote::ProviderType;
use multigit::providers::github::GitHubProvider;
use multigit::providers::traits::{Protocol, Provider};

#[test]
fn test_github_provider_creation() {
    let result = GitHubProvider::new("test_token".into(), "testuser".into());
    assert!(result.is_ok());
}

#[test]
fn test_github_https_url() {
    let provider = GitHubProvider::new("token".into(), "user".into()).unwrap();
    let url = provider.get_remote_url("repo", Protocol::Https);
    assert_eq!(url, "https://github.com/user/repo.git");
}

#[test]
fn test_github_ssh_url() {
    let provider = GitHubProvider::new("token".into(), "user".into()).unwrap();
    let url = provider.get_remote_url("repo", Protocol::Ssh);
    assert_eq!(url, "git@github.com:user/repo.git");
}

#[test]
fn test_provider_type_parsing() {
    assert!(matches!(
        "github".parse::<ProviderType>().unwrap(),
        ProviderType::GitHub
    ));
    assert!(matches!(
        "gitlab".parse::<ProviderType>().unwrap(),
        ProviderType::GitLab
    ));
    assert!(matches!(
        "bitbucket".parse::<ProviderType>().unwrap(),
        ProviderType::Bitbucket
    ));
    assert!(matches!(
        "gitea".parse::<ProviderType>().unwrap(),
        ProviderType::Gitea
    ));
    assert!(matches!(
        "forgejo".parse::<ProviderType>().unwrap(),
        ProviderType::Forgejo
    ));
    assert!(matches!(
        "codeberg".parse::<ProviderType>().unwrap(),
        ProviderType::Codeberg
    ));
}

#[test]
fn test_provider_type_display() {
    assert_eq!(ProviderType::GitHub.display_name(), "GitHub");
    assert_eq!(ProviderType::GitLab.display_name(), "GitLab");
    assert_eq!(ProviderType::Bitbucket.display_name(), "Bitbucket");
    assert_eq!(ProviderType::Gitea.display_name(), "Gitea");
    assert_eq!(ProviderType::Forgejo.display_name(), "Forgejo");
    assert_eq!(ProviderType::Codeberg.display_name(), "Codeberg");
}

#[test]
fn test_provider_default_api_urls() {
    assert_eq!(
        ProviderType::GitHub.default_api_url(),
        "https://api.github.com"
    );
    assert_eq!(
        ProviderType::GitLab.default_api_url(),
        "https://gitlab.com/api/v4"
    );
    assert_eq!(
        ProviderType::Bitbucket.default_api_url(),
        "https://api.bitbucket.org/2.0"
    );
}

#[test]
fn test_self_hosted_providers() {
    assert!(!ProviderType::GitHub.is_self_hosted());
    assert!(!ProviderType::GitLab.is_self_hosted());
    assert!(!ProviderType::Bitbucket.is_self_hosted());
    assert!(ProviderType::Gitea.is_self_hosted());
    assert!(ProviderType::Forgejo.is_self_hosted());
}

#[test]
fn test_https_url_templates() {
    let github_url = ProviderType::GitHub.https_url_template("user", "repo", None);
    assert_eq!(github_url, "https://github.com/user/repo.git");

    let gitlab_url = ProviderType::GitLab.https_url_template("user", "repo", None);
    assert_eq!(gitlab_url, "https://gitlab.com/user/repo.git");

    let bitbucket_url = ProviderType::Bitbucket.https_url_template("user", "repo", None);
    assert_eq!(bitbucket_url, "https://bitbucket.org/user/repo.git");
}

#[test]
fn test_ssh_url_templates() {
    let github_url = ProviderType::GitHub.ssh_url_template("user", "repo", None);
    assert_eq!(github_url, "git@github.com:user/repo.git");

    let gitlab_url = ProviderType::GitLab.ssh_url_template("user", "repo", None);
    assert_eq!(gitlab_url, "git@gitlab.com:user/repo.git");

    let bitbucket_url = ProviderType::Bitbucket.ssh_url_template("user", "repo", None);
    assert_eq!(bitbucket_url, "git@bitbucket.org:user/repo.git");
}

#[test]
fn test_self_hosted_urls() {
    let gitea_https =
        ProviderType::Gitea.https_url_template("user", "repo", Some("https://git.example.com"));
    assert_eq!(gitea_https, "https://git.example.com/user/repo.git");

    let gitea_ssh =
        ProviderType::Gitea.ssh_url_template("user", "repo", Some("https://git.example.com"));
    assert_eq!(gitea_ssh, "git@git.example.com:user/repo.git");
}
