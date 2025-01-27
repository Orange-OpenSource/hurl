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
use std::sync::{mpsc, Arc, Mutex};

use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::typing::Count;

use crate::output;
use crate::parallel::error::JobError;
use crate::parallel::job::{Job, JobQueue, JobResult};
use crate::parallel::message::WorkerMessage;
use crate::parallel::progress::{Mode, ParProgress};
use crate::parallel::worker::{Worker, WorkerId};
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
    tx: Option<Sender<Job>>,
    /// The receiving end of the channel used to communicate to workers.
    rx: Receiver<WorkerMessage>,
    /// Progress reporter to display the advancement of the parallel runs.
    progress: ParProgress,
    /// Output type for each completed job on standard output.
    output_type: OutputType,
    /// Repeat mode for the runner: infinite or finite.
    repeat: Count,
}

/// Represents a worker's state.
#[allow(clippy::large_enum_variant)]
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
    /// The runner can repeat running a list of jobs. For instance, when repeating two times the job
    /// sequence (`a`, `b`, `c`), runner will act as if it runs (`a`, `b`, `c`, `a`, `b`, `c`).
    ///
    /// If `test` mode is `true` the runner is run in "test" mode, reporting the success or failure
    /// of each file on standard error. In addition to the test mode, a `progress_bar` designed for
    /// parallel run progression can be used. When the progress bar is displayed, it's wrapped with
    /// new lines at width `max_width`.
    ///
    /// `color` determines if color if used in standard error.
    pub fn new(
        workers_count: usize,
        output_type: OutputType,
        repeat: Count,
        test: bool,
        progress_bar: bool,
        color: bool,
        max_width: Option<usize>,
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
        let progress = ParProgress::new(MAX_RUNNING_DISPLAYED, mode, color, max_width);

        ParallelRunner {
            workers,
            tx: Some(tx_out),
            rx: rx_in,
            progress,
            output_type,
            repeat,
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

        // Create the jobs queue:
        let mut queue = JobQueue::new(jobs, self.repeat);
        let jobs_count = queue.jobs_count();

        // Initiate the runner, fill our workers:
        self.workers.iter().for_each(|_| {
            if let Some(job) = queue.next() {
                _ = self.tx.as_ref().unwrap().send(job);
            }
        });

        // When dumped HTTP responses, we truncate existing output file on first save, then append
        // it on subsequent write.
        let mut append = false;

        // Start the message pump:
        let mut results = vec![];
        for msg in self.rx.iter() {
            match msg {
                // If we have any error (either a [`WorkerMessage::IOError`] or a [`WorkerMessage::ParsingError`]
                // we don't take any more jobs and exit from the methods in error. This is the same
                // behaviour as when we run sequentially a list of Hurl files.
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
                // matter what the frequency is, to get a "correct" and up-to-date display on any
                // test completion.
                WorkerMessage::Completed(msg) => {
                    self.progress.clear_progress_bar(&mut stderr);

                    // The worker is becoming idle.
                    self.workers[msg.worker_id.0].1 = WorkerState::Idle;

                    // First, we display the job standard error, then the job standard output
                    // (similar to the sequential runner).
                    if !msg.stderr.buffer().is_empty() {
                        stderr.eprint(msg.stderr.buffer());
                    }
                    if !msg.stdout.buffer().is_empty() {
                        let ret = stdout.write_all(msg.stdout.buffer());
                        if ret.is_err() {
                            return Err(JobError::IO("Issue writing to stdout".to_string()));
                        }
                    }

                    // Then, we print job output on standard output (the first response truncates
                    // exiting file, subsequent response appends bytes).
                    self.print_output(&msg.result, &mut stdout, append)?;
                    append = true;

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
                    let job = queue.next();
                    match job {
                        Some(job) => {
                            _ = self.tx.as_ref().unwrap().send(job);
                        }
                        None => {
                            // If we have received all the job results, we can stop the run.
                            if let Some(jobs_count) = jobs_count {
                                if results.len() == jobs_count {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // We gracefully shut down workers, by dropping the sender and wait for each thread workers
        // to join.
        drop(self.tx.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.0.take_thread() {
                thread.join().unwrap();
            }
        }

        // All jobs have been executed, we sort results by sequence number to get the same order
        // as the input jobs list.
        results.sort_unstable_by_key(|result| result.job.seq);
        Ok(results)
    }

    /// Prints a job `result` to standard output `stdout`, either as a raw HTTP response (last
    /// body of the run), or in a structured JSON way.
    /// If `append` is true, any existing file will be appended instead of being truncated.
    fn print_output(
        &self,
        result: &JobResult,
        stdout: &mut Stdout,
        append: bool,
    ) -> Result<(), JobError> {
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
                        append,
                    );
                    if let Err(e) = result {
                        return Err(JobError::Runtime(e.to_string(
                            &filename_in.to_string(),
                            content,
                            None,
                            OutputFormat::Terminal(color),
                        )));
                    }
                }
            }
            OutputType::Json => {
                let result = output::write_json(
                    hurl_result,
                    content,
                    filename_in,
                    filename_out,
                    stdout,
                    append,
                );
                if let Err(e) = result {
                    return Err(JobError::Runtime(e.to_string()));
                }
            }
            OutputType::NoOutput => {}
        }
        Ok(())
    }
}
