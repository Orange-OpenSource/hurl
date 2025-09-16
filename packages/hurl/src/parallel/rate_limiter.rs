/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::error::{JobError, JobResult};

/// A token bucket rate limiter for controlling request rates.
///
/// This implementation uses a token bucket algorithm to limit the rate of operations.
/// Tokens are added to the bucket at a fixed rate, and each operation consumes a token.
/// If no tokens are available, the operation must wait.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimiterInner>>,
}

#[derive(Debug)]
struct RateLimiterInner {
    /// Maximum number of tokens the bucket can hold
    capacity: usize,
    /// Current number of tokens in the bucket
    tokens: f64,
    /// Rate at which tokens are added to the bucket (tokens per second)
    rate: f64,
    /// Last time tokens were added to the bucket
    last_refill: Instant,
}

impl RateLimiter {
    /// Creates a new rate limiter with the specified capacity and rate.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of tokens the bucket can hold
    /// * `rate` - Rate at which tokens are added to the bucket (tokens per second)
    ///
    /// # Returns
    ///
    /// A new `RateLimiter` instance
    pub fn new(capacity: usize, rate: f64) -> Self {
        RateLimiter {
            inner: Arc::new(Mutex::new(RateLimiterInner {
                capacity,
                tokens: capacity as f64,
                rate,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Acquires a token from the bucket, waiting if necessary.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time to wait for a token
    ///
    /// # Returns
    ///
    /// `Ok(())` if a token was acquired, or an error if the timeout was reached
    pub fn acquire(&self, timeout: Option<Duration>) -> JobResult<()> {
        let start = Instant::now();
        
        loop {
            // Try to acquire a token without waiting
            if self.try_acquire()? {
                return Ok(());
            }
            
            // Check if we've exceeded the timeout
            if let Some(timeout) = timeout {
                if start.elapsed() > timeout {
                    return Err(JobError::RateLimit(format!(
                        "Timeout after waiting {:?} for rate limit token",
                        timeout
                    )));
                }
            }
            
            // Wait a bit before trying again
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Tries to acquire a token without waiting.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if a token was acquired, `Ok(false)` if no tokens are available,
    /// or an error if the rate limiter is in an invalid state
    fn try_acquire(&self) -> JobResult<bool> {
        let mut inner = self.inner.lock().map_err(|e| {
            JobError::RateLimit(format!("Failed to acquire rate limiter lock: {}", e))
        })?;
        
        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(inner.last_refill).as_secs_f64();
        let new_tokens = elapsed * inner.rate;
        
        inner.tokens = (inner.tokens + new_tokens).min(inner.capacity as f64);
        inner.last_refill = now;
        
        // Try to consume a token
        if inner.tokens >= 1.0 {
            inner.tokens -= 1.0;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the current rate in tokens per second.
    pub fn rate(&self) -> JobResult<f64> {
        let inner = self.inner.lock().map_err(|e| {
            JobError::RateLimit(format!("Failed to acquire rate limiter lock: {}", e))
        })?;
        
        Ok(inner.rate)
    }

    /// Sets a new rate in tokens per second.
    pub fn set_rate(&self, rate: f64) -> JobResult<()> {
        let mut inner = self.inner.lock().map_err(|e| {
            JobError::RateLimit(format!("Failed to acquire rate limiter lock: {}", e))
        })?;
        
        inner.rate = rate;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_rate_limiter_acquire() {
        let limiter = RateLimiter::new(10, 10.0); // 10 tokens, 10 per second
        
        // Should be able to acquire 10 tokens immediately
        for _ in 0..10 {
            assert!(limiter.try_acquire().unwrap());
        }
        
        // 11th token should not be available
        assert!(!limiter.try_acquire().unwrap());
        
        // Wait for at least one token to be refilled
        std::thread::sleep(Duration::from_millis(100));
        
        // Should be able to acquire at least one more token
        assert!(limiter.try_acquire().unwrap());
    }

    #[test]
    fn test_rate_limiter_timeout() {
        let limiter = RateLimiter::new(1, 1.0); // 1 token, 1 per second
        
        // Acquire the only token
        assert!(limiter.try_acquire().unwrap());
        
        // Try to acquire with a very short timeout
        let result = limiter.acquire(Some(Duration::from_millis(10)));
        assert!(result.is_err());
        
        if let Err(JobError::RateLimit(_)) = result {
            // Expected error
        } else {
            panic!("Expected RateLimit error");
        }
    }
}