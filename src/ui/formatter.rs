//! Output formatting utilities
//!
//! Provides table formatting, colored output, and JSON serialization for CLI output.
//! Supports both human-readable and machine-readable formats.

use serde::Serialize;

/// Output format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable with colors and tables
    Human,
    /// JSON format for scripting
    Json,
    /// Plain text without colors
    Plain,
}

/// Table formatter for aligned column output
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    colors_enabled: bool,
}

impl Table {
    /// Create a new table with headers
    #[must_use] 
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
            colors_enabled: true,
        }
    }

    /// Disable colored output
    #[must_use] 
    pub fn no_colors(mut self) -> Self {
        self.colors_enabled = false;
        self
    }

    /// Add a row to the table
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// Print the table to stdout
    pub fn print(&self) {
        let mut column_widths = vec![0; self.headers.len()];

        // Calculate column widths
        for (i, header) in self.headers.iter().enumerate() {
            column_widths[i] = header.len();
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < column_widths.len() {
                    column_widths[i] = column_widths[i].max(cell.len());
                }
            }
        }

        // Print headers
        print!("  ");
        for (i, header) in self.headers.iter().enumerate() {
            if self.colors_enabled {
                print!("\x1b[1m{:width$}\x1b[0m", header, width = column_widths[i]);
            } else {
                print!("{:width$}", header, width = column_widths[i]);
            }
            if i < self.headers.len() - 1 {
                print!("  ");
            }
        }
        println!();

        // Print separator
        print!("  ");
        for (i, width) in column_widths.iter().enumerate() {
            print!("{}", "-".repeat(*width));
            if i < column_widths.len() - 1 {
                print!("  ");
            }
        }
        println!();

        // Print rows
        for row in &self.rows {
            print!("  ");
            for (i, cell) in row.iter().enumerate() {
                if i < column_widths.len() {
                    print!("{:width$}", cell, width = column_widths[i]);
                    if i < row.len() - 1 {
                        print!("  ");
                    }
                }
            }
            println!();
        }
    }
}

/// Colored text output
pub mod colors {
    /// Color codes
    pub const RESET: &str = "\x1b[0m";
    /// Bold text style
    pub const BOLD: &str = "\x1b[1m";
    /// Dim text style
    pub const DIM: &str = "\x1b[2m";

    // Foreground colors
    /// Black color
    pub const BLACK: &str = "\x1b[30m";
    /// Red color
    pub const RED: &str = "\x1b[31m";
    /// Green color
    pub const GREEN: &str = "\x1b[32m";
    /// Yellow color
    pub const YELLOW: &str = "\x1b[33m";
    /// Blue color
    pub const BLUE: &str = "\x1b[34m";
    /// Magenta color
    pub const MAGENTA: &str = "\x1b[35m";
    /// Cyan color
    pub const CYAN: &str = "\x1b[36m";
    /// White color
    pub const WHITE: &str = "\x1b[37m";

    /// Apply color to text
    #[must_use] 
    pub fn colorize(text: &str, color: &str, enabled: bool) -> String {
        if enabled {
            format!("{color}{text}{RESET}")
        } else {
            text.to_string()
        }
    }

    /// Make text bold
    #[must_use] 
    pub fn bold(text: &str, enabled: bool) -> String {
        if enabled {
            format!("{BOLD}{text}{RESET}")
        } else {
            text.to_string()
        }
    }

    /// Make text dim
    #[must_use] 
    pub fn dim(text: &str, enabled: bool) -> String {
        if enabled {
            format!("{DIM}{text}{RESET}")
        } else {
            text.to_string()
        }
    }

    /// Success text (green)
    #[must_use] 
    pub fn success(text: &str, enabled: bool) -> String {
        colorize(text, GREEN, enabled)
    }

    /// Error text (red)
    #[must_use] 
    pub fn error(text: &str, enabled: bool) -> String {
        colorize(text, RED, enabled)
    }

    /// Warning text (yellow)
    #[must_use] 
    pub fn warning(text: &str, enabled: bool) -> String {
        colorize(text, YELLOW, enabled)
    }

    /// Info text (cyan)
    #[must_use] 
    pub fn info(text: &str, enabled: bool) -> String {
        colorize(text, CYAN, enabled)
    }
}

/// Format output based on the selected format
pub fn format_output<T: Serialize>(
    data: &T,
    format: OutputFormat,
) -> Result<String, serde_json::Error> {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(data),
        OutputFormat::Human | OutputFormat::Plain => {
            // For human/plain, we typically use custom formatting
            // This is a fallback to JSON
            serde_json::to_string_pretty(data)
        }
    }
}

/// Print data in the specified format
pub fn print_output<T: Serialize>(data: &T, format: OutputFormat) -> Result<(), serde_json::Error> {
    let output = format_output(data, format)?;
    println!("{output}");
    Ok(())
}

/// Format a duration in human-readable form
#[must_use] 
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}

/// Format bytes in human-readable form
#[must_use] 
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Print a header with optional color
pub fn print_header(text: &str, color_enabled: bool) {
    let line = "=".repeat(text.len() + 4);
    if color_enabled {
        println!("\n{}", colors::bold(&line, true));
        println!("  {}", colors::bold(text, true));
        println!("{}", colors::bold(&line, true));
    } else {
        println!("\n{line}");
        println!("  {text}");
        println!("{line}");
    }
}

/// Print a section header
pub fn print_section(text: &str, color_enabled: bool) {
    if color_enabled {
        println!("\n{}", colors::bold(text, true));
        println!("{}", colors::dim("-".repeat(text.len()).as_str(), true));
    } else {
        println!("\n{text}");
        println!("{}", "-".repeat(text.len()));
    }
}

/// Status indicator
pub enum Status {
    /// Success status
    Success,
    /// Error status
    Error,
    /// Warning status
    Warning,
    /// Info status
    Info,
    /// Pending status
    Pending,
}

impl Status {
    /// Get the symbol for this status
    #[must_use] 
    pub fn symbol(&self) -> &'static str {
        match self {
            Status::Success => "✓",
            Status::Error => "✗",
            Status::Warning => "⚠",
            Status::Info => "ℹ",
            Status::Pending => "●",
        }
    }

    /// Get the color for this status
    #[must_use] 
    pub fn color(&self) -> &'static str {
        match self {
            Status::Success => colors::GREEN,
            Status::Error => colors::RED,
            Status::Warning => colors::YELLOW,
            Status::Info => colors::CYAN,
            Status::Pending => colors::BLUE,
        }
    }

    /// Format a status line
    #[must_use] 
    pub fn format(&self, message: &str, color_enabled: bool) -> String {
        let symbol = self.symbol();
        if color_enabled {
            format!("  {}{}{} {}", self.color(), symbol, colors::RESET, message)
        } else {
            format!("  {symbol} {message}")
        }
    }
}

/// Print a status line
pub fn print_status(status: Status, message: &str, color_enabled: bool) {
    println!("{}", status.format(message, color_enabled));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let mut table = Table::new(vec!["Name".to_string(), "Status".to_string()]);
        table.add_row(vec!["github".to_string(), "active".to_string()]);
        table.add_row(vec!["gitlab".to_string(), "inactive".to_string()]);
        // Just test it doesn't panic
        table.print();
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
    }

    #[test]
    fn test_colors() {
        let text = "test";
        assert!(colors::success(text, true).contains("\x1b["));
        assert_eq!(colors::success(text, false), text);
    }

    #[test]
    fn test_status() {
        let status = Status::Success;
        assert_eq!(status.symbol(), "✓");
        let formatted = status.format("Test message", false);
        assert!(formatted.contains("Test message"));
    }
}
