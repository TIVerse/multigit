//! Interactive branch switcher
//!
//! Fuzzy-searchable branch switching

use crate::cli::interactive;
use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};
use std::process::Command;

/// Execute interactive branch switch
pub fn execute(new_branch: Option<String>) -> Result<()> {
    let _git_ops =
        GitOperations::open(".").map_err(|_| MultiGitError::other("Not in a git repository."))?;

    if let Some(branch_name) = new_branch {
        // Direct switch
        switch_to_branch(&branch_name)?;
        interactive::print_success(&format!("✅ Switched to branch '{branch_name}'"));
        return Ok(());
    }

    println!("\n🔀 Branch Switcher\n");

    // Get all branches
    let branches = get_branches()?;

    if branches.is_empty() {
        println!("No branches found.");
        return Ok(());
    }

    // Current branch
    let current = get_current_branch()?;
    println!("Current branch: {current}\n");

    // Format branches with indicators
    let branch_options: Vec<String> = branches
        .iter()
        .map(|b| {
            if b == &current {
                format!("→ {b} (current)")
            } else {
                format!("  {b}")
            }
        })
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select branch to switch to (type to search)")
        .items(&branch_options)
        .default(0)
        .interact_opt()?;

    if let Some(idx) = selection {
        let selected_branch = &branches[idx];

        if selected_branch == &current {
            println!("Already on branch '{current}'");
        } else {
            // Check for uncommitted changes
            if has_uncommitted_changes()? {
                println!("\n⚠️  You have uncommitted changes.");
                let proceed = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Switch anyway? (changes will be preserved)")
                    .default(false)
                    .interact()?;

                if !proceed {
                    println!("Switch cancelled.");
                    return Ok(());
                }
            }

            switch_to_branch(selected_branch)?;
            interactive::print_success(&format!("✅ Switched to branch '{selected_branch}'"));
        }
    }

    Ok(())
}

/// Create and switch to new branch
pub fn create_and_switch(from: Option<String>) -> Result<()> {
    println!("\n🌱 Create New Branch\n");

    let branch_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new branch name")
        .validate_with(|input: &String| -> std::result::Result<(), &str> {
            if input.trim().is_empty() {
                Err("Branch name cannot be empty")
            } else if input.contains(' ') {
                Err("Branch name cannot contain spaces")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let base_branch = if let Some(b) = from {
        b
    } else {
        get_current_branch()?
    };

    // Create and switch
    let output = Command::new("git")
        .args(["checkout", "-b", &branch_name, &base_branch])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to create branch: {e}")))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!(
            "Failed to create branch: {error}"
        )));
    }

    interactive::print_success(&format!(
        "✅ Created and switched to branch '{branch_name}'"
    ));

    Ok(())
}

/// Get all branches
fn get_branches() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["branch", "--format=%(refname:short)"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get branches: {e}")))?;

    let branches: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(branches)
}

/// Get current branch
fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get current branch: {e}")))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check for uncommitted changes
fn has_uncommitted_changes() -> Result<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to check status: {e}")))?;

    Ok(!output.stdout.is_empty())
}

/// Switch to branch
fn switch_to_branch(branch: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["checkout", branch])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to switch branch: {e}")))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to switch: {error}")));
    }

    Ok(())
}
