//! Git aliases manager
//!
//! Manage custom git aliases

use crate::utils::error::{MultiGitError, Result};
use std::process::Command;

/// Execute alias manager
pub fn execute() -> Result<()> {
    println!("\n🔗 Git Aliases Manager\n");

    list_aliases()?;

    println!("\nFeature coming soon: Easy alias management");
    println!("Commands:");
    println!("  mg alias list              - List all aliases");
    println!("  mg alias add <name> <cmd>  - Create new alias");
    println!("  mg alias remove <name>     - Remove alias");
    println!("  mg alias edit <name>       - Edit alias");

    Ok(())
}

fn list_aliases() -> Result<()> {
    let output = Command::new("git")
        .args(["config", "--get-regexp", "^alias\\."])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get aliases: {e}")))?;

    let aliases = String::from_utf8_lossy(&output.stdout);

    if aliases.trim().is_empty() {
        println!("No aliases configured.");
    } else {
        println!("Current Git Aliases:");
        println!("─────────────────────────────────────");
        for line in aliases.lines() {
            if let Some((name, cmd)) = line.split_once(' ') {
                let alias_name = name.trim_start_matches("alias.");
                println!("  {alias_name} = {cmd}");
            }
        }
        println!("─────────────────────────────────────");
    }

    Ok(())
}
