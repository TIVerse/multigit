//! Clone command implementation
//!
//! Clone a repository with multi-remote support.

use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use tracing::info;

/// Clone a repository with optional mirror remotes
pub fn execute(url: String, path: Option<String>, mirrors: Vec<String>) -> Result<()> {
    info!("Cloning repository from: {}", url);

    // Determine clone path
    let clone_path = if let Some(p) = path {
        p
    } else {
        // Extract repo name from URL
        extract_repo_name(&url)
    };

    println!("\n📥 Cloning repository...");
    println!("   Source: {url}");
    println!("   Destination: {clone_path}\n");

    // Perform the clone
    let git_ops = GitOperations::clone(&url, &clone_path)?;
    println!("✓ Repository cloned successfully");

    // Add mirror remotes if specified
    if !mirrors.is_empty() {
        println!("\n🔗 Adding mirror remotes...\n");

        for mirror in &mirrors {
            match add_mirror_remote(&git_ops, mirror, &extract_repo_name(&url)) {
                Ok(()) => println!("✓ Added mirror: {mirror}"),
                Err(e) => println!("⚠ Failed to add mirror {mirror}: {e}"),
            }
        }
    }

    println!("\n✅ Clone complete!");
    println!("\n💡 Next steps:");
    println!("   cd {clone_path}");
    println!("   multigit init");

    Ok(())
}

/// Extract repository name from URL
fn extract_repo_name(url: &str) -> String {
    // Remove .git suffix if present
    let url = url.trim_end_matches(".git");

    // Get the last component of the path
    url.rsplit('/').next().unwrap_or("repository").to_string()
}

/// Add a mirror remote to the repository
fn add_mirror_remote(git_ops: &GitOperations, mirror_name: &str, repo_name: &str) -> Result<()> {
    // This is a simplified version - in production you'd construct the full URL
    // based on the provider type and user configuration
    let mirror_url = format!("https://{mirror_name}.com/user/{repo_name}.git");

    git_ops.add_remote(mirror_name, &mirror_url)?;
    Ok(())
}

/// Clone with interactive mirror selection
pub fn execute_interactive(url: String) -> Result<()> {
    use dialoguer::{Input, MultiSelect};

    println!("🎨 Interactive Clone\n");

    // Get clone path
    let default_path = extract_repo_name(&url);
    let path: String = Input::new()
        .with_prompt("Clone to directory")
        .default(default_path)
        .interact_text()
        .map_err(|e| MultiGitError::Other(format!("Input error: {e}")))?;

    // Select mirror providers
    let providers = vec!["github", "gitlab", "bitbucket", "codeberg"];
    let selections = MultiSelect::new()
        .with_prompt("Select mirror remotes (optional)")
        .items(&providers)
        .interact()
        .map_err(|e| MultiGitError::Other(format!("Input error: {e}")))?;

    let mirrors: Vec<String> = selections
        .iter()
        .map(|&i| providers[i].to_string())
        .collect();

    execute(url, Some(path), mirrors)
}
