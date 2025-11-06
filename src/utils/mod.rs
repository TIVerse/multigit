//! Utility modules for MultiGit
//!
//! This module contains utilities for error handling, logging, validation, and secret redaction.

pub mod error;
pub mod logger;
pub mod redact;
pub mod validation;

pub use error::{MultiGitError, Result};
