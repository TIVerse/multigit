//! Core data models for MultiGit
//!
//! This module contains the fundamental data structures used throughout MultiGit,
//! including repository metadata, remote configurations, and sync state tracking.

pub mod config;
pub mod remote;
pub mod repository;
pub mod sync_state;

pub use config::*;
pub use remote::*;
pub use repository::*;
pub use sync_state::*;
