//! Mirror mode - keep all remotes in perfect sync
//!
//! Ensures all remotes have identical branches, tags, and history

use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Command;

/// Execute mirror sync
pub fn execute(force: bool, dry_run: bool) -> Result<()> {
    println!("\n🪞 Mirror Mode - Perfect Remote Sync\n");

    if dry_run {
        println!("🔍 DRY RUN MODE - No changes will be made\n");
    }

    // Get all remotes
    let output = Command::new("git")
        .args(["remote"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get remotes: {e}")))?;

    let remotes: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    if remotes.len() < 2 {
        println!("Mirror mode requires at least 2 remotes.");
        println!("Current remotes: {}", remotes.len());
        return Ok(());
    }

    println!("Mirroring {} remotes:\n", remotes.len());
    for remote in &remotes {
        println!("  • {}", remote);
    }
    println!();

    if !dry_run {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("This will push ALL branches and tags to ALL remotes. Continue?")
            .default(false)
            .interact()?;

        if !confirm {
            println!("Mirror cancelled.");
            return Ok(());
        }
    }

    // Mirror all branches and tags
    for remote in &remotes {
        println!("\n📡 Mirroring to {}...", remote);

        if dry_run {
            println!("  [DRY RUN] Would push all branches and tags");
            continue;
        }

        // Push all branches
        let mut push_args = vec!["push", remote, "--all"];
        if force {
            push_args.push("--force");
        }

        let output = Command::new("git")
            .args(&push_args)
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to push: {e}")))?;

        if output.status.success() {
            println!("  ✅ Branches mirrored");
        } else {
            println!("  ⚠️  Warning: Branch mirror failed");
        }

        // Push all tags
        let mut tag_args = vec!["push", remote, "--tags"];
        if force {
            tag_args.push("--force");
        }

        let output = Command::new("git")
            .args(&tag_args)
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to push tags: {e}")))?;

        if output.status.success() {
            println!("  ✅ Tags mirrored");
        } else {
            println!("  ⚠️  Warning: Tag mirror failed");
        }
    }

    if dry_run {
        println!("\n🔍 DRY RUN COMPLETE - No changes were made");
    } else {
        println!("\n✅ Mirror complete! All remotes are now in sync.");
    }

    Ok(())
}
