//! Repository statistics
//!
//! Show contribution graphs, commit frequency, and more

use crate::utils::error::{MultiGitError, Result};
use std::process::Command;

/// Execute stats viewer
pub fn execute() -> Result<()> {
    println!("\n📊 Repository Statistics\n");

    show_commit_stats()?;
    show_contributor_stats()?;
    show_file_stats()?;

    Ok(())
}

fn show_commit_stats() -> Result<()> {
    println!("📈 Commit Activity\n");

    // Total commits
    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to count commits: {e}")))?;

    let total_commits = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("  Total commits: {total_commits}");

    // Commits this week
    let output = Command::new("git")
        .args(["rev-list", "--count", "--since=1.week", "HEAD"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to count commits: {e}")))?;

    let week_commits = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("  Commits this week: {week_commits}");

    // Commits today
    let output = Command::new("git")
        .args(["rev-list", "--count", "--since=midnight", "HEAD"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to count commits: {e}")))?;

    let today_commits = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("  Commits today: {today_commits}\n");

    Ok(())
}

fn show_contributor_stats() -> Result<()> {
    println!("👥 Top Contributors\n");

    let output = Command::new("git")
        .args(["shortlog", "-sn", "--all", "--no-merges"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get contributors: {e}")))?;

    let contributors = String::from_utf8_lossy(&output.stdout);

    for (i, line) in contributors.lines().take(5).enumerate() {
        println!("  {}. {}", i + 1, line.trim());
    }
    println!();

    Ok(())
}

fn show_file_stats() -> Result<()> {
    println!("📁 Repository Size\n");

    // Count files
    let output = Command::new("git")
        .args(["ls-files"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to list files: {e}")))?;

    let file_count = String::from_utf8_lossy(&output.stdout).lines().count();
    println!("  Tracked files: {file_count}");

    // Count branches
    let output = Command::new("git")
        .args(["branch", "-a"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to count branches: {e}")))?;

    let branch_count = String::from_utf8_lossy(&output.stdout).lines().count();
    println!("  Branches: {branch_count}");

    // Count tags
    let output = Command::new("git")
        .args(["tag"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to count tags: {e}")))?;

    let tag_count = String::from_utf8_lossy(&output.stdout).lines().count();
    println!("  Tags: {tag_count}\n");

    Ok(())
}
