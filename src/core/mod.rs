//! Core functionality for MultiGit
//!
//! This module contains the core business logic including configuration management,
//! authentication, sync operations, and conflict resolution.

pub mod auth;
pub mod config;
pub mod conflict_resolver;
pub mod health_checker;
pub mod sync_manager;

pub use config::Config;
