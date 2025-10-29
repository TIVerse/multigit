//! Doctor diagnostics command
//!
//! Run diagnostics and health checks.

use crate::core::health_checker::HealthChecker;
use crate::utils::error::Result;
use tracing::info;

/// Run diagnostics and health checks
pub fn execute(fix: bool) -> Result<()> {
    info!("Running diagnostics");

    println!("\n🔍 Running MultiGit Doctor...\n");

    let checker = HealthChecker::new(".")?;
    let report = checker.check();

    // Display repository status
    if report.repo_valid {
        println!("✓ Repository: OK");
    } else {
        println!("✗ Repository: FAILED");
    }

    if report.working_dir_clean {
        println!("✓ Working directory: clean");
    } else {
        println!("⚠ Working directory: has uncommitted changes");
    }

    if let Some(ref branch) = report.current_branch {
        println!("✓ Current branch: {}", branch);
    } else {
        println!("⚠ Current branch: DETACHED HEAD");
    }

    // Display remote status
    println!("\nRemotes:");
    if report.remotes.is_empty() {
        println!("  ⚠ No remotes configured");
    } else {
        for remote in &report.remotes {
            let status = if remote.reachable { "✓" } else { "✗" };
            println!("  {} {} ({})", status, remote.name, remote.url);
        }
    }

    // Display issues
    if !report.issues.is_empty() {
        println!("\n⚠ Issues found:");
        for issue in &report.issues {
            println!("  • {}", issue);
        }
    }

    // Display recommendations
    if !report.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for rec in &report.recommendations {
            println!("  • {}", rec);
        }
    }

    if report.issues.is_empty() {
        println!("\n✓ Everything looks good!");
    } else if fix {
        println!("\n⚠ Auto-fix is not yet implemented.");
        println!("Please follow the recommendations above.");
    }

    Ok(())
}
