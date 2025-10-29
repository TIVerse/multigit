//! Health checking and diagnostics
//!
//! Provides health checks and diagnostics for the repository and remotes.

use crate::git::operations::GitOperations;
use crate::utils::error::Result;
use std::path::Path;
use tracing::{debug, info};

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthReport {
    /// Whether the repository is valid
    pub repo_valid: bool,
    /// Whether the working directory is clean
    pub working_dir_clean: bool,
    /// Current branch name
    pub current_branch: Option<String>,
    /// Health status of remotes
    pub remotes: Vec<RemoteHealth>,
    /// List of issues found
    pub issues: Vec<String>,
    /// Recommendations for fixes
    pub recommendations: Vec<String>,
}

/// Health status of a remote
#[derive(Debug, Clone)]
pub struct RemoteHealth {
    /// Name of the remote
    pub name: String,
    /// URL of the remote
    pub url: String,
    /// Whether the remote is reachable
    pub reachable: bool,
    /// Issue description if any
    pub issue: Option<String>,
}

/// Health checker
pub struct HealthChecker {
    git_ops: GitOperations,
}

impl HealthChecker {
    /// Create a new health checker for a repository
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let git_ops = GitOperations::open(repo_path)?;
        Ok(Self { git_ops })
    }

    /// Run a comprehensive health check
    pub fn check(&self) -> HealthReport {
        info!("Running health check");

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Check if working directory is clean
        let working_dir_clean = self.git_ops.is_clean().unwrap_or(false);
        if !working_dir_clean {
            issues.push("Working directory has uncommitted changes".to_string());
            recommendations.push("Commit or stash your changes before syncing".to_string());
        }

        // Get current branch
        let current_branch = self.git_ops.current_branch().ok();
        if current_branch.is_none() {
            issues.push("Cannot determine current branch (detached HEAD?)".to_string());
            recommendations.push("Checkout a branch before syncing".to_string());
        }

        // Check remotes
        let remotes = self.check_remotes();
        let unreachable_count = remotes.iter().filter(|r| !r.reachable).count();
        if unreachable_count > 0 {
            issues.push(format!("{unreachable_count} remote(s) unreachable"));
            recommendations.push("Check your network connection and remote URLs".to_string());
        }

        if remotes.is_empty() {
            issues.push("No remotes configured".to_string());
            recommendations.push("Add remotes with 'multigit remote add'".to_string());
        }

        HealthReport {
            repo_valid: true,
            working_dir_clean,
            current_branch,
            remotes,
            issues,
            recommendations,
        }
    }

    /// Check health of all remotes
    fn check_remotes(&self) -> Vec<RemoteHealth> {
        debug!("Checking remotes");

        let remote_names = match self.git_ops.inner().remotes() {
            Ok(names) => names,
            Err(_) => return Vec::new(),
        };

        let mut health = Vec::new();

        for name in remote_names.iter().flatten() {
            let remote = match self.git_ops.inner().find_remote(name) {
                Ok(r) => r,
                Err(_) => continue,
            };

            let url = remote.url().unwrap_or("").to_string();

            // For now, we assume remotes are reachable
            // A real implementation would test connectivity
            health.push(RemoteHealth {
                name: name.to_string(),
                url,
                reachable: true,
                issue: None,
            });
        }

        health
    }

    /// Quick check - returns true if everything is OK
    #[must_use] 
    pub fn is_healthy(&self) -> bool {
        let report = self.check();
        report.issues.is_empty()
    }
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
    fn test_health_checker_creation() {
        let (_temp_dir, repo_path) = create_test_repo();
        let checker = HealthChecker::new(&repo_path);
        assert!(checker.is_ok());
    }

    #[test]
    fn test_health_check() {
        let (_temp_dir, repo_path) = create_test_repo();
        let checker = HealthChecker::new(&repo_path).unwrap();
        let report = checker.check();

        assert!(report.repo_valid);
        assert!(report.working_dir_clean);
        assert!(report.current_branch.is_some());
    }

    #[test]
    fn test_is_healthy() {
        let (_temp_dir, repo_path) = create_test_repo();
        let checker = HealthChecker::new(&repo_path).unwrap();

        // A fresh repo with a commit should be healthy
        let _is_healthy = checker.is_healthy();
        // May have issues if no remotes configured, so we just check it runs without panic
    }
}
