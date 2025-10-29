//! Basic usage example for MultiGit
//!
//! This example demonstrates the most common workflow:
//! 1. Initialize MultiGit in a repository
//! 2. Configure remotes
//! 3. Push to multiple remotes

use multigit::core::config::{Config, RemoteConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MultiGit Basic Usage Example\n");

    // 1. Create a new configuration
    let mut config = Config::default();

    println!("✓ Created default configuration");

    // 2. Add GitHub remote
    config.add_remote(
        "github".to_string(),
        RemoteConfig {
            username: "myusername".to_string(),
            api_url: None,
            enabled: true,
            provider: Some("github".to_string()),
            use_ssh: false,
            priority: 0,
        },
    );

    println!("✓ Added GitHub remote");

    // 3. Add GitLab remote
    config.add_remote(
        "gitlab".to_string(),
        RemoteConfig {
            username: "myusername".to_string(),
            api_url: Some("https://gitlab.com".to_string()),
            enabled: true,
            provider: Some("gitlab".to_string()),
            use_ssh: false,
            priority: 1,
        },
    );

    println!("✓ Added GitLab remote");

    // 4. List all remotes
    println!("\nConfigured remotes:");
    for (name, remote) in &config.remotes {
        println!("  - {} (@{})", name, remote.username);
    }

    // 5. Get only enabled remotes
    let enabled = config.enabled_remotes();
    println!("\nEnabled remotes: {}", enabled.len());

    // 6. Access specific remote
    if let Some(github) = config.get_remote("github") {
        println!("\nGitHub remote details:");
        println!("  Username: {}", github.username);
        println!("  Enabled: {}", github.enabled);
        println!("  Priority: {}", github.priority);
    }

    println!("\n✅ Configuration complete!");
    println!("\nNext steps:");
    println!("  1. Save this config with: config.save()?");
    println!("  2. Run: multigit sync");

    Ok(())
}
