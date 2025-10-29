//! Remote management commands
//!
//! Handles adding, removing, listing, testing, and updating Git hosting remotes.
//! Integrates with the authentication manager for secure credential storage.

use crate::cli::interactive;
use crate::core::auth::{AuthBackend, AuthManager};
use crate::core::config::{Config, RemoteConfig};
use crate::providers::bitbucket::BitbucketProvider;
use crate::providers::gitea::GiteaProvider;
use crate::providers::github::GitHubProvider;
use crate::providers::gitlab::GitLabProvider;
use crate::providers::traits::Provider;
use crate::utils::error::{MultiGitError, Result};
use std::sync::Arc;
use tracing::{info, warn};

/// Add a new remote provider
pub async fn add_remote(
    provider_name: String,
    username: String,
    api_url: Option<String>,
    interactive_mode: bool,
) -> Result<()> {
    info!("Adding remote: {} for user {}", provider_name, username);

    // Validate provider name
    let provider_lower = provider_name.to_lowercase();
    if !is_supported_provider(&provider_lower) {
        return Err(MultiGitError::other(format!(
            "Unsupported provider '{provider_name}'. Supported providers: github, gitlab, bitbucket, codeberg, gitea"
        )));
    }

    // Load config
    let mut config = Config::load()?;

    // Check if remote already exists
    if config.remotes.contains_key(&provider_lower) {
        if interactive_mode {
            let overwrite = interactive::confirm(&format!(
                "Remote '{provider_lower}' already exists. Overwrite?"
            ))?;

            if !overwrite {
                interactive::print_info("Operation cancelled");
                return Ok(());
            }
        } else {
            return Err(MultiGitError::other(format!(
                "Remote '{provider_lower}' already exists. Use 'update' command to modify it."
            )));
        }
    }

    // Get token from user
    let token = if interactive_mode {
        interactive::prompt_token(&provider_lower)?
    } else {
        // In non-interactive mode, check for environment variable
        let env_var = format!("MULTIGIT_{}_TOKEN", provider_lower.to_uppercase());
        std::env::var(&env_var).map_err(|_| {
            MultiGitError::auth(
                provider_lower.clone(),
                format!(
                    "Token not provided. Set {env_var} environment variable or use interactive mode"
                ),
            )
        })?
    };

    // Test connection before saving
    interactive::print_info(&format!("Testing connection to {provider_name}..."));
    let provider = create_provider(&provider_lower, &username, &token, api_url.as_deref())?;

    match provider.test_connection().await {
        Ok(true) => {
            interactive::print_success(&format!("Successfully connected to {provider_name}"));
        }
        Ok(false) => {
            return Err(MultiGitError::auth(
                provider_lower,
                "Authentication failed. Please check your credentials".to_string(),
            ));
        }
        Err(e) => {
            return Err(MultiGitError::network(format!(
                "Failed to connect to {provider_name}: {e}"
            )));
        }
    }

    // Store credentials
    let auth_manager = AuthManager::new(AuthBackend::Keyring, config.security.audit_log);

    auth_manager.store_credential(&provider_lower, &username, &token)?;
    interactive::print_success("Credentials stored securely");

    // Update config
    let remote_config = RemoteConfig {
        username: username.clone(),
        api_url,
        enabled: true,
        provider: Some(provider_lower.clone()),
        use_ssh: false,
        priority: 0,
    };

    config.remotes.insert(provider_lower.clone(), remote_config);
    config.save()?;

    interactive::print_success(&format!(
        "Remote '{provider_lower}' added successfully for user {username}"
    ));

    Ok(())
}

/// List all configured remotes
pub fn list_remotes(detailed: bool) -> Result<()> {
    let config = Config::load()?;

    if config.remotes.is_empty() {
        interactive::print_info("No remotes configured.");
        interactive::print_info("Add a remote with: multigit remote add <provider> <username>");
        return Ok(());
    }

    println!("\n📋 Configured Remotes:\n");

    for (name, remote_config) in &config.remotes {
        let status = if remote_config.enabled { "✓" } else { "✗" };
        let provider_display = remote_config
            .provider
            .as_ref().map_or_else(|| name.clone(), std::string::ToString::to_string);

        println!("  {status} {name} ({provider_display})");

        if detailed {
            println!("      Username: {}", remote_config.username);

            if let Some(url) = &remote_config.api_url {
                println!("      API URL: {url}");
            }

            println!("      Enabled: {}", remote_config.enabled);
            println!(
                "      Protocol: {}",
                if remote_config.use_ssh {
                    "SSH"
                } else {
                    "HTTPS"
                }
            );
            println!();
        }
    }

    println!();
    Ok(())
}

/// Remove a remote from configuration
pub fn remove_remote(name: String, force: bool) -> Result<()> {
    let mut config = Config::load()?;

    let name_lower = name.to_lowercase();

    if !config.remotes.contains_key(&name_lower) {
        return Err(MultiGitError::other(format!("Remote '{name}' not found")));
    }

    // Confirm deletion if not forced
    if !force {
        let confirm = interactive::confirm(&format!(
            "Are you sure you want to remove remote '{name_lower}'? This will also delete stored credentials."
        ))?;

        if !confirm {
            interactive::print_info("Operation cancelled");
            return Ok(());
        }
    }

    // Remove from config
    let remote_config = config.remotes.remove(&name_lower).unwrap();
    config.save()?;

    // Remove credentials
    let auth_manager = AuthManager::new(AuthBackend::Keyring, config.security.audit_log);

    // Attempt to remove credentials (don't fail if they don't exist)
    if let Err(e) = auth_manager.remove_credential(&name_lower, &remote_config.username) {
        warn!("Failed to remove credentials: {}", e);
        interactive::print_warning("Could not remove stored credentials (they may not exist)");
    }

    interactive::print_success(&format!("Remote '{name_lower}' removed successfully"));

    Ok(())
}

/// Test connection to a remote
pub async fn test_remote(name: String) -> Result<()> {
    let config = Config::load()?;

    let name_lower = name.to_lowercase();

    let remote_config = config
        .remotes
        .get(&name_lower)
        .ok_or_else(|| MultiGitError::other(format!("Remote '{name}' not found")))?;

    if !remote_config.enabled {
        interactive::print_warning(&format!("Remote '{name_lower}' is disabled"));
    }

    // Get credentials
    let auth_manager = AuthManager::new(AuthBackend::Keyring, config.security.audit_log);

    let token = auth_manager
        .retrieve_credential(&name_lower, &remote_config.username)
        .map_err(|e| {
            MultiGitError::auth(
                name_lower.clone(),
                format!("Could not retrieve credentials: {e}"),
            )
        })?;

    // Create provider and test
    interactive::print_info(&format!("Testing connection to {name_lower}..."));

    let provider = create_provider(
        &name_lower,
        &remote_config.username,
        &token,
        remote_config.api_url.as_deref(),
    )?;

    match provider.test_connection().await {
        Ok(true) => {
            interactive::print_success(&format!("✓ {name_lower} connection successful"));

            // Try to get rate limit info
            if let Ok(rate_limit) = provider.get_rate_limit().await {
                println!("\n  Rate Limit Information:");
                println!("    Limit: {}", rate_limit.limit);
                println!("    Remaining: {}", rate_limit.remaining);
                println!("    Reset: {}", rate_limit.reset_at);
            }

            Ok(())
        }
        Ok(false) => Err(MultiGitError::auth(
            name_lower,
            "Authentication failed. Credentials may be invalid or expired".to_string(),
        )),
        Err(e) => Err(MultiGitError::network(format!(
            "Connection test failed: {e}"
        ))),
    }
}

/// Update remote credentials
pub async fn update_remote(name: String, interactive_mode: bool) -> Result<()> {
    let config = Config::load()?;

    let name_lower = name.to_lowercase();

    let remote_config = config
        .remotes
        .get(&name_lower)
        .ok_or_else(|| MultiGitError::other(format!("Remote '{name}' not found")))?;

    // Get new token
    let token = if interactive_mode {
        println!("\nUpdating credentials for '{name_lower}'\n");
        interactive::prompt_token(&name_lower)?
    } else {
        let env_var = format!("MULTIGIT_{}_TOKEN", name_lower.to_uppercase());
        std::env::var(&env_var).map_err(|_| {
            MultiGitError::auth(
                name_lower.clone(),
                format!(
                    "Token not provided. Set {env_var} environment variable or use interactive mode"
                ),
            )
        })?
    };

    // Test new credentials
    interactive::print_info("Testing new credentials...");

    let provider = create_provider(
        &name_lower,
        &remote_config.username,
        &token,
        remote_config.api_url.as_deref(),
    )?;

    match provider.test_connection().await {
        Ok(true) => {
            interactive::print_success("New credentials are valid");
        }
        Ok(false) => {
            return Err(MultiGitError::auth(
                name_lower,
                "Authentication failed with new credentials".to_string(),
            ));
        }
        Err(e) => {
            return Err(MultiGitError::network(format!(
                "Failed to test new credentials: {e}"
            )));
        }
    }

    // Update credentials
    let auth_manager = AuthManager::new(AuthBackend::Keyring, config.security.audit_log);

    auth_manager.store_credential(&name_lower, &remote_config.username, &token)?;

    interactive::print_success(&format!(
        "Credentials for '{name_lower}' updated successfully"
    ));

    Ok(())
}

/// Test all configured remotes
pub async fn test_all_remotes() -> Result<()> {
    let config = Config::load()?;

    if config.remotes.is_empty() {
        interactive::print_info("No remotes configured");
        return Ok(());
    }

    println!("\n🔍 Testing all remotes...\n");

    let mut success_count = 0;
    let mut fail_count = 0;

    for name in config.remotes.keys() {
        match test_remote(name.clone()).await {
            Ok(()) => success_count += 1,
            Err(e) => {
                interactive::print_error(&format!("✗ {name}: {e}"));
                fail_count += 1;
            }
        }
        println!();
    }

    println!("\nTest Results:");
    println!("  ✓ Success: {success_count}");
    println!("  ✗ Failed: {fail_count}");

    if fail_count > 0 {
        println!(
            "\nSome remotes failed to connect. Run 'multigit remote test <name>' for details."
        );
    }

    Ok(())
}

/// Create a provider instance
fn create_provider(
    provider: &str,
    username: &str,
    token: &str,
    api_url: Option<&str>,
) -> Result<Arc<dyn Provider>> {
    let provider_instance: Arc<dyn Provider> = match provider {
        "github" => {
            let p = GitHubProvider::new(token.to_string(), username.to_string())?;
            Arc::new(p)
        }
        "gitlab" => {
            let url = api_url.map(std::string::ToString::to_string);
            let p = GitLabProvider::new(token.to_string(), username.to_string(), url)?;
            Arc::new(p)
        }
        "bitbucket" => {
            let p = BitbucketProvider::new(username.to_string(), token.to_string())?;
            Arc::new(p)
        }
        "gitea" => {
            let url = api_url.ok_or_else(|| {
                MultiGitError::config("Gitea requires an API URL. Use --url flag".to_string())
            })?;
            let p = GiteaProvider::new(token.to_string(), username.to_string(), url.to_string())?;
            Arc::new(p)
        }
        "codeberg" => {
            let p = GiteaProvider::new(
                token.to_string(),
                username.to_string(),
                "https://codeberg.org".to_string(),
            )?;
            Arc::new(p)
        }
        _ => {
            return Err(MultiGitError::other(format!(
                "Unsupported provider: {provider}"
            )));
        }
    };

    Ok(provider_instance)
}

/// Check if a provider is supported
fn is_supported_provider(provider: &str) -> bool {
    matches!(
        provider,
        "github" | "gitlab" | "bitbucket" | "codeberg" | "gitea"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_providers() {
        assert!(is_supported_provider("github"));
        assert!(is_supported_provider("gitlab"));
        assert!(is_supported_provider("bitbucket"));
        assert!(is_supported_provider("codeberg"));
        assert!(is_supported_provider("gitea"));
        assert!(!is_supported_provider("invalid"));
    }
}
