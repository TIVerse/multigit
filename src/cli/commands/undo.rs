//! Safe undo operations
//!
//! Undo commits, unstage files, or discard changes safely

use crate::cli::interactive;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::process::Command;

/// Execute undo helper
pub fn execute() -> Result<()> {
    println!("\n⏮️  Undo Helper\n");

    let options = vec![
        "⏮️  Undo last commit (keep changes)",
        "🗑️  Undo last commit (discard changes)",
        "📤 Unstage all files",
        "🔄 Discard all uncommitted changes",
        "📝 Discard changes in specific files",
        "⏪ Reset to a previous commit",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to undo?")
        .items(&options)
        .default(0)
        .interact()?;

    match choice {
        0 => undo_last_commit(true)?,
        1 => undo_last_commit(false)?,
        2 => unstage_all()?,
        3 => discard_all_changes()?,
        4 => discard_file_changes()?,
        5 => reset_to_commit()?,
        _ => {}
    }

    Ok(())
}

/// Undo last commit
fn undo_last_commit(keep_changes: bool) -> Result<()> {
    // Show last commit
    let output = Command::new("git")
        .args(["log", "-1", "--oneline"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get last commit: {e}")))?;

    let last_commit = String::from_utf8_lossy(&output.stdout);

    println!("\nLast commit:");
    println!("{last_commit}");

    let action = if keep_changes {
        "undo commit and keep changes in working directory"
    } else {
        "⚠️  undo commit and DISCARD all changes"
    };

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Proceed to {action}?"))
        .default(false)
        .interact()?;

    if !confirm {
        println!("Cancelled.");
        return Ok(());
    }

    let reset_type = if keep_changes { "soft" } else { "hard" };

    let output = Command::new("git")
        .args(["reset", &format!("--{reset_type}"), "HEAD~1"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to reset: {e}")))?;

    if output.status.success() {
        if keep_changes {
            interactive::print_success("✅ Commit undone, changes preserved!");
        } else {
            interactive::print_success("✅ Commit undone and changes discarded!");
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to undo: {error}")));
    }

    Ok(())
}

/// Unstage all files
fn unstage_all() -> Result<()> {
    let output = Command::new("git")
        .args(["reset", "HEAD"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to unstage: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ All files unstaged!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to unstage: {error}")));
    }

    Ok(())
}

/// Discard all uncommitted changes
fn discard_all_changes() -> Result<()> {
    println!("\n⚠️  WARNING: This will discard ALL uncommitted changes!");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you absolutely sure?")
        .default(false)
        .interact()?;

    if !confirm {
        println!("Cancelled.");
        return Ok(());
    }

    // Discard changes in tracked files
    let output = Command::new("git")
        .args(["checkout", "."])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to discard changes: {e}")))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!(
            "Failed to discard changes: {error}"
        )));
    }

    // Clean untracked files
    let confirm_clean = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Also remove untracked files?")
        .default(false)
        .interact()?;

    if confirm_clean {
        Command::new("git")
            .args(["clean", "-fd"])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to clean: {e}")))?;
    }

    interactive::print_success("✅ Changes discarded!");

    Ok(())
}

/// Discard changes in specific files
fn discard_file_changes() -> Result<()> {
    // Get modified files
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get status: {e}")))?;

    let status_output = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = status_output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                Some(parts[1].trim().to_string())
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        println!("No modified files.");
        return Ok(());
    }

    use dialoguer::MultiSelect;

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select files to discard changes (⚠️ cannot be undone!)")
        .items(&files)
        .interact()?;

    if selections.is_empty() {
        println!("No files selected.");
        return Ok(());
    }

    for &idx in &selections {
        let file = &files[idx];
        Command::new("git")
            .args(["checkout", "--", file])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to discard {file}: {e}")))?;
    }

    interactive::print_success(&format!(
        "✅ Discarded changes in {} file(s)!",
        selections.len()
    ));

    Ok(())
}

/// Reset to a previous commit
fn reset_to_commit() -> Result<()> {
    println!("\n📜 Recent commits:\n");

    // Show recent commits
    let output = Command::new("git")
        .args(["log", "--oneline", "-10"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get log: {e}")))?;

    let log_output = String::from_utf8_lossy(&output.stdout);
    let commits: Vec<&str> = log_output.lines().collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commit to reset to")
        .items(&commits)
        .default(0)
        .interact()?;

    let commit_hash = commits[selection].split_whitespace().next().unwrap_or("");

    println!("\nReset options:");
    let reset_options = vec![
        "Soft (keep changes staged)",
        "Mixed (keep changes unstaged)",
        "Hard (⚠️ discard all changes)",
    ];

    let reset_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select reset type")
        .items(&reset_options)
        .default(1)
        .interact()?;

    let reset_type = match reset_choice {
        0 => "soft",
        1 => "mixed",
        2 => "hard",
        _ => "mixed",
    };

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Reset to {commit_hash} using --{reset_type}?"))
        .default(false)
        .interact()?;

    if !confirm {
        println!("Cancelled.");
        return Ok(());
    }

    let output = Command::new("git")
        .args(["reset", &format!("--{reset_type}"), commit_hash])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to reset: {e}")))?;

    if output.status.success() {
        interactive::print_success(&format!("✅ Reset to {commit_hash} ({reset_type})"));
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to reset: {error}")));
    }

    Ok(())
}
