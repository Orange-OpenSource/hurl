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

/// The minimum duration between two progress bar redraw (to avoid flickering).
const UPDATE_INTERVAL: Duration = Duration::from_millis(100);

impl ParProgress {
    /// Creates a new instance.
    pub fn new(max_running_displayed: usize, mode: Mode, color: bool) -> Self {
        ParProgress {
            max_running_displayed,
            mode,
            color,
            throttle: Throttle::new(UPDATE_INTERVAL),
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

        let Some(progress) = build_progress(
            workers,
            completed,
            count,
            self.max_running_displayed,
            self.color,
        ) else {
            return;
        };

        stderr.set_progress_bar(&progress);
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

    /// Returns `true` if there has been sufficient time elapsed since the last progress bar
    /// refresh, `false` otherwise.
    pub fn can_update(&mut self) -> bool {
        self.throttle.allowed()
    }

    /// For the next progress bar update to be effectively drawn.
    pub fn force_next_update(&mut self) {
        self.throttle.reset();
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
    /// Refresh interval
    interval: Duration,
}

impl Throttle {
    /// Creates a new instances.
    fn new(interval: Duration) -> Self {
        Throttle {
            last_update: None,
            interval,
        }
    }

    /// Returns `true` if there has been sufficient time elapsed since the last refresh.
    fn allowed(&self) -> bool {
        match self.last_update {
            None => true,
            Some(update) => update.elapsed() >= self.interval,
        }
    }

    fn update(&mut self) {
        self.last_update = Some(Instant::now());
    }

    fn reset(&mut self) {
        self.last_update = None;
    }
}

/// Returns a progress string, given a list of `workers`, a number of `completed` jobs and the
/// total number of jobs.
///
/// `max_running_displayed` is used to limit the number of running progress bar. If more jobs are
/// running, a label "...x more" is displayed.
/// `color` is `true` when the returned progress string uses color.
fn build_progress(
    workers: &[(Worker, WorkerState)],
    completed: usize,
    count: usize,
    max_running_displayed: usize,
    color: bool,
) -> Option<String> {
    // Select the running workers to be displayed
    let mut workers = workers
        .iter()
        .filter(|(_, state)| matches!(state, WorkerState::Running { .. }))
        .collect::<Vec<_>>();
    if workers.is_empty() {
        return None;
    }

    // We sort the running workers by job sequence id, this way a given job will be displayed
    // on the same line, independently of the worker id.
    workers.sort_unstable_by_key(|(_, state)| match state {
        WorkerState::Running { job, .. } => job.seq,
        WorkerState::Idle => usize::MAX,
    });
    let running = workers.len();

    // We keep a reasonable number of worker to displayed, from the oldest to the newest.
    workers.truncate(max_running_displayed);

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
            let progress = if color {
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
    if running > max_running_displayed {
        all_progress.push_str(&format!("...{} more\n", running - max_running_displayed));
    }
    Some(all_progress)
}

#[cfg(test)]
mod tests {
    use crate::parallel::job::Job;
    use crate::parallel::progress::build_progress;
    use crate::parallel::runner::WorkerState;
    use crate::parallel::worker::{Worker, WorkerId};
    use crate::runner::{Input, RunnerOptionsBuilder};
    use crate::util::logger::LoggerOptionsBuilder;
    use std::collections::HashMap;
    use std::sync::{mpsc, Arc, Mutex};

    fn new_workers() -> (Worker, Worker, Worker, Worker, Worker) {
        let (tx_out, _) = mpsc::channel();
        let (_, rx_in) = mpsc::channel();
        let rx_in = Arc::new(Mutex::new(rx_in));

        let w0 = Worker::new(WorkerId::from(0), &tx_out, &rx_in);
        let w1 = Worker::new(WorkerId::from(1), &tx_out, &rx_in);
        let w2 = Worker::new(WorkerId::from(2), &tx_out, &rx_in);
        let w3 = Worker::new(WorkerId::from(3), &tx_out, &rx_in);
        let w4 = Worker::new(WorkerId::from(4), &tx_out, &rx_in);

        (w0, w1, w2, w3, w4)
    }

    fn new_jobs() -> Vec<Job> {
        let variables = HashMap::new();
        let runner_options = RunnerOptionsBuilder::default().build();
        let logger_options = LoggerOptionsBuilder::default().build();
        let files = [
            "a.hurl", "b.hurl", "c.hurl", "d.hurl", "e.hurl", "f.hurl", "g.hurl",
        ];
        files
            .iter()
            .enumerate()
            .map(|(index, file)| {
                Job::new(
                    &Input::new(file),
                    index,
                    &runner_options,
                    &variables,
                    &logger_options,
                )
            })
            .collect()
    }

    fn new_running_state(job: &Job, entry_index: usize, entry_count: usize) -> WorkerState {
        WorkerState::Running {
            job: job.clone(),
            entry_index,
            entry_count,
        }
    }

    #[test]
    fn all_workers_running() {
        let (w0, w1, w2, w3, w4) = new_workers();
        let jobs = new_jobs();
        let completed = 75;
        let total = 100;
        let max_displayed = 3;

        let mut workers = vec![
            (w0, WorkerState::Idle),
            (w1, WorkerState::Idle),
            (w2, WorkerState::Idle),
            (w3, WorkerState::Idle),
            (w4, WorkerState::Idle),
        ];

        let progress = build_progress(&workers, completed, total, max_displayed, false);
        assert!(progress.is_none());

        workers[0].1 = new_running_state(&jobs[0], 0, 10);
        workers[1].1 = new_running_state(&jobs[1], 0, 2);
        workers[2].1 = new_running_state(&jobs[2], 0, 5);
        workers[3].1 = new_running_state(&jobs[3], 0, 7);
        workers[4].1 = new_running_state(&jobs[4], 0, 4);

        let progress = build_progress(&workers, completed, total, max_displayed, false);
        assert_eq!(
            progress.unwrap(),
            "\
Executed files: 75/100 (75%)\n\
[>                       ] 1/10  a.hurl: Running\n\
[>                       ] 1/2   b.hurl: Running\n\
[>                       ] 1/5   c.hurl: Running\n\
...2 more\n\
"
        );

        workers[0].1 = new_running_state(&jobs[0], 5, 10);
        workers[1].1 = new_running_state(&jobs[1], 1, 2);
        workers[2].1 = new_running_state(&jobs[2], 2, 5);
        workers[3].1 = new_running_state(&jobs[3], 3, 7);
        workers[4].1 = new_running_state(&jobs[4], 1, 4);

        let progress = build_progress(&workers, completed, total, max_displayed, false);
        assert_eq!(
            progress.unwrap(),
            "\
Executed files: 75/100 (75%)\n\
[============>           ] 6/10  a.hurl: Running\n\
[============>           ] 2/2   b.hurl: Running\n\
[=========>              ] 3/5   c.hurl: Running\n\
...2 more\n\
"
        );

        workers[0].1 = new_running_state(&jobs[0], 9, 10);
        workers[1].1 = new_running_state(&jobs[5], 0, 6);
        workers[2].1 = new_running_state(&jobs[2], 4, 5);
        workers[3].1 = new_running_state(&jobs[3], 5, 7);
        workers[4].1 = new_running_state(&jobs[4], 2, 4);

        let progress = build_progress(&workers, completed, total, max_displayed, false);
        assert_eq!(
            progress.unwrap(),
            "\
Executed files: 75/100 (75%)\n\
[=====================>  ] 10/10 a.hurl: Running\n\
[===================>    ] 5/5   c.hurl: Running\n\
[=================>      ] 6/7   d.hurl: Running\n\
...2 more\n\
"
        );

        workers[0].1 = WorkerState::Idle;
        workers[1].1 = new_running_state(&jobs[5], 2, 6);
        workers[2].1 = WorkerState::Idle;
        workers[3].1 = WorkerState::Idle;
        workers[4].1 = new_running_state(&jobs[4], 3, 4);

        let progress = build_progress(&workers, completed, total, max_displayed, false);
        assert_eq!(
            progress.unwrap(),
            "\
Executed files: 75/100 (75%)\n\
[==================>     ] 4/4 e.hurl: Running\n\
[========>               ] 3/6 f.hurl: Running\n\
"
        );

        workers[0].1 = WorkerState::Idle;
        workers[1].1 = new_running_state(&jobs[5], 5, 6);
        workers[2].1 = WorkerState::Idle;
        workers[3].1 = WorkerState::Idle;
        workers[4].1 = WorkerState::Idle;

        let progress = build_progress(&workers, completed, total, max_displayed, false);
        assert_eq!(
            progress.unwrap(),
            "\
Executed files: 75/100 (75%)\n\
[====================>   ] 6/6 f.hurl: Running\n\
"
        );
    }
}
