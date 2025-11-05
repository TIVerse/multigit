//! Fetch command implementation
//!
//! Fetch changes from remotes.

use crate::core::config::Config;
use crate::core::sync_manager::SyncManager;
use crate::utils::error::Result;
use tracing::info;

/// Fetch from remotes
pub async fn execute(remotes: Vec<String>, all: bool) -> Result<()> {
    info!("Executing fetch command");

    // Load config to get settings
    let config = Config::load().unwrap_or_default();
    let has_multigit_remotes = !config.enabled_remotes().is_empty();

    // Determine which remotes to fetch from
    let fetch_remotes = if all {
        // Load all enabled remotes from config
        if has_multigit_remotes {
            let enabled: Vec<String> = config.enabled_remotes().keys().cloned().collect();
            enabled
        } else {
            // Fallback to git fetch --all
            info!("No MultiGit remotes, falling back to git fetch --all");
            use std::process::Command;
            let output = Command::new("git")
                .args(["fetch", "--all"])
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .output()
                .map_err(|e| {
                    crate::utils::error::MultiGitError::other(format!("Failed to execute git: {e}"))
                })?;

            if !output.status.success() {
                std::process::exit(output.status.code().unwrap_or(1));
            }
            return Ok(());
        }
    } else if !remotes.is_empty() {
        remotes
    } else {
        // Default to origin if no remotes specified
        vec!["origin".to_string()]
    };

    let manager = SyncManager::new(".")?.with_max_parallel(config.settings.max_parallel);

    println!("\n📡 Fetching from {} remote(s)...\n", fetch_remotes.len());

    // Perform parallel fetch
    let results = manager.fetch_all(&fetch_remotes).await?;

    // Display results
    let mut success_count = 0;
    let mut failed_count = 0;

    for result in &results {
        if result.success {
            println!("✓ {} - fetched successfully", result.remote);
            success_count += 1;
        } else {
            println!("✗ {} - {}", result.remote, result.message);
            failed_count += 1;
        }
    }

    println!("\n📊 Summary: {success_count} succeeded, {failed_count} failed");

    if success_count > 0 {
        println!("\n💡 Use 'multigit status' to see changes");
    }

    Ok(())
}
