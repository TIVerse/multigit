//! Repository model representing a Git repository with its metadata

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a Git repository with metadata from a hosting provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    /// Repository name (e.g., "multigit")
    pub name: String,

    /// Full repository name including owner (e.g., "TIVerse/multigit")
    pub full_name: Option<String>,

    /// HTTPS clone URL
    pub url: String,

    /// SSH clone URL
    pub ssh_url: String,

    /// Whether the repository is private
    pub private: bool,

    /// Default branch name (e.g., "main" or "master")
    pub default_branch: String,

    /// Optional description
    pub description: Option<String>,

    /// Repository URL on the hosting platform (web interface)
    pub html_url: Option<String>,

    /// When the repository was created
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// When the repository was last updated
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Repository {
    /// Create a new repository instance with required fields
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        ssh_url: impl Into<String>,
        private: bool,
        default_branch: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            full_name: None,
            url: url.into(),
            ssh_url: ssh_url.into(),
            private,
            default_branch: default_branch.into(),
            description: None,
            html_url: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Set the full name (owner/repo)
    pub fn with_full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the HTML URL
    pub fn with_html_url(mut self, html_url: impl Into<String>) -> Self {
        self.html_url = Some(html_url.into());
        self
    }
}

/// Configuration for creating a new repository on a hosting platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    /// Repository name
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Whether the repository should be private
    pub private: bool,

    /// Default branch name
    pub default_branch: String,

    /// Initialize with a README
    #[serde(default)]
    pub auto_init: bool,

    /// .gitignore template to use
    pub gitignore_template: Option<String>,

    /// License template to use
    pub license_template: Option<String>,
}

impl RepoConfig {
    /// Create a new repository configuration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            private: false,
            default_branch: "main".to_string(),
            auto_init: false,
            gitignore_template: None,
            license_template: None,
        }
    }

    /// Make the repository private
    pub fn private(mut self) -> Self {
        self.private = true;
        self
    }

    /// Set a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the default branch
    pub fn with_branch(mut self, branch: impl Into<String>) -> Self {
        self.default_branch = branch.into();
        self
    }

    /// Enable auto-initialization with README
    pub fn auto_init(mut self) -> Self {
        self.auto_init = true;
        self
    }
}

/// Local repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalRepo {
    /// Path to the local repository
    pub path: PathBuf,

    /// Current branch
    pub current_branch: Option<String>,

    /// Whether the working directory is clean
    pub is_clean: bool,

    /// Number of commits ahead of remote
    pub ahead: usize,

    /// Number of commits behind remote
    pub behind: usize,

    /// Last sync timestamp
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
}

impl LocalRepo {
    /// Create a new local repository reference
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            current_branch: None,
            is_clean: true,
            ahead: 0,
            behind: 0,
            last_sync: None,
        }
    }

    /// Check if the repository has uncommitted changes
    pub fn has_changes(&self) -> bool {
        !self.is_clean
    }

    /// Check if the repository is in sync (no ahead/behind commits)
    pub fn is_synced(&self) -> bool {
        self.ahead == 0 && self.behind == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(
            "test-repo",
            "https://github.com/user/test-repo.git",
            "git@github.com:user/test-repo.git",
            false,
            "main",
        );

        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.default_branch, "main");
        assert!(!repo.private);
    }

    #[test]
    fn test_repository_builder() {
        let repo = Repository::new(
            "test",
            "https://example.com",
            "git@example.com",
            true,
            "main",
        )
        .with_full_name("owner/test")
        .with_description("Test repository")
        .with_html_url("https://example.com/owner/test");

        assert_eq!(repo.full_name, Some("owner/test".to_string()));
        assert_eq!(repo.description, Some("Test repository".to_string()));
        assert!(repo.private);
    }

    #[test]
    fn test_repo_config() {
        let config = RepoConfig::new("my-repo")
            .private()
            .with_description("My awesome project")
            .with_branch("develop")
            .auto_init();

        assert_eq!(config.name, "my-repo");
        assert!(config.private);
        assert_eq!(config.default_branch, "develop");
        assert!(config.auto_init);
    }

    #[test]
    fn test_local_repo() {
        let local = LocalRepo::new(PathBuf::from("/tmp/repo"));

        assert!(local.is_clean);
        assert!(local.is_synced());
        assert!(!local.has_changes());
    }
}
