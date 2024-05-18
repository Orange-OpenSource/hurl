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
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

use crate::output;
use crate::parallel::error::JobError;
use crate::parallel::job::{Job, JobResult};
use crate::parallel::message::WorkerMessage;
use crate::parallel::progress::{Mode, ParProgress};
use crate::parallel::worker::{Worker, WorkerId};
use crate::util::logger;
use crate::util::term::{Stderr, Stdout, WriteMode};

/// A parallel runner manages a list of `Worker`. Each worker is either idle or is running a
/// [`Job`]. To run jobs, the [`ParallelRunner::run`] method much be executed on the main thread.
/// Each worker has its own thread that it uses to run a Hurl file, and communicates with the main
/// thread. Standard multi-producer single-producer channels are used between the main runner and
/// the workers to send job request and receive job result.
///
/// The parallel runner is responsible to manage the state of the workers, and to display standard
/// output and standard error, in the main thread. Each worker reports its progression to the
/// parallel runner, which updates the workers states and displays a progress bar.
/// Inside each worker, logs (messages on standard error) and HTTP response (output on
/// standard output) are buffered and send to the runner to be eventually displayed.
///
/// By design, the workers state is read and modified on the main thread.
pub struct ParallelRunner {
    /// The list of workers, running Hurl file in their inner thread.
    workers: Vec<(Worker, WorkerState)>,
    /// The transmit end of the channel used to send messages to workers.
    tx: Sender<Job>,
    /// The receiving end of the channel used to communicate to workers.
    rx: Receiver<WorkerMessage>,
    /// Progress reporter to display the advancement of the parallel runs.
    progress: ParProgress,
    /// Output type for each completed job on standard output.
    output_type: OutputType,
}

/// Represents a worker's state.
pub enum WorkerState {
    /// Worker has no job to run.
    Idle,
    /// Worker is currently running a `job`, the entry being executed is at 0-based index
    /// `entry_index`, the total number of entries being `entry_count`.
    Running {
        job: Job,
        entry_index: usize,
        entry_count: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputType {
    /// The last HTTP response body of a Hurl file is outputted on standard output.
    ResponseBody { include_headers: bool, color: bool },
    /// The whole Hurl file run is exported in a structured JSON export on standard output.
    Json,
    /// Nothing is outputted on standard output when a Hurl file run is completed.
    NoOutput,
}

const MAX_RUNNING_DISPLAYED: usize = 8;

impl ParallelRunner {
    /// Creates a new parallel runner, with `worker_count` worker thread.
    ///
    /// The runner runs a list of [`Job`] in parallel. It creates two channels to communicate
    /// with the workers:
    ///
    /// - `runner -> worker`: is used to send [`Job`] processing request to a worker,
    /// - `worker -> runner`: is used to send [`WorkerMessage`] to update job progression, from a
    ///    worker to the runner.
    ///
    /// When a job is completed, depending on `output_type`, it can be outputted to standard output:
    /// whether as a raw response body bytes, or in a structured JSON output.
    ///
    /// If `test` mode is `true` the runner is run in "test" mode, reporting the success or failure
    /// of each file on standard error. Additionally to the test mode, a `progress_bar` designed for
    /// parallel run progression can be use.
    ///
    /// `color` determines if color if used in standard error.
    pub fn new(
        workers_count: usize,
        output_type: OutputType,
        test: bool,
        progress_bar: bool,
        color: bool,
    ) -> Self {
        // Worker are running on theirs own thread, while parallel runner is running in the main
        // thread.
        // We create the channel to communicate from workers to the parallel runner.
        let (tx_in, rx_in) = mpsc::channel();
        // We create the channel to communicate from the parallel runner to the workers.
        let (tx_out, rx_out) = mpsc::channel();
        let rx_out = Arc::new(Mutex::new(rx_out));

        // Create the workers:
        let workers = (0..workers_count)
            .map(|i| {
                let worker = Worker::new(WorkerId::from(i), &tx_in, &rx_out);
                let state = WorkerState::Idle;
                (worker, state)
            })
            .collect::<Vec<_>>();

        let mode = Mode::new(test, progress_bar);
        let progress = ParProgress::new(MAX_RUNNING_DISPLAYED, mode, color);

        ParallelRunner {
            workers,
            tx: tx_out,
            rx: rx_in,
            progress,
            output_type,
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
        let mut stdout = Stdout::new(WriteMode::Immediate);
        let mut stderr = Stderr::new(WriteMode::Immediate);

        // Create the jobs queue (last item is the first job to run):
        let jobs_count = jobs.len();
        let mut jobs = jobs.iter().rev().collect::<Vec<_>>();

        // Initiate the runner, fill our workers
        self.workers.iter().for_each(|_| {
            if let Some(job) = jobs.pop() {
                _ = self.tx.send(job.clone());
            }
        });

        // Start the message pump:
        let mut results = vec![];
        for msg in self.rx.iter() {
            match msg {
                // If we any error (either a [`WorkerMessage::IOError`] or a [`WorkerMessage::ParsingError`]
                // we don't take any more jobs and exit form the methods in error. This is the
                // same behaviour than when we run sequentially a list of Hurl files.
                WorkerMessage::IOError(msg) => {
                    self.progress.clear_progress_bar(&mut stderr);

                    let filename = msg.job.filename;
                    let error = msg.error;
                    let message = format!("Issue reading from {filename}: {error}");
                    return Err(JobError::IO(message));
                }
                WorkerMessage::ParsingError(msg) => {
                    // Like [`hurl::runner::run`] method, the display of parsing error is done here
                    // instead of being done in [`hurl::run_par`] method.
                    self.progress.clear_progress_bar(&mut stderr);

                    stderr.eprint(msg.stderr.buffer());
                    return Err(JobError::Parsing);
                }
                // Everything is OK, we report the progress. As we can receive a lot of running
                // messages, we don't want to update the progress bar too often to avoid flickering.
                WorkerMessage::Running(msg) => {
                    self.workers[msg.worker_id.0].1 = WorkerState::Running {
                        job: msg.job,
                        entry_index: msg.entry_index,
                        entry_count: msg.entry_count,
                    };

                    if self.progress.can_update() {
                        self.progress.clear_progress_bar(&mut stderr);
                        self.progress.update_progress_bar(
                            &self.workers,
                            results.len(),
                            jobs_count,
                            &mut stderr,
                        );
                    }
                }
                // A new job has been completed, we take a new job if the queue is not empty.
                // Contrary to when we receive a running message, we clear the progress bar no
                // matter what the frequency is.
                WorkerMessage::Completed(msg) => {
                    self.progress.clear_progress_bar(&mut stderr);

                    // The worker is becoming idle.
                    self.workers[msg.worker_id.0].1 = WorkerState::Idle;

                    // First, we display the job standard error, then the job standard output
                    // (similar to the sequential runner).
                    if !msg.stderr.buffer().is_empty() {
                        stderr.eprint(msg.stderr.buffer());
                    }

                    // Then, we print job output on standard output.
                    self.print_output(&msg.result, &mut stdout)?;

                    // Report the completion of this job and update the progress.
                    self.progress.print_completed(&msg.result, &mut stderr);

                    results.push(msg.result);

                    self.progress.update_progress_bar(
                        &self.workers,
                        results.len(),
                        jobs_count,
                        &mut stderr,
                    );
                    // We want to force the next refresh of the progress bar (when we receive a
                    // running message) to be sure that the new next jobs will be printed. This
                    // is needed because we've a throttle on the progress bar refresh and not every
                    // running messages received leads to a progress bar refresh.
                    self.progress.force_next_update();

                    // We run the next job to process:
                    let job = jobs.pop();
                    match job {
                        Some(job) => {
                            _ = self.tx.send(job.clone());
                        }
                        None => {
                            // If we have received all the job results, we can stop the run.
                            if results.len() == jobs_count {
                                break;
                            }
                        }
                    }
                }
                // Graceful shutdown for the worker.
                WorkerMessage::ShutDown => {}
            }
        }

        // All jobs have been executed, we sort results by sequence number to get the same order
        // as the input jobs list.
        results.sort_unstable_by_key(|result| result.job.seq);
        Ok(results)
    }

    /// Prints a job `result` to standard output `stdout`, either as a raw HTTP response (last
    /// body of the run), or in a structured JSON way.
    fn print_output(&self, result: &JobResult, stdout: &mut Stdout) -> Result<(), JobError> {
        let job = &result.job;
        let content = &result.content;
        let hurl_result = &result.hurl_result;
        let filename_in = &job.filename;
        let filename_out = job.runner_options.output.as_ref();

        match self.output_type {
            OutputType::ResponseBody {
                include_headers,
                color,
            } => {
                if hurl_result.success {
                    let result = output::write_last_body(
                        hurl_result,
                        include_headers,
                        color,
                        filename_out,
                        stdout,
                    );
                    if let Err(e) = result {
                        return Err(JobError::Runtime(logger::error_string(
                            &filename_in.to_string(),
                            content,
                            &e,
                            None,
                            color,
                        )));
                    }
                }
            }
            OutputType::Json => {
                let result =
                    output::write_json(hurl_result, content, filename_in, filename_out, stdout);
                if let Err(e) = result {
                    return Err(JobError::Runtime(e.to_string()));
                }
            }
            OutputType::NoOutput => {}
        }
        Ok(())
    }
}
