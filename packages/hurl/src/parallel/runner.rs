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
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use crate::parallel::error::JobError;
use crate::parallel::job::{Job, JobResult};
use crate::parallel::message::WorkerMessage;
use crate::parallel::progress_bar::ParProgressBar;
use crate::parallel::worker::{Worker, WorkerId};
use crate::util::term::{Stderr, WriteMode};

/// A parallel runner manages a list of `Worker`. Each worker is either idle, or running a
/// [`Job`]. To run jobs, the [`ParallelRunner::run`] method much be executed on the main thread.
/// Each worker has it's own thread that it used to run a Hurl file, and communicate with the main
/// thread. The communication use a multi-producer, single-consumer channel: workers are the
/// producers, and the parallel runner is the consumer.
///
/// The parallel runner is responsible to manage the state of the workers, and to display standard
/// output and standard error, in the main thread. Each worker reports its progression to the
/// parallel runner, which updates the workers states and display a progress bar.
/// Inside each worker, logs (messages on standard error) and HTTP response (output on
/// standard output) are buffered and send to the runner to be eventually displayed.
///
/// By design, the workers stats in read and modified on the main thread.
pub struct ParallelRunner {
    /// The list of workers, running Hurl file in their inner thread.
    workers: Vec<(Worker, WorkerState)>,
    /// The receiving end of the channel used to communicate with workers.
    rx: Receiver<WorkerMessage>,
    /// Optional progress bar to report the advancement of the parallel runs.
    progress_bar: Option<ParProgressBar>,
}

/// Represents a worker's state.
pub enum WorkerState {
    /// Worker has no job to run.
    Idle,
    /// Worker is currently running a job.
    Running(Job),
}

impl ParallelRunner {
    /// Creates a new parallel runner, with `worker_count` worker.
    ///
    /// If `progress_bar` is `true`, run progression can be reported on standard error by a progress
    /// bar. `color` determines if color if used or not to print the progress bar.
    pub fn new(workers_count: usize, progress_bar: bool, color: bool) -> Self {
        // Create the channel to communicate from workers to the parallel runner (worker are running
        // on theirs own thread, while parallel runner is running in the main thread).
        let (tx, rx) = mpsc::channel();

        // Create the workers:
        let workers = (0..workers_count)
            .map(|i| {
                let worker = Worker::new(WorkerId::from(i), tx.clone());
                let state = WorkerState::Idle;
                (worker, state)
            })
            .collect::<Vec<_>>();

        let progress_bar = if progress_bar {
            Some(ParProgressBar::new(color, 6))
        } else {
            None
        };

        ParallelRunner {
            workers,
            rx,
            progress_bar,
        }
    }

    /// Runs a list of [`Job`] in parallel and returns the results.
    ///
    /// Results are returned ordered by the sequence number, and not their execution order. So, the
    /// order of the `jobs` is the same as the order of the `jobs` results, independently of the
    /// worker's count.
    pub fn run(&mut self, jobs: &[Job]) -> Result<Vec<JobResult>, JobError> {
        // The parallel runner runs on the main thread. It's responsible for displaying standard
        // output and standard error. Workers are buffering their output and error in memory, and
        // delegate the display to the runners.
        let mut stderr = Stderr::new(WriteMode::Immediate);

        // Create the jobs queue (last item is the first job to run):
        let jobs_count = jobs.len();
        let mut jobs = jobs.iter().rev().collect::<Vec<_>>();

        // Initiate the runner, fill our workers
        for (worker, _) in &self.workers {
            if let Some(job) = jobs.pop() {
                worker.run(job);
            }
        }

        // Start the message pump:
        let mut results = vec![];
        for msg in self.rx.iter() {
            match msg {
                // If we any error (either a [`WorkerMessage::IOError`] or a [`WorkerMessage::ParsingError`]
                // we don't take any more jobs and exit form the methods in error. This is the
                // same behaviour than when we run sequentially a list of Hurl files.
                WorkerMessage::IOError(msg) => {
                    if let Some(pb) = &self.progress_bar {
                        pb.clear_progress(&mut stderr);
                    }
                    let filename = msg.job.filename;
                    let error = msg.error;
                    let message = format!("Issue reading from {filename}: {error}");
                    return Err(JobError::IO(message));
                }
                WorkerMessage::ParsingError(msg) => {
                    // Like [`hurl::runner::run`] method, the display of parsing error is done here
                    // instead of being done in [`hurl::run_par`] method.
                    if let Some(pb) = &self.progress_bar {
                        pb.clear_progress(&mut stderr);
                    }
                    stderr.eprint(msg.stderr.buffer());
                    return Err(JobError::Parsing);
                }
                // Everything is OK, we report the progress
                WorkerMessage::Running(msg) => {
                    self.workers[msg.worker_id.0].1 = WorkerState::Running(msg.job);

                    if let Some(pb) = &self.progress_bar {
                        pb.update_progress(&self.workers, &mut stderr);
                    }
                }
                // A new job has been completed, we take a new job if the queue is not empty.
                WorkerMessage::Completed(msg) => {
                    if let Some(pb) = &self.progress_bar {
                        pb.print_completed(&msg.result, &mut stderr);
                    }

                    results.push(msg.result);

                    // We run the next job to process:
                    let job = jobs.pop();
                    match job {
                        Some(job) => self.workers[msg.worker_id.0].0.run(job),
                        None => {
                            // A worker is becoming idle.
                            self.workers[msg.worker_id.0].1 = WorkerState::Idle;

                            if let Some(pb) = &self.progress_bar {
                                pb.update_progress(&self.workers, &mut stderr);
                            }
                            // If we have received all the job results, we can stop the run.
                            if results.len() == jobs_count {
                                break;
                            }
                        }
                    }
                }
            }
        }

        // All jobs have been executed, we sort results by sequence number to get the same order
        // as the input jobs list.
        results.sort_unstable_by_key(|result| result.job.seq);
        Ok(results)
    }
}
