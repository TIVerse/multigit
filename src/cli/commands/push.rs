//! Push command implementation
//!
//! Push to all configured remotes in parallel.

use crate::core::config::Config;
use crate::core::sync_manager::SyncManager;
use crate::utils::error::Result;
use tracing::info;

/// Push to all configured remotes
pub async fn execute(branch: Option<String>, force: bool, remotes: Vec<String>) -> Result<()> {
    info!("Executing push command");

    let manager = SyncManager::new(".")?;

    // Get branch to push
    let branch_name = match branch {
        Some(b) => b,
        None => manager.current_branch()?,
    };

    // Check if working directory is clean
    if !manager.is_clean()? {
        println!("⚠ Warning: Working directory has uncommitted changes");
    }

    // Get remotes to push to
    let push_remotes = if remotes.is_empty() {
        // Load all enabled remotes from config
        let config = Config::load().unwrap_or_default();
        let enabled: Vec<String> = config.enabled_remotes().keys().cloned().collect();
        if enabled.is_empty() {
            println!("⚠ No remotes configured. Use 'multigit remote add' to configure remotes.");
            return Ok(());
        }
        enabled
    } else {
        remotes
    };

    if force {
        println!("⚠ Force push requested - this will overwrite remote history!");
    }

    println!(
        "\n🚀 Pushing '{}' to {} remote(s)...\n",
        branch_name,
        push_remotes.len()
    );

    // Perform parallel push
    let results = manager.push_all(&branch_name, &push_remotes).await?;

    // Display results
    let mut success_count = 0;
    let mut failed_count = 0;

    for result in &results {
        if result.success {
            println!("✓ {} - pushed in {}ms", result.remote, result.duration_ms);
            success_count += 1;
        } else {
            println!("✗ {} - {}", result.remote, result.message);
            failed_count += 1;
        }
    }

    println!("\n📊 Summary: {success_count} succeeded, {failed_count} failed");

    if failed_count > 0 {
        return Err(crate::utils::error::MultiGitError::Other(format!(
            "{failed_count} push(es) failed"
        )));
    }

    Ok(())
}
