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

    println!("\n⬇️  Pulling '{}' from {}...\n", branch_name, remote_name);

    // Check if working directory is clean
    if !git_ops.is_clean()? {
        println!("⚠️  Warning: Working directory has uncommitted changes");
        println!("Please commit or stash your changes before pulling.\n");
        return Ok(());
    }

    // Fetch from the remote
    println!("📡 Fetching from {}...", remote_name);
    git_ops.fetch(&remote_name, &[])?;
    println!("✓ Fetch complete");

    // Check if we're ahead, behind, or diverged
    let (ahead, behind) = git_ops.get_ahead_behind(&branch_name, &remote_name)?;

    if ahead > 0 && behind > 0 {
        println!("\n⚠️  Divergence detected!");
        println!("   Local is {} commits ahead", ahead);
        println!("   Remote is {} commits behind", behind);
        println!("\n💡 Consider using 'multigit sync' to handle this situation");
        return Ok(());
    }

    if behind == 0 {
        println!("\n✓ Already up to date!");
        return Ok(());
    }

    println!("\n📥 Pulling {} commit(s)...", behind);

    // For now, we'll advise using git directly for the actual pull
    // This requires implementing merge/rebase logic
    println!("\n⚠️  Automatic pull merge not yet implemented.");
    println!("Please use: git pull {} {}", remote_name, branch_name);

    Ok(())
}
