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
use std::cmp::min;
use std::path::Path;

use hurl::parallel::job::{Job, JobResult};
use hurl::parallel::runner::ParallelRunner;
use hurl::runner::{HurlResult, Output, VariableSet};
use hurl::util::term::{Stdout, WriteMode};
use hurl::{output, parallel, runner};
use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::input::Input;
use hurl_core::typing::Count;

use crate::cli::options::CliOptions;
use crate::cli::CliError;
use crate::{cli, HurlRun};

/// Runs Hurl `files` sequentially, given a current directory and command-line options (see
/// [`crate::cli::options::CliOptions`]). This function returns a list of [`HurlRun`] results or
/// an error.
pub fn run_seq(
    files: &[Input],
    current_dir: &Path,
    options: &CliOptions,
) -> Result<Vec<HurlRun>, CliError> {
    let mut runs = vec![];

    let repeat = options.repeat.unwrap_or(Count::Finite(1));
    let queue = InputQueue::new(files, repeat);

    // When dumped HTTP responses, we truncate existing output file on first save, then append
    // it on subsequent write.
    let mut append = false;

    for filename in queue {
        let content = filename.read_to_string();
        let content = match content {
            Ok(c) => c,
            Err(error) => {
                let error = CliError::IO(format!("Issue reading from {filename}: {error}"));
                return Err(error);
            }
        };
        let mut variables = VariableSet::from(&options.variables);
        // By runtime, construction, there is no two secrets having the same name so we can safely
        // insert all the secrets in the variable set.
        options.secrets.iter().for_each(|(name, value)| {
            variables
                .insert_secret(name.clone(), value.clone())
                .unwrap();
        });
        let runner_options = options.to_runner_options(&filename, current_dir);
        let logger_options = options.to_logger_options();

        // Run our Hurl file now, we can only fail if there is a parsing error.
        // The parsing error is displayed in the `execute` call, that's why we gobble the error
        // string.
        let Ok(hurl_result) = runner::run(
            &content,
            Some(&filename),
            &runner_options,
            &variables,
            &logger_options,
        ) else {
            return Err(CliError::Parsing);
        };

        // We can output the result, either the last raw body response or a structured JSON
        // representation of the full Hurl result.
        // In sequential run, we use an immediate (non-buffered) standard output.
        // When we output to a file, the first time we truncate the output file, and we append it
        // for subsequent writes.
        let mut stdout = Stdout::new(WriteMode::Immediate);
        print_output(
            &hurl_result,
            &content,
            &filename,
            options,
            &mut stdout,
            append,
        )?;
        append = true;

        let run = HurlRun {
            content,
            filename: filename.clone(),
            hurl_result,
        };
        runs.push(run);
    }

    Ok(runs)
}

/// Prints a `hurl_result` either as a raw HTTP response (last body of the run), or in a structured
/// JSON way.
///
/// If options contains an output file, the result is dumped to this file, or `stdout` is used. If
/// `append` is true, any existing file will be appended instead of being truncated. `content` (the
/// source string), `filename` (the source file) are used in JSON output (for errors and asserts
/// construction).
fn print_output(
    hurl_result: &HurlResult,
    content: &str,
    filename: &Input,
    options: &CliOptions,
    stdout: &mut Stdout,
    append: bool,
) -> Result<(), CliError> {
    let output_last_body = hurl_result.success
        && !options.interactive
        && matches!(options.output_type, cli::OutputType::ResponseBody);
    if output_last_body {
        let result = output::write_last_body(
            hurl_result,
            options.include,
            options.color,
            options.output.as_ref(),
            stdout,
            append,
        );
        if let Err(e) = result {
            return Err(CliError::Runtime(e.to_string(
                &filename.to_string(),
                content,
                None,
                OutputFormat::Terminal(options.color),
            )));
        }
    }
    if matches!(options.output_type, cli::OutputType::Json) {
        let result = output::write_json(
            hurl_result,
            content,
            filename,
            options.output.as_ref(),
            stdout,
            append,
        );
        if let Err(e) = result {
            let filename = if let Some(Output::File(filename)) = &options.output {
                filename.display().to_string()
            } else {
                "stdout".to_string()
            };
            let message = format!("{filename} can not be written ({})", e);
            return Err(CliError::Runtime(message));
        }
    }
    Ok(())
}

/// Runs Hurl `files` in parallel, given a current directory and command-line options (see
/// [`crate::cli::options::CliOptions`]). This function returns a list of [`HurlRun`] results or
/// an error.
pub fn run_par(
    files: &[Input],
    current_dir: &Path,
    options: &CliOptions,
    workers_count: usize,
) -> Result<Vec<HurlRun>, CliError> {
    // We're going to use the right numbers of workers. We don't need to use more workers than there
    // are input files (repeat option act as if we're dealing with a multiplied number of files)
    let workers_count = match options.repeat {
        Some(Count::Finite(n)) => min(files.len() * n, workers_count),
        Some(Count::Infinite) => workers_count,
        None => min(files.len(), workers_count),
    };
    let mut variables = VariableSet::from(&options.variables);
    // By runtime, construction, there is no two secrets having the same name so we can safely
    // insert all the secrets in the variable set.
    options.secrets.iter().for_each(|(name, value)| {
        variables
            .insert_secret(name.clone(), value.clone())
            .unwrap();
    });
    let output_type = options
        .output_type
        .to_output_type(options.include, options.color);
    let max_width = terminal_size::terminal_size().map(|(w, _)| w.0 as usize);

    let jobs = files
        .iter()
        .enumerate()
        .map(|(seq, input)| {
            let runner_options = options.to_runner_options(input, current_dir);
            let logger_options = options.to_logger_options();
            Job::new(input, seq, &runner_options, &variables, &logger_options)
        })
        .collect::<Vec<_>>();

    let mut runner = ParallelRunner::new(
        workers_count,
        output_type,
        options.repeat.unwrap_or(Count::Finite(1)),
        options.test,
        options.progress_bar,
        options.color,
        max_width,
    );
    let results = runner.run(&jobs)?;
    let results = results.into_iter().map(HurlRun::from).collect();
    Ok(results)
}

impl From<JobResult> for HurlRun {
    fn from(job_result: JobResult) -> Self {
        HurlRun {
            content: job_result.content,
            filename: job_result.job.filename,
            hurl_result: job_result.hurl_result,
        }
    }
}

impl cli::OutputType {
    fn to_output_type(&self, include_headers: bool, color: bool) -> parallel::runner::OutputType {
        match self {
            cli::OutputType::ResponseBody => parallel::runner::OutputType::ResponseBody {
                include_headers,
                color,
            },
            cli::OutputType::Json => parallel::runner::OutputType::Json,
            cli::OutputType::NoOutput => parallel::runner::OutputType::NoOutput,
        }
    }
}

/// An input queue to manage a queue of [`Input`].
///
/// The queue implements [`Iterator`] trait, and can return a new input to use each time its
/// `next` method is called. This queue can repeat its input sequence a certain number of times, or
/// can loop forever.
pub struct InputQueue<'input> {
    /// The input list.
    inputs: &'input [Input],
    /// Current index of the input, referencing the input list.
    index: usize,
    /// Repeat mode of this queue (finite or infinite).
    repeat: Count,
    /// Current index of the repeat.
    repeat_index: usize,
}

impl<'input> InputQueue<'input> {
    /// Create a new queue, with a list of `inputs` and a `repeat` mode.
    pub fn new(inputs: &'input [Input], repeat: Count) -> Self {
        InputQueue {
            inputs,
            index: 0,
            repeat,
            repeat_index: 0,
        }
    }

    /// Returns a new input at the given `index`.
    fn input_at(&self, index: usize) -> Input {
        self.inputs[index].clone()
    }
}

impl Iterator for InputQueue<'_> {
    type Item = Input;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inputs.len() {
            self.repeat_index = self.repeat_index.checked_add(1).unwrap_or(0);
            match self.repeat {
                Count::Finite(n) => {
                    if self.repeat_index >= n {
                        None
                    } else {
                        self.index = 1;
                        Some(self.input_at(0))
                    }
                }
                Count::Infinite => {
                    self.index = 1;
                    Some(self.input_at(0))
                }
            }
        } else {
            self.index += 1;
            Some(self.input_at(self.index - 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::input::Input;
    use hurl_core::typing::Count;

    use crate::run::InputQueue;

    #[test]
    fn input_queue_is_finite() {
        let files = [Input::new("a"), Input::new("b"), Input::new("c")];

        let mut queue = InputQueue::new(&files, Count::Finite(4));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("b")));
        assert_eq!(queue.next(), Some(Input::new("c")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("b")));
        assert_eq!(queue.next(), Some(Input::new("c")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("b")));
        assert_eq!(queue.next(), Some(Input::new("c")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("b")));
        assert_eq!(queue.next(), Some(Input::new("c")));
        assert_eq!(queue.next(), None);
    }

    #[test]
    fn input_queue_is_infinite() {
        let files = [Input::new("a")];

        let mut queue = InputQueue::new(&files, Count::Infinite);
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        assert_eq!(queue.next(), Some(Input::new("a")));
        // etc...
    }
}
