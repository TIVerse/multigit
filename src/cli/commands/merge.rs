//! Smart merge from multiple remotes
//!
//! Merge changes from multiple remotes with conflict preview

use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::process::Command;

/// Execute smart merge
pub fn execute(from_remote: Option<String>, branch: Option<String>) -> Result<()> {
    println!("\n🔀 Smart Merge\n");

    if let Some(remote) = from_remote {
        let branch_name = branch.unwrap_or_else(|| "main".to_string());
        merge_from_remote(&remote, &branch_name)?;
    } else {
        interactive_merge()?;
    }

    Ok(())
}

fn interactive_merge() -> Result<()> {
    // Get remotes
    let output = Command::new("git")
        .args(["remote"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get remotes: {e}")))?;

    let remotes: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(std::string::ToString::to_string)
        .collect();

    if remotes.is_empty() {
        println!("No remotes configured.");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select remote to merge from")
        .items(&remotes)
        .default(0)
        .interact()?;

    let remote = &remotes[selection];

    // Get current branch
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get branch: {e}")))?;

    let current_branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

    merge_from_remote(remote, &current_branch)?;

    Ok(())
}

fn merge_from_remote(remote: &str, branch: &str) -> Result<()> {
    println!("Merging {remote}/{branch} into current branch...\n");

    // Fetch first
    let output = Command::new("git")
        .args(["fetch", remote])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to fetch: {e}")))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Fetch failed: {error}")));
    }

    // Check for conflicts
    let remote_branch = format!("{remote}/{branch}");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Proceed to merge {remote_branch}?"))
        .default(true)
        .interact()?;

    if !confirm {
        println!("Merge cancelled.");
        return Ok(());
    }

    // Merge
    let output = Command::new("git")
        .args(["merge", &remote_branch])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to merge: {e}")))?;

    if output.status.success() {
        println!("✅ Merge successful!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        if error.contains("CONFLICT") {
            println!("⚠️  Conflicts detected! Use 'mg conflict' to resolve.");
        } else {
            return Err(MultiGitError::other(format!("Merge failed: {error}")));
        }
    }

    Ok(())
}
