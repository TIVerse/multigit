//! `MultiGit` - Universal Git Multi-Remote Automation Tool
//!
//! `MultiGit` manages a single local Git repository synchronized across multiple
//! remote Git hosting platforms (GitHub, GitLab, Bitbucket, Codeberg, Gitea, Forgejo).
//!
//! # Features
//!
//! - **Multi-Remote Sync**: Push/pull to multiple Git hosts simultaneously
//! - **Secure by Default**: OS keyring integration with encrypted fallback
//! - **Parallel Operations**: Async operations powered by Tokio
//! - **Smart Conflict Detection**: Prevents data loss with intelligent merging
//! - **Rich CLI/TUI**: Beautiful progress bars and interactive interfaces
//! - **Daemon Mode**: Background sync with scheduling
//! - **Cross-Platform**: Linux, macOS, and Windows support
//!
//! # Quick Start
//!
//! ```no_run
//! use multigit::core::Config;
//!
//! // Initialize MultiGit
//! Config::initialize().expect("Failed to initialize");
//!
//! // Load configuration
//! let config = Config::load().expect("Failed to load config");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::unused_self)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::items_after_statements)]

/// API client utilities for provider communication
pub mod api;

/// Command-line interface
pub mod cli;

/// Core functionality (config, auth, sync)
pub mod core;

/// Daemon and scheduling
pub mod daemon;

/// Git operations wrapper
pub mod git;

/// Data models
pub mod models;

/// Git provider implementations
pub mod providers;

/// Security and credential management
pub mod security;

/// User interface components
pub mod ui;

/// Utility functions and helpers
pub mod utils;

// Re-export commonly used types for convenience
pub use core::Config;
pub use models::{Remote, Repository};
pub use utils::{MultiGitError, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Get the full version string
#[must_use]
pub fn version() -> String {
    format!("{NAME} v{VERSION}")
}
