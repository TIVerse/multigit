//! Daemon service implementation
//!
//! Background daemon process for automatic sync operations.
//! Handles PID files, signal handling, and graceful shutdown.

use crate::core::config::Config;
use crate::daemon::scheduler::Scheduler;
use crate::utils::error::{MultiGitError, Result};
use std::fs;
use std::path::PathBuf;
use std::process;
use tokio::signal;
use tracing::{debug, error, info, warn};

/// Daemon service for background operations
pub struct DaemonService {
    pid_file: PathBuf,
    log_file: Option<PathBuf>,
    interval_seconds: u64,
}

impl DaemonService {
    /// Create a new daemon service
    #[must_use]
    pub fn new(interval_seconds: u64) -> Self {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from(".multigit"));

        let pid_file = config_dir.join("multigit").join("daemon.pid");
        let log_file = Some(config_dir.join("multigit").join("daemon.log"));

        Self {
            pid_file,
            log_file,
            interval_seconds,
        }
    }

    /// Start the daemon service
    pub async fn start(&self) -> Result<()> {
        // Check if daemon is already running
        if self.is_running()? {
            return Err(MultiGitError::daemon(
                "Daemon is already running".to_string(),
            ));
        }

        info!("Starting MultiGit daemon service...");
        info!("Sync interval: {}s", self.interval_seconds);

        // Write PID file
        self.write_pid_file()?;
        info!("PID file created: {}", self.pid_file.display());

        // Setup signal handlers
        let pid_file_clone = self.pid_file.clone();
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(()) => {
                    info!("Received SIGINT, shutting down daemon...");
                    let _ = fs::remove_file(&pid_file_clone);
                    process::exit(0);
                }
                Err(err) => {
                    error!("Error setting up signal handler: {}", err);
                }
            }
        });

        // Create scheduler
        let scheduler = Scheduler::new(self.interval_seconds);

        // Define the sync task
        let sync_task = || async move {
            info!("[Daemon] Running scheduled sync...");
            perform_sync().await
        };

        // Run the scheduler
        match scheduler.start(sync_task).await {
            Ok(()) => {
                info!("Daemon scheduler stopped normally");
            }
            Err(e) => {
                error!("Daemon scheduler error: {}", e);
            }
        }

        // Cleanup
        self.cleanup()?;

        Ok(())
    }

    /// Stop the daemon service
    pub fn stop(&self) -> Result<()> {
        info!("Stopping daemon service...");

        if !self.is_running()? {
            return Err(MultiGitError::daemon("Daemon is not running".to_string()));
        }

        let pid = self.read_pid()?;
        info!("Found daemon PID: {}", pid);

        // Send SIGTERM to the process
        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid;

            let pid = Pid::from_raw(pid as i32);
            match kill(pid, Signal::SIGTERM) {
                Ok(()) => {
                    info!("Sent SIGTERM to daemon process");
                    // Wait a bit for graceful shutdown
                    std::thread::sleep(std::time::Duration::from_secs(2));

                    // Check if still running
                    if self.is_running()? {
                        warn!("Daemon did not stop gracefully, sending SIGKILL");
                        if let Err(e) = kill(pid, Signal::SIGKILL) {
                            return Err(MultiGitError::daemon(format!(
                                "Failed to kill daemon: {e}"
                            )));
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to send signal: {}", e);
                    return Err(MultiGitError::daemon(format!("Failed to stop daemon: {e}")));
                }
            }
        }

        #[cfg(not(unix))]
        {
            warn!("Daemon stop not fully implemented on this platform");
            // On Windows, we'd use a different approach
        }

        // Remove PID file
        self.cleanup()?;

        info!("Daemon stopped successfully");
        Ok(())
    }

    /// Check if daemon is running
    pub fn is_running(&self) -> Result<bool> {
        if !self.pid_file.exists() {
            return Ok(false);
        }

        let pid = self.read_pid()?;

        // Check if process with this PID exists
        #[cfg(unix)]
        {
            use nix::sys::signal::kill;
            use nix::unistd::Pid;

            let pid = Pid::from_raw(pid as i32);
            // Use kill with NULL signal (0) to check if process exists
            match kill(pid, None) {
                Ok(()) => Ok(true),
                Err(nix::errno::Errno::ESRCH) => {
                    // Process doesn't exist, clean up stale PID file
                    let _ = fs::remove_file(&self.pid_file);
                    Ok(false)
                }
                Err(_) => Ok(false),
            }
        }

        #[cfg(not(unix))]
        {
            // On Windows, just check if PID file exists
            Ok(true)
        }
    }

    /// Get daemon status
    pub fn status(&self) -> Result<DaemonStatus> {
        let running = self.is_running()?;

        if running {
            let pid = self.read_pid()?;
            Ok(DaemonStatus {
                running: true,
                pid: Some(pid),
                log_file: self.log_file.clone(),
            })
        } else {
            Ok(DaemonStatus {
                running: false,
                pid: None,
                log_file: self.log_file.clone(),
            })
        }
    }

    /// Write PID file
    fn write_pid_file(&self) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.pid_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let pid = process::id();
        fs::write(&self.pid_file, pid.to_string())?;

        Ok(())
    }

    /// Read PID from file
    fn read_pid(&self) -> Result<u32> {
        let content = fs::read_to_string(&self.pid_file)
            .map_err(|e| MultiGitError::daemon(format!("Failed to read PID file: {e}")))?;

        content
            .trim()
            .parse::<u32>()
            .map_err(|e| MultiGitError::daemon(format!("Invalid PID in file: {e}")))
    }

    /// Cleanup daemon files
    fn cleanup(&self) -> Result<()> {
        if self.pid_file.exists() {
            fs::remove_file(&self.pid_file)?;
            info!("PID file removed");
        }
        Ok(())
    }
}

/// Daemon status information
#[derive(Debug, Clone)]
pub struct DaemonStatus {
    /// Whether the daemon is running
    pub running: bool,
    /// Process ID of the daemon
    pub pid: Option<u32>,
    /// Path to the log file
    pub log_file: Option<PathBuf>,
}

/// Perform a sync operation - logs sync information
///
/// Note: Full sync implementation in daemon mode requires Send trait bounds
/// on libgit2 Repository type, which is not available in v1.0.
/// For now, this logs what would be synced. Use `multigit sync` manually
/// or configure a cron job to run the CLI command for actual syncing.
#[allow(clippy::unused_async)]
async fn perform_sync() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Performing background sync check...");

    // Load config
    let config =
        Config::load().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    // Get enabled remotes
    let enabled: Vec<String> = config.enabled_remotes().keys().cloned().collect();
    if enabled.is_empty() {
        info!("[Daemon] No remotes configured");
        return Ok(());
    }

    info!(
        "[Daemon] Would sync with {} enabled remotes: {:?}",
        enabled.len(),
        enabled
    );
    info!("[Daemon] To enable full sync in daemon mode, run: multigit sync");
    info!("[Daemon] Or set up a cron job: */5 * * * * cd /path/to/repo && multigit sync");

    Ok(())
}

impl Drop for DaemonService {
    fn drop(&mut self) {
        // Cleanup on drop
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_creation() {
        let daemon = DaemonService::new(300);
        assert!(daemon.pid_file.to_string_lossy().contains("daemon.pid"));
    }

    #[test]
    fn test_daemon_status_not_running() {
        let daemon = DaemonService::new(300);
        let status = daemon.status().unwrap();
        assert!(!status.running);
        assert!(status.pid.is_none());
    }
}
