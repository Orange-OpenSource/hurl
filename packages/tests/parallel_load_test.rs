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
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use hurl::parallel::job::{Job, JobResult};
use hurl::parallel::metrics::Metrics;
use hurl::parallel::runner::{OutputType, ParallelRunner};
use hurl::runner::{RunnerOptionsBuilder, VariableSet};
use hurl::util::logger::LoggerOptionsBuilder;
use hurl_core::input::Input;
use hurl_core::typing::Count;

// This test is marked as ignored by default because it requires a local HTTP server
// To run it: cargo test --test parallel_load_test -- --ignored
#[test]
#[ignore]
fn test_parallel_runner_load() {
    // Create a temporary directory for test files
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create test files
    let files = create_test_files(temp_path, 100);
    
    // Create jobs from files
    let jobs = create_jobs(&files);
    
    // Run with different worker counts and measure performance
    let worker_counts = [1, 2, 4, 8, 16];
    
    println!("Running load test with {} files", files.len());
    println!("----------------------------------------");
    println!("| Workers | Time (ms) | Throughput (req/s) |");
    println!("----------------------------------------");
    
    for &workers in &worker_counts {
        // Create a runner with the specified number of workers
        let mut runner = ParallelRunner::new(
            workers,
            OutputType::NoOutput,
            Count::Finite(1),
            true,
            false,
            false,
            None,
        );
        
        // Add rate limiting to avoid overwhelming the server
        runner.with_rate_limit(workers * 10, workers as f64 * 50.0);
        
        // Run the jobs
        let results = runner.run(&jobs).unwrap();
        
        // Get metrics
        let metrics = runner.metrics();
        let total_time = metrics.total_time();
        let throughput = jobs.len() as f64 / total_time.as_secs_f64();
        
        // Print results
        println!(
            "| {:7} | {:9} | {:18.2} |",
            workers,
            total_time.as_millis(),
            throughput
        );
        
        // Verify all jobs completed successfully
        assert_eq!(results.len(), jobs.len());
        
        // Check metrics
        assert_eq!(metrics.get_counter("jobs_completed"), jobs.len());
        assert!(metrics.get_counter("jobs_successful") > 0);
    }
    
    // Clean up
    temp_dir.close().unwrap();
}

// Creates test Hurl files in the specified directory
fn create_test_files(dir: &std::path::Path, count: usize) -> Vec<String> {
    let mut files = Vec::with_capacity(count);
    
    for i in 0..count {
        let filename = format!("test_{}.hurl", i);
        let path = dir.join(&filename);
        
        // Create a simple Hurl file that makes a GET request
        let content = format!(
            "# Test file {}\nGET http://localhost:8000/test?id={}\n\nHTTP/1.1 200",
            i, i
        );
        
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        files.push(path.to_string_lossy().to_string());
    }
    
    files
}

// Creates jobs from file paths
fn create_jobs(files: &[String]) -> Vec<Job> {
    let runner_options = RunnerOptionsBuilder::default().build();
    let variables = VariableSet::new();
    let logger_options = LoggerOptionsBuilder::default().build();
    
    files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            Job::new(
                &Input::new(file),
                i,
                &runner_options,
                &variables,
                &logger_options,
            )
        })
        .collect()
}

// This test verifies that rate limiting works correctly
#[test]
fn test_rate_limiting() {
    use hurl::parallel::rate_limiter::RateLimiter;
    use std::time::Instant;
    
    // Create a rate limiter with 10 tokens and 5 tokens per second
    let limiter = RateLimiter::new(10, 5.0);
    
    // Should be able to acquire 10 tokens immediately
    for _ in 0..10 {
        limiter.acquire(None).unwrap();
    }
    
    // The 11th token should take approximately 200ms (1/5 second)
    let start = Instant::now();
    limiter.acquire(None).unwrap();
    let elapsed = start.elapsed();
    
    // Allow some margin for timing variations
    assert!(elapsed.as_millis() >= 150, "Rate limiting too fast: {:?}", elapsed);
    assert!(elapsed.as_millis() <= 300, "Rate limiting too slow: {:?}", elapsed);
}

// This test verifies that metrics collection works correctly
#[test]
fn test_metrics_collection() {
    let metrics = Metrics::new();
    
    // Test counters
    metrics.increment_counter("test_counter");
    metrics.increment_counter("test_counter");
    metrics.add_to_counter("test_counter", 3);
    assert_eq!(metrics.get_counter("test_counter"), 5);
    
    // Test timers
    metrics.record_timer("test_timer", Duration::from_millis(100));
    metrics.record_timer("test_timer", Duration::from_millis(200));
    let avg = metrics.get_average_time("test_timer").unwrap();
    assert_eq!(avg.as_millis(), 150);
    
    // Test gauges
    metrics.set_gauge("test_gauge", 42.5);
    assert_eq!(metrics.get_gauge("test_gauge"), Some(42.5));
    
    // Test summary
    let summary = metrics.summary();
    assert!(summary.contains("test_counter: 5"));
    assert!(summary.contains("test_timer"));
    assert!(summary.contains("test_gauge: 42.5"));
}