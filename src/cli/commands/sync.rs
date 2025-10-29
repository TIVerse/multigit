//! Sync command implementation
//!
//! Synchronize across all remotes (fetch + push).

use crate::core::config::Config;
use crate::core::sync_manager::SyncManager;
use crate::utils::error::Result;
use tracing::info;

/// Synchronize across all remotes
pub async fn execute(branch: Option<String>, dry_run: bool) -> Result<()> {
    info!("Executing sync command");

    let manager = SyncManager::new(".")?;

    // Get branch to sync
    let branch_name = match branch {
        Some(b) => b,
        None => manager.current_branch()?,
    };

    // Check if working directory is clean
    if !manager.is_clean()? {
        println!("⚠ Warning: Working directory has uncommitted changes");
        println!("Commit or stash changes before syncing.\n");
        return Ok(());
    }

    // Load configured remotes from config
    let config = Config::load().unwrap_or_default();
    let enabled: Vec<String> = config.enabled_remotes().keys().cloned().collect();

    if enabled.is_empty() {
        println!("⚠ No remotes configured yet.");
        println!("Use 'multigit remote add' to configure remotes.");
        println!("\nExample:");
        println!("  multigit remote add github your-username");
        return Ok(());
    }

    if dry_run {
        println!(
            "\n[DRY RUN] Would sync branch '{}' with {} remote(s):",
            branch_name,
            enabled.len()
        );
        for remote in &enabled {
            println!("  - {remote}");
        }
        println!("\n[DRY RUN] No changes were made.");
        return Ok(());
    }

    println!(
        "\n🔄 Syncing branch '{}' with {} remote(s)...\n",
        branch_name,
        enabled.len()
    );

    // Fetch from all remotes
    println!("📥 Fetching updates...");
    let fetch_results = manager.fetch_all(&enabled).await?;
    for result in &fetch_results {
        if result.success {
            println!("  ✓ {} - fetched", result.remote);
        } else {
            println!("  ✗ {} - {}", result.remote, result.message);
        }
    }

    // Push to all remotes
    println!("\n📤 Pushing changes...");
    let push_results = manager.push_all(&branch_name, &enabled).await?;
    for result in &push_results {
        if result.success {
            println!("  ✓ {} - pushed", result.remote);
        } else {
            println!("  ✗ {} - {}", result.remote, result.message);
        }
    }

    println!("\n✅ Sync complete!");

    Ok(())
}
