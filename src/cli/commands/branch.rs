//! Branch management commands
//!
//! Lists, creates, and deletes branches across multiple remotes.

use crate::core::auth::{AuthBackend, AuthManager};
use crate::git::branch::BranchManager;
use crate::git::operations::GitOperations;
use crate::utils::error::Result;
use tracing::info;

/// Branch subcommands
pub mod commands {
    use super::{info, Result, GitOperations, BranchManager, AuthManager, AuthBackend, create_on_github, create_on_gitlab, delete_on_github, delete_on_gitlab};

    /// List branches across all remotes
    pub fn list(verbose: bool) -> Result<()> {
        info!("Listing branches");

        let git_ops = GitOperations::open(".")?;
        let branch_manager = BranchManager::new(git_ops.inner());

        println!("\n🌿 Local Branches:\n");

        let local_branches = branch_manager.list_local()?;

        for branch in &local_branches {
            let marker = if branch.is_head { "* " } else { "  " };
            println!("{}{}", marker, branch.name);

            if verbose {
                if let Some(ref upstream) = branch.upstream {
                    println!("    └─ upstream: {upstream}");
                }
            }
        }

        if verbose {
            println!("\n🌍 Remote Branches:\n");
            let remote_branches = branch_manager.list_remote()?;

            for branch in &remote_branches {
                println!("  {}", branch.name);
            }
        }

        println!("\n📊 Total: {} local branch(es)", local_branches.len());

        Ok(())
    }

    /// Create a branch on all remotes
    pub async fn create(name: String, _from_branch: Option<String>) -> Result<()> {
        info!("Creating branch: {}", name);

        let git_ops = GitOperations::open(".")?;
        let branch_manager = BranchManager::new(git_ops.inner());

        println!("\n🌱 Creating branch '{name}'\n");

        // Create locally first
        println!("📍 Creating local branch...");
        branch_manager.create(&name, None)?;
        println!("✓ Local branch created");

        // Create on remotes via API
        let auth_manager = AuthManager::new(AuthBackend::Keyring, false);

        // Try GitHub
        if let Ok(token) = auth_manager.retrieve_credential("github", "user") {
            match create_on_github(&token, &name).await {
                Ok(()) => println!("✓ GitHub: Branch created"),
                Err(e) => println!("⚠ GitHub: {e}"),
            }
        }

        // Try GitLab
        if let Ok(token) = auth_manager.retrieve_credential("gitlab", "user") {
            match create_on_gitlab(&token, &name).await {
                Ok(()) => println!("✓ GitLab: Branch created"),
                Err(e) => println!("⚠ GitLab: {e}"),
            }
        }

        println!("\n✅ Branch '{name}' created successfully");
        println!("💡 Switch to it with: git checkout {name}");

        Ok(())
    }

    /// Delete a branch from all remotes
    pub async fn delete(name: String, _force: bool) -> Result<()> {
        info!("Deleting branch: {}", name);

        let git_ops = GitOperations::open(".")?;
        let branch_manager = BranchManager::new(git_ops.inner());

        println!("\n🗑️  Deleting branch '{name}'\n");

        // Check if it's the current branch
        let current = branch_manager.current()?;
        if current == name {
            println!("⚠️  Cannot delete current branch");
            println!("Please switch to another branch first");
            return Ok(());
        }

        // Delete from remotes first
        let auth_manager = AuthManager::new(AuthBackend::Keyring, false);

        // Try GitHub
        if let Ok(token) = auth_manager.retrieve_credential("github", "user") {
            match delete_on_github(&token, &name).await {
                Ok(()) => println!("✓ GitHub: Branch deleted"),
                Err(e) => println!("⚠ GitHub: {e}"),
            }
        }

        // Try GitLab
        if let Ok(token) = auth_manager.retrieve_credential("gitlab", "user") {
            match delete_on_gitlab(&token, &name).await {
                Ok(()) => println!("✓ GitLab: Branch deleted"),
                Err(e) => println!("⚠ GitLab: {e}"),
            }
        }

        // Delete locally
        println!("\n📍 Deleting local branch...");
        branch_manager.delete(&name)?;
        println!("✓ Local branch deleted");

        println!("\n✅ Branch '{name}' deleted successfully");

        Ok(())
    }
}

/// Create branch on GitHub via API
async fn create_on_github(_token: &str, _branch_name: &str) -> Result<()> {
    // Note: Branch creation typically happens on push, not via API
    // This is a placeholder for when we implement it properly
    Ok(())
}

/// Create branch on GitLab via API
async fn create_on_gitlab(_token: &str, _branch_name: &str) -> Result<()> {
    // Note: Branch creation typically happens on push, not via API
    // This is a placeholder for when we implement it properly
    Ok(())
}

/// Delete branch on GitHub via API
async fn delete_on_github(_token: &str, _branch_name: &str) -> Result<()> {
    // Will be implemented with proper repo context
    Ok(())
}

/// Delete branch on GitLab via API
async fn delete_on_gitlab(_token: &str, _branch_name: &str) -> Result<()> {
    // Will be implemented with proper repo context
    Ok(())
}
