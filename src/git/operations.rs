//! Core Git operations
//!
//! Provides a wrapper around git2 for common repository operations.

use crate::utils::error::{MultiGitError, Result};
use git2::{BranchType, Commit, Oid, Repository, StatusOptions};
use std::path::Path;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Default timeout for network operations (5 minutes)
const DEFAULT_NETWORK_TIMEOUT_SECS: u64 = 300;

/// Wrapper for Git operations using libgit2
pub struct GitOperations {
    repo: Repository,
    network_timeout: Duration,
}

impl GitOperations {
    /// Open an existing repository at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        debug!("Opening repository at: {}", path.display());

        let repo = Repository::open(path).map_err(MultiGitError::GitError)?;

        info!("Successfully opened repository at {}", path.display());
        Ok(Self {
            repo,
            network_timeout: Duration::from_secs(DEFAULT_NETWORK_TIMEOUT_SECS),
        })
    }

    /// Initialize a new repository at the given path
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        debug!("Initializing repository at: {}", path.display());

        let repo = Repository::init(path).map_err(MultiGitError::GitError)?;

        info!("Successfully initialized repository at {}", path.display());
        Ok(Self {
            repo,
            network_timeout: Duration::from_secs(DEFAULT_NETWORK_TIMEOUT_SECS),
        })
    }

    /// Set the network timeout for fetch/push/clone operations
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.network_timeout = timeout;
        self
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head().map_err(MultiGitError::GitError)?;

        if !head.is_branch() {
            return Err(MultiGitError::Other(
                "HEAD is not pointing to a branch (detached HEAD state)".to_string(),
            ));
        }

        let branch_name = head
            .shorthand()
            .ok_or_else(|| MultiGitError::Other("Invalid branch name".to_string()))?
            .to_string();

        debug!("Current branch: {}", branch_name);
        Ok(branch_name)
    }

    /// Alias for `current_branch` - Get the current branch name
    pub fn get_current_branch(&self) -> Result<String> {
        self.current_branch()
    }

    /// Check if the working directory is clean (no uncommitted changes)
    pub fn is_clean(&self) -> Result<bool> {
        let statuses = self
            .repo
            .statuses(Some(
                StatusOptions::new()
                    .include_untracked(true)
                    .include_ignored(false),
            ))
            .map_err(MultiGitError::GitError)?;

        let is_clean = statuses.is_empty();
        debug!("Repository clean status: {}", is_clean);
        Ok(is_clean)
    }

    /// Get the repository path
    #[must_use]
    pub fn path(&self) -> &Path {
        self.repo.path()
    }

    /// Get the working directory path
    pub fn workdir(&self) -> Result<&Path> {
        self.repo.workdir().ok_or_else(|| {
            MultiGitError::Other("Repository has no working directory (bare repo)".to_string())
        })
    }

    /// Fetch from a remote
    pub fn fetch(&self, remote_name: &str, refspecs: &[&str]) -> Result<()> {
        info!("Fetching from remote: {} (timeout: {}s)", remote_name, self.network_timeout.as_secs());

        let mut remote = self
            .repo
            .find_remote(remote_name)
            .map_err(MultiGitError::GitError)?;

        // Set up fetch options with callbacks and timeout
        let mut fetch_options = git2::FetchOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();

        // Track start time for timeout checking
        let start_time = std::time::Instant::now();
        let timeout = self.network_timeout;

        callbacks.transfer_progress(move |stats| {
            // Check for timeout
            if start_time.elapsed() > timeout {
                warn!("Fetch operation timed out after {}s", timeout.as_secs());
                return false; // Abort the transfer
            }

            if stats.received_objects() == stats.total_objects() {
                debug!(
                    "Resolving deltas {}/{}",
                    stats.indexed_deltas(),
                    stats.total_deltas()
                );
            } else if stats.total_objects() > 0 {
                debug!(
                    "Received {}/{} objects",
                    stats.received_objects(),
                    stats.total_objects()
                );
            }
            true
        });

        fetch_options.remote_callbacks(callbacks);

        remote
            .fetch(refspecs, Some(&mut fetch_options), None)
            .map_err(|e| {
                if start_time.elapsed() > timeout {
                    MultiGitError::Other(format!("Fetch timed out after {}s", timeout.as_secs()))
                } else {
                    MultiGitError::GitError(e)
                }
            })?;

        info!("Successfully fetched from {}", remote_name);
        Ok(())
    }

    /// Push to a remote
    pub fn push(&self, remote_name: &str, refspecs: &[&str]) -> Result<()> {
        info!("Pushing to remote: {} (timeout: {}s)", remote_name, self.network_timeout.as_secs());

        let mut remote = self
            .repo
            .find_remote(remote_name)
            .map_err(MultiGitError::GitError)?;

        let mut push_options = git2::PushOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();

        // Track start time for timeout checking
        let start_time = std::time::Instant::now();
        let timeout = self.network_timeout;

        callbacks.push_transfer_progress(move |current, total, bytes| {
            // Check for timeout
            if start_time.elapsed() > timeout {
                warn!("Push operation timed out after {}s", timeout.as_secs());
                return;
            }

            debug!("Push progress: {}/{} ({} bytes)", current, total, bytes);
        });

        push_options.remote_callbacks(callbacks);

        remote
            .push(refspecs, Some(&mut push_options))
            .map_err(|e| {
                if start_time.elapsed() > timeout {
                    MultiGitError::Other(format!("Push timed out after {}s", timeout.as_secs()))
                } else {
                    MultiGitError::GitError(e)
                }
            })?;

        info!("Successfully pushed to {}", remote_name);
        Ok(())
    }

    /// Get the number of commits ahead and behind compared to a remote branch
    pub fn get_ahead_behind(
        &self,
        local_branch: &str,
        remote_name: &str,
    ) -> Result<(usize, usize)> {
        let local_ref = format!("refs/heads/{local_branch}");
        let remote_ref = format!("refs/remotes/{remote_name}/{local_branch}");

        let local_oid = self
            .repo
            .refname_to_id(&local_ref)
            .map_err(MultiGitError::GitError)?;

        let remote_oid = self
            .repo
            .refname_to_id(&remote_ref)
            .map_err(MultiGitError::GitError)?;

        let (ahead, behind) = self
            .repo
            .graph_ahead_behind(local_oid, remote_oid)
            .map_err(MultiGitError::GitError)?;

        debug!(
            "Branch {} is {} ahead, {} behind {}",
            local_branch, ahead, behind, remote_name
        );
        Ok((ahead, behind))
    }

    /// Get the commit at HEAD
    pub fn head_commit(&self) -> Result<Commit<'_>> {
        let head = self.repo.head().map_err(MultiGitError::GitError)?;
        let oid = head
            .target()
            .ok_or_else(|| MultiGitError::Other("HEAD has no target".to_string()))?;

        self.repo.find_commit(oid).map_err(MultiGitError::GitError)
    }

    /// Get a commit by OID
    pub fn find_commit(&self, oid: Oid) -> Result<Commit<'_>> {
        self.repo.find_commit(oid).map_err(MultiGitError::GitError)
    }

    /// List all local branches
    pub fn list_local_branches(&self) -> Result<Vec<String>> {
        let branches = self
            .repo
            .branches(Some(BranchType::Local))
            .map_err(MultiGitError::GitError)?;

        let mut branch_names = Vec::new();
        for branch in branches {
            let (branch, _) = branch.map_err(MultiGitError::GitError)?;
            if let Some(name) = branch.name().map_err(MultiGitError::GitError)? {
                branch_names.push(name.to_string());
            }
        }

        debug!("Found {} local branches", branch_names.len());
        Ok(branch_names)
    }

    /// Check if repository is bare
    #[must_use]
    pub fn is_bare(&self) -> bool {
        self.repo.is_bare()
    }

    /// Get the underlying `git2::Repository` reference
    #[must_use]
    pub fn inner(&self) -> &Repository {
        &self.repo
    }

    /// Get the URL of a remote
    pub fn get_remote_url(&self, remote_name: &str) -> Result<String> {
        let remote = self
            .repo
            .find_remote(remote_name)
            .map_err(MultiGitError::GitError)?;

        let url = remote
            .url()
            .ok_or_else(|| MultiGitError::Other(format!("Remote '{remote_name}' has no URL")))?
            .to_string();

        debug!("Remote {} URL: {}", remote_name, url);
        Ok(url)
    }

    /// Compare local branch with remote branch (returns ahead, behind)
    pub fn compare_with_remote(&self, remote_name: &str, branch: &str) -> Result<(usize, usize)> {
        // First fetch to ensure we have latest remote state
        // Use empty refspecs to fetch all default refs from remote
        self.fetch(remote_name, &[])?;

        // Use existing get_ahead_behind method
        self.get_ahead_behind(branch, remote_name)
    }

    /// Create a new `GitOperations` from a path (alias for open)
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::open(path)
    }

    /// Clone a repository from a URL to a path
    pub fn clone<P: AsRef<Path>>(url: &str, path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Cloning repository from {} to {}", url, path.display());

        let repo = Repository::clone(url, path).map_err(MultiGitError::GitError)?;

        info!("Successfully cloned repository to {}", path.display());
        Ok(Self {
            repo,
            network_timeout: Duration::from_secs(DEFAULT_NETWORK_TIMEOUT_SECS),
        })
    }

    /// Add a remote to the repository
    pub fn add_remote(&self, name: &str, url: &str) -> Result<()> {
        debug!("Adding remote {} with URL: {}", name, url);

        self.repo
            .remote(name, url)
            .map_err(MultiGitError::GitError)?;

        info!("Successfully added remote: {}", name);
        Ok(())
    }

    /// Remove a remote from the repository
    pub fn remove_remote(&self, name: &str) -> Result<()> {
        debug!("Removing remote: {}", name);

        self.repo
            .remote_delete(name)
            .map_err(MultiGitError::GitError)?;

        info!("Successfully removed remote: {}", name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_repository() {
        let temp_dir = TempDir::new().unwrap();
        let ops = GitOperations::init(temp_dir.path()).unwrap();

        assert!(ops.is_clean().unwrap());
        assert!(!ops.is_bare());
    }

    #[test]
    fn test_open_repository() {
        let temp_dir = TempDir::new().unwrap();
        GitOperations::init(temp_dir.path()).unwrap();

        let ops = GitOperations::open(temp_dir.path()).unwrap();
        assert!(ops.is_clean().unwrap());
    }

    #[test]
    fn test_current_branch_fails_on_new_repo() {
        let temp_dir = TempDir::new().unwrap();
        let ops = GitOperations::init(temp_dir.path()).unwrap();

        // New repos don't have a current branch until first commit
        assert!(ops.current_branch().is_err());
    }
}
