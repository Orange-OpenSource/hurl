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
use crate::parallel::job::JobResult;
use crate::parallel::runner::WorkerState;
use crate::parallel::worker::Worker;
use crate::util::term::Stderr;
use colored::Colorize;

/// A progress bar to display advancement of parallel runs execution.
pub struct ParProgressBar {
    color: bool,
    max_running: usize,
}

impl ParProgressBar {
    pub fn new(color: bool, max_running: usize) -> Self {
        ParProgressBar { color, max_running }
    }

    /// Clear the progress bar.
    pub fn clear_progress(&self, stderr: &mut Stderr) {
        stderr.clear_progress();
    }

    /// Displays progression of `workers` on standard error `stderr`.
    pub fn update_progress(&self, workers: &[(Worker, WorkerState)], stderr: &mut Stderr) {
        let mut progress = String::new();
        let mut running = 0;
        for (_, state) in workers {
            if let WorkerState::Running(job) = state {
                let message = if self.color {
                    format!(
                        "{}: {}\n",
                        job.filename.to_string().bold(),
                        "Running".cyan().bold()
                    )
                } else {
                    format!("{}: Running\n", job.filename)
                };
                progress.push_str(&message);
                running += 1;
                if running >= self.max_running {
                    break;
                }
            }
        }
        stderr.clear_progress();
        stderr.set_progress(&progress);
    }

    /// Displays the completion of a job `result`.
    pub fn print_completed(&self, result: &JobResult, stderr: &mut Stderr) {
        let message = if self.color {
            let state = if result.hurl_result.success {
                "Success".green().bold()
            } else {
                "Failure".red().bold()
            };
            format!("{}: {state}", result.job.filename.to_string().bold())
        } else {
            let state = if result.hurl_result.success {
                "Success"
            } else {
                "Failure"
            };
            format!("{}: {state}", result.job.filename)
        };
        stderr.eprintln(&message);
    }
}
