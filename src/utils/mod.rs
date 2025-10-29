//! Utility modules for MultiGit
//!
//! This module contains utilities for error handling, logging, and validation.

pub mod error;
pub mod logger;
pub mod validation;

pub use error::{MultiGitError, Result};
