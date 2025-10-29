//! UI formatting example - Beautiful terminal output
//!
//! This example demonstrates the UI formatting capabilities.

use multigit::ui::formatter::{
    colors, format_bytes, format_duration, print_header, print_section, print_status, Status, Table,
};

fn main() {
    println!("\nMultiGit UI Formatting Example\n");

    // 1. Format durations
    print_header("Duration Formatting", true);
    println!("  45 seconds   = {}", format_duration(45));
    println!("  90 seconds   = {}", format_duration(90));
    println!("  3661 seconds = {}", format_duration(3661));

    // 2. Format bytes
    print_section("Byte Formatting", true);
    println!("  512 bytes    = {}", format_bytes(512));
    println!("  1536 bytes   = {}", format_bytes(1536));
    println!("  1048576 B    = {}", format_bytes(1048576));
    println!("  1073741824 B = {}", format_bytes(1073741824));

    // 3. Status indicators
    print_section("Status Indicators", true);
    print_status(Status::Success, "Operation completed successfully", true);
    print_status(Status::Error, "Operation failed", true);
    print_status(Status::Warning, "Warning: check configuration", true);
    print_status(Status::Info, "Information message", true);
    print_status(Status::Pending, "Operation in progress", true);

    // 4. Colored text
    print_section("Colored Text", true);
    println!("  {}", colors::success("Success message", true));
    println!("  {}", colors::error("Error message", true));
    println!("  {}", colors::warning("Warning message", true));
    println!("  {}", colors::info("Info message", true));
    println!("  {}", colors::bold("Bold text", true));
    println!("  {}", colors::dim("Dimmed text", true));

    // 5. Tables
    print_section("Table Formatting", true);
    let mut table = Table::new(vec![
        "Remote".to_string(),
        "Status".to_string(),
        "Commits".to_string(),
    ]);

    table.add_row(vec![
        "github".to_string(),
        "synced".to_string(),
        "125".to_string(),
    ]);
    table.add_row(vec![
        "gitlab".to_string(),
        "synced".to_string(),
        "125".to_string(),
    ]);
    table.add_row(vec![
        "bitbucket".to_string(),
        "pending".to_string(),
        "120".to_string(),
    ]);

    table.print();

    println!("\n✅ Formatting examples complete!\n");
}
