//! Rate limiting for API requests
//!
//! Implements token bucket algorithm for rate limiting API requests.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, warn};

/// Token bucket rate limiter
#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `max_tokens` - Maximum number of tokens in the bucket
    /// * `refill_rate` - Number of tokens to add per second
    pub fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: max_tokens,
                max_tokens,
                refill_rate,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Create a rate limiter for GitHub API (5000 requests per hour)
    pub fn github() -> Self {
        // GitHub: 5000 requests per hour = ~1.39 per second
        // We use a slightly lower rate to be safe
        Self::new(5000.0, 1.3)
    }

    /// Create a rate limiter for GitLab API (600 requests per minute)
    pub fn gitlab() -> Self {
        // GitLab: 600 requests per minute = 10 per second
        Self::new(600.0, 10.0)
    }

    /// Create a rate limiter for Bitbucket API (1000 requests per hour)
    pub fn bitbucket() -> Self {
        // Bitbucket: 1000 requests per hour = ~0.28 per second
        Self::new(1000.0, 0.27)
    }

    /// Wait until a token is available, then consume it
    pub async fn acquire(&self) -> Result<(), String> {
        loop {
            let wait_time = {
                let mut state = self.state.lock().unwrap();
                state.refill();

                if state.tokens >= 1.0 {
                    state.tokens -= 1.0;
                    debug!("Rate limiter: token acquired, {} remaining", state.tokens);
                    return Ok(());
                } else {
                    // Calculate how long to wait for the next token
                    let tokens_needed = 1.0 - state.tokens;
                    let wait_secs = tokens_needed / state.refill_rate;
                    Duration::from_secs_f64(wait_secs)
                }
            };

            warn!("Rate limiter: waiting {:?} for token", wait_time);
            sleep(wait_time).await;
        }
    }

    /// Try to acquire a token without waiting
    pub fn try_acquire(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        state.refill();

        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            debug!("Rate limiter: token acquired, {} remaining", state.tokens);
            true
        } else {
            debug!("Rate limiter: no tokens available");
            false
        }
    }

    /// Get the number of available tokens
    pub fn available_tokens(&self) -> f64 {
        let mut state = self.state.lock().unwrap();
        state.refill();
        state.tokens
    }
}

impl RateLimiterState {
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();

        if elapsed > 0.0 {
            let tokens_to_add = elapsed * self.refill_rate;
            self.tokens = (self.tokens + tokens_to_add).min(self.max_tokens);
            self.last_refill = now;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new(10.0, 1.0);
        assert!(limiter.available_tokens() > 9.0);
    }

    #[test]
    fn test_try_acquire() {
        let limiter = RateLimiter::new(5.0, 1.0);

        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());

        let remaining = limiter.available_tokens();
        assert!(remaining < 3.0 && remaining >= 2.0);
    }

    #[tokio::test]
    async fn test_acquire() {
        let limiter = RateLimiter::new(2.0, 10.0); // Fast refill for testing

        limiter.acquire().await.unwrap();
        limiter.acquire().await.unwrap();

        // This should wait briefly
        limiter.acquire().await.unwrap();
    }
}
