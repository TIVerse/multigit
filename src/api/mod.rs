//! API client utilities for communicating with Git hosting providers
//!
//! This module provides HTTP client functionality, rate limiting, and retry logic.

pub mod client;
pub mod rate_limiter;
pub mod retry;

// TODO: Implement API client with reqwest
// TODO: Add rate limiting per provider
// TODO: Add exponential backoff retry logic
