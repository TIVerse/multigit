//! Unit tests for UI formatter

use multigit::ui::formatter::{colors, format_bytes, format_duration, Status, Table};

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(0), "0s");
    assert_eq!(format_duration(30), "30s");
    assert_eq!(format_duration(59), "59s");
    assert_eq!(format_duration(60), "1m 0s");
    assert_eq!(format_duration(90), "1m 30s");
    assert_eq!(format_duration(3600), "1h 0m");
    assert_eq!(format_duration(3661), "1h 1m");
    assert_eq!(format_duration(7200), "2h 0m");
}

#[test]
fn test_format_bytes() {
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(512), "512 B");
    assert_eq!(format_bytes(1023), "1023 B");
    assert_eq!(format_bytes(1024), "1.00 KB");
    assert_eq!(format_bytes(1536), "1.50 KB");
    assert_eq!(format_bytes(1048576), "1.00 MB");
    assert_eq!(format_bytes(1073741824), "1.00 GB");
}

#[test]
fn test_status_symbols() {
    assert_eq!(Status::Success.symbol(), "✓");
    assert_eq!(Status::Error.symbol(), "✗");
    assert_eq!(Status::Warning.symbol(), "⚠");
    assert_eq!(Status::Info.symbol(), "ℹ");
    assert_eq!(Status::Pending.symbol(), "●");
}

#[test]
fn test_status_colors() {
    assert_eq!(Status::Success.color(), colors::GREEN);
    assert_eq!(Status::Error.color(), colors::RED);
    assert_eq!(Status::Warning.color(), colors::YELLOW);
    assert_eq!(Status::Info.color(), colors::CYAN);
    assert_eq!(Status::Pending.color(), colors::BLUE);
}

#[test]
fn test_status_format() {
    let status = Status::Success;
    let formatted = status.format("Test message", false);
    assert!(formatted.contains("✓"));
    assert!(formatted.contains("Test message"));
}

#[test]
fn test_status_format_with_colors() {
    let status = Status::Success;
    let formatted = status.format("Test", true);
    assert!(formatted.contains("\x1b[")); // Has ANSI codes
}

#[test]
fn test_colors_colorize() {
    let text = "test";

    // With colors enabled
    let colored = colors::colorize(text, colors::RED, true);
    assert!(colored.starts_with(colors::RED));
    assert!(colored.ends_with(colors::RESET));
    assert!(colored.contains(text));

    // With colors disabled
    let plain = colors::colorize(text, colors::RED, false);
    assert_eq!(plain, text);
}

#[test]
fn test_colors_helpers() {
    let text = "test";

    assert!(colors::success(text, true).contains(colors::GREEN));
    assert!(colors::error(text, true).contains(colors::RED));
    assert!(colors::warning(text, true).contains(colors::YELLOW));
    assert!(colors::info(text, true).contains(colors::CYAN));

    assert_eq!(colors::success(text, false), text);
    assert_eq!(colors::error(text, false), text);
}

#[test]
fn test_colors_bold() {
    let text = "bold";

    let bolded = colors::bold(text, true);
    assert!(bolded.contains(colors::BOLD));

    let plain = colors::bold(text, false);
    assert_eq!(plain, text);
}

#[test]
fn test_colors_dim() {
    let text = "dim";

    let dimmed = colors::dim(text, true);
    assert!(dimmed.contains(colors::DIM));

    let plain = colors::dim(text, false);
    assert_eq!(plain, text);
}

#[test]
fn test_table_creation() {
    let mut table = Table::new(vec!["Name".to_string(), "Status".to_string()]);
    assert_eq!(
        table.add_row(vec!["test1".to_string(), "active".to_string()]),
        ()
    );
    assert_eq!(
        table.add_row(vec!["test2".to_string(), "inactive".to_string()]),
        ()
    );

    // Just test it doesn't panic
    table.print();
}

#[test]
fn test_table_no_colors() {
    let table = Table::new(vec!["Col1".to_string()]).no_colors();
    // Test that no_colors() returns a table
    table.print();
}
