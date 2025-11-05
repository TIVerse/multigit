//! Create command implementation
//!
//! Create repositories on all configured platforms.

use crate::core::auth::{AuthBackend, AuthManager};
use crate::providers::traits::{Provider, RepoConfig};
use crate::providers::{
    bitbucket::BitbucketProvider, github::GitHubProvider, gitlab::GitLabProvider,
};
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{Confirm, Input};
use tracing::info;

/// Create a repository on all configured platforms
pub async fn execute(name: String, description: Option<String>, private: bool) -> Result<()> {
    info!("Creating repository: {}", name);

    println!("\n📦 Creating repository '{name}' on all platforms...\n");

    // Get repository configuration
    let repo_config = RepoConfig {
        name: name.clone(),
        description: description.unwrap_or_default(),
        private,
    };

    // Load config to get configured remotes
    let config = crate::core::config::Config::load().unwrap_or_default();

    // Initialize auth manager
    let auth_manager = AuthManager::new(AuthBackend::Keyring, false);

    // Track results
    let mut results = Vec::new();

    // Try to create on GitHub
    if let Some(github_config) = config.remotes.get("github") {
        if let Ok(token) = auth_manager.retrieve_credential("github", &github_config.username) {
            match create_on_github(&token, &github_config.username, &repo_config).await {
                Ok(url) => {
                    println!("✓ GitHub: Created successfully");
                    println!("  URL: {url}");
                    results.push(("github", true));
                }
                Err(e) => {
                    println!("✗ GitHub: Failed - {e}");
                    results.push(("github", false));
                }
            }
        } else {
            println!("⊘ GitHub: Credentials not found");
        }
    } else {
        println!("⊘ GitHub: Not configured (run 'multigit remote add github <username>')");
    }

    // Try to create on GitLab
    if let Some(gitlab_config) = config.remotes.get("gitlab") {
        if let Ok(token) = auth_manager.retrieve_credential("gitlab", &gitlab_config.username) {
            match create_on_gitlab(&token, &gitlab_config.username, &repo_config).await {
                Ok(url) => {
                    println!("✓ GitLab: Created successfully");
                    println!("  URL: {url}");
                    results.push(("gitlab", true));
                }
                Err(e) => {
                    println!("✗ GitLab: Failed - {e}");
                    results.push(("gitlab", false));
                }
            }
        } else {
            println!("⊘ GitLab: Credentials not found");
        }
    } else {
        println!("⊘ GitLab: Not configured (run 'multigit remote add gitlab <username>')");
    }

    // Try to create on Bitbucket
    if let Some(bitbucket_config) = config.remotes.get("bitbucket") {
        if let Ok(password) = auth_manager.retrieve_credential("bitbucket", &bitbucket_config.username) {
            match create_on_bitbucket(&bitbucket_config.username, &password, &repo_config).await {
                Ok(url) => {
                    println!("✓ Bitbucket: Created successfully");
                    println!("  URL: {url}");
                    results.push(("bitbucket", true));
                }
                Err(e) => {
                    println!("✗ Bitbucket: Failed - {e}");
                    results.push(("bitbucket", false));
                }
            }
        } else {
            println!("⊘ Bitbucket: Credentials not found");
        }
    } else {
        println!("⊘ Bitbucket: Not configured (run 'multigit remote add bitbucket <username>')");
    }

    // Summary
    let success_count = results.iter().filter(|(_, success)| *success).count();
    let total_count = results.len();

    println!("\n📊 Summary: Created on {success_count}/{total_count} platforms");

    if success_count > 0 {
        println!("\n💡 Next steps:");
        println!("  1. Add git remotes: multigit remote sync");
        println!("  2. Push your code: multigit push all");
    }

    Ok(())
}

/// Create repository on GitHub
async fn create_on_github(token: &str, username: &str, config: &RepoConfig) -> Result<String> {
    let provider = GitHubProvider::new(token.to_string(), username.to_string())?;
    let repo = provider
        .create_repo(config.clone())
        .await
        .map_err(|e| MultiGitError::Other(format!("GitHub API error: {e}")))?;
    Ok(repo.html_url.unwrap_or(repo.url))
}

/// Create repository on GitLab
async fn create_on_gitlab(token: &str, username: &str, config: &RepoConfig) -> Result<String> {
    let provider = GitLabProvider::new(token.to_string(), username.to_string(), None)?;
    let repo = provider
        .create_repo(config.clone())
        .await
        .map_err(|e| MultiGitError::Other(format!("GitLab API error: {e}")))?;
    Ok(repo.html_url.unwrap_or(repo.url))
}

/// Create repository on Bitbucket
async fn create_on_bitbucket(
    username: &str,
    password: &str,
    config: &RepoConfig,
) -> Result<String> {
    let provider = BitbucketProvider::new(username.to_string(), password.to_string())?;
    let repo = provider
        .create_repo(config.clone())
        .await
        .map_err(|e| MultiGitError::Other(format!("Bitbucket API error: {e}")))?;
    Ok(repo.html_url.unwrap_or(repo.url))
}

/// Interactive repository creation
pub async fn execute_interactive() -> Result<()> {
    println!("🎨 Interactive Repository Creation\n");

    let name: String = Input::new()
        .with_prompt("Repository name")
        .interact_text()
        .map_err(|e| MultiGitError::Other(format!("Input error: {e}")))?;

    let description: String = Input::new()
        .with_prompt("Description (optional)")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| MultiGitError::Other(format!("Input error: {e}")))?;

    let private = Confirm::new()
        .with_prompt("Make repository private?")
        .default(true)
        .interact()
        .map_err(|e| MultiGitError::Other(format!("Input error: {e}")))?;

    let desc = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    execute(name, desc, private).await
}
