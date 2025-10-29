//! Status command implementation
//!
//! Display sync status across all remotes.

use crate::core::config::Config;
use crate::core::sync_manager::SyncManager;
use crate::utils::error::Result;
use tracing::info;

/// Show sync status
pub fn execute(verbose: bool) -> Result<()> {
    info!("Checking sync status");

    let manager = SyncManager::new(".")?;

    // Get current branch
    let branch = manager.current_branch()?;
    let is_clean = manager.is_clean()?;

    println!("\nℹ MultiGit Status");
    println!("\nCurrent branch: {branch}");
    println!(
        "Working directory: {}",
        if is_clean { "clean" } else { "has changes" }
    );

    // Get configured remotes from config
    println!("\nRemote status:");
    let config = Config::load().unwrap_or_default();
    let remotes = config.enabled_remotes();

    if remotes.is_empty() {
        println!("  No remotes configured.");
        println!("  Configure remotes with: multigit remote add");
    } else {
        for (name, remote_config) in remotes {
            println!("  ✓ {} (@{})", name, remote_config.username);
        }
    }

    if verbose {
        println!("\nVerbose mode - additional details:");
        println!("  Repository path: .");
        println!("  Config: .multigit/config.toml");
    }

    Ok(())
}
