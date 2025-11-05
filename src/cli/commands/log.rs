//! Interactive commit history browser
//!
//! Beautiful, searchable commit history viewer

use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;

/// Execute interactive log viewer
pub fn execute(limit: Option<usize>, branch: Option<String>, author: Option<String>) -> Result<()> {
    println!("\n📜 Commit History Browser\n");

    let _git_ops =
        GitOperations::open(".").map_err(|_| MultiGitError::other("Not in a git repository."))?;

    let limit_val = limit.unwrap_or(20);

    // Build git log command
    let mut args = vec![
        "log".to_string(),
        format!("-{}", limit_val),
        "--pretty=format:%h|%an|%ar|%s".to_string(),
    ];

    if let Some(ref b) = branch {
        args.push(b.clone());
    }

    if let Some(ref a) = author {
        args.push(format!("--author={a}"));
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get log: {e}")))?;

    let log_output = String::from_utf8_lossy(&output.stdout);
    let commits: Vec<&str> = log_output.lines().collect();

    if commits.is_empty() {
        println!("No commits found.");
        return Ok(());
    }

    // Display commits
    println!("Recent commits:");
    println!("─────────────────────────────────────────────────────────────────");

    let commit_options: Vec<String> = commits
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                format!(
                    "{} {} • {} • {}",
                    "📝",
                    parts[0], // hash
                    parts[1], // author
                    parts[2], // time
                )
            } else {
                (*line).to_string()
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commit to view details")
        .items(&commit_options)
        .default(0)
        .interact_opt()?;

    if let Some(idx) = selection {
        let parts: Vec<&str> = commits[idx].split('|').collect();
        if parts.len() >= 4 {
            let hash = parts[0];
            show_commit_details(hash)?;
        }
    }

    Ok(())
}

/// Show detailed commit information
fn show_commit_details(hash: &str) -> Result<()> {
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Commit Details: {hash}");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Show full commit
    let output = Command::new("git")
        .args(["show", "--stat", "--pretty=fuller", hash])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to show commit: {e}")))?;

    let details = String::from_utf8_lossy(&output.stdout);
    println!("{details}");

    Ok(())
}

/// Show graphical log
pub fn show_graph(limit: Option<usize>) -> Result<()> {
    println!("\n🌳 Commit Graph\n");

    let limit_val = limit.unwrap_or(15);

    let output = Command::new("git")
        .args([
            "log",
            &format!("-{limit_val}"),
            "--graph",
            "--pretty=format:%C(yellow)%h%Creset %C(blue)%an%Creset %C(green)%ar%Creset - %s",
            "--abbrev-commit",
            "--all",
        ])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get graph: {e}")))?;

    let graph = String::from_utf8_lossy(&output.stdout);
    println!("{graph}");

    Ok(())
}
