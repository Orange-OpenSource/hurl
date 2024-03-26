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

use crate::runner::{EventListener, HurlResult};
use crate::util;
use crate::util::term::Stderr;

/// Implements a progress report specifically used for sequential execution of multiple Hurl files,
///
/// In `--test` mode, this progress can display a progress bar or not. This progress uses standard
/// error to report information, us
pub struct SeqProgress {
    /// Filename of the running Hurl file.
    filename: String,
    /// 0-based index of the running Hurl file.
    current_file: usize,
    /// The total number of files that will be executed sequentially.
    total_files: usize,
    /// Mode of the progress reporter
    mode: Mode,
    /// The standard error uses color or not.
    color: bool,
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

impl SeqProgress {
    /// Creates a new instance.
    pub fn new(
        filename: &str,
        current_file: usize,
        total_files: usize,
        mode: Mode,
        color: bool,
    ) -> Self {
        SeqProgress {
            filename: filename.to_string(),
            current_file,
            total_files,
            mode,
            color,
        }
    }
}

impl SeqProgress {
    /// Displays start of the entry processing on `stderr`.
    pub fn print_test_start(&self, stderr: &mut Stderr) {
        if matches!(self.mode, Mode::Default) {
            return;
        }

        let current = self.current_file + 1;
        let total = self.total_files;
        let message = if self.color {
            format!(
                "{}: {} [{current}/{total}]",
                self.filename.bold(),
                "Running".cyan().bold()
            )
        } else {
            format!("{}: Running [{current}/{total}]", self.filename)
        };
        stderr.eprintln(&message);
    }

    /// Displays the test result of an entry run, on `stderr`.
    pub fn print_test_completed(&self, result: &HurlResult, stderr: &mut Stderr) {
        if matches!(self.mode, Mode::Default) {
            return;
        }

        let count = result.entries.iter().flat_map(|r| &r.calls).count();
        let duration = result.time_in_ms;
        let message = if self.color {
            let state = if result.success {
                "Success".green().bold()
            } else {
                "Failure".red().bold()
            };
            let filename = self.filename.bold();
            format!("{filename}: {state} ({count} request(s) in {duration} ms)")
        } else {
            let state = if result.success { "Success" } else { "Failure" };
            let filename = &self.filename;
            format!("{filename}: {state} ({count} request(s) in {duration} ms)")
        };
        stderr.eprintln(&message);
    }
}

impl EventListener for SeqProgress {
    fn on_running(&self, entry_index: usize, entry_count: usize, stderr: &mut Stderr) {
        if !matches!(self.mode, Mode::TestWithProgress) {
            return;
        }
        let bar = util::progress_bar(entry_index + 1, entry_count);
        stderr.set_progress_bar(&format!(" {bar}\r"));
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
