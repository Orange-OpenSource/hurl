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
use std::io;

use crate::parallel::job::{Job, JobResult};
use crate::parallel::worker::WorkerId;
use crate::util::term::{Stderr, Stdout};

/// Represents a message sent from the worker to the runner (running on the main thread).
pub enum WorkerMessage {
    /// Error raised when the file can't be read.
    IOError(IOErrorMsg),
    /// Error raised when the file isn't a valid Hurl content.
    ParsingError(ParsingErrorMsg),
    /// Sent when the Hurl file is running.
    Running(RunningMsg),
    /// Sent when the Hurl file is completed, whether successful or failed.
    Completed(CompletedMsg),
}

/// A message sent from worker to runner when the input file can't be read.
pub struct IOErrorMsg {
    pub worker_id: WorkerId,
    pub job: Job,
    pub error: io::Error,
}

impl IOErrorMsg {
    pub fn new(worker_id: WorkerId, job: &Job, error: io::Error) -> Self {
        IOErrorMsg {
            worker_id,
            job: job.clone(),
            error,
        }
    }
}

/// A message sent from worker to runner when the input file can't be parsed.
pub struct ParsingErrorMsg {
    pub worker_id: WorkerId,
    pub job: Job,
    pub stderr: Stderr,
}

impl ParsingErrorMsg {
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
    pub worker_id: WorkerId,
    /// Current job for this message
    pub job: Job,
    /// 0-based index of the current entry
    pub entry_index: usize,
    /// Number of entries
    pub entry_count: usize,
}

impl RunningMsg {
    pub fn new(worker_id: WorkerId, job: &Job, entry_index: usize, entry_count: usize) -> Self {
        RunningMsg {
            worker_id,
            job: job.clone(),
            entry_index,
            entry_count,
        }
    }
}

/// A message sent from worker to runner when a Hurl file has completed, whether successful or not.
pub struct CompletedMsg {
    pub worker_id: WorkerId,
    pub result: JobResult,
    pub stdout: Stdout,
    pub stderr: Stderr,
}

impl CompletedMsg {
    pub fn new(worker_id: WorkerId, result: JobResult, stdout: Stdout, stderr: Stderr) -> Self {
        CompletedMsg {
            worker_id,
            result,
            stdout,
            stderr,
        }
    }
}
