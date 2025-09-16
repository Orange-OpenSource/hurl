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
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A simple metrics collector for parallel execution.
#[derive(Debug, Clone)]
pub struct Metrics {
    inner: Arc<Mutex<MetricsInner>>,
}

#[derive(Debug)]
struct MetricsInner {
    /// Start time of the execution
    start_time: Instant,
    /// Counters for various metrics
    counters: HashMap<String, usize>,
    /// Timers for various operations
    timers: HashMap<String, Vec<Duration>>,
    /// Gauges for various metrics
    gauges: HashMap<String, f64>,
}

impl Metrics {
    /// Creates a new metrics collector.
    pub fn new() -> Self {
        Metrics {
            inner: Arc::new(Mutex::new(MetricsInner {
                start_time: Instant::now(),
                counters: HashMap::new(),
                timers: HashMap::new(),
                gauges: HashMap::new(),
            })),
        }
    }

    /// Increments a counter by 1.
    pub fn increment_counter(&self, name: &str) {
        let mut inner = self.inner.lock().unwrap();
        *inner.counters.entry(name.to_string()).or_insert(0) += 1;
    }

    /// Increments a counter by a specific value.
    pub fn add_to_counter(&self, name: &str, value: usize) {
        let mut inner = self.inner.lock().unwrap();
        *inner.counters.entry(name.to_string()).or_insert(0) += value;
    }

    /// Records a duration for a specific operation.
    pub fn record_timer(&self, name: &str, duration: Duration) {
        let mut inner = self.inner.lock().unwrap();
        inner
            .timers
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(duration);
    }

    /// Sets a gauge to a specific value.
    pub fn set_gauge(&self, name: &str, value: f64) {
        let mut inner = self.inner.lock().unwrap();
        inner.gauges.insert(name.to_string(), value);
    }

    /// Returns the total execution time.
    pub fn total_time(&self) -> Duration {
        let inner = self.inner.lock().unwrap();
        inner.start_time.elapsed()
    }

    /// Returns the value of a counter.
    pub fn get_counter(&self, name: &str) -> usize {
        let inner = self.inner.lock().unwrap();
        *inner.counters.get(name).unwrap_or(&0)
    }

    /// Returns the average duration for a specific operation.
    pub fn get_average_time(&self, name: &str) -> Option<Duration> {
        let inner = self.inner.lock().unwrap();
        let timers = inner.timers.get(name)?;
        if timers.is_empty() {
            return None;
        }
        
        let total_nanos: u128 = timers.iter().map(|d| d.as_nanos()).sum();
        let avg_nanos = total_nanos / timers.len() as u128;
        Some(Duration::from_nanos(avg_nanos as u64))
    }

    /// Returns the value of a gauge.
    pub fn get_gauge(&self, name: &str) -> Option<f64> {
        let inner = self.inner.lock().unwrap();
        inner.gauges.get(name).copied()
    }

    /// Returns a summary of all metrics.
    pub fn summary(&self) -> String {
        let inner = self.inner.lock().unwrap();
        let mut result = String::new();
        
        result.push_str(&format!("Total execution time: {:?}\n", inner.start_time.elapsed()));
        
        if !inner.counters.is_empty() {
            result.push_str("\nCounters:\n");
            for (name, value) in &inner.counters {
                result.push_str(&format!("  {}: {}\n", name, value));
            }
        }
        
        if !inner.timers.is_empty() {
            result.push_str("\nTimers (average):\n");
            for (name, durations) in &inner.timers {
                if !durations.is_empty() {
                    let total_nanos: u128 = durations.iter().map(|d| d.as_nanos()).sum();
                    let avg_nanos = total_nanos / durations.len() as u128;
                    let avg = Duration::from_nanos(avg_nanos as u64);
                    result.push_str(&format!("  {}: {:?}\n", name, avg));
                }
            }
        }
        
        if !inner.gauges.is_empty() {
            result.push_str("\nGauges:\n");
            for (name, value) in &inner.gauges {
                result.push_str(&format!("  {}: {}\n", name, value));
            }
        }
        
        result
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// A timer that automatically records its duration when dropped.
pub struct Timer<'a> {
    metrics: &'a Metrics,
    name: String,
    start: Instant,
}

impl<'a> Timer<'a> {
    /// Creates a new timer that will record its duration to the given metrics when dropped.
    pub fn new(metrics: &'a Metrics, name: &str) -> Self {
        Timer {
            metrics,
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_timer(&self.name, duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_counter() {
        let metrics = Metrics::new();
        metrics.increment_counter("test_counter");
        metrics.increment_counter("test_counter");
        metrics.add_to_counter("test_counter", 3);
        
        assert_eq!(metrics.get_counter("test_counter"), 5);
    }

    #[test]
    fn test_timer() {
        let metrics = Metrics::new();
        
        // Record a few durations
        metrics.record_timer("test_timer", Duration::from_millis(100));
        metrics.record_timer("test_timer", Duration::from_millis(200));
        
        let avg = metrics.get_average_time("test_timer").unwrap();
        assert!(avg.as_millis() == 150);
    }

    #[test]
    fn test_timer_struct() {
        let metrics = Metrics::new();
        
        {
            let _timer = Timer::new(&metrics, "test_auto_timer");
            thread::sleep(Duration::from_millis(10));
        }
        
        let avg = metrics.get_average_time("test_auto_timer").unwrap();
        assert!(avg.as_millis() >= 10);
    }

    #[test]
    fn test_gauge() {
        let metrics = Metrics::new();
        metrics.set_gauge("test_gauge", 42.5);
        
        assert_eq!(metrics.get_gauge("test_gauge"), Some(42.5));
    }
}