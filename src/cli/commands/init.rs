//! Init command implementation
//!
//! Initialize a repository for MultiGit management.

use crate::core::config::Config;
use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use std::fs;
use std::path::Path;
use tracing::info;

/// Initialize MultiGit for a repository
pub fn execute(path: &str) -> Result<()> {
    info!("Initializing MultiGit at: {}", path);

    let repo_path = Path::new(path);

    // Check if it's a git repository
    let _git_ops = GitOperations::open(repo_path).map_err(|_| {
        MultiGitError::Other(format!(
            "Not a git repository: {}\n\nRun 'git init' first.",
            path
        ))
    })?;

    // Create .multigit directory
    let multigit_dir = repo_path.join(".multigit");
    if !multigit_dir.exists() {
        fs::create_dir_all(&multigit_dir).map_err(|e| {
            MultiGitError::Other(format!("Failed to create .multigit directory: {}", e))
        })?;
        info!("Created .multigit directory");
    }

    // Create config file if it doesn't exist
    let config_path = multigit_dir.join("config.toml");
    if !config_path.exists() {
        let default_config = Config::default();
        let config_str = toml::to_string_pretty(&default_config)
            .map_err(|e| MultiGitError::Other(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_path, config_str)
            .map_err(|e| MultiGitError::Other(format!("Failed to write config: {}", e)))?;

        info!("Created config file at: {}", config_path.display());
    }

    println!("\n✓ MultiGit initialized successfully!");
    println!("\nNext steps:");
    println!(
        "  1. Add remotes: multigit remote add <name> --provider <provider> --username <user>"
    );
    println!("  2. Configure credentials (stored securely in OS keyring)");
    println!("  3. Push to all remotes: multigit push");
    println!("\nConfiguration file: {}", config_path.display());

    Ok(())
}
