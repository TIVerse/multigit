//! Setup wizard for easy MultiGit configuration
//!
//! Provides an interactive, user-friendly setup experience.

use crate::cli::interactive;
use crate::core::auth::{AuthBackend, AuthManager};
use crate::core::config::{Config, RemoteConfig};
use crate::providers::github::GitHubProvider;
use crate::providers::gitlab::GitLabProvider;
use crate::providers::traits::Provider;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};
use std::sync::Arc;

/// Popular Git hosting providers
const PROVIDERS: &[(&str, &str)] = &[
    ("GitHub", "github"),
    ("GitLab", "gitlab"),
    ("Bitbucket", "bitbucket"),
    ("Codeberg", "codeberg"),
    ("Gitea (self-hosted)", "gitea"),
];

/// Run the interactive setup wizard
pub async fn run_wizard() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║                                              ║");
    println!("║     🚀 Welcome to MultiGit Setup Wizard     ║");
    println!("║                                              ║");
    println!("╚══════════════════════════════════════════════╝\n");

    println!("This wizard will help you set up MultiGit in 3 easy steps:\n");
    println!("  1️⃣  Initialize MultiGit");
    println!("  2️⃣  Add your Git hosting providers");
    println!("  3️⃣  Configure your preferences\n");

    let should_continue = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Ready to start?")
        .default(true)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Setup cancelled: {e}")))?;

    if !should_continue {
        println!("\n👋 Setup cancelled. Run 'multigit setup' anytime to try again.");
        return Ok(());
    }

    // Step 1: Initialize
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Step 1: Initialize MultiGit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if Config::is_initialized() {
        println!("✅ MultiGit is already initialized in this repository.");
    } else {
        Config::initialize()?;
        println!("✅ MultiGit initialized successfully!");
    }

    // Step 2: Add providers
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔗 Step 2: Add Git Hosting Providers");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Select which providers you want to use:");
    println!("(Use Space to select, Enter to confirm)\n");

    let provider_names: Vec<&str> = PROVIDERS.iter().map(|(name, _)| *name).collect();
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select providers")
        .items(&provider_names)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Selection failed: {e}")))?;

    if selections.is_empty() {
        println!("\n⚠️  No providers selected. You can add them later with:");
        println!("   multigit remote add <provider> <username>");
        return Ok(());
    }

    let mut config = Config::load()?;

    for &idx in &selections {
        let (provider_display, provider_id) = PROVIDERS[idx];
        
        println!("\n╭──────────────────────────────────────────╮");
        println!("│  Setting up: {}                    ", provider_display);
        println!("╰──────────────────────────────────────────╯\n");

        // Add provider with guided setup
        if let Err(e) = add_provider_guided(&mut config, provider_id).await {
            println!("⚠️  Failed to set up {}: {}", provider_display, e);
            println!("   You can try again later with: multigit remote add {} <username>", provider_id);
        }
    }

    config.save()?;

    // Step 3: Configure preferences
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("⚙️  Step 3: Configure Preferences (Optional)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let configure_advanced = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Configure advanced settings?")
        .default(false)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Prompt failed: {e}")))?;

    if configure_advanced {
        configure_preferences(&mut config)?;
        config.save()?;
    } else {
        println!("✅ Using default settings (you can change these anytime)");
    }

    // Done!
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║                                              ║");
    println!("║     🎉 Setup Complete! You're ready!         ║");
    println!("║                                              ║");
    println!("╚══════════════════════════════════════════════╝\n");

    println!("📚 Next Steps:\n");
    println!("   1. Check your configuration:");
    println!("      multigit status\n");
    println!("   2. Test your connections:");
    println!("      multigit remote test --all\n");
    println!("   3. Push to all remotes:");
    println!("      multigit push\n");
    println!("   4. Start background sync:");
    println!("      multigit daemon start --interval 15m\n");

    println!("💡 Need help? Run: multigit --help");
    println!("📖 Full docs: https://github.com/TIVerse/multigit\n");

    Ok(())
}

/// Add a provider with guided setup
async fn add_provider_guided(config: &mut Config, provider: &str) -> Result<()> {
    // Get username
    let username = interactive::prompt_text(
        &format!("Enter your {} username", provider),
        None,
    )?;

    // Show token instructions
    show_token_instructions(provider);

    // Get token
    let token = interactive::prompt_token(provider)?;

    // Get API URL for self-hosted (if applicable)
    let api_url = if provider == "gitea" {
        Some(interactive::prompt_text(
            "Enter your Gitea instance URL (e.g., https://git.example.com)",
            None,
        )?)
    } else if provider == "gitlab" {
        let use_custom = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Use self-hosted GitLab?")
            .default(false)
            .interact()
            .map_err(|e| MultiGitError::other(format!("Prompt failed: {e}")))?;
        
        if use_custom {
            Some(interactive::prompt_text(
                "Enter your GitLab instance URL",
                Some("https://gitlab.com"),
            )?)
        } else {
            None
        }
    } else {
        None
    };

    // Test connection
    println!("\n🔍 Testing connection...");

    let test_provider = create_test_provider(provider, &username, &token, api_url.as_deref())?;
    
    match test_provider.test_connection().await {
        Ok(true) => {
            println!("✅ Connection successful!");
        }
        Ok(false) | Err(_) => {
            return Err(MultiGitError::auth(
                provider,
                "Connection failed. Please check your token and try again.",
            ));
        }
    }

    // Store credentials
    let auth_manager = AuthManager::new(AuthBackend::Keyring, false);
    auth_manager.store_credential(provider, &username, &token)?;
    println!("✅ Credentials stored securely");

    // Add to config
    let remote_config = RemoteConfig {
        username: username.clone(),
        api_url,
        enabled: true,
        provider: Some(provider.to_string()),
        use_ssh: false,
        priority: 0,
    };

    config.add_remote(provider.to_string(), remote_config);
    println!("✅ {} added to configuration", provider);

    Ok(())
}

/// Show token instructions for a provider
fn show_token_instructions(provider: &str) {
    println!("\n📝 How to get your {} token:\n", provider);

    match provider {
        "github" => {
            println!("   1. Go to: https://github.com/settings/tokens");
            println!("   2. Click 'Generate new token (classic)'");
            println!("   3. Select scopes: repo, read:user");
            println!("   4. Click 'Generate token' and copy it");
        }
        "gitlab" => {
            println!("   1. Go to: https://gitlab.com/-/profile/personal_access_tokens");
            println!("   2. Click 'Add new token'");
            println!("   3. Select scopes: api, write_repository");
            println!("   4. Click 'Create personal access token' and copy it");
        }
        "bitbucket" => {
            println!("   1. Go to: https://bitbucket.org/account/settings/app-passwords/");
            println!("   2. Click 'Create app password'");
            println!("   3. Select: Repositories (Read, Write)");
            println!("   4. Click 'Create' and copy the password");
        }
        "codeberg" => {
            println!("   1. Go to: https://codeberg.org/user/settings/applications");
            println!("   2. Click 'Generate new token'");
            println!("   3. Select scopes: write:repository, read:user");
            println!("   4. Generate and copy the token");
        }
        "gitea" => {
            println!("   1. Go to your Gitea instance settings");
            println!("   2. Navigate to Applications");
            println!("   3. Generate a new token with repository permissions");
            println!("   4. Copy the generated token");
        }
        _ => {
            println!("   Generate a personal access token with repository permissions");
        }
    }

    println!("\n🔒 Your token will be stored securely in your OS keyring.");
    println!("   It will NEVER be stored in plain text.\n");
}

/// Create a test provider instance
fn create_test_provider(
    provider: &str,
    username: &str,
    token: &str,
    api_url: Option<&str>,
) -> Result<Arc<dyn Provider>> {
    let provider_instance: Arc<dyn Provider> = match provider {
        "github" => Arc::new(GitHubProvider::new(token.to_string(), username.to_string())?),
        "gitlab" => {
            let url = api_url.map(std::string::ToString::to_string);
            Arc::new(GitLabProvider::new(token.to_string(), username.to_string(), url)?)
        }
        _ => return Err(MultiGitError::other(format!("Provider {} not yet supported in wizard", provider))),
    };

    Ok(provider_instance)
}

/// Configure advanced preferences
fn configure_preferences(config: &mut Config) -> Result<()> {
    println!("\n⚙️  Advanced Settings:\n");

    // Parallel operations
    let parallel = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable parallel push/fetch? (recommended)")
        .default(true)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Prompt failed: {e}")))?;

    config.settings.parallel_push = parallel;

    if parallel {
        let parallel_count = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Max parallel operations")
            .items(&["2", "4 (recommended)", "8", "16"])
            .default(1)
            .interact()
            .map_err(|e| MultiGitError::other(format!("Selection failed: {e}")))?;

        config.settings.max_parallel = match parallel_count {
            0 => 2,
            1 => 4,
            2 => 8,
            3 => 16,
            _ => 4,
        };
    }

    // Colored output
    let colored = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable colored output?")
        .default(true)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Prompt failed: {e}")))?;

    config.settings.colored_output = colored;

    // Conflict detection
    let detect_conflicts = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable automatic conflict detection?")
        .default(true)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Prompt failed: {e}")))?;

    config.sync.detect_conflicts = detect_conflicts;

    println!("\n✅ Preferences configured!");

    Ok(())
}

/// Quick setup for a single provider (simpler flow)
pub async fn quick_setup(provider: &str, _username: String) -> Result<()> {
    println!("\n🚀 Quick Setup for {}\n", provider);

    let mut config = if Config::is_initialized() {
        Config::load()?
    } else {
        println!("Initializing MultiGit...");
        Config::initialize()?;
        Config::load()?
    };

    add_provider_guided(&mut config, provider).await?;
    config.save()?;

    println!("\n✅ {} setup complete!", provider);
    println!("💡 Run 'multigit status' to see your configuration");

    Ok(())
}
