//! Background daemon for automated synchronization
//!
//! Provides daemon service for background sync operations with scheduling.

pub mod scheduler;
pub mod service;

// Re-export key types
pub use scheduler::{Schedule, Scheduler, SchedulerHandle};
pub use service::{DaemonService, DaemonStatus};
