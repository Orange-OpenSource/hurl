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
use crate::runner::HurlResult;
use crate::util::term::Stderr;
use colored::Colorize;

/// This trait represents the operation progress of the execution of one Hurl file.
pub trait Progress {
    /// Call at the beginning of the execution of a Hurl file, just before the parsing of content.
    /// Should be called only once during the execution of a file.
    fn on_start(&self, stderr: &mut Stderr);

    /// Call when starting a new entry, `current` is the entry 0-based index in the Hurl file,
    /// and `total` is the total number of entries in the Hurl file.
    fn on_entry(&self, current: usize, total: usize, stderr: &mut Stderr);

    /// Call when an Hurl file has been executed, whether the run is successful or failed. `result`
    /// can be used to check the success of the run.
    /// Note: a parsing error will only trigger [`on_start`] and not [`on_complete`], as we consider
    /// that there hasn't been any execution.
    fn on_completed(&self, result: &HurlResult, stderr: &mut Stderr);
}

/// Implements a progress report specifically used for sequential execution of multiple Hurl files,
/// in `--test` mode.
/// This progress bar uses standard error to report information.
pub struct SeqProgress {
    /// Filename of the running Hurl file.
    filename: String,
    /// 0-based index of the running Hurl file.
    current_file: usize,
    /// The total number of files that will be executed sequentially.
    total_files: usize,
    /// Report start and completion event.
    test: bool,
    /// Show a progress bar or not (usually `true` in interactive mode)
    progress_bar: bool,
    /// Is the progress bar using color.
    color: bool,
}

impl SeqProgress {
    /// Creates a new instance.
    pub fn new(
        filename: &str,
        current_file: usize,
        total_files: usize,
        test: bool,
        progress_bar: bool,
        color: bool,
    ) -> Self {
        SeqProgress {
            filename: filename.to_string(),
            current_file,
            total_files,
            test,
            progress_bar,
            color,
        }
    }
}

impl Progress for SeqProgress {
    fn on_start(&self, stderr: &mut Stderr) {
        if !self.test {
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

    fn on_entry(&self, current: usize, total: usize, stderr: &mut Stderr) {
        if !self.progress_bar {
            return;
        }
        let progress = progress_string(current + 1, total);
        stderr.set_progress(&format!(" {progress}\r"));
    }

    fn on_completed(&self, result: &HurlResult, stderr: &mut Stderr) {
        if !self.test {
            return;
        }
        let message = if self.color {
            let state = if result.success {
                "Success".green().bold()
            } else {
                "Failure".red().bold()
            };
            let count = result.entries.iter().flat_map(|r| &r.calls).count();
            format!(
                "{}: {} ({} request(s) in {} ms)",
                self.filename.bold(),
                state,
                count,
                result.time_in_ms
            )
        } else {
            let state = if result.success { "Success" } else { "Failure" };
            let count = result.entries.iter().flat_map(|r| &r.calls).count();
            format!(
                "{}: {} ({} request(s) in {} ms)",
                self.filename, state, count, result.time_in_ms
            )
        };
        stderr.eprintln(&message);
    }
}

/// Returns the progress string with the current entry at `entry_index` (1-based index).
fn progress_string(entry_index: usize, count: usize) -> String {
    const WIDTH: usize = 24;
    // We report the number of entries already processed.
    let progress = (entry_index - 1) as f64 / count as f64;
    let col = (progress * WIDTH as f64) as usize;
    let completed = if col > 0 {
        "=".repeat(col)
    } else {
        String::new()
    };
    let void = " ".repeat(WIDTH - col - 1);
    format!("[{completed}>{void}] {entry_index}/{count}")
}

#[cfg(test)]
mod tests {
    use crate::runner::progress::progress_string;

    #[rustfmt::skip]
    #[test]
    fn test_progress_string() {
        // Progress strings with 20 entries:
        assert_eq!(progress_string(1,  20), "[>                       ] 1/20");
        assert_eq!(progress_string(2,  20), "[=>                      ] 2/20");
        assert_eq!(progress_string(5,  20), "[====>                   ] 5/20");
        assert_eq!(progress_string(10, 20), "[==========>             ] 10/20");
        assert_eq!(progress_string(15, 20), "[================>       ] 15/20");
        assert_eq!(progress_string(20, 20), "[======================> ] 20/20");

        // Progress strings with 3 entries:
        assert_eq!(progress_string(1, 3), "[>                       ] 1/3");
        assert_eq!(progress_string(2, 3), "[========>               ] 2/3");
        assert_eq!(progress_string(3, 3), "[================>       ] 3/3");

        // Progress strings with 1 entries:
        assert_eq!(progress_string(1, 1), "[>                       ] 1/1");
    }
}
