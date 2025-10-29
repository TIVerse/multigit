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

    let manager = SyncManager::new(".")?;

    // Determine which remotes to fetch from
    let fetch_remotes = if all {
        // Load all enabled remotes from config
        let config = Config::load().unwrap_or_default();
        let enabled: Vec<String> = config.enabled_remotes().keys().cloned().collect();
        if enabled.is_empty() {
            println!("⚠️  No remotes configured. Use 'multigit remote add' to add remotes.");
            return Ok(());
        }
        enabled
    } else if !remotes.is_empty() {
        remotes
    } else {
        // Default to origin if no remotes specified
        vec!["origin".to_string()]
    };

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

    println!(
        "\n📊 Summary: {success_count} succeeded, {failed_count} failed"
    );

    if success_count > 0 {
        println!("\n💡 Use 'multigit status' to see changes");
    }

    Ok(())
}
