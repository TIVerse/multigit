//! Configuration models for `MultiGit` settings

use serde::{Deserialize, Serialize};

/// Settings for general `MultiGit` behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Default branch to use for new operations
    #[serde(default = "default_branch")]
    pub default_branch: String,

    /// Enable parallel push operations
    #[serde(default = "default_true")]
    pub parallel_push: bool,

    /// Maximum number of parallel operations
    #[serde(default = "default_parallel")]
    pub max_parallel: usize,

    /// Enable colored output
    #[serde(default = "default_true")]
    pub colored_output: bool,

    /// Verbosity level (0-3: error, warn, info, debug)
    #[serde(default)]
    pub verbosity: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_branch: default_branch(),
            parallel_push: true,
            max_parallel: default_parallel(),
            colored_output: true,
            verbosity: 1, // warn level
        }
    }
}

fn default_branch() -> String {
    "main".into()
}

fn default_true() -> bool {
    true
}

fn default_parallel() -> usize {
    4
}

/// Synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Enable automatic synchronization
    #[serde(default)]
    pub auto_sync: bool,

    /// Primary source remote for pull operations
    pub primary_source: Option<String>,

    /// Sync strategy to use
    #[serde(default = "default_strategy")]
    pub strategy: SyncStrategy,

    /// Branches to include in sync (empty = all)
    #[serde(default)]
    pub include_branches: Vec<String>,

    /// Branches to exclude from sync
    #[serde(default)]
    pub exclude_branches: Vec<String>,

    /// Enable conflict detection before sync
    #[serde(default = "default_true")]
    pub detect_conflicts: bool,

    /// Automatically resolve conflicts if possible
    #[serde(default)]
    pub auto_resolve: bool,
}

fn default_strategy() -> SyncStrategy {
    SyncStrategy::FastForward
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: false,
            primary_source: None,
            strategy: default_strategy(),
            include_branches: Vec::new(),
            exclude_branches: Vec::new(),
            detect_conflicts: true, // Default to true for safety
            auto_resolve: false,
        }
    }
}

/// Strategy for handling synchronization
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SyncStrategy {
    /// Only allow fast-forward merges (safest)
    FastForward,

    /// Allow merge commits
    Merge,

    /// Rebase changes
    Rebase,

    /// Force push (dangerous - use with caution)
    Force,
}

impl Default for SyncStrategy {
    fn default() -> Self {
        Self::FastForward
    }
}

impl std::fmt::Display for SyncStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FastForward => write!(f, "fast-forward"),
            Self::Merge => write!(f, "merge"),
            Self::Rebase => write!(f, "rebase"),
            Self::Force => write!(f, "force"),
        }
    }
}

/// Security and authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication backend to use
    #[serde(default = "default_auth")]
    pub auth_backend: AuthBackend,

    /// Enable audit logging of sensitive operations
    #[serde(default)]
    pub audit_log: bool,

    /// Path to custom encryption key (optional)
    pub encryption_key_path: Option<String>,

    /// Enable SSH agent forwarding
    #[serde(default = "default_true")]
    pub ssh_agent: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_backend: default_auth(),
            audit_log: false,
            encryption_key_path: None,
            ssh_agent: true,
        }
    }
}

fn default_auth() -> AuthBackend {
    AuthBackend::Keyring
}

/// Authentication backend options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuthBackend {
    /// Use OS native keyring
    Keyring,

    /// Use encrypted file storage
    EncryptedFile,

    /// Use environment variables (not recommended for production)
    Environment,
}

impl std::fmt::Display for AuthBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyring => write!(f, "keyring"),
            Self::EncryptedFile => write!(f, "encrypted-file"),
            Self::Environment => write!(f, "environment"),
        }
    }
}

/// Daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DaemonConfig {
    /// Enable daemon on startup
    #[serde(default)]
    pub enabled: bool,

    /// Sync interval in seconds
    #[serde(default = "default_interval")]
    pub interval_seconds: u64,

    /// Run daemon in background
    #[serde(default = "default_true")]
    pub background: bool,

    /// Log file path for daemon
    pub log_file: Option<String>,

    /// PID file path
    pub pid_file: Option<String>,
}

fn default_interval() -> u64 {
    300 // 5 minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.default_branch, "main");
        assert!(settings.parallel_push);
        assert_eq!(settings.max_parallel, 4);
    }

    #[test]
    fn test_sync_strategy_display() {
        assert_eq!(SyncStrategy::FastForward.to_string(), "fast-forward");
        assert_eq!(SyncStrategy::Merge.to_string(), "merge");
    }

    #[test]
    fn test_auth_backend() {
        assert_eq!(AuthBackend::Keyring.to_string(), "keyring");
        assert_eq!(AuthBackend::EncryptedFile.to_string(), "encrypted-file");
    }

    #[test]
    fn test_sync_config_default() {
        let config = SyncConfig::default();
        assert!(!config.auto_sync);
        assert_eq!(config.strategy, SyncStrategy::FastForward);
        assert!(config.detect_conflicts);
    }
}
