//! Interactive prompts and user input
//!
//! Provides interactive UI components for user interactions using dialoguer.
//! Includes token input with masking, provider selection, and confirmation prompts.

use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};

/// Supported provider types for selection
const PROVIDERS: &[&str] = &["github", "gitlab", "bitbucket", "codeberg", "gitea"];

/// Prompt for a provider selection
pub fn prompt_provider() -> Result<String> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Git hosting provider")
        .items(PROVIDERS)
        .default(0)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Provider selection failed: {e}")))?;

    Ok(PROVIDERS[selection].to_string())
}

/// Prompt for username input
pub fn prompt_username(provider: &str) -> Result<String> {
    let username = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter your {provider} username"))
        .interact_text()
        .map_err(|e| MultiGitError::other(format!("Username input failed: {e}")))?;

    if username.trim().is_empty() {
        return Err(MultiGitError::other("Username cannot be empty".to_string()));
    }

    Ok(username.trim().to_string())
}

/// Prompt for token/password with masked input
pub fn prompt_token(provider: &str) -> Result<String> {
    println!("\n📝 Token/Password Requirements:");
    match provider {
        "github" => {
            println!("  • Go to: https://github.com/settings/tokens");
            println!("  • Click 'Generate new token (classic)'");
            println!("  • Required scopes: repo, read:user");
        }
        "gitlab" => {
            println!("  • Go to: https://gitlab.com/-/profile/personal_access_tokens");
            println!("  • Required scopes: api, read_user, write_repository");
        }
        "bitbucket" => {
            println!("  • Go to: https://bitbucket.org/account/settings/app-passwords/");
            println!("  • Required permissions: Repositories: Read, Write");
        }
        "codeberg" => {
            println!("  • Go to: https://codeberg.org/user/settings/applications");
            println!("  • Required scopes: write:repository, read:user");
        }
        "gitea" => {
            println!("  • Go to your Gitea instance settings");
            println!("  • Generate an access token with repository permissions");
        }
        _ => {
            println!("  • Generate a personal access token with repository permissions");
        }
    }
    println!();

    let token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter your {provider} token/password"))
        .interact()
        .map_err(|e| MultiGitError::other(format!("Token input failed: {e}")))?;

    if token.trim().is_empty() {
        return Err(MultiGitError::other("Token cannot be empty".to_string()));
    }

    Ok(token.trim().to_string())
}

/// Prompt for custom API URL (for self-hosted instances)
pub fn prompt_api_url(provider: &str) -> Result<Option<String>> {
    if provider != "gitea" && provider != "gitlab" {
        return Ok(None);
    }

    let use_custom = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use custom/self-hosted instance?")
        .default(false)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Confirmation failed: {e}")))?;

    if !use_custom {
        return Ok(None);
    }

    let url = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter API URL (e.g., https://gitea.example.com)")
        .interact_text()
        .map_err(|e| MultiGitError::other(format!("URL input failed: {e}")))?;

    if url.trim().is_empty() {
        return Ok(None);
    }

    Ok(Some(url.trim().to_string()))
}

/// Prompt for confirmation
pub fn confirm(message: &str) -> Result<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(false)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Confirmation failed: {e}")))
}

/// Prompt for text input
pub fn prompt_text(prompt: &str, default: Option<&str>) -> Result<String> {
    let theme = ColorfulTheme::default();
    let mut input = Input::<String>::with_theme(&theme).with_prompt(prompt);

    if let Some(default_val) = default {
        input = input.default(default_val.to_string());
    }

    input
        .interact_text()
        .map_err(|e| MultiGitError::other(format!("Input failed: {e}")))
}

/// Prompt for repository name
pub fn prompt_repo_name(default: Option<&str>) -> Result<String> {
    let repo_name = if let Some(default_name) = default {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Repository name")
            .default(default_name.to_string())
            .interact_text()
    } else {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Repository name")
            .interact_text()
    }
    .map_err(|e| MultiGitError::other(format!("Repository name input failed: {e}")))?;

    if repo_name.trim().is_empty() {
        return Err(MultiGitError::other(
            "Repository name cannot be empty".to_string(),
        ));
    }

    // Validate repository name format
    if !is_valid_repo_name(&repo_name) {
        return Err(MultiGitError::other(
            "Invalid repository name. Use only alphanumeric characters, hyphens, and underscores"
                .to_string(),
        ));
    }

    Ok(repo_name.trim().to_string())
}

/// Prompt for repository description
pub fn prompt_repo_description() -> Result<Option<String>> {
    let description = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Repository description (optional)")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| MultiGitError::other(format!("Description input failed: {e}")))?;

    if description.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(description.trim().to_string()))
    }
}

/// Prompt for repository visibility
pub fn prompt_repo_visibility() -> Result<bool> {
    let choices = &["Public", "Private"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Repository visibility")
        .items(choices)
        .default(1) // Default to private
        .interact()
        .map_err(|e| MultiGitError::other(format!("Visibility selection failed: {e}")))?;

    Ok(selection == 1) // true if Private
}

/// Select conflict resolution strategy
pub fn select_resolution_strategy() -> Result<String> {
    let strategies = &[
        "Ours (keep local changes)",
        "Theirs (accept remote changes)",
        "Primary (use primary remote as source)",
        "Manual (resolve manually)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select conflict resolution strategy")
        .items(strategies)
        .default(3) // Default to manual
        .interact()
        .map_err(|e| MultiGitError::other(format!("Strategy selection failed: {e}")))?;

    let strategy = match selection {
        0 => "ours",
        1 => "theirs",
        2 => "primary",
        3 => "manual",
        _ => "manual",
    };

    Ok(strategy.to_string())
}

/// Select a remote from a list
pub fn select_remote(remotes: &[String]) -> Result<String> {
    if remotes.is_empty() {
        return Err(MultiGitError::other("No remotes available".to_string()));
    }

    if remotes.len() == 1 {
        return Ok(remotes[0].clone());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select remote")
        .items(remotes)
        .default(0)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Remote selection failed: {e}")))?;

    Ok(remotes[selection].clone())
}

/// Validate repository name format
fn is_valid_repo_name(name: &str) -> bool {
    // Repository names should contain only alphanumeric, hyphens, underscores, and dots
    // They should not start or end with special characters
    if name.is_empty() || name.len() > 100 {
        return false;
    }

    // Safe: we've already checked that name is not empty
    let first_char = match name.chars().next() {
        Some(c) => c,
        None => return false,
    };
    let last_char = match name.chars().last() {
        Some(c) => c,
        None => return false,
    };

    if !first_char.is_alphanumeric() || !last_char.is_alphanumeric() {
        return false;
    }

    name.chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// Print a warning message
pub fn print_warning(message: &str) {
    eprintln!("⚠️  {message}");
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("❌ {message}");
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("✅ {message}");
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("ℹ️  {message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_repo_names() {
        assert!(is_valid_repo_name("my-repo"));
        assert!(is_valid_repo_name("my_repo"));
        assert!(is_valid_repo_name("my.repo"));
        assert!(is_valid_repo_name("my-repo-123"));
        assert!(is_valid_repo_name("123-repo"));
    }

    #[test]
    fn test_invalid_repo_names() {
        assert!(!is_valid_repo_name(""));
        assert!(!is_valid_repo_name("-repo"));
        assert!(!is_valid_repo_name("repo-"));
        assert!(!is_valid_repo_name(".repo"));
        assert!(!is_valid_repo_name("repo."));
        assert!(!is_valid_repo_name("repo space"));
        assert!(!is_valid_repo_name("repo@name"));
    }
}
