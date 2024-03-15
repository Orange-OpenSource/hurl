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
use std::fmt;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use hurl_core::parser;

use crate::parallel::job::{Job, JobResult};
use crate::parallel::message::{
    CompletedMsg, IOErrorMsg, ParsingErrorMsg, RunningMsg, WorkerMessage,
};
use crate::runner::HurlResult;
use crate::util::logger::Logger;
use crate::util::term::{Stderr, WriteMode};

/// A worker runs job in its own thread.
pub struct Worker {
    /// The id of this worker.
    worker_id: WorkerId,
    /// The transmit end of the message, allowing to pass message to the parallel runner, on main thread.
    tx: Sender<WorkerMessage>,
}

/// Identifier of a worker.
#[derive(Copy, Clone, Debug)]
pub struct WorkerId(pub usize);

impl From<usize> for WorkerId {
    fn from(value: usize) -> Self {
        WorkerId(value)
    }
}

impl fmt::Display for WorkerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Worker {
    /// Creates a new worker, with id `worker_id` and sender `tx`.
    pub fn new(worker_id: WorkerId, tx: Sender<WorkerMessage>) -> Self {
        Worker { worker_id, tx }
    }

    /// Requests to process a `job`.
    ///
    /// This method returns immediately. To get the potential result of the job, a runner
    /// has to receive messages from this worker and process them (see [`WorkerMessage`]).
    pub fn run(&self, job: &Job) {
        let tx = self.tx.clone();
        let worker_id = self.worker_id;
        let job = job.clone();

        thread::spawn(move || {
            // In parallel execution, standard output and standard error messages are buffered
            // (in sequential mode, we'll use immediate standard output and error).
            let stderr = Stderr::new(WriteMode::Buffered);

            // We also create a common logger for this run (logger verbosity can eventually be
            // mutated on each entry).
            let mut logger = Logger::new(&job.logger_options, stderr);

            let content = job.filename.read_to_string();
            let content = match content {
                Ok(c) => c,
                Err(e) => {
                    let msg = IOErrorMsg::new(worker_id, job.clone(), e);
                    return tx.send(WorkerMessage::IOError(msg));
                }
            };

            let msg = RunningMsg::new(worker_id, job.clone());
            _ = tx.send(WorkerMessage::Running(msg));

            // Try to parse the content
            let hurl_file = parser::parse_hurl_file(&content);
            let _hurl_file = match hurl_file {
                Ok(h) => h,
                Err(e) => {
                    logger.error_parsing_rich(&content, &e);
                    let stderr = logger.stderr().clone();
                    let msg = ParsingErrorMsg::new(worker_id, job.clone(), stderr);
                    return tx.send(WorkerMessage::ParsingError(msg));
                }
            };

            // TODO: execute Hurl file!
            thread::sleep(Duration::from_secs(1));

            // Placeholder for execution
            let result = HurlResult {
                entries: vec![],
                time_in_ms: 0,
                success: true,
                cookies: vec![],
                timestamp: 0,
            };
            let job_result = JobResult::new(job, content, result);
            let msg = CompletedMsg::new(worker_id, job_result);
            tx.send(WorkerMessage::Completed(msg))
        });
    }
}
