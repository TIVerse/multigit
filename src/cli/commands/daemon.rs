//! Daemon management commands
//!
//! Start, stop, status, and logs for the background daemon service.

use crate::daemon::service::DaemonService;
use crate::ui::formatter::{colors, Status};
use crate::utils::error::{MultiGitError, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use tracing::{info, warn};

/// Start the daemon
pub async fn start(interval_minutes: u64) -> Result<()> {
    info!("Starting daemon with interval: {}m", interval_minutes);

    let interval_seconds = interval_minutes * 60;
    let daemon = DaemonService::new(interval_seconds);

    // Check if already running
    if daemon.is_running()? {
        println!(
            "\n{}",
            Status::Warning.format("Daemon is already running", true)
        );

        let status = daemon.status()?;
        if let Some(pid) = status.pid {
            println!("  PID: {}", colors::info(&pid.to_string(), true));
        }

        println!("\nUse 'multigit daemon stop' to stop it first.");
        return Ok(());
    }

    println!("\n🚀 Starting MultiGit daemon...");
    println!("  Interval: {}m ({}s)", interval_minutes, interval_seconds);

    // In a real implementation, we'd fork/daemonize here
    // For now, we'll run in foreground (user can use nohup/systemd)
    println!(
        "\n{}",
        Status::Info.format("Running in foreground (use Ctrl+C to stop)", true)
    );
    println!(
        "{}",
        Status::Info.format(
            "For background mode, use: nohup multigit daemon start &",
            true
        )
    );
    println!();

    // Start the daemon
    daemon.start().await?;

    Ok(())
}

/// Stop the daemon
pub fn stop() -> Result<()> {
    info!("Stopping daemon");

    let daemon = DaemonService::new(300); // Interval doesn't matter for stop

    // Check if running
    if !daemon.is_running()? {
        println!(
            "\n{}",
            Status::Warning.format("Daemon is not running", true)
        );
        return Ok(());
    }

    println!("\n🛑 Stopping MultiGit daemon...");

    daemon.stop()?;

    println!(
        "\n{}",
        Status::Success.format("Daemon stopped successfully", true)
    );

    Ok(())
}

/// Show daemon status
pub fn status() -> Result<()> {
    info!("Checking daemon status");

    let daemon = DaemonService::new(300);
    let status = daemon.status()?;

    println!("\n📊 Daemon Status:\n");

    if status.running {
        println!("  Status: {}", colors::success("✓ Running", true));

        if let Some(pid) = status.pid {
            println!("  PID: {}", colors::info(&pid.to_string(), true));
        }

        if let Some(log_file) = status.log_file {
            println!("  Log file: {}", log_file.display());
        }
    } else {
        println!("  Status: {}", colors::warning("✗ Not running", true));
        println!(
            "\n  Start with: {}",
            colors::dim("multigit daemon start", true)
        );
    }

    println!();

    Ok(())
}

/// Show daemon logs
pub fn logs(lines: usize) -> Result<()> {
    info!("Reading daemon logs (last {} lines)", lines);

    let daemon = DaemonService::new(300);
    let status = daemon.status()?;

    let log_file = status
        .log_file
        .ok_or_else(|| MultiGitError::other("Log file path not configured"))?;

    if !log_file.exists() {
        println!(
            "\n{}",
            Status::Warning.format("Log file does not exist", true)
        );
        println!("  Expected: {}", log_file.display());
        return Ok(());
    }

    println!("\n📜 Daemon Logs (last {} lines):\n", lines);
    println!("{}", "=".repeat(60));

    // Read last N lines
    let file = fs::File::open(&log_file)
        .map_err(|e| MultiGitError::other(format!("Failed to open log file: {}", e)))?;

    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let start = if all_lines.len() > lines {
        all_lines.len() - lines
    } else {
        0
    };

    for line in &all_lines[start..] {
        println!("{}", line);
    }

    println!("{}", "=".repeat(60));
    println!("\nLog file: {}", log_file.display());
    println!();

    Ok(())
}

/// Restart the daemon
pub async fn restart(interval_minutes: u64) -> Result<()> {
    info!("Restarting daemon");

    println!("\n🔄 Restarting MultiGit daemon...\n");

    // Stop if running
    let daemon = DaemonService::new(300);
    if daemon.is_running()? {
        println!("  Stopping current daemon...");
        daemon.stop()?;
        println!("  {}", Status::Success.format("Stopped", true));
    }

    // Small delay
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Start new daemon
    println!("  Starting daemon...");
    start(interval_minutes).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_status_check() {
        // Just test that status doesn't panic
        let result = status();
        // It's ok if it errors (daemon not running)
        let _ = result;
    }
}
