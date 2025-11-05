//! Work session tracker
//!
//! Track time spent on branches and features

use crate::utils::error::{MultiGitError, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    branch: String,
    start_time: DateTime<Local>,
    end_time: Option<DateTime<Local>>,
    duration_minutes: Option<i64>,
}

/// Execute session tracker
pub fn execute() -> Result<()> {
    println!("\n⏱️  Work Session Tracker\n");
    println!("Feature coming soon: Track time spent on branches");
    println!("Commands:");
    println!("  mg session start    - Start tracking current branch");
    println!("  mg session stop     - Stop current session");
    println!("  mg session status   - View current session");
    println!("  mg session report   - Generate time report");
    Ok(())
}

fn get_session_file() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("multigit")
        .join("sessions.json")
}
