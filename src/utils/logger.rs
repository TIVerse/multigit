//! Logging configuration and utilities
//!
//! This module sets up tracing for structured logging across MultiGit.
//! Supports multiple log levels, colored output, and JSON formatting.

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Only errors
    Error,
    /// Errors and warnings
    Warn,
    /// Errors, warnings, and info (default)
    Info,
    /// Everything including debug messages
    Debug,
    /// Maximum verbosity including trace
    Trace,
}

impl LogLevel {
    /// Convert to tracing filter directive
    pub fn as_filter(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }

    /// Create from verbosity level (0-4)
    pub fn from_verbosity(level: u8) -> Self {
        match level {
            0 => Self::Error,
            1 => Self::Warn,
            2 => Self::Info,
            3 => Self::Debug,
            _ => Self::Trace,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_filter())
    }
}

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Log level
    pub level: LogLevel,

    /// Enable colored output
    pub colored: bool,

    /// Use JSON format instead of pretty text
    pub json: bool,

    /// Show timestamps
    pub timestamps: bool,

    /// Show target module names
    pub show_target: bool,

    /// Log to file path (optional)
    pub file_path: Option<String>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            colored: true,
            json: false,
            timestamps: true,
            show_target: false,
            file_path: None,
        }
    }
}

impl LoggerConfig {
    /// Create a new logger configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the log level
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Enable or disable colored output
    pub fn with_color(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }

    /// Enable JSON output
    pub fn json(mut self) -> Self {
        self.json = true;
        self
    }

    /// Set log file path
    pub fn with_file(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Hide timestamps
    pub fn without_timestamps(mut self) -> Self {
        self.timestamps = false;
        self
    }

    /// Show target module names
    pub fn with_target(mut self) -> Self {
        self.show_target = true;
        self
    }
}

/// Initialize the global logger
pub fn init_logger(config: LoggerConfig) -> anyhow::Result<()> {
    // Build the env filter
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(config.level.as_filter()))?;

    if config.json {
        // JSON formatted output
        let json_layer = fmt::layer()
            .json()
            .with_span_events(FmtSpan::CLOSE)
            .with_filter(env_filter);

        tracing_subscriber::registry().with(json_layer).init();
    } else {
        // Pretty formatted output
        if config.timestamps {
            let fmt_layer = fmt::layer()
                .with_ansi(config.colored)
                .with_target(config.show_target)
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(env_filter);

            tracing_subscriber::registry().with(fmt_layer).init();
        } else {
            let fmt_layer = fmt::layer()
                .with_ansi(config.colored)
                .with_target(config.show_target)
                .with_span_events(FmtSpan::CLOSE)
                .without_time()
                .with_filter(env_filter);

            tracing_subscriber::registry().with(fmt_layer).init();
        }
    }

    tracing::debug!("Logger initialized with level: {}", config.level);

    Ok(())
}

/// Initialize a simple logger with default settings
pub fn init_simple() -> anyhow::Result<()> {
    init_logger(LoggerConfig::default())
}

/// Initialize logger for testing (no output unless RUST_LOG is set)
pub fn init_test_logger() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .try_init();
}

/// Create a logger configuration from CLI arguments
pub fn logger_from_args(verbosity: u8, json: bool, no_color: bool) -> LoggerConfig {
    let mut config = LoggerConfig::new()
        .with_level(LogLevel::from_verbosity(verbosity))
        .with_color(!no_color);

    if json {
        config = config.json();
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level() {
        assert_eq!(LogLevel::Error.as_filter(), "error");
        assert_eq!(LogLevel::Info.as_filter(), "info");
        assert_eq!(LogLevel::Debug.as_filter(), "debug");
    }

    #[test]
    fn test_log_level_from_verbosity() {
        assert_eq!(LogLevel::from_verbosity(0), LogLevel::Error);
        assert_eq!(LogLevel::from_verbosity(1), LogLevel::Warn);
        assert_eq!(LogLevel::from_verbosity(2), LogLevel::Info);
        assert_eq!(LogLevel::from_verbosity(3), LogLevel::Debug);
        assert_eq!(LogLevel::from_verbosity(4), LogLevel::Trace);
    }

    #[test]
    fn test_logger_config_builder() {
        let config = LoggerConfig::new()
            .with_level(LogLevel::Debug)
            .with_color(false)
            .json()
            .with_file("/tmp/test.log");

        assert_eq!(config.level, LogLevel::Debug);
        assert!(!config.colored);
        assert!(config.json);
        assert_eq!(config.file_path, Some("/tmp/test.log".to_string()));
    }

    #[test]
    fn test_logger_default() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(config.colored);
        assert!(!config.json);
        assert!(config.timestamps);
    }
}
