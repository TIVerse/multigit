//! CLI argument parsing utilities
//!
//! Defines the command-line interface using clap.

use clap::{Parser, Subcommand};

/// MultiGit - Universal Git multi-remote automation tool
#[derive(Parser, Debug)]
#[command(name = "multigit")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Increase logging verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress output (quiet mode)
    #[arg(short, long)]
    pub quiet: bool,

    /// Output format (text, json)
    #[arg(long, default_value = "text")]
    pub format: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize MultiGit for a repository
    Init {
        /// Repository path (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Manage remotes
    Remote {
        #[command(subcommand)]
        action: RemoteAction,
    },

    /// Push to all configured remotes
    Push {
        /// Branch to push (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Force push (dangerous!)
        #[arg(short, long)]
        force: bool,

        /// Specific remotes to push to
        #[arg(long)]
        remotes: Vec<String>,
    },

    /// Pull from primary remote
    Pull {
        /// Branch to pull (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Remote to pull from
        #[arg(short, long)]
        remote: Option<String>,
    },

    /// Synchronize across all remotes
    Sync {
        /// Branch to sync (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Dry run - show what would be done
        #[arg(long)]
        dry_run: bool,
    },

    /// Show sync status
    Status {
        /// Show detailed status
        #[arg(short, long)]
        verbose: bool,
    },

    /// Manage conflicts
    Conflict {
        #[command(subcommand)]
        action: ConflictAction,
    },

    /// Manage background daemon
    Daemon {
        #[command(subcommand)]
        action: DaemonAction,
    },

    /// Run diagnostics and health checks
    Doctor {
        /// Attempt to auto-fix issues
        #[arg(long)]
        fix: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum RemoteAction {
    /// Add a new remote
    Add {
        /// Remote name (e.g., github, gitlab)
        name: String,

        /// Provider type (github, gitlab, bitbucket, codeberg, gitea)
        #[arg(short, long)]
        provider: String,

        /// Username on the provider
        #[arg(short, long)]
        username: String,

        /// API URL for self-hosted instances
        #[arg(long)]
        api_url: Option<String>,

        /// Use SSH instead of HTTPS
        #[arg(long)]
        ssh: bool,
    },

    /// List all configured remotes
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Remove a remote
    Remove {
        /// Remote name to remove
        name: String,
    },

    /// Update remote configuration
    Update {
        /// Remote name to update
        name: String,

        /// New username
        #[arg(short, long)]
        username: Option<String>,

        /// New API URL
        #[arg(long)]
        api_url: Option<String>,
    },

    /// Test remote connection
    Test {
        /// Remote name to test (tests all if not specified)
        name: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConflictAction {
    /// List detected conflicts
    List,

    /// Resolve conflicts
    Resolve {
        /// Resolution strategy (fast-forward, prefer-remote, manual, force)
        #[arg(short, long, default_value = "fast-forward")]
        strategy: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum DaemonAction {
    /// Start the daemon
    Start {
        /// Sync interval in minutes
        #[arg(short, long, default_value = "60")]
        interval: u64,
    },

    /// Stop the daemon
    Stop,

    /// Show daemon status
    Status,

    /// Show daemon logs
    Logs {
        /// Number of log lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,
    },
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
