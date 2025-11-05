//! Auto-generate changelog from conventional commits
//!
//! Parse conventional commits and create formatted CHANGELOG.md

use crate::utils::error::{MultiGitError, Result};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

/// Generate changelog
pub fn execute(since: Option<String>, output: Option<String>) -> Result<()> {
    println!("\n📋 Generating Changelog...\n");

    let since_ref = since.unwrap_or_else(|| {
        // Try to find last tag
        get_last_tag().unwrap_or_else(|_| "HEAD~10".to_string())
    });

    let commits = get_commits_since(&since_ref)?;
    let changelog = format_changelog(&commits);

    let output_file = output.unwrap_or_else(|| "CHANGELOG.md".to_string());

    // Append or create
    let existing = fs::read_to_string(&output_file).unwrap_or_default();

    let final_content = if existing.is_empty() {
        format!("# Changelog\n\n{changelog}")
    } else {
        format!("{existing}\n\n{changelog}")
    };

    fs::write(&output_file, final_content)
        .map_err(|e| MultiGitError::other(format!("Failed to write changelog: {e}")))?;

    println!("✅ Changelog written to {output_file}");

    Ok(())
}

fn get_last_tag() -> Result<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get last tag: {e}")))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_commits_since(since: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["log", &format!("{since}..HEAD"), "--pretty=format:%s"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get commits: {e}")))?;

    let commits: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(std::string::ToString::to_string)
        .collect();

    Ok(commits)
}

fn format_changelog(commits: &[String]) -> String {
    let mut categorized: HashMap<String, Vec<String>> = HashMap::new();

    for commit in commits {
        let (category, message) = parse_conventional_commit(commit);
        categorized.entry(category).or_default().push(message);
    }

    let mut changelog = String::new();
    let date = chrono::Local::now().format("%Y-%m-%d");
    writeln!(changelog, "## [Unreleased] - {date}\n").unwrap();

    // Order: feat, fix, docs, style, refactor, perf, test, build, ci, chore
    let order = vec![
        "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
    ];

    for category in order {
        if let Some(messages) = categorized.get(category) {
            let title = match category {
                "feat" => "### ✨ Features",
                "fix" => "### 🐛 Bug Fixes",
                "docs" => "### 📚 Documentation",
                "style" => "### 💎 Styles",
                "refactor" => "### ♻️  Refactoring",
                "perf" => "### ⚡ Performance",
                "test" => "### ✅ Tests",
                "build" => "### 🔨 Build",
                "ci" => "### 👷 CI/CD",
                "chore" => "### 🔧 Chores",
                _ => "### Other",
            };

            changelog.push_str(title);
            changelog.push_str("\n\n");

            for msg in messages {
                writeln!(changelog, "- {msg}").unwrap();
            }
            changelog.push('\n');
        }
    }

    changelog
}

fn parse_conventional_commit(commit: &str) -> (String, String) {
    // Parse: type(scope): message
    if let Some(colon_pos) = commit.find(':') {
        let prefix = &commit[..colon_pos];
        let message = commit[colon_pos + 1..].trim();

        let commit_type = if let Some(paren_pos) = prefix.find('(') {
            &prefix[..paren_pos]
        } else {
            prefix
        };

        (commit_type.trim().to_string(), message.to_string())
    } else {
        ("other".to_string(), commit.to_string())
    }
}
