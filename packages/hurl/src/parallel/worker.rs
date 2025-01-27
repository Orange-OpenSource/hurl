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
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{fmt, thread};

use hurl_core::parser;

use crate::parallel::job::{Job, JobResult};
use crate::parallel::message::{
    CompletedMsg, IOErrorMsg, ParsingErrorMsg, RunningMsg, WorkerMessage,
};
use crate::runner;
use crate::runner::EventListener;
use crate::util::logger::Logger;
use crate::util::term::{Stderr, Stdout, WriteMode};

/// A worker runs job in its own thread.
pub struct Worker {
    /// The id of this worker.
    worker_id: WorkerId,
    /// The thread handle of this worker.
    thread: Option<thread::JoinHandle<()>>,
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}", self.worker_id)
    }
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
    /// Creates a new worker, with id `worker_id`.
    ///
    /// The worker spawns a new thread and process [`Job`] sent by the parallel runner through `rx`
    /// (the receiving part of the `runner -> worker` channel). Worker send message back to the
    /// runner to update the job progression thorough `tx` (the sending part of the `worker -> runner`.
    pub fn new(
        worker_id: WorkerId,
        tx: &Sender<WorkerMessage>,
        rx: &Arc<Mutex<Receiver<Job>>>,
    ) -> Self {
        let rx = Arc::clone(rx);
        let tx = tx.clone();

        let thread = thread::spawn(move || loop {
            let Ok(job) = rx.lock().unwrap().recv() else {
                return;
            };
            // In parallel execution, standard output and standard error messages are buffered
            // (in sequential mode, we'll use immediate standard output and error).
            let mut stdout = Stdout::new(WriteMode::Buffered);
            let stderr = Stderr::new(WriteMode::Buffered);

            // We also create a common logger for this run (logger verbosity can eventually be
            // mutated on each entry).
            let secrets = job.variables.secrets();
            let mut logger = Logger::new(&job.logger_options, stderr, &secrets);

            // Create a worker progress listener.
            let progress = WorkerProgress::new(worker_id, &job, &tx);

            let content = job.filename.read_to_string();
            let content = match content {
                Ok(c) => c,
                Err(e) => {
                    let msg = IOErrorMsg::new(worker_id, &job, e);
                    _ = tx.send(WorkerMessage::IOError(msg));
                    return;
                }
            };

            // Try to parse the content
            let hurl_file = parser::parse_hurl_file(&content);
            let hurl_file = match hurl_file {
                Ok(h) => h,
                Err(e) => {
                    logger.error_parsing_rich(&content, Some(&job.filename), &e);
                    let msg = ParsingErrorMsg::new(worker_id, &job, &logger.stderr);
                    _ = tx.send(WorkerMessage::ParsingError(msg));
                    return;
                }
            };

            // Now, we have a syntactically correct HurlFile instance, we can run it.
            let result = runner::run_entries(
                &hurl_file.entries,
                &content,
                Some(&job.filename),
                &job.runner_options,
                &job.variables,
                &mut stdout,
                Some(&progress),
                &mut logger,
            );

            if result.success && result.entries.last().is_none() {
                logger.warning(&format!(
                    "No entry have been executed for file {}",
                    job.filename
                ));
            }
            let job_result = JobResult::new(job, content, result);
            let msg = CompletedMsg::new(worker_id, job_result, stdout, logger.stderr);
            _ = tx.send(WorkerMessage::Completed(msg));
        });

        Worker {
            worker_id,
            thread: Some(thread),
        }
    }

    /// Takes the thread out of the worker, leaving a None in its place.
    pub fn take_thread(&mut self) -> Option<thread::JoinHandle<()>> {
        self.thread.take()
    }
}

struct WorkerProgress {
    worker_id: WorkerId,
    job: Job,
    tx: Sender<WorkerMessage>,
}

impl WorkerProgress {
    fn new(worker_id: WorkerId, job: &Job, tx: &Sender<WorkerMessage>) -> Self {
        WorkerProgress {
            worker_id,
            job: job.clone(),
            tx: tx.clone(),
        }
    }
}

impl EventListener for WorkerProgress {
    fn on_running(&self, entry_index: usize, entry_count: usize) {
        let msg = RunningMsg::new(self.worker_id, &self.job, entry_index, entry_count);
        _ = self.tx.send(WorkerMessage::Running(msg));
    }
}
