//! Git remote management
//!
//! Operations for managing Git remotes.

use crate::utils::error::{MultiGitError, Result};
use git2::Repository;
use tracing::{debug, info};

/// Remote management operations
pub struct RemoteManager<'repo> {
    repo: &'repo Repository,
}

impl<'repo> RemoteManager<'repo> {
    /// Create a new remote manager for a repository
    #[must_use]
    pub fn new(repo: &'repo Repository) -> Self {
        Self { repo }
    }

    /// Add a new remote
    pub fn add(&self, name: &str, url: &str) -> Result<()> {
        info!("Adding remote '{}' with URL: {}", name, url);

        // Check if remote already exists
        if self.exists(name)? {
            return Err(MultiGitError::Other(format!(
                "Remote '{name}' already exists. Use update to change the URL."
            )));
        }

        self.repo
            .remote(name, url)
            .map_err(MultiGitError::GitError)?;

        info!("Successfully added remote '{}'", name);
        Ok(())
    }

    /// Remove a remote
    pub fn remove(&self, name: &str) -> Result<()> {
        info!("Removing remote '{}'", name);

        if !self.exists(name)? {
            return Err(MultiGitError::RemoteNotFound(name.to_string()));
        }

        self.repo
            .remote_delete(name)
            .map_err(MultiGitError::GitError)?;

        info!("Successfully removed remote '{}'", name);
        Ok(())
    }

    /// Update a remote's URL
    pub fn update(&self, name: &str, url: &str) -> Result<()> {
        info!("Updating remote '{}' to URL: {}", name, url);

        if !self.exists(name)? {
            return Err(MultiGitError::RemoteNotFound(name.to_string()));
        }

        self.repo
            .remote_set_url(name, url)
            .map_err(MultiGitError::GitError)?;

        info!("Successfully updated remote '{}'", name);
        Ok(())
    }

    /// List all remotes
    pub fn list(&self) -> Result<Vec<String>> {
        let remotes = self.repo.remotes().map_err(MultiGitError::GitError)?;

        let remote_names: Vec<String> = remotes
            .iter()
            .filter_map(|name| name.map(String::from))
            .collect();

        debug!("Found {} remotes", remote_names.len());
        Ok(remote_names)
    }

    /// Get the URL of a remote
    pub fn get_url(&self, name: &str) -> Result<String> {
        let remote = self
            .repo
            .find_remote(name)
            .map_err(|_| MultiGitError::RemoteNotFound(name.to_string()))?;

        let url = remote
            .url()
            .ok_or_else(|| MultiGitError::Other(format!("Remote '{name}' has no URL")))?
            .to_string();

        debug!("Remote '{}' URL: {}", name, url);
        Ok(url)
    }

    /// Check if a remote exists
    pub fn exists(&self, name: &str) -> Result<bool> {
        let remotes = self.list()?;
        Ok(remotes.contains(&name.to_string()))
    }

    /// Rename a remote
    pub fn rename(&self, old_name: &str, new_name: &str) -> Result<()> {
        info!("Renaming remote '{}' to '{}'", old_name, new_name);

        if !self.exists(old_name)? {
            return Err(MultiGitError::RemoteNotFound(old_name.to_string()));
        }

        if self.exists(new_name)? {
            return Err(MultiGitError::Other(format!(
                "Remote '{new_name}' already exists"
            )));
        }

        self.repo
            .remote_rename(old_name, new_name)
            .map_err(MultiGitError::GitError)?;

        info!(
            "Successfully renamed remote '{}' to '{}'",
            old_name, new_name
        );
        Ok(())
    }

    /// Get push URL for a remote (falls back to fetch URL if not set)
    pub fn get_push_url(&self, name: &str) -> Result<String> {
        let remote = self
            .repo
            .find_remote(name)
            .map_err(|_| MultiGitError::RemoteNotFound(name.to_string()))?;

        // Try push URL first, fall back to fetch URL
        let url = remote
            .pushurl()
            .or_else(|| remote.url())
            .ok_or_else(|| MultiGitError::Other(format!("Remote '{name}' has no URL")))?
            .to_string();

        Ok(url)
    }
}

/// Helper functions for working with remote URLs
pub mod url_utils {
    use url::Url;

    /// Convert an HTTPS URL to SSH format
    pub fn https_to_ssh(https_url: &str) -> Result<String, String> {
        let url = Url::parse(https_url).map_err(|e| e.to_string())?;

        let host = url.host_str().ok_or("No host in URL")?;
        let path = url.path().trim_start_matches('/');

        Ok(format!("git@{host}:{path}"))
    }

    /// Convert an SSH URL to HTTPS format
    pub fn ssh_to_https(ssh_url: &str) -> Result<String, String> {
        // Handle git@github.com:user/repo.git format
        if ssh_url.starts_with("git@") {
            let parts: Vec<&str> = ssh_url.split(':').collect();
            if parts.len() != 2 {
                return Err("Invalid SSH URL format".to_string());
            }

            let host = parts[0].trim_start_matches("git@");
            let path = parts[1];

            return Ok(format!("https://{host}/{path}"));
        }

        // Handle ssh://git@github.com/user/repo.git format
        if let Ok(url) = Url::parse(ssh_url) {
            if let Some(host) = url.host_str() {
                let path = url.path().trim_start_matches('/');
                return Ok(format!("https://{host}/{path}"));
            }
        }

        Err("Unsupported SSH URL format".to_string())
    }

    /// Extract repository name from URL
    #[must_use]
    pub fn extract_repo_name(url: &str) -> Option<String> {
        let path = if url.contains("://") {
            Url::parse(url).ok()?.path().to_string()
        } else if url.contains(':') {
            url.split(':').nth(1)?.to_string()
        } else {
            return None;
        };

        let name = path
            .trim_start_matches('/')
            .trim_end_matches(".git")
            .split('/')
            .next_back()?
            .to_string();

        Some(name)
    }
}

#[cfg(test)]
mod tests {
    use super::url_utils::*;
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_add_and_list_remotes() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        let manager = RemoteManager::new(&repo);

        manager
            .add("origin", "https://github.com/user/repo.git")
            .unwrap();
        manager
            .add("upstream", "https://github.com/upstream/repo.git")
            .unwrap();

        let remotes = manager.list().unwrap();
        assert_eq!(remotes.len(), 2);
        assert!(remotes.contains(&"origin".to_string()));
        assert!(remotes.contains(&"upstream".to_string()));
    }

    #[test]
    fn test_remove_remote() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        let manager = RemoteManager::new(&repo);

        manager
            .add("origin", "https://github.com/user/repo.git")
            .unwrap();
        assert!(manager.exists("origin").unwrap());

        manager.remove("origin").unwrap();
        assert!(!manager.exists("origin").unwrap());
    }

    #[test]
    fn test_update_remote() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        let manager = RemoteManager::new(&repo);

        let old_url = "https://github.com/user/repo.git";
        let new_url = "https://github.com/newuser/repo.git";

        manager.add("origin", old_url).unwrap();
        assert_eq!(manager.get_url("origin").unwrap(), old_url);

        manager.update("origin", new_url).unwrap();
        assert_eq!(manager.get_url("origin").unwrap(), new_url);
    }

    #[test]
    fn test_url_conversion() {
        let https = "https://github.com/user/repo.git";
        let ssh = "git@github.com:user/repo.git";

        assert_eq!(https_to_ssh(https).unwrap(), ssh);
        assert_eq!(ssh_to_https(ssh).unwrap(), https);
    }

    #[test]
    fn test_extract_repo_name() {
        assert_eq!(
            extract_repo_name("https://github.com/user/myrepo.git"),
            Some("myrepo".to_string())
        );
        assert_eq!(
            extract_repo_name("git@github.com:user/myrepo.git"),
            Some("myrepo".to_string())
        );
    }
}
