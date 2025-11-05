//! MultiGit CLI entry point

use anyhow::Result;
use clap::{Parser, Subcommand};
use multigit::utils::logger::{init_logger, LogLevel, LoggerConfig};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(version)]
#[command(about = "Universal Git multi-remote automation tool")]
#[command(long_about = "MultiGit (mg) - Synchronize repositories across multiple Git hosting providers.\n\nAvailable as both 'multigit' and 'mg' commands for your convenience.")]
struct Cli {
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Output in JSON format
    #[arg(long)]
    json: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize MultiGit in the current repository
    Init {
        /// Skip interactive setup
        #[arg(short, long)]
        no_interactive: bool,
    },

    /// Interactive setup wizard (easiest way to get started)
    Setup {
        /// Quick setup for a specific provider
        #[arg(short, long)]
        provider: Option<String>,

        /// Username for quick setup
        #[arg(short, long)]
        username: Option<String>,
    },

    /// Interactive staging - visually select files to stage
    Add,

    /// Create repository on all configured platforms
    Create {
        /// Repository name
        name: String,

        /// Repository description
        #[arg(short, long)]
        description: Option<String>,

        /// Make repository private
        #[arg(short, long)]
        private: bool,

        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Manage remotes
    Remote {
        #[command(subcommand)]
        action: RemoteCommands,
    },

    /// Push to all remotes
    Push {
        /// Branch to push (default: current branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Force push (use with caution)
        #[arg(short, long)]
        force: bool,

        /// Specific remotes to push to
        #[arg(long)]
        remotes: Vec<String>,
    },

    /// Pull from primary remote
    Pull {
        /// Remote to pull from
        #[arg(long, default_value = "origin")]
        from: String,
    },

    /// Fetch from remotes
    Fetch {
        /// Specific remotes to fetch from
        remotes: Vec<String>,

        /// Fetch from all configured remotes
        #[arg(short, long)]
        all: bool,
    },

    /// Synchronize all remotes
    Sync {
        /// Branch to sync (default: current branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Dry run - show what would be done
        #[arg(long)]
        dry_run: bool,
    },

    /// Show sync status
    Status {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Branch operations
    Branch {
        #[command(subcommand)]
        action: BranchCommands,
    },

    /// Tag operations
    Tag {
        #[command(subcommand)]
        action: TagCommands,
    },

    /// Manage conflicts
    Conflict {
        #[command(subcommand)]
        action: ConflictCommands,
    },

    /// Daemon operations
    Daemon {
        #[command(subcommand)]
        action: DaemonCommands,
    },

    /// Run diagnostics and auto-fix issues
    Doctor {
        /// Automatically fix issues without prompting
        #[arg(short, long)]
        fix: bool,
    },

    /// Interactive conventional commit helper
    #[command(name = "cc", alias = "commit")]
    Cc,

    /// Interactive commit history browser
    Log {
        /// Number of commits to show
        #[arg(short, long)]
        limit: Option<usize>,

        /// Show commits from specific branch
        #[arg(short, long)]
        branch: Option<String>,

        /// Filter by author
        #[arg(short, long)]
        author: Option<String>,

        /// Show graph view
        #[arg(short, long)]
        graph: bool,
    },

    /// Interactive branch switcher
    Switch {
        /// Branch name to switch to
        branch: Option<String>,

        /// Create new branch
        #[arg(short, long)]
        create: bool,

        /// Base branch for new branch
        #[arg(short = 'f', long)]
        from: Option<String>,
    },

    /// Interactive stash manager
    Stash,

    /// Undo operations (commits, changes, staging)
    Undo,

    /// Amend last commit
    Amend {
        /// Amend without editing message
        #[arg(long)]
        no_edit: bool,
    },

    /// Generate changelog from conventional commits
    Changelog {
        /// Generate since this tag/commit
        #[arg(short, long)]
        since: Option<String>,

        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Repository statistics
    Stats,

    /// Smart merge from remotes
    Merge {
        /// Remote to merge from
        #[arg(short, long)]
        from: Option<String>,

        /// Branch to merge
        #[arg(short, long)]
        branch: Option<String>,
    },

    /// Backup to all remotes
    Backup {
        /// Automatic mode (no prompts)
        #[arg(short, long)]
        auto: bool,
    },

    /// Mirror mode - sync all remotes perfectly
    Mirror {
        /// Force push
        #[arg(short, long)]
        force: bool,

        /// Dry run
        #[arg(long)]
        dry_run: bool,
    },

    /// Work session tracker
    Session,

    /// Commit message templates
    Template,

    /// Git hooks manager
    Hooks,

    /// Git aliases manager
    Alias,

    /// Show version information
    Version,
}

#[derive(Subcommand)]
enum RemoteCommands {
    /// Add a new remote
    Add {
        /// Provider name (github, gitlab, bitbucket, etc.)
        provider: String,

        /// Username on the provider
        username: String,

        /// Custom API URL for self-hosted instances
        #[arg(long)]
        url: Option<String>,
    },

    /// List configured remotes
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Remove a remote
    Remove {
        /// Remote name to remove
        name: String,
    },

    /// Test remote connection
    Test {
        /// Remote name to test (omit to test all)
        name: Option<String>,
    },

    /// Update remote credentials
    Update {
        /// Remote name to update
        name: String,
    },
}

#[derive(Subcommand)]
enum BranchCommands {
    /// List branches
    List {
        /// Show verbose information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Create a new branch
    Create {
        /// Branch name
        name: String,

        /// Create from specific branch
        #[arg(short, long)]
        from: Option<String>,
    },

    /// Delete a branch
    Delete {
        /// Branch name
        name: String,

        /// Force deletion
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum TagCommands {
    /// List tags
    List {
        /// Show tags from specific remote
        #[arg(short, long)]
        remote: Option<String>,
    },

    /// Create a new tag
    Create {
        /// Tag name
        name: String,

        /// Tag message (creates annotated tag)
        #[arg(short, long)]
        message: Option<String>,

        /// Sign the tag with GPG
        #[arg(short, long)]
        sign: bool,
    },

    /// Delete a tag
    Delete {
        /// Tag name
        name: String,
    },
}

#[derive(Subcommand)]
enum ConflictCommands {
    /// List detected conflicts
    List,

    /// Resolve conflicts interactively
    Resolve,
}

#[derive(Subcommand)]
enum DaemonCommands {
    /// Start the daemon
    Start {
        /// Sync interval in minutes
        #[arg(short, long, default_value = "5")]
        interval: u64,
    },

    /// Stop the daemon
    Stop,

    /// Show daemon status
    Status,

    /// Show daemon logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger based on verbosity
    let log_level = LogLevel::from_verbosity(cli.verbose);
    let logger_config = LoggerConfig::new()
        .with_level(log_level)
        .with_color(!cli.no_color);

    let logger_config = if cli.json {
        logger_config.json()
    } else {
        logger_config
    };

    init_logger(logger_config)?;

    tracing::debug!("MultiGit {} starting", multigit::VERSION);

    // Handle commands
    let runtime = tokio::runtime::Runtime::new()?;

    match cli.command {
        Commands::Init { no_interactive: _ } => {
            use multigit::cli::commands::init;
            init::execute(".")?;
        }

        Commands::Setup { provider, username } => {
            use multigit::cli::commands::setup;
            if let (Some(prov), Some(user)) = (provider, username) {
                // Quick setup mode
                runtime.block_on(setup::quick_setup(&prov, user))?;
            } else {
                // Full wizard mode
                runtime.block_on(setup::run_wizard())?;
            }
        }

        Commands::Add => {
            use multigit::cli::commands::add;
            add::execute()?;
        }

        Commands::Create {
            name,
            description,
            private,
            interactive,
        } => {
            use multigit::cli::commands::create;
            if interactive {
                runtime.block_on(create::execute_interactive())?;
            } else {
                runtime.block_on(create::execute(name, description, private))?;
            }
        }

        Commands::Remote { action } => {
            handle_remote_command(action)?;
        }

        Commands::Push { branch, force, remotes } => {
            use multigit::cli::commands::push;
            runtime.block_on(push::execute(branch, force, remotes))?;
        }

        Commands::Pull { from } => {
            use multigit::cli::commands::pull;
            pull::execute(Some(from), None)?;
        }

        Commands::Fetch { remotes, all } => {
            use multigit::cli::commands::fetch;
            runtime.block_on(fetch::execute(remotes, all))?;
        }

        Commands::Sync { branch, dry_run } => {
            use multigit::cli::commands::sync;
            runtime.block_on(sync::execute(branch, dry_run))?;
        }

        Commands::Status { detailed } => {
            use multigit::cli::commands::status;
            status::execute(detailed)?;
        }

        Commands::Branch { action } => {
            use multigit::cli::commands::branch::commands;
            match action {
                BranchCommands::List { verbose } => {
                    commands::list(verbose)?;
                }
                BranchCommands::Create { name, from } => {
                    runtime.block_on(commands::create(name, from))?;
                }
                BranchCommands::Delete { name, force } => {
                    runtime.block_on(commands::delete(name, force))?;
                }
            }
        }

        Commands::Tag { action } => {
            use multigit::cli::commands::tag::commands;
            match action {
                TagCommands::List { remote } => {
                    commands::list(remote)?;
                }
                TagCommands::Create {
                    name,
                    message,
                    sign,
                } => {
                    commands::create(name, message, sign)?;
                }
                TagCommands::Delete { name } => {
                    commands::delete(name)?;
                }
            }
        }

        Commands::Doctor { fix } => {
            use multigit::cli::commands::doctor;
            doctor::execute(fix)?;
        }

        Commands::Cc => {
            use multigit::cli::commands::conventional_commit;
            conventional_commit::execute()?;
        }

        Commands::Log { limit, branch, author, graph } => {
            use multigit::cli::commands::log;
            if graph {
                log::show_graph(limit)?;
            } else {
                log::execute(limit, branch, author)?;
            }
        }

        Commands::Switch { branch, create, from } => {
            use multigit::cli::commands::switch;
            if create {
                switch::create_and_switch(from)?;
            } else {
                switch::execute(branch)?;
            }
        }

        Commands::Stash => {
            use multigit::cli::commands::stash;
            stash::execute()?;
        }

        Commands::Undo => {
            use multigit::cli::commands::undo;
            undo::execute()?;
        }

        Commands::Amend { no_edit } => {
            use multigit::cli::commands::amend;
            amend::execute(no_edit)?;
        }

        Commands::Changelog { since, output } => {
            use multigit::cli::commands::changelog;
            changelog::execute(since, output)?;
        }

        Commands::Stats => {
            use multigit::cli::commands::stats;
            stats::execute()?;
        }

        Commands::Merge { from, branch } => {
            use multigit::cli::commands::merge;
            merge::execute(from, branch)?;
        }

        Commands::Backup { auto } => {
            use multigit::cli::commands::backup;
            backup::execute(auto)?;
        }

        Commands::Mirror { force, dry_run } => {
            use multigit::cli::commands::mirror;
            mirror::execute(force, dry_run)?;
        }

        Commands::Session => {
            use multigit::cli::commands::session;
            session::execute()?;
        }

        Commands::Template => {
            use multigit::cli::commands::template;
            template::execute()?;
        }

        Commands::Hooks => {
            use multigit::cli::commands::hooks;
            hooks::execute()?;
        }

        Commands::Alias => {
            use multigit::cli::commands::alias;
            alias::execute()?;
        }

        Commands::Conflict { action } => {
            use multigit::cli::commands::conflict;
            match action {
                ConflictCommands::List => {
                    conflict::detect_conflicts()?;
                }
                ConflictCommands::Resolve => {
                    // Use default fast-forward strategy
                    use multigit::core::conflict_resolver::ResolutionStrategy;
                    conflict::resolve_conflicts(ResolutionStrategy::FastForwardOnly)?;
                }
            }
        }

        Commands::Daemon { action } => {
            use multigit::cli::commands::daemon;
            match action {
                DaemonCommands::Start { interval } => {
                    runtime.block_on(daemon::start(interval))?;
                }
                DaemonCommands::Stop => {
                    daemon::stop()?;
                }
                DaemonCommands::Status => {
                    daemon::status()?;
                }
                DaemonCommands::Logs { lines } => {
                    daemon::logs(lines)?;
                }
            }
        }

        Commands::Version => {
            println!("{}", multigit::version());
        }
    }

    Ok(())
}

fn handle_remote_command(action: RemoteCommands) -> Result<()> {
    use multigit::cli::commands::remote;

    let runtime = tokio::runtime::Runtime::new()?;

    match action {
        RemoteCommands::Add {
            provider,
            username,
            url,
        } => {
            runtime.block_on(remote::add_remote(provider, username, url, true))?;
        }

        RemoteCommands::List { detailed } => {
            remote::list_remotes(detailed)?;
        }

        RemoteCommands::Remove { name } => {
            remote::remove_remote(name, true)?;
        }

        RemoteCommands::Test { name } => {
            if let Some(remote_name) = name {
                runtime.block_on(remote::test_remote(remote_name))?;
            } else {
                runtime.block_on(remote::test_all_remotes())?;
            }
        }

        RemoteCommands::Update { name } => {
            runtime.block_on(remote::update_remote(name, true))?;
        }
    }

    Ok(())
}
