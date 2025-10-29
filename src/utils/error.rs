//! Error types for `MultiGit`
//!
//! This module defines all error types used throughout the application,
//! with detailed messages to help users diagnose and fix issues.

use thiserror::Error;

/// Main error type for `MultiGit` operations
#[derive(Debug, Error)]
pub enum MultiGitError {
    /// Git operation failed
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    /// Network error during API calls
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Authentication failure
    #[error("Authentication failed for {provider}: {reason}")]
    AuthError {
        /// Provider name
        provider: String,
        /// Reason for failure
        reason: String,
    },

    /// Repository not found
    #[error("Repository not found: {0}")]
    RepoNotFound(String),

    /// Remote not configured
    #[error("Remote '{0}' not configured")]
    RemoteNotFound(String),

    /// Conflict detected during sync
    #[error("Conflict detected: {0}")]
    ConflictError(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded for {provider}. Please try again in {minutes} minutes.")]
    RateLimitError {
        /// Provider name
        provider: String,
        /// Minutes until reset
        minutes: u32,
    },

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Invalid input or arguments
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerdeError(String),

    /// TOML parsing error
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Keyring error (credential storage)
    #[error("Keyring error: {0}")]
    KeyringError(String),

    /// Provider-specific error
    #[error("Provider error ({provider}): {message}")]
    ProviderError {
        /// Provider name
        provider: String,
        /// Error message
        message: String,
    },

    /// Daemon error
    #[error("Daemon error: {0}")]
    DaemonError(String),

    /// Not initialized error
    #[error("MultiGit not initialized in this repository. Run 'multigit init' first.")]
    NotInitialized,

    /// Already initialized error
    #[error("MultiGit already initialized in this repository")]
    AlreadyInitialized,

    /// Generic error for other cases
    #[error("{0}")]
    Other(String),
}

impl MultiGitError {
    /// Create an authentication error
    pub fn auth(provider: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::AuthError {
            provider: provider.into(),
            reason: reason.into(),
        }
    }

    /// Create a provider error
    pub fn provider(provider: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ProviderError {
            provider: provider.into(),
            message: message.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(provider: impl Into<String>, reset_in_seconds: u64) -> Self {
        Self::RateLimitError {
            provider: provider.into(),
            minutes: ((reset_in_seconds + 59) / 60) as u32, // Round up to minutes
        }
    }

    /// Create a configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError(message.into())
    }

    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::ConflictError(message.into())
    }

    /// Create a generic other error
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }

    /// Create a network error from a message
    pub fn network(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }

    /// Create a daemon error
    pub fn daemon(message: impl Into<String>) -> Self {
        Self::DaemonError(message.into())
    }

    /// Get a user-friendly error message with troubleshooting hints
    #[must_use]
    pub fn user_message(&self) -> String {
        match self {
            Self::AuthError { provider, reason } => {
                format!(
                    "Authentication failed for {provider}.\n\n\
                     Reason: {reason}\n\n\
                     Troubleshooting:\n\
                     1. Verify your credentials with: multigit remote test {provider}\n\
                     2. Update your token with: multigit remote update {provider}\n\
                     3. Check token permissions on the provider's website\n\
                     4. Ensure your token hasn't expired"
                )
            }
            Self::RateLimitError { provider, minutes } => {
                format!(
                    "Rate limit exceeded for {provider}. Please wait {minutes} minutes before trying again.\n\n\
                     Tip: Use 'multigit status --remote {provider}' to check rate limit status."
                )
            }
            Self::NotInitialized => "MultiGit is not initialized in this repository.\n\n\
                 Run 'multigit init' to get started."
                .to_string(),
            Self::ConfigError(msg) => {
                format!(
                    "Configuration error: {msg}\n\n\
                     Check your configuration files:\n\
                     - Repository: .multigit/config.toml\n\
                     - User: ~/.config/multigit/config.toml\n\n\
                     Run 'multigit doctor' to diagnose configuration issues."
                )
            }
            Self::ConflictError(msg) => {
                format!(
                    "Conflict detected: {msg}\n\n\
                     Use 'multigit conflict list' to see all conflicts.\n\
                     Use 'multigit conflict resolve' for interactive resolution."
                )
            }
            _ => self.to_string(),
        }
    }

    /// Check if this error is retryable
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::NetworkError(_) | Self::RateLimitError { .. })
    }

    /// Check if this is a authentication-related error
    #[must_use]
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError { .. } | Self::KeyringError(_))
    }
}

// Convert keyring errors to our error type
impl From<keyring::Error> for MultiGitError {
    fn from(err: keyring::Error) -> Self {
        Self::KeyringError(err.to_string())
    }
}

/// Result type alias for `MultiGit` operations
pub type Result<T> = std::result::Result<T, MultiGitError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_error() {
        let err = MultiGitError::auth("github", "Invalid token");
        assert!(err.is_auth_error());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_rate_limit_error() {
        let err = MultiGitError::rate_limit("gitlab", 3600);
        assert!(err.is_retryable());

        if let MultiGitError::RateLimitError { provider, minutes } = err {
            assert_eq!(provider, "gitlab");
            assert_eq!(minutes, 60);
        } else {
            panic!("Expected RateLimitError");
        }
    }

    #[test]
    fn test_provider_error() {
        let err = MultiGitError::provider("github", "Repository creation failed");
        match err {
            MultiGitError::ProviderError { provider, message } => {
                assert_eq!(provider, "github");
                assert_eq!(message, "Repository creation failed");
            }
            _ => panic!("Expected ProviderError"),
        }
    }

    #[test]
    fn test_user_message() {
        let err = MultiGitError::NotInitialized;
        let msg = err.user_message();
        assert!(msg.contains("multigit init"));
    }

    #[test]
    fn test_error_display() {
        let err = MultiGitError::RemoteNotFound("github".to_string());
        assert_eq!(err.to_string(), "Remote 'github' not configured");
    }
}
