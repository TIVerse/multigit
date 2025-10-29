//! Conflict management commands
//!
//! Handles detection, resolution, and management of conflicts across multiple remotes.
//! Provides interactive conflict resolution with multiple strategies.

use crate::cli::interactive;
use crate::core::config::Config;
use crate::core::conflict_resolver::{Conflict, ConflictResolver, Resolution, ResolutionStrategy};
use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use std::path::Path;
use tracing::{debug, info, warn};

/// Detect conflicts across all configured remotes
pub async fn detect_conflicts() -> Result<()> {
    info!("Detecting conflicts across remotes");

    let config = Config::load()?;

    if config.remotes.is_empty() {
        interactive::print_info("No remotes configured");
        return Ok(());
    }

    // Get current repository
    let repo_path = Path::new(".");
    let git_ops = GitOperations::new(repo_path)?;

    // Get current branch
    let current_branch = git_ops.get_current_branch()?;
    info!("Checking branch: {}", current_branch);

    println!(
        "\n🔍 Checking for conflicts on branch '{}'...\n",
        current_branch
    );

    let mut conflicts_found = false;
    let mut remote_states = Vec::new();

    // Check each remote
    for (remote_name, remote_config) in &config.remotes {
        if !remote_config.enabled {
            debug!("Skipping disabled remote: {}", remote_name);
            continue;
        }

        // Check if remote exists in git config
        match git_ops.get_remote_url(remote_name) {
            Ok(_url) => {
                // Compare local with remote
                match git_ops.compare_with_remote(remote_name, &current_branch) {
                    Ok((ahead, behind)) => {
                        let status = if ahead == 0 && behind == 0 {
                            "✓ In sync"
                        } else if ahead > 0 && behind > 0 {
                            conflicts_found = true;
                            "⚠ CONFLICT: Diverged"
                        } else if ahead > 0 {
                            "↑ Ahead"
                        } else {
                            "↓ Behind"
                        };

                        println!("  {} {}", status, remote_name);

                        if ahead > 0 || behind > 0 {
                            println!("      Local: {} commits ahead", ahead);
                            println!("      Remote: {} commits ahead", behind);

                            if ahead > 0 && behind > 0 {
                                println!(
                                    "      ⚠️  Branches have diverged - manual resolution required"
                                );
                            }
                            println!();
                        }

                        remote_states.push(RemoteState {
                            name: remote_name.clone(),
                            ahead,
                            behind,
                            diverged: ahead > 0 && behind > 0,
                        });
                    }
                    Err(e) => {
                        warn!("Could not compare with remote {}: {}", remote_name, e);
                        println!("  ✗ {} (error: {})", remote_name, e);
                    }
                }
            }
            Err(_) => {
                interactive::print_warning(&format!(
                    "Remote '{}' not configured in git. Run 'git remote add {} <url>'",
                    remote_name, remote_name
                ));
            }
        }
    }

    println!();

    if conflicts_found {
        interactive::print_warning(
            "Conflicts detected! Run 'multigit conflict resolve' to resolve them.",
        );
        return Err(MultiGitError::conflict(
            "Conflicts detected across remotes".to_string(),
        ));
    } else {
        interactive::print_success("No conflicts detected");
    }

    Ok(())
}

/// Resolve conflicts interactively
pub async fn resolve_conflicts(strategy: Option<String>) -> Result<()> {
    info!("Resolving conflicts");

    let config = Config::load()?;

    if config.remotes.is_empty() {
        interactive::print_info("No remotes configured");
        return Ok(());
    }

    // Get repository
    let repo_path = Path::new(".");
    let git_ops = GitOperations::new(repo_path)?;
    let current_branch = git_ops.get_current_branch()?;

    println!(
        "\n🔧 Resolving conflicts on branch '{}'...\n",
        current_branch
    );

    // Detect conflicts first
    let mut conflicts = Vec::new();

    for (remote_name, remote_config) in &config.remotes {
        if !remote_config.enabled {
            continue;
        }

        if let Ok((ahead, behind)) = git_ops.compare_with_remote(remote_name, &current_branch) {
            if ahead > 0 && behind > 0 {
                conflicts.push(Conflict::new(current_branch.clone(), ahead, behind));
                println!("  ⚠ Conflict detected with '{}'", remote_name);
                println!("      {} commits ahead, {} commits behind", ahead, behind);
            }
        }
    }

    if conflicts.is_empty() {
        interactive::print_success("No conflicts to resolve");
        return Ok(());
    }

    println!();

    // Determine resolution strategy
    let resolution_strategy = if let Some(strat) = strategy {
        parse_strategy(&strat)?
    } else {
        // Interactive strategy selection
        let strategy_str = interactive::select_resolution_strategy()?;
        parse_strategy(&strategy_str)?
    };

    let resolver = ConflictResolver::new(resolution_strategy);

    // Apply resolution
    for conflict in &conflicts {
        let resolution = resolver.resolve(conflict)?;

        match resolution {
            Resolution::NoAction => {
                interactive::print_info("No action needed - branches are in sync");
            }
            Resolution::Push => {
                interactive::print_info("Resolution: Push local changes to remotes");
                println!("  Run: multigit push");
            }
            Resolution::Pull => {
                interactive::print_info("Resolution: Pull remote changes to local");
                println!("  Run: multigit pull");
            }
            Resolution::ForcePush => {
                interactive::print_warning(
                    "Resolution: Force push (will overwrite remote changes)",
                );

                if interactive::confirm(
                    "Are you sure you want to force push? This will overwrite remote changes.",
                )? {
                    println!("  Run: multigit push --force");
                } else {
                    interactive::print_info("Force push cancelled");
                }
            }
            Resolution::RequiresManual => {
                interactive::print_warning("Manual resolution required");
                println!("\nBranches have diverged. You need to manually resolve this:");
                println!("  1. Fetch changes: git fetch --all");
                println!("  2. Review changes: git log --oneline --graph --all");
                println!("  3. Choose one approach:");
                println!("     a) Merge: git merge <remote>/<branch>");
                println!("     b) Rebase: git rebase <remote>/<branch>");
                println!(
                    "     c) Reset: git reset --hard <remote>/<branch> (discards local changes)"
                );
                println!("  4. After resolving, run: multigit sync");
            }
        }
    }

    Ok(())
}

/// Set primary remote for conflict resolution
pub async fn set_primary_remote(remote_name: String) -> Result<()> {
    info!("Setting primary remote: {}", remote_name);

    let mut config = Config::load()?;

    let remote_lower = remote_name.to_lowercase();

    // Check if remote exists
    if !config.remotes.contains_key(&remote_lower) {
        return Err(MultiGitError::other(format!(
            "Remote '{}' not found. Available remotes: {}",
            remote_name,
            config
                .remotes
                .keys()
                .map(|k| k.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    // Update primary source in sync config
    config.sync.primary_source = Some(remote_lower.clone());
    config.save()?;

    interactive::print_success(&format!(
        "Primary remote set to '{}' for conflict resolution",
        remote_lower
    ));

    println!(
        "\nWhen conflicts occur, changes from '{}' will be preferred.",
        remote_lower
    );

    Ok(())
}

/// Show current conflict resolution strategy
pub async fn show_strategy() -> Result<()> {
    let config = Config::load()?;

    println!("\n🔧 Current Conflict Resolution Configuration:\n");

    let strategy = config.sync.strategy.to_string();
    println!("  Strategy: {}", strategy);

    if let Some(primary) = &config.sync.primary_source {
        println!("  Primary Source: {}", primary);
    } else {
        println!("  Primary Source: Not set");
    }

    println!("\nAvailable strategies:");
    println!("  - fast-forward: Only allow fast-forward merges (safest)");
    println!("  - merge: Merge diverged branches");
    println!("  - rebase: Rebase local changes onto remote");
    println!("  - primary: Always prefer changes from primary remote");
    println!("  - manual: Always require manual resolution");

    println!("\nChange strategy in config file: ~/.config/multigit/config.toml");
    println!();

    Ok(())
}

/// Parse strategy string into ResolutionStrategy enum
fn parse_strategy(strategy: &str) -> Result<ResolutionStrategy> {
    match strategy.to_lowercase().as_str() {
        "ours" | "local" => Ok(ResolutionStrategy::FastForwardOnly),
        "theirs" | "remote" => Ok(ResolutionStrategy::PreferRemote),
        "primary" => Ok(ResolutionStrategy::PreferRemote),
        "manual" => Ok(ResolutionStrategy::Manual),
        "force" => Ok(ResolutionStrategy::Force),
        "fast-forward" => Ok(ResolutionStrategy::FastForwardOnly),
        _ => Err(MultiGitError::other(format!(
            "Invalid strategy '{}'. Valid strategies: ours, theirs, primary, manual, force",
            strategy
        ))),
    }
}

/// Remote state for conflict detection
#[derive(Debug, Clone)]
struct RemoteState {
    name: String,
    ahead: usize,
    behind: usize,
    diverged: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_strategy() {
        assert!(matches!(
            parse_strategy("ours").unwrap(),
            ResolutionStrategy::FastForwardOnly
        ));
        assert!(matches!(
            parse_strategy("theirs").unwrap(),
            ResolutionStrategy::PreferRemote
        ));
        assert!(matches!(
            parse_strategy("manual").unwrap(),
            ResolutionStrategy::Manual
        ));
        assert!(matches!(
            parse_strategy("force").unwrap(),
            ResolutionStrategy::Force
        ));
        assert!(parse_strategy("invalid").is_err());
    }
}
