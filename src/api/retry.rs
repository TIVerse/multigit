//! Retry logic with exponential backoff
//!
//! Provides utilities for retrying failed operations with configurable backoff strategies.

use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create a retry config for API requests
    #[must_use]
    pub fn for_api() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(60),
            backoff_multiplier: 2.0,
        }
    }

    /// Create a retry config for network operations
    #[must_use]
    pub fn for_network() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }

    /// Calculate backoff duration for a given attempt
    #[must_use]
    pub fn backoff_duration(&self, attempt: usize) -> Duration {
        let attempt_exp = attempt.try_into().unwrap_or(i32::MAX);
        let backoff_secs =
            self.initial_backoff.as_secs_f64() * self.backoff_multiplier.powi(attempt_exp);

        let backoff = Duration::from_secs_f64(backoff_secs);
        backoff.min(self.max_backoff)
    }
}

/// Retry a fallible async operation with exponential backoff
///
/// # Example
/// ```ignore
/// let result = retry_async(RetryConfig::default(), || async {
///     api_call().await
/// }).await?;
/// ```
pub async fn retry_async<F, Fut, T, E>(config: RetryConfig, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut attempt = 0;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    debug!("Operation succeeded on attempt {}", attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                if attempt >= config.max_attempts {
                    warn!("Operation failed after {} attempts: {}", attempt, e);
                    return Err(e);
                }

                let backoff = config.backoff_duration(attempt - 1);
                warn!(
                    "Operation failed (attempt {}/{}): {}. Retrying in {:?}...",
                    attempt, config.max_attempts, e, backoff
                );

                sleep(backoff).await;
            }
        }
    }
}

/// Retry a fallible operation with exponential backoff (synchronous)
pub fn retry_sync<F, T, E>(config: RetryConfig, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut attempt = 0;

    loop {
        attempt += 1;

        match operation() {
            Ok(result) => {
                if attempt > 1 {
                    debug!("Operation succeeded on attempt {}", attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                if attempt >= config.max_attempts {
                    warn!("Operation failed after {} attempts: {}", attempt, e);
                    return Err(e);
                }

                let backoff = config.backoff_duration(attempt - 1);
                warn!(
                    "Operation failed (attempt {}/{}): {}. Retrying in {:?}...",
                    attempt, config.max_attempts, e, backoff
                );

                std::thread::sleep(backoff);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_retry_config() {
        let config = RetryConfig::default();

        let backoff1 = config.backoff_duration(0);
        let backoff2 = config.backoff_duration(1);
        let backoff3 = config.backoff_duration(2);

        assert!(backoff2 > backoff1);
        assert!(backoff3 > backoff2);
    }

    #[tokio::test]
    async fn test_retry_async_success() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        let config = RetryConfig {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        };

        let result = retry_async(config, || {
            let counter = counter_clone.clone();
            async move {
                let mut count = counter.lock().unwrap();
                *count += 1;

                if *count < 3 {
                    Err("Temporary error")
                } else {
                    Ok("Success")
                }
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(*counter.lock().unwrap(), 3);
    }

    #[tokio::test]
    async fn test_retry_async_failure() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        };

        let result = retry_async(config, || async { Err::<(), _>("Always fails") }).await;

        assert!(result.is_err());
    }
}
