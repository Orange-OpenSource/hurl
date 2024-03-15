/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::runner::{HurlResult, Input, RunnerOptions};
use crate::util::logger::LoggerOptions;

/// Represents the job to run. A job instance groups the input data to execute, and has no methods
/// associated to it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    /// The Hurl source content
    pub filename: Input,
    /// The options to run this file.
    pub runner_options: RunnerOptions,
    /// The logger options for this run
    pub logger_options: LoggerOptions,
    /// The job 0-based index in the jobs list
    pub seq: usize,
}

impl Job {
    /// Creates a new job.
    pub fn new(
        filename: &Input,
        seq: usize,
        runner_options: &RunnerOptions,
        logger_options: &LoggerOptions,
    ) -> Self {
        Job {
            filename: filename.clone(),
            runner_options: runner_options.clone(),
            logger_options: logger_options.clone(),
            seq,
        }
    }
}

pub struct JobResult {
    /// The job corresponding to this job result.
    pub job: Job,
    /// The source content of the job.
    pub content: String,
    /// The result of execution of the job.
    pub hurl_result: HurlResult,
}

impl JobResult {
    /// Creates a new job result.
    pub fn new(job: Job, content: String, hurl_result: HurlResult) -> Self {
        JobResult {
            job,
            content,
            hurl_result,
        }
    }
}
