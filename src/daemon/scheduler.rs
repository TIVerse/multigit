//! Task scheduler for periodic sync operations
//!
//! Provides interval-based scheduling for automatic sync operations.
//! Supports configurable intervals and graceful shutdown.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error, info};

/// Task scheduler for periodic operations
pub struct Scheduler {
    interval_seconds: u64,
    running: Arc<AtomicBool>,
}

impl Scheduler {
    /// Create a new scheduler with the given interval
    #[must_use]
    pub fn new(interval_seconds: u64) -> Self {
        Self {
            interval_seconds,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the scheduler and run the task periodically
    pub async fn start<F, Fut>(
        &self,
        task: F,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + 'static,
    {
        self.running.store(true, Ordering::SeqCst);
        info!(
            "Scheduler started with interval: {}s",
            self.interval_seconds
        );

        let mut ticker = interval(Duration::from_secs(self.interval_seconds));

        while self.running.load(Ordering::SeqCst) {
            ticker.tick().await;

            if !self.running.load(Ordering::SeqCst) {
                break;
            }

            debug!("Scheduler tick - executing task");

            match task().await {
                Ok(()) => {
                    info!("Scheduled task completed successfully");
                }
                Err(e) => {
                    error!("Scheduled task failed: {}", e);
                    // Continue running even if task fails
                }
            }
        }

        info!("Scheduler stopped");
        Ok(())
    }

    /// Stop the scheduler
    pub fn stop(&self) {
        info!("Stopping scheduler...");
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the scheduler is running
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get the interval in seconds
    #[must_use]
    pub fn interval_seconds(&self) -> u64 {
        self.interval_seconds
    }

    /// Get a handle to stop the scheduler
    #[must_use]
    pub fn stop_handle(&self) -> SchedulerHandle {
        SchedulerHandle {
            running: self.running.clone(),
        }
    }
}

/// Handle to control the scheduler from another thread
#[derive(Clone)]
pub struct SchedulerHandle {
    running: Arc<AtomicBool>,
}

impl SchedulerHandle {
    /// Stop the scheduler
    pub fn stop(&self) {
        info!("Scheduler stop requested via handle");
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if scheduler is running
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

/// Simple cron-like schedule parser (simplified version)
#[derive(Debug, Clone)]
pub struct Schedule {
    interval_seconds: u64,
}

impl Schedule {
    /// Create a schedule from a duration string (e.g., "5m", "1h", "30s")
    pub fn from_duration_str(s: &str) -> Result<Self, String> {
        let s = s.trim().to_lowercase();

        if s.ends_with('s') {
            let num = s
                .trim_end_matches('s')
                .parse::<u64>()
                .map_err(|e| format!("Invalid seconds: {e}"))?;
            Ok(Self {
                interval_seconds: num,
            })
        } else if s.ends_with('m') {
            let num = s
                .trim_end_matches('m')
                .parse::<u64>()
                .map_err(|e| format!("Invalid minutes: {e}"))?;
            Ok(Self {
                interval_seconds: num * 60,
            })
        } else if s.ends_with('h') {
            let num = s
                .trim_end_matches('h')
                .parse::<u64>()
                .map_err(|e| format!("Invalid hours: {e}"))?;
            Ok(Self {
                interval_seconds: num * 3600,
            })
        } else {
            // Default to seconds if no suffix
            let num = s
                .parse::<u64>()
                .map_err(|e| format!("Invalid number: {e}"))?;
            Ok(Self {
                interval_seconds: num,
            })
        }
    }

    /// Get the interval in seconds
    #[must_use]
    pub fn interval_seconds(&self) -> u64 {
        self.interval_seconds
    }

    /// Create a schedule for every N seconds
    #[must_use]
    pub fn every_seconds(seconds: u64) -> Self {
        Self {
            interval_seconds: seconds,
        }
    }

    /// Create a schedule for every N minutes
    #[must_use]
    pub fn every_minutes(minutes: u64) -> Self {
        Self {
            interval_seconds: minutes * 60,
        }
    }

    /// Create a schedule for every N hours
    #[must_use]
    pub fn every_hours(hours: u64) -> Self {
        Self {
            interval_seconds: hours * 3600,
        }
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::every_minutes(5) // Default: every 5 minutes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_parsing() {
        assert_eq!(
            Schedule::from_duration_str("30s")
                .unwrap()
                .interval_seconds(),
            30
        );
        assert_eq!(
            Schedule::from_duration_str("5m")
                .unwrap()
                .interval_seconds(),
            300
        );
        assert_eq!(
            Schedule::from_duration_str("2h")
                .unwrap()
                .interval_seconds(),
            7200
        );
        assert_eq!(
            Schedule::from_duration_str("120")
                .unwrap()
                .interval_seconds(),
            120
        );
    }

    #[test]
    fn test_schedule_creation() {
        assert_eq!(Schedule::every_seconds(45).interval_seconds(), 45);
        assert_eq!(Schedule::every_minutes(10).interval_seconds(), 600);
        assert_eq!(Schedule::every_hours(1).interval_seconds(), 3600);
    }

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new(300);
        assert_eq!(scheduler.interval_seconds(), 300);
        assert!(!scheduler.is_running());
    }

    #[test]
    fn test_scheduler_handle() {
        let scheduler = Scheduler::new(60);
        let handle = scheduler.stop_handle();
        assert!(!handle.is_running());
    }
}
