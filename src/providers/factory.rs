//! Provider factory for creating provider instances
//!
//! Centralizes provider creation logic to avoid duplication across commands.

use crate::providers::bitbucket::BitbucketProvider;
use crate::providers::gitea::GiteaProvider;
use crate::providers::github::GitHubProvider;
use crate::providers::gitlab::GitLabProvider;
use crate::providers::traits::Provider;
use crate::utils::error::{MultiGitError, Result};
use std::sync::Arc;

/// Create a provider instance from configuration
///
/// # Arguments
/// * `provider` - Provider name (github, gitlab, bitbucket, codeberg, gitea)
/// * `username` - Username on the provider
/// * `token` - Authentication token
/// * `api_url` - Optional custom API URL (required for self-hosted instances)
///
/// # Returns
/// Arc-wrapped provider instance implementing the Provider trait
pub fn create_provider(
    provider: &str,
    username: &str,
    token: &str,
    api_url: Option<&str>,
) -> Result<Arc<dyn Provider>> {
    let provider_instance: Arc<dyn Provider> = match provider {
        "github" => {
            let p = GitHubProvider::new(token.to_string(), username.to_string())?;
            Arc::new(p)
        }
        "gitlab" => {
            let url = api_url.map(std::string::ToString::to_string);
            let p = GitLabProvider::new(token.to_string(), username.to_string(), url)?;
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
            let p = GiteaProvider::new(token.to_string(), username.to_string(), url.to_string())?;
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
}
