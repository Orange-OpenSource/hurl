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
use crate::util::term::{Stderr, Stdout};
use hurl_core::types::Index;
use std::io;

use super::job::{Job, JobResult};
use super::worker::WorkerId;

/// Represents a message sent from the worker to the runner (running on the main thread).
#[allow(clippy::large_enum_variant)]
pub enum WorkerMessage {
    /// Error raised when the file can't be read.
    InputReadError(InputReadErrorMsg),
    /// Error raised when the file isn't a valid Hurl content.
    ParsingError(ParsingErrorMsg),
    /// Sent when the Hurl file is in progress (file has been parsed and HTTP exchanges have started).
    Running(RunningMsg),
    /// Sent when the Hurl file is completed, whether successful or failed.
    Completed(CompletedMsg),
}

/// A message sent from worker to runner when the input file can't be read.
pub struct InputReadErrorMsg {
    /// Identifier of the worker sending this message.
    #[allow(dead_code)]
    pub worker_id: WorkerId,
    /// Job originator of this message.
    pub job: Job,
    /// Inner error that has triggered this message.
    pub error: io::Error,
}

impl InputReadErrorMsg {
    /// Creates a new I/O error message.
    pub fn new(worker_id: WorkerId, job: &Job, error: io::Error) -> Self {
        InputReadErrorMsg {
            worker_id,
            job: job.clone(),
            error,
        }
    }
}

/// A message sent from worker to runner when the input file can't be parsed.
pub struct ParsingErrorMsg {
    /// Identifier of the worker sending this message.
    #[allow(dead_code)]
    pub worker_id: WorkerId,
    /// Job originator of this message.
    #[allow(dead_code)]
    pub job: Job,
    /// Standard error of the worker for this job.
    pub stderr: Stderr,
}

impl ParsingErrorMsg {
    /// Creates a new parsing error message.
    pub fn new(worker_id: WorkerId, job: &Job, stderr: &Stderr) -> Self {
        ParsingErrorMsg {
            worker_id,
            job: job.clone(),
            stderr: stderr.clone(),
        }
    }
}

/// A message sent from worker to runner at regular time to inform that the job is being run.
pub struct RunningMsg {
    /// Identifier of the worker sending this message.
    pub worker_id: WorkerId,
    /// Job originator of this message.
    pub job: Job,
    /// Index of the current entry.
    pub current_entry: Index,
    /// Index of the last entry to be run.
    pub last_entry: Index,
    /// Number of actual retries
    pub retry_count: usize,
}

impl RunningMsg {
    /// Creates a new running message: the job is in progress.
    pub fn new(
        worker_id: WorkerId,
        job: &Job,
        current_entry: Index,
        last_entry: Index,
        retry_count: usize,
    ) -> Self {
        RunningMsg {
            worker_id,
            job: job.clone(),
            current_entry,
            last_entry,
            retry_count,
        }
    }
}

/// A message sent from worker to runner when a Hurl file has completed, whether successful or not.
pub struct CompletedMsg {
    /// Identifier of the worker sending this message.
    pub worker_id: WorkerId,
    /// Result execution of the originator job, can successful or failed.
    pub result: JobResult,
    /// Standard output of the worker for this job.
    pub stdout: Stdout,
    /// Standard error of the worker for this job.
    pub stderr: Stderr,
}

impl CompletedMsg {
    /// Creates a new completed message: the job has completed, successfully or not.
    pub fn new(worker_id: WorkerId, result: JobResult, stdout: Stdout, stderr: Stderr) -> Self {
        CompletedMsg {
            worker_id,
            result,
            stdout,
            stderr,
        }
    }
}
