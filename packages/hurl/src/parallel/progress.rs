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
use colored::Colorize;
use std::time::{Duration, Instant};

use crate::parallel::job::JobResult;
use crate::parallel::runner::WorkerState;
use crate::parallel::worker::Worker;
use crate::util;
use crate::util::term::Stderr;

/// A progress reporter to display advancement of parallel runs execution in test mode.
pub struct ParProgress {
    /// The maximum number of running workers displayed in the progress bar.
    max_running_displayed: usize,
    /// Mode of the progress reporter
    mode: Mode,
    /// The standard error uses color or not.
    color: bool,
    /// Save last progress bar refresh to limits flickering.
    throttle: Throttle,
}

#[derive(Copy, Clone)]
pub enum Mode {
    /// Run without --test
    Default,
    /// Run with --test and with a progress bar
    TestWithProgress,
    /// Run with --test and no progress bar
    TestWithoutProgress,
}

impl ParProgress {
    /// Creates a new instance.
    pub fn new(max_running_displayed: usize, mode: Mode, color: bool) -> Self {
        ParProgress {
            max_running_displayed,
            mode,
            color,
            throttle: Throttle::new(),
        }
    }

    /// Clear the progress bar.
    pub fn clear_progress_bar(&self, stderr: &mut Stderr) {
        if !matches!(self.mode, Mode::TestWithProgress) {
            return;
        }
        stderr.clear_progress_bar();
    }

    /// Displays progression of `workers` on standard error `stderr`.
    ///
    /// This method is called on the parallel runner thread (usually the main thread).
    pub fn update_progress_bar(
        &mut self,
        workers: &[(Worker, WorkerState)],
        completed: usize,
        count: usize,
        stderr: &mut Stderr,
    ) {
        if !matches!(self.mode, Mode::TestWithProgress) {
            return;
        }
        self.throttle.update();

        // Select the running workers to be displayed
        let mut workers = workers
            .iter()
            .filter(|(_, state)| matches!(state, WorkerState::Running { .. }))
            .collect::<Vec<_>>();
        if workers.is_empty() {
            return;
        }

        // We sort the running workers by job sequence id, this way a given job will be displayed
        // on the same line, independently of the worker id.
        workers.sort_unstable_by_key(|(_, state)| match state {
            WorkerState::Running { job, .. } => job.seq,
            WorkerState::Idle => usize::MAX,
        });
        let running = workers.len();

        // We keep a reasonable number of worker to displayed, from the oldest to the newest.
        workers.truncate(self.max_running_displayed);

        // Computes maximum size of the string "[current request] / [nb of request]" to left align
        // the column.
        let max = workers
            .iter()
            .map(|(_, state)| match state {
                WorkerState::Running { entry_count, .. } => *entry_count,
                WorkerState::Idle => 0,
            })
            .max()
            .unwrap();
        let max_width = 2 * (((max as f64).log10() as usize) + 1) + 1;

        // Construct all the progress strings
        let mut all_progress = String::new();
        let percent = (completed as f64 * 100.0 / count as f64) as usize;
        let progress = format!("Executed files: {completed}/{count} ({percent}%)\n");
        all_progress.push_str(&progress);

        for (_, state) in &workers {
            if let WorkerState::Running {
                job,
                entry_index,
                entry_count,
            } = state
            {
                let entry_index = entry_index + 1; // entry index display is 1-based
                let requests = format!("{entry_index}/{entry_count}");
                let padding = " ".repeat(max_width - requests.len());
                let bar = util::progress_bar(entry_index, *entry_count);
                let progress = if self.color {
                    format!(
                        "{bar}{padding} {}: {}\n",
                        job.filename.to_string().bold(),
                        "Running".cyan().bold()
                    )
                } else {
                    format!("{bar}{padding} {}: Running\n", job.filename)
                };
                all_progress.push_str(&progress);
            }
        }
        // If the number of running workers is greater that those displayed, we add the remaining
        // number of not displayed running.
        if running > self.max_running_displayed {
            all_progress.push_str(&format!(
                "...{} more\n",
                running - self.max_running_displayed
            ));
        }

        stderr.set_progress_bar(&all_progress);
    }

    /// Displays the completion of a job `result`.
    pub fn print_completed(&mut self, result: &JobResult, stderr: &mut Stderr) {
        if matches!(self.mode, Mode::Default) {
            return;
        }
        let count = result
            .hurl_result
            .entries
            .iter()
            .flat_map(|r| &r.calls)
            .count();
        let duration = result.hurl_result.time_in_ms;
        let message = if self.color {
            let state = if result.hurl_result.success {
                "Success".green().bold()
            } else {
                "Failure".red().bold()
            };
            let filename = result.job.filename.to_string().bold();
            format!("{filename}: {state} ({count} request(s) in {duration} ms)")
        } else {
            let state = if result.hurl_result.success {
                "Success"
            } else {
                "Failure"
            };
            let filename = &result.job.filename;
            format!("{filename}: {state} ({count} request(s) in {duration} ms)")
        };
        stderr.eprintln(&message);
    }

    pub fn allowed_update(&mut self) -> bool {
        self.throttle.allowed()
    }
}

impl Mode {
    pub fn new(test: bool, progress_bar: bool) -> Self {
        match (test, progress_bar) {
            (true, true) => Mode::TestWithProgress,
            (true, false) => Mode::TestWithoutProgress,
            _ => Mode::Default,
        }
    }
}

/// Records the instant when a progress bar is refreshed on the terminal.
/// We don't want to update the progress bar too often as it can cause excessive performance loss
/// just putting stuff onto the terminal. We also want to avoid flickering by not drawing anything
/// that goes away too quickly.
struct Throttle {
    /// Last time the progress bar has be refreshed on the terminal.
    last_update: Option<Instant>,
}

impl Throttle {
    /// Creates a new instances.
    fn new() -> Self {
        Throttle { last_update: None }
    }

    /// Returns `true` if there has been sufficient time elapsed since the last refresh.
    fn allowed(&mut self) -> bool {
        let interval = Duration::from_millis(100);
        let can_update = match self.last_update {
            None => true,
            Some(update) => update.elapsed() >= interval,
        };
        if can_update {
            self.update();
        }
        can_update
    }

    fn update(&mut self) {
        self.last_update = Some(Instant::now());
    }
}
