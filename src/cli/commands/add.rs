//! Interactive staging command
//!
//! Visual file selection for staging, similar to `git add -p` but easier

use crate::cli::interactive;
use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};
use std::process::Command;

/// Execute interactive add
pub fn execute() -> Result<()> {
    println!("\n📦 Interactive Staging\n");

    // Check if we're in a git repository
    let git_ops = GitOperations::open(".").map_err(|_| {
        MultiGitError::other("Not in a git repository. Run this command from a git repository.")
    })?;

    // Get modified files
    let modified_files = get_modified_files(&git_ops)?;

    if modified_files.is_empty() {
        interactive::print_info("No modified files found. Working tree is clean!");
        return Ok(());
    }

    // Show staging options
    let options = vec![
        "📂 Stage all files",
        "🎯 Select files to stage",
        "📝 Stage all modified (exclude new files)",
        "✨ Stage all new files",
        "🔍 View diff before staging",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(1)
        .interact()?;

    match choice {
        0 => {
            // Stage all files
            stage_all_files(&modified_files)?;
            interactive::print_success("✅ All files staged!");
        }
        1 => {
            // Select files
            let selected = select_files_to_stage(&modified_files)?;
            if selected.is_empty() {
                interactive::print_info("No files selected.");
            } else {
                stage_files(&selected)?;
                interactive::print_success(&format!("✅ {} file(s) staged!", selected.len()));
            }
        }
        2 => {
            // Stage modified only
            let modified: Vec<_> = modified_files
                .iter()
                .filter(|(_, status)| *status == "modified")
                .map(|(path, _)| path.clone())
                .collect();

            if modified.is_empty() {
                interactive::print_info("No modified files found.");
            } else {
                stage_files(&modified)?;
                interactive::print_success(&format!("✅ {} modified file(s) staged!", modified.len()));
            }
        }
        3 => {
            // Stage new files only
            let new_files: Vec<_> = modified_files
                .iter()
                .filter(|(_, status)| *status == "new")
                .map(|(path, _)| path.clone())
                .collect();

            if new_files.is_empty() {
                interactive::print_info("No new files found.");
            } else {
                stage_files(&new_files)?;
                interactive::print_success(&format!("✅ {} new file(s) staged!", new_files.len()));
            }
        }
        4 => {
            // View diff and stage
            view_diff_and_stage(&modified_files)?;
        }
        _ => {}
    }

    // Show what's staged
    show_staged_files()?;

    Ok(())
}

/// Get list of modified files
fn get_modified_files(git_ops: &GitOperations) -> Result<Vec<(String, String)>> {
    let repo = git_ops.inner();
    let mut files = Vec::new();

    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);
    status_opts.exclude_submodules(true);

    let statuses = repo
        .statuses(Some(&mut status_opts))
        .map_err(|e| MultiGitError::other(format!("Failed to get status: {e}")))?;

    for entry in statuses.iter() {
        let status = entry.status();

        if status.is_ignored() {
            continue;
        }

        let path = entry.path().unwrap_or("").to_string();

        let status_str = if status.is_wt_new() {
            "new"
        } else if status.is_wt_modified() {
            "modified"
        } else if status.is_wt_deleted() {
            "deleted"
        } else if status.is_wt_renamed() {
            "renamed"
        } else if status.is_index_new() || status.is_index_modified() {
            continue; // Skip already staged
        } else {
            "changed"
        };

        if !path.is_empty() {
            files.push((path, status_str.to_string()));
        }
    }

    Ok(files)
}

/// Let user select files to stage
fn select_files_to_stage(files: &[(String, String)]) -> Result<Vec<String>> {
    println!();
    let options: Vec<String> = files
        .iter()
        .map(|(path, status)| {
            let emoji = match status.as_str() {
                "new" => "✨",
                "modified" => "📝",
                "deleted" => "🗑️",
                "renamed" => "📋",
                _ => "📄",
            };
            format!("{} [{}] {}", emoji, status, path)
        })
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select files to stage (Space to toggle, Enter to confirm)")
        .items(&options)
        .interact()?;

    let selected: Vec<String> = selections.iter().map(|&i| files[i].0.clone()).collect();

    Ok(selected)
}

/// Stage all files
fn stage_all_files(files: &[(String, String)]) -> Result<()> {
    for (file, _) in files {
        Command::new("git")
            .args(["add", file])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to stage {}: {}", file, e)))?;
    }
    Ok(())
}

/// Stage specific files
fn stage_files(files: &[String]) -> Result<()> {
    for file in files {
        Command::new("git")
            .args(["add", file])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to stage {}: {}", file, e)))?;
    }
    Ok(())
}

/// View diff and interactively stage
fn view_diff_and_stage(files: &[(String, String)]) -> Result<()> {
    println!("\n📊 Viewing changes...\n");

    for (file, status) in files {
        println!("─────────────────────────────────────");
        println!("File: {} [{}]", file, status);
        println!("─────────────────────────────────────");

        // Show diff
        let output = Command::new("git")
            .args(["diff", file])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to get diff: {e}")))?;

        let diff = String::from_utf8_lossy(&output.stdout);
        if !diff.is_empty() {
            println!("{}", diff);
        } else if *status == "new" {
            println!("(New file - no diff to show)");
        }

        println!();
        let stage = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Stage {}?", file))
            .default(true)
            .interact()?;

        if stage {
            stage_files(&[file.clone()])?;
            println!("✅ Staged\n");
        } else {
            println!("⏭️  Skipped\n");
        }
    }

    Ok(())
}

/// Show what's currently staged
fn show_staged_files() -> Result<()> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-status"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get staged files: {e}")))?;

    let staged = String::from_utf8_lossy(&output.stdout);

    if !staged.trim().is_empty() {
        println!("\n📋 Currently Staged:");
        println!("─────────────────────────────────────");
        for line in staged.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let status = match parts[0] {
                    "A" => "new",
                    "M" => "modified",
                    "D" => "deleted",
                    "R" => "renamed",
                    _ => "changed",
                };
                println!("  [{}] {}", status, parts[1]);
            }
        }
        println!("─────────────────────────────────────");
    }

    Ok(())
}
