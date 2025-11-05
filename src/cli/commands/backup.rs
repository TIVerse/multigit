//! Backup manager for automated backups
//!
//! Configure and manage repository backups to multiple remotes

use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Command;

/// Execute backup manager
pub fn execute(auto: bool) -> Result<()> {
    println!("\n💾 Backup Manager\n");

    if auto {
        perform_auto_backup()?;
    } else {
        interactive_backup()?;
    }

    Ok(())
}

fn interactive_backup() -> Result<()> {
    println!("This feature will backup your repository to all configured remotes.\n");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Create backup now?")
        .default(true)
        .interact()?;

    if confirm {
        perform_auto_backup()?;
    }

    Ok(())
}

fn perform_auto_backup() -> Result<()> {
    println!("Creating backup...\n");

    // Get all remotes
    let output = Command::new("git")
        .args(["remote"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get remotes: {e}")))?;

    let remotes_output = String::from_utf8_lossy(&output.stdout);
    let remotes: Vec<&str> = remotes_output.lines().collect();

    if remotes.is_empty() {
        println!("No remotes configured. Add remotes first with 'mg remote add'");
        return Ok(());
    }

    // Push to all remotes
    for remote in &remotes {
        println!("Backing up to {}...", remote);
        
        let output = Command::new("git")
            .args(["push", remote, "--all"])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to push: {e}")))?;

        if output.status.success() {
            println!("  ✅ Backed up to {}", remote);
        } else {
            println!("  ⚠️  Warning: Backup to {} failed", remote);
        }
    }

    // Push tags
    println!("\nBacking up tags...");
    for remote in &remotes {
        let output = Command::new("git")
            .args(["push", remote, "--tags"])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to push tags: {e}")))?;

        if output.status.success() {
            println!("  ✅ Tags backed up to {}", remote);
        } else {
            println!("  ⚠️  Warning: Tag backup to {} failed", remote);
        }
    }

    println!("\n✅ Backup complete!");

    Ok(())
}
