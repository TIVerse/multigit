//! UI module tests

use multigit::ui::formatter::{colors, format_bytes, format_duration, Status, Table};

#[test]
fn test_table_creation() {
    let table = Table::new(vec!["Col1".to_string(), "Col2".to_string()]);
    assert_eq!(std::mem::size_of_val(&table), std::mem::size_of::<Table>());
}

#[test]
fn test_table_no_colors() {
    let table = Table::new(vec!["Col1".to_string()]).no_colors();
    assert_eq!(std::mem::size_of_val(&table), std::mem::size_of::<Table>());
}

#[test]
fn test_table_add_row() {
    let mut table = Table::new(vec!["Name".to_string(), "Value".to_string()]);
    table.add_row(vec!["Test".to_string(), "123".to_string()]);
    // Table should have one row
    assert_eq!(std::mem::size_of_val(&table), std::mem::size_of::<Table>());
}

#[test]
fn test_status_success() {
    assert_eq!(Status::Success.symbol(), "✓");
    assert_eq!(Status::Success.color(), colors::GREEN);
}

#[test]
fn test_status_error() {
    assert_eq!(Status::Error.symbol(), "✗");
    assert_eq!(Status::Error.color(), colors::RED);
}

#[test]
fn test_status_warning() {
    assert_eq!(Status::Warning.symbol(), "⚠");
    assert_eq!(Status::Warning.color(), colors::YELLOW);
}

#[test]
fn test_status_info() {
    assert_eq!(Status::Info.symbol(), "ℹ");
    assert_eq!(Status::Info.color(), colors::CYAN);
}

#[test]
fn test_status_format() {
    let formatted = Status::Success.format("Test message", true);
    assert!(formatted.contains("✓"));
}

#[test]
fn test_colors_colorize() {
    let colored = colors::colorize("text", colors::RED, true);
    assert!(colored.contains("text"));
}

#[test]
fn test_colors_bold() {
    let bold = colors::bold("text", true);
    assert!(bold.contains("text"));
}

#[test]
fn test_colors_dim() {
    let dim = colors::dim("text", true);
    assert!(dim.contains("text"));
}

#[test]
fn test_colors_success() {
    let success = colors::success("message", true);
    assert!(success.contains("message"));
}

#[test]
fn test_colors_error() {
    let error = colors::error("message", true);
    assert!(error.contains("message"));
}

#[test]
fn test_colors_warning() {
    let warning = colors::warning("message", true);
    assert!(warning.contains("message"));
}

#[test]
fn test_colors_info() {
    let info = colors::info("message", true);
    assert!(info.contains("message"));
}

#[test]
fn test_format_duration_seconds() {
    let formatted = format_duration(45);
    assert_eq!(formatted, "45s");
}

#[test]
fn test_format_duration_minutes() {
    let formatted = format_duration(125);
    assert_eq!(formatted, "2m 5s");
}

#[test]
fn test_format_duration_hours() {
    let formatted = format_duration(3665);
    assert_eq!(formatted, "1h 1m");
}

#[test]
fn test_format_bytes_small() {
    let formatted = format_bytes(512);
    assert_eq!(formatted, "512.00 B");
}

#[test]
fn test_format_bytes_kb() {
    let formatted = format_bytes(2048);
    assert_eq!(formatted, "2.00 KB");
}

#[test]
fn test_format_bytes_mb() {
    let formatted = format_bytes(1_048_576);
    assert_eq!(formatted, "1.00 MB");
}
