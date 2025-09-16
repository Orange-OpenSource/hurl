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
use std::io;
use std::sync::mpsc::SendError;
use thiserror::Error;

use super::job::Job;
use super::message::WorkerMessage;

/// Error triggered when running a [`crate::parallel::job::Job`].
#[derive(Error, Debug)]
pub enum JobError {
    /// An error has occurred while reading input.
    #[error("Input read error: {0}")]
    InputRead(String),

    /// An error has occurred during parsing.
    #[error("Parsing error")]
    Parsing,

    /// An error has occurred while writing to output.
    #[error("Output write error: {0}")]
    OutputWrite(String),

    /// An error has occurred during job execution.
    #[error("Job execution error: {0}")]
    Execution(String),

    /// An error has occurred during worker communication.
    #[error("Worker communication error: {0}")]
    Communication(String),

    /// An error has occurred due to timeout.
    #[error("Job timeout after {0} seconds")]
    Timeout(u64),

    /// An error has occurred due to rate limiting.
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// An error has occurred with thread operations.
    #[error("Thread error: {0}")]
    Thread(String),

    /// An error has occurred with I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

/// Error triggered when communicating with workers.
#[derive(Error, Debug)]
pub enum WorkerError {
    /// An error has occurred when sending a message to a worker.
    #[error("Failed to send message to worker: {0}")]
    SendError(String),

    /// An error has occurred when receiving a message from a worker.
    #[error("Failed to receive message from worker: {0}")]
    ReceiveError(String),

    /// An error has occurred when a worker panicked.
    #[error("Worker panicked: {0}")]
    Panic(String),
}

impl<T> From<SendError<T>> for WorkerError {
    fn from(error: SendError<T>) -> Self {
        WorkerError::SendError(error.to_string())
    }
}

/// Result type for operations related to job execution.
pub type JobResult<T> = Result<T, JobError>;

/// Result type for operations related to worker communication.
pub type WorkerResult<T> = Result<T, WorkerError>;

/// Converts a WorkerError to a JobError.
impl From<WorkerError> for JobError {
    fn from(error: WorkerError) -> Self {
        JobError::Communication(error.to_string())
    }
}