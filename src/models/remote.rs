//! Remote configuration models for Git hosting platforms

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a configured Git hosting platform remote
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Remote {
    /// Remote name (e.g., "github", "gitlab")
    pub name: String,

    /// Provider type
    pub provider: ProviderType,

    /// Username on the platform
    pub username: String,

    /// Custom API URL for self-hosted instances
    pub api_url: Option<String>,

    /// Whether this remote is enabled for sync operations
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Whether to use SSH instead of HTTPS
    #[serde(default)]
    pub use_ssh: bool,

    /// Priority for conflict resolution (higher = preferred)
    #[serde(default)]
    pub priority: i32,
}

fn default_true() -> bool {
    true
}

impl Remote {
    /// Create a new remote configuration
    pub fn new(
        name: impl Into<String>,
        provider: ProviderType,
        username: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            provider,
            username: username.into(),
            api_url: None,
            enabled: true,
            use_ssh: false,
            priority: 0,
        }
    }

    /// Set a custom API URL (for self-hosted instances)
    pub fn with_api_url(mut self, url: impl Into<String>) -> Self {
        self.api_url = Some(url.into());
        self
    }

    /// Enable SSH protocol
    pub fn with_ssh(mut self) -> Self {
        self.use_ssh = true;
        self
    }

    /// Set priority for conflict resolution
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Disable this remote
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Get the Git remote name (e.g., "multigit-github")
    pub fn git_remote_name(&self) -> String {
        format!("multigit-{}", self.name)
    }
}

/// Supported Git hosting providers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    GitHub,
    GitLab,
    Bitbucket,
    Codeberg,
    Gitea,
    Forgejo,
}

impl ProviderType {
    /// Get the default API URL for this provider
    pub fn default_api_url(&self) -> &'static str {
        match self {
            Self::GitHub => "https://api.github.com",
            Self::GitLab => "https://gitlab.com/api/v4",
            Self::Bitbucket => "https://api.bitbucket.org/2.0",
            Self::Codeberg => "https://codeberg.org/api/v1",
            Self::Gitea => "",   // Must be provided by user
            Self::Forgejo => "", // Must be provided by user
        }
    }

    /// Get the display name for this provider
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::GitHub => "GitHub",
            Self::GitLab => "GitLab",
            Self::Bitbucket => "Bitbucket",
            Self::Codeberg => "Codeberg",
            Self::Gitea => "Gitea",
            Self::Forgejo => "Forgejo",
        }
    }

    /// Check if this is a self-hosted provider type
    pub fn is_self_hosted(&self) -> bool {
        matches!(self, Self::Gitea | Self::Forgejo)
    }

    /// Get the HTTPS clone URL template
    pub fn https_url_template(&self, username: &str, repo: &str, api_url: Option<&str>) -> String {
        match self {
            Self::GitHub => format!("https://github.com/{}/{}.git", username, repo),
            Self::GitLab => format!("https://gitlab.com/{}/{}.git", username, repo),
            Self::Bitbucket => format!("https://bitbucket.org/{}/{}.git", username, repo),
            Self::Codeberg => format!("https://codeberg.org/{}/{}.git", username, repo),
            Self::Gitea | Self::Forgejo => {
                let base_url = api_url.unwrap_or("https://localhost");
                format!("{}/{}/{}.git", base_url, username, repo)
            }
        }
    }

    /// Get the SSH clone URL template
    pub fn ssh_url_template(&self, username: &str, repo: &str, api_url: Option<&str>) -> String {
        match self {
            Self::GitHub => format!("git@github.com:{}/{}.git", username, repo),
            Self::GitLab => format!("git@gitlab.com:{}/{}.git", username, repo),
            Self::Bitbucket => format!("git@bitbucket.org:{}/{}.git", username, repo),
            Self::Codeberg => format!("git@codeberg.org:{}/{}.git", username, repo),
            Self::Gitea | Self::Forgejo => {
                // Extract host from API URL
                let host = api_url
                    .and_then(|u| url::Url::parse(u).ok())
                    .and_then(|u| u.host_str().map(String::from))
                    .unwrap_or_else(|| "localhost".to_string());
                format!("git@{}:{}/{}.git", host, username, repo)
            }
        }
    }
}

impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for ProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(Self::GitHub),
            "gitlab" => Ok(Self::GitLab),
            "bitbucket" => Ok(Self::Bitbucket),
            "codeberg" => Ok(Self::Codeberg),
            "gitea" => Ok(Self::Gitea),
            "forgejo" => Ok(Self::Forgejo),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

/// Protocol to use for Git operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Https,
    Ssh,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Https => write!(f, "HTTPS"),
            Self::Ssh => write!(f, "SSH"),
        }
    }
}

/// Rate limiting information from a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Total requests allowed in the time window
    pub limit: u32,

    /// Remaining requests in the current window
    pub remaining: u32,

    /// When the rate limit resets
    pub reset_at: chrono::DateTime<chrono::Utc>,
}

impl RateLimit {
    /// Check if we're close to hitting the rate limit (< 10% remaining)
    pub fn is_low(&self) -> bool {
        self.remaining < (self.limit / 10)
    }

    /// Check if the rate limit has been exceeded
    pub fn is_exceeded(&self) -> bool {
        self.remaining == 0
    }

    /// Get the duration until the rate limit resets
    pub fn time_until_reset(&self) -> chrono::Duration {
        self.reset_at - chrono::Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_creation() {
        let remote = Remote::new("github", ProviderType::GitHub, "testuser");

        assert_eq!(remote.name, "github");
        assert_eq!(remote.provider, ProviderType::GitHub);
        assert_eq!(remote.username, "testuser");
        assert!(remote.enabled);
        assert!(!remote.use_ssh);
    }

    #[test]
    fn test_remote_builder() {
        let remote = Remote::new("gitlab", ProviderType::GitLab, "user")
            .with_ssh()
            .with_priority(10);

        assert!(remote.use_ssh);
        assert_eq!(remote.priority, 10);
    }

    #[test]
    fn test_git_remote_name() {
        let remote = Remote::new("github", ProviderType::GitHub, "user");
        assert_eq!(remote.git_remote_name(), "multigit-github");
    }

    #[test]
    fn test_provider_type_parsing() {
        assert_eq!(
            "github".parse::<ProviderType>().unwrap(),
            ProviderType::GitHub
        );
        assert_eq!(
            "GitLab".parse::<ProviderType>().unwrap(),
            ProviderType::GitLab
        );
        assert!("invalid".parse::<ProviderType>().is_err());
    }

    #[test]
    fn test_provider_urls() {
        let github = ProviderType::GitHub;

        let https = github.https_url_template("user", "repo", None);
        assert_eq!(https, "https://github.com/user/repo.git");

        let ssh = github.ssh_url_template("user", "repo", None);
        assert_eq!(ssh, "git@github.com:user/repo.git");
    }

    #[test]
    fn test_self_hosted_urls() {
        let gitea = ProviderType::Gitea;

        let https = gitea.https_url_template("user", "repo", Some("https://git.example.com"));
        assert_eq!(https, "https://git.example.com/user/repo.git");
    }

    #[test]
    fn test_rate_limit() {
        let rate_limit = RateLimit {
            limit: 5000,
            remaining: 100,
            reset_at: chrono::Utc::now() + chrono::Duration::hours(1),
        };

        assert!(rate_limit.is_low());
        assert!(!rate_limit.is_exceeded());
    }
}
