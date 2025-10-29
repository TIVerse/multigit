//! Configuration management for MultiGit
//!
//! Implements hierarchical configuration loading from:
//! 1. CLI flags (highest priority)
//! 2. Repository config (.multigit/config.toml)
//! 3. User config (~/.config/multigit/config.toml)
//! 4. Default values (lowest priority)

use crate::models::{DaemonConfig, SecurityConfig, Settings, SyncConfig};
use crate::utils::error::{MultiGitError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// General settings
    #[serde(default)]
    pub settings: Settings,

    /// Synchronization configuration
    #[serde(default)]
    pub sync: SyncConfig,

    /// Security and authentication configuration
    #[serde(default)]
    pub security: SecurityConfig,

    /// Daemon configuration
    #[serde(default)]
    pub daemon: DaemonConfig,

    /// Configured remotes
    #[serde(default)]
    pub remotes: HashMap<String, RemoteConfig>,
}

/// Remote configuration stored in config file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// Username on the provider
    pub username: String,

    /// Custom API URL (for self-hosted instances)
    pub api_url: Option<String>,

    /// Whether this remote is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Provider type (github, gitlab, etc.)
    pub provider: Option<String>,

    /// Use SSH instead of HTTPS
    #[serde(default)]
    pub use_ssh: bool,

    /// Priority for conflict resolution
    #[serde(default)]
    pub priority: i32,
}

fn default_true() -> bool {
    true
}

impl Config {
    /// Load configuration from all sources
    pub fn load() -> Result<Self> {
        // Start with defaults
        let mut config = Self::default();

        // Load user config if it exists
        if let Some(user_config) = Self::load_user_config()? {
            config = config.merge(user_config);
        }

        // Load repo config if it exists (highest priority)
        if let Some(repo_config) = Self::load_repo_config()? {
            config = config.merge(repo_config);
        }

        Ok(config)
    }

    /// Load user-level configuration from ~/.config/multigit/config.toml
    fn load_user_config() -> Result<Option<Self>> {
        let config_path = Self::user_config_path()?;

        if !config_path.exists() {
            tracing::debug!("User config not found at: {}", config_path.display());
            return Ok(None);
        }

        tracing::debug!("Loading user config from: {}", config_path.display());
        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| MultiGitError::config(format!("Failed to parse user config: {}", e)))?;

        Ok(Some(config))
    }

    /// Load repository-level configuration from .multigit/config.toml
    fn load_repo_config() -> Result<Option<Self>> {
        let config_path = PathBuf::from(".multigit/config.toml");

        if !config_path.exists() {
            tracing::debug!("Repo config not found at: {}", config_path.display());
            return Ok(None);
        }

        tracing::debug!("Loading repo config from: {}", config_path.display());
        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| MultiGitError::config(format!("Failed to parse repo config: {}", e)))?;

        Ok(Some(config))
    }

    /// Get the user config directory path
    pub fn user_config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| MultiGitError::config("Cannot determine config directory"))?;
        Ok(config_dir.join("multigit"))
    }

    /// Get the user config file path
    pub fn user_config_path() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("config.toml"))
    }

    /// Get the repo config directory path
    pub fn repo_config_dir() -> PathBuf {
        PathBuf::from(".multigit")
    }

    /// Get the repo config file path
    pub fn repo_config_path() -> PathBuf {
        Self::repo_config_dir().join("config.toml")
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| MultiGitError::config(format!("Failed to serialize config: {}", e)))?;

        fs::write(path, content)?;
        tracing::info!("Saved configuration to: {}", path.display());

        Ok(())
    }

    /// Save to user config file
    pub fn save_user_config(&self) -> Result<()> {
        let path = Self::user_config_path()?;
        self.save_to_file(&path)
    }

    /// Save to repository config file
    pub fn save_repo_config(&self) -> Result<()> {
        let path = Self::repo_config_path();
        self.save_to_file(&path)
    }

    /// Save configuration (defaults to user config)
    pub fn save(&self) -> Result<()> {
        self.save_user_config()
    }

    /// Merge another config into this one (other has priority)
    fn merge(mut self, other: Config) -> Self {
        // Merge remotes (other overwrites)
        self.remotes.extend(other.remotes);

        // For other fields, we take from 'other' if they're not default
        // This is a simplified merge - in production you might want more sophisticated merging
        if other.settings.default_branch != Settings::default().default_branch {
            self.settings.default_branch = other.settings.default_branch;
        }

        self.settings.parallel_push = other.settings.parallel_push;
        self.settings.max_parallel = other.settings.max_parallel;
        self.settings.colored_output = other.settings.colored_output;
        self.settings.verbosity = other.settings.verbosity;

        // Merge sync config
        if other.sync.auto_sync {
            self.sync.auto_sync = true;
        }
        if other.sync.primary_source.is_some() {
            self.sync.primary_source = other.sync.primary_source;
        }
        if !other.sync.include_branches.is_empty() {
            self.sync.include_branches = other.sync.include_branches;
        }
        if !other.sync.exclude_branches.is_empty() {
            self.sync.exclude_branches = other.sync.exclude_branches;
        }

        // Merge security config
        self.security = other.security;

        // Merge daemon config
        if other.daemon.enabled {
            self.daemon = other.daemon;
        }

        self
    }

    /// Add a remote to the configuration
    pub fn add_remote(&mut self, name: String, config: RemoteConfig) {
        self.remotes.insert(name, config);
    }

    /// Remove a remote from the configuration
    pub fn remove_remote(&mut self, name: &str) -> Option<RemoteConfig> {
        self.remotes.remove(name)
    }

    /// Get a remote configuration
    pub fn get_remote(&self, name: &str) -> Option<&RemoteConfig> {
        self.remotes.get(name)
    }

    /// Get all enabled remotes
    pub fn enabled_remotes(&self) -> HashMap<String, &RemoteConfig> {
        self.remotes
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(name, config)| (name.clone(), config))
            .collect()
    }

    /// Check if MultiGit is initialized in the current directory
    pub fn is_initialized() -> bool {
        Self::repo_config_dir().exists()
    }

    /// Initialize MultiGit in the current directory
    pub fn initialize() -> Result<()> {
        let config_dir = Self::repo_config_dir();

        if config_dir.exists() {
            return Err(MultiGitError::AlreadyInitialized);
        }

        // Create .multigit directory
        fs::create_dir_all(&config_dir)?;

        // Create default config file
        let default_config = Config::default();
        default_config.save_repo_config()?;

        tracing::info!("Initialized MultiGit in current directory");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.settings.default_branch, "main");
        assert!(config.settings.parallel_push);
        assert_eq!(config.settings.max_parallel, 4);
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = Config::default();
        config1.settings.default_branch = "master".to_string();

        let mut config2 = Config::default();
        config2.settings.default_branch = "develop".to_string();
        config2.settings.max_parallel = 8;

        let merged = config1.merge(config2);
        assert_eq!(merged.settings.default_branch, "develop");
        assert_eq!(merged.settings.max_parallel, 8);
    }

    #[test]
    fn test_add_remove_remote() {
        let mut config = Config::default();

        let remote_config = RemoteConfig {
            username: "testuser".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        };

        config.add_remote("github".to_string(), remote_config);
        assert!(config.get_remote("github").is_some());

        config.remove_remote("github");
        assert!(config.get_remote("github").is_none());
    }

    #[test]
    fn test_enabled_remotes() {
        let mut config = Config::default();

        let enabled = RemoteConfig {
            username: "user1".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        };

        let disabled = RemoteConfig {
            username: "user2".to_string(),
            api_url: None,
            enabled: false,
            provider: Some("gitlab".to_string()),
            use_ssh: false,
            priority: 0,
        };

        config.add_remote("github".to_string(), enabled);
        config.add_remote("gitlab".to_string(), disabled);

        let enabled_remotes = config.enabled_remotes();
        assert_eq!(enabled_remotes.len(), 1);
        assert!(enabled_remotes.contains_key("github"));
    }

    #[test]
    fn test_save_and_load_config() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let mut config = Config::default();
        config.settings.default_branch = "develop".to_string();
        config.save_to_file(&config_path)?;

        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path)?;
        assert!(content.contains("develop"));

        Ok(())
    }
}
