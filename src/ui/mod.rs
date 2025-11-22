//! User interface components
//!
//! Provides progress bars, output formatting, and terminal UI components.

pub mod conflict_resolver;
pub mod formatter;
pub mod progress;
pub mod sync_monitor;
pub mod tui;

// Re-export commonly used items
pub use conflict_resolver::ConflictResolver;
pub use formatter::{colors, OutputFormat, Status, Table};
pub use progress::{MultiRemoteProgress, ProgressCounter, Spinner};
pub use sync_monitor::SyncMonitor;
pub use tui::{start_dashboard, App, Theme};
