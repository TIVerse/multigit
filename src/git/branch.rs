//! Git branch operations
//!
//! Operations for managing Git branches.

use crate::utils::error::{MultiGitError, Result};
use git2::{Branch, BranchType, Oid, Repository};
use tracing::{debug, info};

/// Branch management operations
pub struct BranchManager<'repo> {
    repo: &'repo Repository,
}

impl<'repo> BranchManager<'repo> {
    /// Create a new branch manager for a repository
    pub fn new(repo: &'repo Repository) -> Self {
        Self { repo }
    }

    /// Create a new branch
    pub fn create(&self, name: &str, target_commit_oid: Option<Oid>) -> Result<()> {
        info!("Creating branch '{}'", name);

        // Get target commit (defaults to HEAD)
        let commit = if let Some(oid) = target_commit_oid {
            self.repo
                .find_commit(oid)
                .map_err(MultiGitError::GitError)?
        } else {
            let head = self.repo.head().map_err(MultiGitError::GitError)?;
            let oid = head
                .target()
                .ok_or_else(|| MultiGitError::Other("HEAD has no target".to_string()))?;
            self.repo
                .find_commit(oid)
                .map_err(MultiGitError::GitError)?
        };

        // Create the branch
        self.repo
            .branch(name, &commit, false)
            .map_err(MultiGitError::GitError)?;

        info!("Successfully created branch '{}'", name);
        Ok(())
    }

    /// Delete a local branch
    pub fn delete(&self, name: &str) -> Result<()> {
        info!("Deleting branch '{}'", name);

        let mut branch = self
            .repo
            .find_branch(name, BranchType::Local)
            .map_err(|_| MultiGitError::Other(format!("Branch '{}' not found", name)))?;

        branch.delete().map_err(MultiGitError::GitError)?;

        info!("Successfully deleted branch '{}'", name);
        Ok(())
    }

    /// List all local branches
    pub fn list_local(&self) -> Result<Vec<BranchInfo>> {
        let branches = self
            .repo
            .branches(Some(BranchType::Local))
            .map_err(MultiGitError::GitError)?;

        let mut branch_list = Vec::new();
        for branch in branches {
            let (branch, _) = branch.map_err(MultiGitError::GitError)?;
            if let Some(info) = self.branch_to_info(branch)? {
                branch_list.push(info);
            }
        }

        debug!("Found {} local branches", branch_list.len());
        Ok(branch_list)
    }

    /// List all remote branches
    pub fn list_remote(&self) -> Result<Vec<BranchInfo>> {
        let branches = self
            .repo
            .branches(Some(BranchType::Remote))
            .map_err(MultiGitError::GitError)?;

        let mut branch_list = Vec::new();
        for branch in branches {
            let (branch, _) = branch.map_err(MultiGitError::GitError)?;
            if let Some(info) = self.branch_to_info(branch)? {
                branch_list.push(info);
            }
        }

        debug!("Found {} remote branches", branch_list.len());
        Ok(branch_list)
    }

    /// Get information about a specific branch
    pub fn get_info(&self, name: &str) -> Result<BranchInfo> {
        let branch = self
            .repo
            .find_branch(name, BranchType::Local)
            .map_err(|_| MultiGitError::Other(format!("Branch '{}' not found", name)))?;

        self.branch_to_info(branch)?.ok_or_else(|| {
            MultiGitError::Other(format!("Could not get info for branch '{}'", name))
        })
    }

    /// Check if a branch exists
    pub fn exists(&self, name: &str) -> Result<bool> {
        match self.repo.find_branch(name, BranchType::Local) {
            Ok(_) => Ok(true),
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
            Err(e) => Err(MultiGitError::GitError(e)),
        }
    }

    /// Rename a branch
    pub fn rename(&self, old_name: &str, new_name: &str, force: bool) -> Result<()> {
        info!("Renaming branch '{}' to '{}'", old_name, new_name);

        let mut branch = self
            .repo
            .find_branch(old_name, BranchType::Local)
            .map_err(|_| MultiGitError::Other(format!("Branch '{}' not found", old_name)))?;

        branch
            .rename(new_name, force)
            .map_err(MultiGitError::GitError)?;

        info!(
            "Successfully renamed branch '{}' to '{}'",
            old_name, new_name
        );
        Ok(())
    }

    /// Get the current branch name
    pub fn current(&self) -> Result<String> {
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

        Ok(branch_name)
    }

    /// Checkout a branch
    pub fn checkout(&self, name: &str) -> Result<()> {
        info!("Checking out branch '{}'", name);

        // Find the branch
        let branch = self
            .repo
            .find_branch(name, BranchType::Local)
            .map_err(|_| MultiGitError::Other(format!("Branch '{}' not found", name)))?;

        let refname = branch
            .get()
            .name()
            .ok_or_else(|| MultiGitError::Other("Invalid reference name".to_string()))?;

        // Set HEAD to point to this branch
        self.repo
            .set_head(refname)
            .map_err(MultiGitError::GitError)?;

        // Checkout the branch
        self.repo
            .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .map_err(MultiGitError::GitError)?;

        info!("Successfully checked out branch '{}'", name);
        Ok(())
    }

    /// Compare two branches and get the number of commits ahead/behind
    pub fn compare(&self, branch1: &str, branch2: &str) -> Result<(usize, usize)> {
        let ref1 = format!("refs/heads/{}", branch1);
        let ref2 = format!("refs/heads/{}", branch2);

        let oid1 = self
            .repo
            .refname_to_id(&ref1)
            .map_err(MultiGitError::GitError)?;

        let oid2 = self
            .repo
            .refname_to_id(&ref2)
            .map_err(MultiGitError::GitError)?;

        let (ahead, behind) = self
            .repo
            .graph_ahead_behind(oid1, oid2)
            .map_err(MultiGitError::GitError)?;

        debug!(
            "Branch '{}' is {} ahead, {} behind '{}'",
            branch1, ahead, behind, branch2
        );
        Ok((ahead, behind))
    }

    /// Get the upstream branch for a local branch
    pub fn get_upstream(&self, name: &str) -> Result<Option<String>> {
        let branch = self
            .repo
            .find_branch(name, BranchType::Local)
            .map_err(|_| MultiGitError::Other(format!("Branch '{}' not found", name)))?;

        match branch.upstream() {
            Ok(upstream) => {
                let upstream_name = upstream
                    .name()
                    .map_err(MultiGitError::GitError)?
                    .map(String::from);
                Ok(upstream_name)
            }
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
            Err(e) => Err(MultiGitError::GitError(e)),
        }
    }

    /// Helper to convert a git2::Branch to BranchInfo
    fn branch_to_info(&self, branch: Branch) -> Result<Option<BranchInfo>> {
        let name = match branch.name().map_err(MultiGitError::GitError)? {
            Some(n) => n.to_string(),
            None => return Ok(None),
        };

        let is_head = branch.is_head();
        let reference = branch.get();
        let target = reference.target();

        let upstream = if !branch.get().is_remote() {
            branch
                .upstream()
                .ok()
                .and_then(|u| u.name().ok().and_then(|n| n.map(String::from)))
        } else {
            None
        };

        Ok(Some(BranchInfo {
            name,
            is_head,
            target,
            upstream,
        }))
    }
}

/// Information about a branch
#[derive(Debug, Clone)]
pub struct BranchInfo {
    /// Branch name
    pub name: String,
    /// Whether this is the current HEAD branch
    pub is_head: bool,
    /// The commit OID this branch points to
    pub target: Option<Oid>,
    /// Upstream branch if configured
    pub upstream: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_test_repo() -> (TempDir, Repository) {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        // Create an initial commit
        let sig = git2::Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        {
            let tree = repo.find_tree(tree_id).unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
                .unwrap();
        }

        (temp_dir, repo)
    }

    #[test]
    fn test_create_branch() {
        let (_temp_dir, repo) = create_test_repo();
        let manager = BranchManager::new(&repo);

        manager.create("feature", None).unwrap();
        assert!(manager.exists("feature").unwrap());
    }

    #[test]
    fn test_delete_branch() {
        let (_temp_dir, repo) = create_test_repo();
        let manager = BranchManager::new(&repo);

        manager.create("temp-branch", None).unwrap();
        assert!(manager.exists("temp-branch").unwrap());

        manager.delete("temp-branch").unwrap();
        assert!(!manager.exists("temp-branch").unwrap());
    }

    #[test]
    fn test_list_branches() {
        let (_temp_dir, repo) = create_test_repo();
        let manager = BranchManager::new(&repo);

        manager.create("branch1", None).unwrap();
        manager.create("branch2", None).unwrap();

        let branches = manager.list_local().unwrap();
        assert!(branches.len() >= 3); // master/main + branch1 + branch2
    }

    #[test]
    fn test_current_branch() {
        let (_temp_dir, repo) = create_test_repo();
        let manager = BranchManager::new(&repo);

        let current = manager.current().unwrap();
        assert!(!current.is_empty());
    }
}
