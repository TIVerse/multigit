//! Provider trait definition
//!
//! Defines the common interface that all Git hosting providers must implement.

use crate::models::{RateLimit, Repository};
use async_trait::async_trait;

/// Configuration for creating a repository
#[derive(Debug, Clone)]
pub struct RepoConfig {
    /// Repository name
    pub name: String,
    /// Optional description
    pub description: String,
    /// Whether the repository is private
    pub private: bool,
}

/// Git protocol options
pub enum Protocol {
    /// HTTPS protocol
    Https,
    /// SSH protocol
    Ssh,
}

/// Provider trait that all hosting platforms implement
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Test connection to the provider
    async fn test_connection(&self) -> anyhow::Result<bool>;

    /// Create a new repository
    async fn create_repo(&self, config: RepoConfig) -> anyhow::Result<Repository>;

    /// Get repository information
    async fn get_repo(&self, name: &str) -> anyhow::Result<Repository>;

    /// Get the remote URL for a repository
    fn get_remote_url(&self, name: &str, protocol: Protocol) -> String;

    /// Create a branch
    async fn create_branch(&self, repo: &str, branch: &str) -> anyhow::Result<()>;

    /// Delete a branch
    async fn delete_branch(&self, repo: &str, branch: &str) -> anyhow::Result<()>;

    /// Get rate limit information
    async fn get_rate_limit(&self) -> anyhow::Result<RateLimit>;
}

// TODO: Implement this trait for each provider in Phase 3
