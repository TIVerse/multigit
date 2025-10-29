//! Progress indicators and bars
//!
//! Provides multi-progress bar support for parallel operations using indicatif.
//! Displays real-time progress for push/pull/sync operations across multiple remotes.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::time::Duration;

/// Progress tracker for multi-remote operations
pub struct MultiRemoteProgress {
    multi: Arc<MultiProgress>,
    bars: Vec<ProgressBar>,
}

impl MultiRemoteProgress {
    /// Create a new multi-remote progress tracker
    pub fn new(remote_names: &[String]) -> Self {
        let multi = Arc::new(MultiProgress::new());
        let mut bars = Vec::new();

        for remote_name in remote_names {
            let pb = multi.add(ProgressBar::new(100));
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{prefix:>12} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("█▓▒░ "),
            );
            pb.set_prefix(remote_name.clone());
            pb.set_message("Initializing...");
            bars.push(pb);
        }

        Self { multi, bars }
    }

    /// Update progress for a specific remote
    pub fn update(&self, index: usize, pos: u64, total: u64, message: &str) {
        if let Some(pb) = self.bars.get(index) {
            pb.set_length(total);
            pb.set_position(pos);
            pb.set_message(message.to_string());
        }
    }

    /// Mark a remote as complete
    pub fn finish(&self, index: usize, message: &str) {
        if let Some(pb) = self.bars.get(index) {
            pb.finish_with_message(message.to_string());
        }
    }

    /// Mark a remote as failed
    pub fn error(&self, index: usize, message: &str) {
        if let Some(pb) = self.bars.get(index) {
            pb.abandon_with_message(message.to_string());
        }
    }

    /// Get the underlying MultiProgress for custom additions
    pub fn multi(&self) -> Arc<MultiProgress> {
        self.multi.clone()
    }
}

/// Simple progress spinner for single operations
pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    /// Create a new spinner with a message
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self { pb }
    }

    /// Update the spinner message
    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    /// Finish the spinner with a success message
    pub fn finish_with_message(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }

    /// Finish the spinner and clear it
    pub fn finish_and_clear(&self) {
        self.pb.finish_and_clear();
    }
}

/// Progress bar for a single operation with known size
pub struct ProgressCounter {
    pb: ProgressBar,
}

impl ProgressCounter {
    /// Create a new progress counter
    pub fn new(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {per_sec}")
                .unwrap()
                .progress_chars("█▓▒░ "),
        );
        pb.set_message(message.to_string());

        Self { pb }
    }

    /// Increment the progress by 1
    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }

    /// Set the current position
    pub fn set_position(&self, pos: u64) {
        self.pb.set_position(pos);
    }

    /// Update the message
    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    /// Finish the progress bar
    pub fn finish(&self) {
        self.pb.finish();
    }

    /// Finish with a custom message
    pub fn finish_with_message(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }
}

/// Create a simple progress bar for downloads/uploads
pub fn create_transfer_progress(remote: &str, total_bytes: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_bytes);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{{prefix:>12}} [{{bar:40.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{bytes_per_sec}}) {{msg}}"
            ))
            .unwrap()
            .progress_chars("█▓▒░ ")
    );
    pb.set_prefix(remote.to_string());
    pb
}

/// Show a simple message without progress
pub fn show_message(message: &str) {
    println!("  {}", message);
}

/// Show a success message
pub fn show_success(message: &str) {
    println!("  ✅ {}", message);
}

/// Show an error message
pub fn show_error(message: &str) {
    eprintln!("  ❌ {}", message);
}

/// Show a warning message
pub fn show_warning(message: &str) {
    println!("  ⚠️  {}", message);
}

/// Show an info message
pub fn show_info(message: &str) {
    println!("  ℹ️  {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_progress_creation() {
        let remotes = vec!["github".to_string(), "gitlab".to_string()];
        let progress = MultiRemoteProgress::new(&remotes);
        assert_eq!(progress.bars.len(), 2);
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new("Testing...");
        spinner.finish_and_clear();
    }

    #[test]
    fn test_progress_counter() {
        let counter = ProgressCounter::new(100, "Processing");
        counter.inc(10);
        counter.set_position(50);
        counter.finish();
    }
}
