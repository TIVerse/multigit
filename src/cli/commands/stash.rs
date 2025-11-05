//! Interactive stash manager
//!
//! Save, apply, and manage stashed changes

use crate::cli::interactive;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::process::Command;

/// Execute stash manager
pub fn execute() -> Result<()> {
    println!("\n💾 Stash Manager\n");

    let options = vec![
        "💾 Save changes to stash",
        "📂 List stashes",
        "✅ Apply latest stash",
        "🗑️  Pop latest stash",
        "👁️  View stash contents",
        "🗑️  Drop a stash",
        "🧹 Clear all stashes",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact()?;

    match choice {
        0 => save_stash()?,
        1 => list_stashes()?,
        2 => apply_stash(None)?,
        3 => pop_stash()?,
        4 => view_stash()?,
        5 => drop_stash()?,
        6 => clear_stashes()?,
        _ => {}
    }

    Ok(())
}

/// Save current changes to stash
fn save_stash() -> Result<()> {
    let message: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Stash message (optional)")
        .allow_empty(true)
        .interact_text()?;

    let mut args = vec!["stash", "push"];
    if !message.is_empty() {
        args.push("-m");
        args.push(&message);
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to stash: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ Changes stashed successfully!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to stash: {}", error)));
    }

    Ok(())
}

/// List all stashes
fn list_stashes() -> Result<()> {
    let output = Command::new("git")
        .args(["stash", "list"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to list stashes: {e}")))?;

    let stashes = String::from_utf8_lossy(&output.stdout);

    if stashes.trim().is_empty() {
        println!("No stashes found.");
    } else {
        println!("\n📋 Stash List:");
        println!("─────────────────────────────────────────────────────────────");
        println!("{}", stashes);
        println!("─────────────────────────────────────────────────────────────");
    }

    Ok(())
}

/// Apply a stash
fn apply_stash(stash_id: Option<String>) -> Result<()> {
    let mut args = vec!["stash", "apply"];
    
    if let Some(ref id) = stash_id {
        args.push(id);
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to apply stash: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ Stash applied successfully!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to apply stash: {}", error)));
    }

    Ok(())
}

/// Pop latest stash
fn pop_stash() -> Result<()> {
    let output = Command::new("git")
        .args(["stash", "pop"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to pop stash: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ Stash popped successfully!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to pop stash: {}", error)));
    }

    Ok(())
}

/// View stash contents
fn view_stash() -> Result<()> {
    // First list stashes
    let output = Command::new("git")
        .args(["stash", "list"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to list stashes: {e}")))?;

    let stashes_output = String::from_utf8_lossy(&output.stdout);
    let stashes: Vec<&str> = stashes_output.lines().collect();

    if stashes.is_empty() {
        println!("No stashes found.");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select stash to view")
        .items(&stashes)
        .default(0)
        .interact()?;

    // Extract stash id (e.g., stash@{0})
    let stash_line = stashes[selection];
    let stash_id = stash_line.split(':').next().unwrap_or("stash@{0}");

    // Show stash diff
    let output = Command::new("git")
        .args(["stash", "show", "-p", stash_id])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to show stash: {e}")))?;

    let diff = String::from_utf8_lossy(&output.stdout);
    println!("\n{}", diff);

    Ok(())
}

/// Drop a specific stash
fn drop_stash() -> Result<()> {
    let output = Command::new("git")
        .args(["stash", "list"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to list stashes: {e}")))?;

    let stashes_output = String::from_utf8_lossy(&output.stdout);
    let stashes: Vec<&str> = stashes_output.lines().collect();

    if stashes.is_empty() {
        println!("No stashes found.");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select stash to drop")
        .items(&stashes)
        .default(0)
        .interact()?;

    let stash_line = stashes[selection];
    let stash_id = stash_line.split(':').next().unwrap_or("stash@{0}");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Drop {}?", stash_id))
        .default(false)
        .interact()?;

    if confirm {
        let output = Command::new("git")
            .args(["stash", "drop", stash_id])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to drop stash: {e}")))?;

        if output.status.success() {
            interactive::print_success(&format!("✅ Dropped {}", stash_id));
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(MultiGitError::other(format!("Failed to drop stash: {}", error)));
        }
    }

    Ok(())
}

/// Clear all stashes
fn clear_stashes() -> Result<()> {
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("⚠️  Clear ALL stashes? This cannot be undone!")
        .default(false)
        .interact()?;

    if confirm {
        let output = Command::new("git")
            .args(["stash", "clear"])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to clear stashes: {e}")))?;

        if output.status.success() {
            interactive::print_success("✅ All stashes cleared!");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(MultiGitError::other(format!("Failed to clear stashes: {}", error)));
        }
    }

    Ok(())
}
