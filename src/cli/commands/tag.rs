//! Tag command implementation
//!
//! Manage tags across all remotes.

use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use git2::{ObjectType, Signature};
use tracing::info;

/// Tag subcommands
pub mod commands {
    use super::*;

    /// List tags
    pub fn list(remote: Option<String>) -> Result<()> {
        info!("Listing tags");

        let git_ops = GitOperations::open(".")?;

        println!("\n🏷️  Tags:\n");

        let tag_names = git_ops
            .inner()
            .tag_names(None)
            .map_err(|e| MultiGitError::GitError(e))?;

        if tag_names.is_empty() {
            println!("  No tags found");
        } else {
            for tag_name in tag_names.iter().flatten() {
                println!("  {}", tag_name);
            }

            println!("\n📊 Total: {} tag(s)", tag_names.len());
        }

        Ok(())
    }

    /// Create a tag locally and push to all remotes
    pub async fn create(name: String, message: Option<String>, sign: bool) -> Result<()> {
        info!("Creating tag: {}", name);

        let git_ops = GitOperations::open(".")?;

        println!("\n🏷️  Creating tag '{}'\n", name);

        // Get the current HEAD commit
        let head_commit = git_ops.head_commit()?;
        let target_oid = head_commit.id();

        // Create the tag
        if let Some(msg) = message {
            // Annotated tag
            println!("📝 Creating annotated tag...");

            let sig = Signature::now("MultiGit", "multigit@local")
                .map_err(|e| MultiGitError::GitError(e))?;

            git_ops
                .inner()
                .tag(&name, &head_commit.as_object(), &sig, &msg, false)
                .map_err(|e| MultiGitError::GitError(e))?;

            println!("✓ Annotated tag created");
        } else {
            // Lightweight tag
            println!("📌 Creating lightweight tag...");

            let obj = git_ops
                .inner()
                .find_object(target_oid, Some(ObjectType::Commit))
                .map_err(|e| MultiGitError::GitError(e))?;

            git_ops
                .inner()
                .tag_lightweight(&name, &obj, false)
                .map_err(|e| MultiGitError::GitError(e))?;

            println!("✓ Lightweight tag created");
        }

        println!("\n💡 To push tags to all remotes, use:");
        println!("   git push --tags <remote>");
        println!("   Or: multigit push --tags");

        Ok(())
    }

    /// Delete a tag locally and from all remotes
    pub async fn delete(name: String) -> Result<()> {
        info!("Deleting tag: {}", name);

        let git_ops = GitOperations::open(".")?;

        println!("\n🗑️  Deleting tag '{}'\n", name);

        // Find and delete the tag
        let tag_ref = format!("refs/tags/{}", name);

        match git_ops.inner().find_reference(&tag_ref) {
            Ok(mut tag_reference) => {
                tag_reference
                    .delete()
                    .map_err(|e| MultiGitError::GitError(e))?;
                println!("✓ Local tag deleted");
            }
            Err(_) => {
                println!("⚠️  Tag '{}' not found locally", name);
            }
        }

        println!("\n💡 To delete from remotes, use:");
        println!("   git push <remote> :refs/tags/{}", name);
        println!("   Or: multigit push --delete-tag {}", name);

        Ok(())
    }
}
