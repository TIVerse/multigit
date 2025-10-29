//! Sync manager for coordinating multi-remote operations
//!
//! Coordinates push/pull/sync operations across multiple Git remotes.

use crate::git::operations::GitOperations;
use crate::utils::error::Result;
use std::collections::HashMap;
use std::path::Path;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

/// Result of a push operation to a single remote
#[derive(Debug, Clone)]
pub struct PushResult {
    /// Name of the remote
    pub remote: String,
    /// Whether the push was successful
    pub success: bool,
    /// Message describing the result
    pub message: String,
    /// Duration of the operation in milliseconds
    pub duration_ms: u64,
}

/// Result of a fetch operation from a single remote
#[derive(Debug, Clone)]
pub struct FetchResult {
    /// Name of the remote
    pub remote: String,
    /// Whether the fetch was successful
    pub success: bool,
    /// Message describing the result
    pub message: String,
    /// Number of commits fetched
    pub commits_fetched: usize,
}

/// Synchronization manager
pub struct SyncManager {
    git_ops: GitOperations,
    max_parallel: usize,
}

impl SyncManager {
    /// Create a new sync manager for a repository
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let git_ops = GitOperations::open(repo_path)?;

        Ok(Self {
            git_ops,
            max_parallel: 4, // Default parallel operations
        })
    }

    /// Set maximum parallel operations
    #[must_use] 
    pub fn with_max_parallel(mut self, max: usize) -> Self {
        self.max_parallel = max.max(1);
        self
    }

    /// Push to all configured remotes in parallel
    pub async fn push_all(&self, branch: &str, remotes: &[String]) -> Result<Vec<PushResult>> {
        info!("Pushing branch '{}' to {} remotes", branch, remotes.len());

        let refspec = format!("refs/heads/{branch}:refs/heads/{branch}");
        let mut tasks: Vec<JoinHandle<PushResult>> = Vec::new();

        // Create tasks for each remote
        for remote_name in remotes {
            let remote = remote_name.clone();
            let refspec = refspec.clone();
            let repo_path = self.git_ops.workdir()?.to_path_buf();

            let task = tokio::spawn(async move {
                let start = std::time::Instant::now();

                // Open a new GitOperations instance for this task
                let ops = match GitOperations::open(&repo_path) {
                    Ok(ops) => ops,
                    Err(e) => {
                        return PushResult {
                            remote,
                            success: false,
                            message: format!("Failed to open repo: {e}"),
                            duration_ms: start.elapsed().as_millis() as u64,
                        };
                    }
                };

                // Perform the push
                match ops.push(&remote, &[&refspec]) {
                    Ok(()) => {
                        info!("Successfully pushed to {}", remote);
                        PushResult {
                            remote,
                            success: true,
                            message: "Push successful".to_string(),
                            duration_ms: start.elapsed().as_millis() as u64,
                        }
                    }
                    Err(e) => {
                        warn!("Failed to push to {}: {}", remote, e);
                        PushResult {
                            remote,
                            success: false,
                            message: format!("Push failed: {e}"),
                            duration_ms: start.elapsed().as_millis() as u64,
                        }
                    }
                }
            });

            tasks.push(task);

            // Limit concurrent tasks
            if tasks.len() >= self.max_parallel {
                // Wait for at least one task to complete
                if let Some(task) = tasks.first_mut() {
                    let _ = task.await;
                }
            }
        }

        // Wait for all remaining tasks to complete
        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Task join error: {}", e);
                }
            }
        }

        let success_count = results.iter().filter(|r| r.success).count();
        info!(
            "Push complete: {}/{} successful",
            success_count,
            results.len()
        );

        Ok(results)
    }

    /// Fetch from all configured remotes in parallel
    pub async fn fetch_all(&self, remotes: &[String]) -> Result<Vec<FetchResult>> {
        info!("Fetching from {} remotes", remotes.len());

        let mut tasks: Vec<JoinHandle<FetchResult>> = Vec::new();

        for remote_name in remotes {
            let remote = remote_name.clone();
            let repo_path = self.git_ops.workdir()?.to_path_buf();

            let task = tokio::spawn(async move {
                let ops = match GitOperations::open(&repo_path) {
                    Ok(ops) => ops,
                    Err(e) => {
                        return FetchResult {
                            remote,
                            success: false,
                            message: format!("Failed to open repo: {e}"),
                            commits_fetched: 0,
                        };
                    }
                };

                // Fetch all refs from the remote
                match ops.fetch(&remote, &[]) {
                    Ok(()) => {
                        info!("Successfully fetched from {}", remote);
                        FetchResult {
                            remote,
                            success: true,
                            message: "Fetch successful".to_string(),
                            commits_fetched: 0, // TODO: Count actual commits
                        }
                    }
                    Err(e) => {
                        warn!("Failed to fetch from {}: {}", remote, e);
                        FetchResult {
                            remote,
                            success: false,
                            message: format!("Fetch failed: {e}"),
                            commits_fetched: 0,
                        }
                    }
                }
            });

            tasks.push(task);
        }

        // Wait for all tasks
        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Task join error: {}", e);
                }
            }
        }

        let success_count = results.iter().filter(|r| r.success).count();
        info!(
            "Fetch complete: {}/{} successful",
            success_count,
            results.len()
        );

        Ok(results)
    }

    /// Get the status of the repository relative to remotes
    pub fn get_sync_status(
        &self,
        branch: &str,
        remotes: &[String],
    ) -> Result<HashMap<String, SyncStatus>> {
        debug!("Getting sync status for branch '{}'", branch);

        let mut statuses = HashMap::new();

        for remote in remotes {
            let status = match self.git_ops.get_ahead_behind(branch, remote) {
                Ok((ahead, behind)) => SyncStatus {
                    remote: remote.clone(),
                    ahead,
                    behind,
                    in_sync: ahead == 0 && behind == 0,
                },
                Err(e) => {
                    warn!("Failed to get status for {}: {}", remote, e);
                    SyncStatus {
                        remote: remote.clone(),
                        ahead: 0,
                        behind: 0,
                        in_sync: false,
                    }
                }
            };

            statuses.insert(remote.clone(), status);
        }

        Ok(statuses)
    }

    /// Check if the working directory is clean
    pub fn is_clean(&self) -> Result<bool> {
        self.git_ops.is_clean()
    }

    /// Get the current branch
    pub fn current_branch(&self) -> Result<String> {
        self.git_ops.current_branch()
    }
}

/// Sync status for a single remote
#[derive(Debug, Clone)]
pub struct SyncStatus {
    /// Name of the remote
    pub remote: String,
    /// Number of commits ahead
    pub ahead: usize,
    /// Number of commits behind
    pub behind: usize,
    /// Whether the local and remote are in sync
    pub in_sync: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use tempfile::TempDir;

    fn create_test_repo() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().to_str().unwrap().to_string();

        let repo = Repository::init(&repo_path).unwrap();

        // Create initial commit
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();

        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        (temp_dir, repo_path)
    }

    #[test]
    fn test_sync_manager_creation() {
        let (_temp_dir, repo_path) = create_test_repo();
        let manager = SyncManager::new(&repo_path);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_is_clean() {
        let (_temp_dir, repo_path) = create_test_repo();
        let manager = SyncManager::new(&repo_path).unwrap();
        assert!(manager.is_clean().unwrap());
    }

    #[test]
    fn test_current_branch() {
        let (_temp_dir, repo_path) = create_test_repo();
        let manager = SyncManager::new(&repo_path).unwrap();
        let branch = manager.current_branch().unwrap();
        assert!(!branch.is_empty());
    }
}
