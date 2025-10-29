//! Main test runner for MultiGit
//!
//! This file serves as the entry point for the test suite.
//! Run with: cargo test

// Import all test modules
mod fixtures;
mod integration;
mod unit;

// Re-export fixtures for use in other tests
pub use fixtures::*;

#[cfg(test)]
mod runner_tests {
    #[test]
    fn test_runner_works() {
        // Simple test to verify test infrastructure works
        assert_eq!(2 + 2, 4);
    }
}
