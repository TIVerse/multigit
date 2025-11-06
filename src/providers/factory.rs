//! Provider factory for creating provider instances
//!
//! Centralizes provider creation logic to avoid duplication across commands.

use crate::providers::bitbucket::BitbucketProvider;
use crate::providers::gitea::GiteaProvider;
use crate::providers::github::GitHubProvider;
use crate::providers::gitlab::GitLabProvider;
use crate::providers::traits::Provider;
use crate::utils::error::{MultiGitError, Result};
use crate::utils::validation::{extract_host_from_url, validate_https_url};
use std::sync::Arc;

/// Create a provider instance from configuration
///
/// # Arguments
/// * `provider` - Provider name (github, gitlab, bitbucket, codeberg, gitea)
/// * `username` - Username on the provider
/// * `token` - Authentication token
/// * `api_url` - Optional custom API URL (required for self-hosted instances)
/// * `allow_insecure` - Whether to allow HTTP URLs (default: false)
///
/// # Returns
/// Arc-wrapped provider instance implementing the Provider trait
pub fn create_provider(
    provider: &str,
    username: &str,
    token: &str,
    api_url: Option<&str>,
    allow_insecure: bool,
) -> Result<Arc<dyn Provider>> {
    let provider_instance: Arc<dyn Provider> = match provider {
        "github" => {
            let p = GitHubProvider::new(token.to_string(), username.to_string())?;
            Arc::new(p)
        }
        "gitlab" => {
            let validated_url = if let Some(url) = api_url {
                Some(validate_https_url(url, allow_insecure)?)
            } else {
                None
            };
            let p = GitLabProvider::new(token.to_string(), username.to_string(), validated_url)?;
            Arc::new(p)
        }
        "bitbucket" => {
            let p = BitbucketProvider::new(username.to_string(), token.to_string())?;
            Arc::new(p)
        }
        "gitea" => {
            let url = api_url.ok_or_else(|| {
                MultiGitError::config("Gitea requires an API URL. Use --url flag".to_string())
            })?;
            let validated_url = validate_https_url(url, allow_insecure)?;
            let p = GiteaProvider::new(token.to_string(), username.to_string(), validated_url)?;
            Arc::new(p)
        }
        "codeberg" => {
            let p = GiteaProvider::new(
                token.to_string(),
                username.to_string(),
                "https://codeberg.org".to_string(),
            )?;
            Arc::new(p)
        }
        _ => {
            return Err(MultiGitError::other(format!(
                "Unsupported provider: {provider}"
            )));
        }
    };

    Ok(provider_instance)
}

/// Check if a provider name is supported
///
/// # Arguments
/// * `provider` - Provider name to validate
///
/// # Returns
/// `true` if the provider is supported, `false` otherwise
#[must_use]
pub fn is_supported_provider(provider: &str) -> bool {
    matches!(
        provider,
        "github" | "gitlab" | "bitbucket" | "codeberg" | "gitea"
    )
}

/// Get list of all supported provider names
#[must_use]
pub fn supported_providers() -> &'static [&'static str] {
    &["github", "gitlab", "bitbucket", "codeberg", "gitea"]
}

/// Get the canonical host for a provider (for credential binding)
///
/// # Arguments
/// * `provider` - Provider name
/// * `api_url` - Optional custom API URL (for self-hosted instances)
/// * `allow_insecure` - Whether to allow HTTP URLs (default: false)
///
/// # Returns
/// The host string to use for credential binding
pub fn get_provider_host(
    provider: &str,
    api_url: Option<&str>,
    allow_insecure: bool,
) -> Result<String> {
    match provider {
        "github" => Ok("github.com".to_string()),
        "bitbucket" => Ok("bitbucket.org".to_string()),
        "codeberg" => Ok("codeberg.org".to_string()),
        "gitlab" => {
            if let Some(url) = api_url {
                // Validate and extract host from custom URL
                let validated_url = validate_https_url(url, allow_insecure)?;
                extract_host_from_url(&validated_url)
            } else {
                Ok("gitlab.com".to_string())
            }
        }
        "gitea" => {
            let url = api_url
                .ok_or_else(|| MultiGitError::config("Gitea requires an API URL".to_string()))?;
            // Validate and extract host
            let validated_url = validate_https_url(url, allow_insecure)?;
            extract_host_from_url(&validated_url)
        }
        _ => Err(MultiGitError::other(format!(
            "Unsupported provider: {provider}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_providers() {
        assert!(is_supported_provider("github"));
        assert!(is_supported_provider("gitlab"));
        assert!(is_supported_provider("bitbucket"));
        assert!(is_supported_provider("codeberg"));
        assert!(is_supported_provider("gitea"));
        assert!(!is_supported_provider("invalid"));
    }

    #[test]
    fn test_supported_providers_list() {
        let providers = supported_providers();
        assert_eq!(providers.len(), 5);
        assert!(providers.contains(&"github"));
        assert!(providers.contains(&"gitlab"));
    }

    #[test]
    fn test_get_provider_host_saas() {
        // SaaS providers should return canonical hosts
        assert_eq!(
            get_provider_host("github", None, false).unwrap(),
            "github.com"
        );
        assert_eq!(
            get_provider_host("gitlab", None, false).unwrap(),
            "gitlab.com"
        );
        assert_eq!(
            get_provider_host("bitbucket", None, false).unwrap(),
            "bitbucket.org"
        );
        assert_eq!(
            get_provider_host("codeberg", None, false).unwrap(),
            "codeberg.org"
        );
    }

    #[test]
    fn test_get_provider_host_self_hosted() {
        // Self-hosted GitLab
        let host = get_provider_host("gitlab", Some("https://gitlab.example.com"), false);
        assert_eq!(host.unwrap(), "gitlab.example.com");

        // Self-hosted Gitea
        let host = get_provider_host("gitea", Some("https://git.example.com"), false);
        assert_eq!(host.unwrap(), "git.example.com");

        // HTTP should be rejected by default
        let host = get_provider_host("gitea", Some("http://git.example.com"), false);
        assert!(host.is_err());

        // HTTP should work with allow_insecure
        let host = get_provider_host("gitea", Some("http://git.example.com"), true);
        assert_eq!(host.unwrap(), "git.example.com");
    }
}
