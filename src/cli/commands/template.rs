//! Commit message templates
//!
//! Manage reusable commit templates

use crate::utils::error::{MultiGitError, Result};
use std::fs;
use std::path::PathBuf;

/// Execute template manager
pub fn execute() -> Result<()> {
    println!("\n📝 Commit Templates\n");
    println!("Feature coming soon: Reusable commit message templates");
    println!("Commands:");
    println!("  mg template create <name>  - Create new template");
    println!("  mg template list           - List all templates");
    println!("  mg template use <name>     - Use template for commit");
    println!("  mg template delete <name>  - Delete template");
    Ok(())
}

fn get_templates_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("multigit")
        .join("templates")
}
