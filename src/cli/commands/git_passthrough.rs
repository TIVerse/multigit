//! Git passthrough for native git commands
//!
//! Allows mg to act as a drop-in git replacement

use crate::utils::error::{MultiGitError, Result};
use std::process::Command;

/// Execute a git command directly
pub fn execute(args: Vec<String>) -> Result<()> {
    let output = Command::new("git")
        .args(&args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to execute git: {e}")))?;

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}

/// Check if git is available
#[must_use]
pub fn check_git_available() -> bool {
    Command::new("git").arg("--version").output().is_ok()
}
