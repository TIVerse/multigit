//! Pull command implementation
//!
//! Pull changes from a specified remote.

use crate::git::operations::GitOperations;
use crate::utils::error::Result;
use tracing::info;

/// Pull from a specified remote
pub fn execute(remote: Option<String>, branch: Option<String>) -> Result<()> {
    info!("Executing pull command");

    let git_ops = GitOperations::open(".")?;

    // Get branch to pull
    let branch_name = match branch {
        Some(b) => b,
        None => git_ops.current_branch()?,
    };

    // Get remote to pull from
    let remote_name = remote.unwrap_or_else(|| "origin".to_string());

    println!("\n⬇️  Pulling '{branch_name}' from {remote_name}...\n");

    // Check if working directory is clean
    if !git_ops.is_clean()? {
        println!("⚠️  Warning: Working directory has uncommitted changes");
        println!("Please commit or stash your changes before pulling.\n");
        return Ok(());
    }

    // Fetch from the remote
    println!("📡 Fetching from {remote_name}...");
    git_ops.fetch(&remote_name, &[])?;
    println!("✓ Fetch complete");

    // Check if we're ahead, behind, or diverged
    let (ahead, behind) = git_ops.get_ahead_behind(&branch_name, &remote_name)?;

    if ahead > 0 && behind > 0 {
        println!("\n⚠️  Divergence detected!");
        println!("   Local is {ahead} commits ahead");
        println!("   Remote is {behind} commits behind");
        println!("\n💡 Consider using 'multigit sync' to handle this situation");
        return Ok(());
    }

    if behind == 0 {
        println!("\n✓ Already up to date!");
        return Ok(());
    }

    println!("\n📥 Pulling {behind} commit(s)...");

    // Attempt fast-forward merge
    if ahead == 0 {
        println!("✓ Fast-forward merge possible");
        println!("\nAttempting fast-forward merge...");

        // Fast-forward is safe since we have no local commits
        let local_ref = format!("refs/heads/{branch_name}");
        let remote_ref = format!("refs/remotes/{remote_name}/{branch_name}");

        // Get the remote OID
        let remote_oid = git_ops
            .inner()
            .refname_to_id(&remote_ref)
            .map_err(crate::utils::error::MultiGitError::GitError)?;

        // Update the local branch to point to remote
        git_ops
            .inner()
            .reference(&local_ref, remote_oid, true, "fast-forward pull")
            .map_err(crate::utils::error::MultiGitError::GitError)?;

        // Update working tree if we're on this branch
        if git_ops.current_branch()? == branch_name {
            git_ops
                .inner()
                .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .map_err(crate::utils::error::MultiGitError::GitError)?;
        }

        println!("✓ Fast-forward merge successful!");
        println!("\n📊 Pulled {behind} commit(s) from {remote_name}");
    } else {
        println!("\n⚠️  Cannot fast-forward (you have {ahead} local commit(s))");
        println!("\nTo merge, use one of:");
        println!("  1. Merge: git pull {remote_name} {branch_name}");
        println!("  2. Rebase: git pull --rebase {remote_name} {branch_name}");
        println!("  3. Sync all: multigit sync");
    }

    Ok(())
}
