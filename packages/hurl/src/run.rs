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
use std::cmp::min;
use std::path::Path;

use crate::cli::options::CliOptions;
use crate::cli::CliError;
use crate::{cli, HurlRun};

use hurl::parallel::job::{Job, JobResult};
use hurl::parallel::runner::ParallelRunner;
use hurl::runner::Input;
use hurl::{output, runner};

/// Runs Hurl `files` sequentially, given a current directory and command-line options (see
/// [`crate::cli::options::CliOptions`]). This function returns a list of [`HurlRun`] results or
/// an error.
pub fn run_seq(
    files: &[Input],
    current_dir: &Path,
    options: &CliOptions,
) -> Result<Vec<HurlRun>, CliError> {
    let mut runs = vec![];

    for (current, filename) in files.iter().enumerate() {
        let content = filename.read_to_string();
        let content = match content {
            Ok(c) => c,
            Err(error) => {
                let error = CliError::IO(format!("Issue reading from {filename}: {error}"));
                return Err(error);
            }
        };
        let total = files.len();
        let variables = &options.variables;
        let runner_options = options.to_runner_options(filename, current_dir);
        let logger_options = options.to_logger_options(filename, current, total);

        // Run our Hurl file now, we can only fail if there is a parsing error.
        // The parsing error is displayed in the `execute` call, that's why we gobble the error
        // string.
        let Ok(hurl_result) = runner::run(&content, &runner_options, variables, &logger_options)
        else {
            return Err(CliError::Parsing);
        };

        let success = hurl_result.success;

        // We can output the result, either the raw body or a structured JSON representation.
        let output_body = success
            && !options.interactive
            && matches!(options.output_type, cli::OutputType::ResponseBody);
        if output_body {
            if let Some(last_entry) = hurl_result.entries.last() {
                let include_headers = options.include;
                let result =
                    output::write_body(last_entry, include_headers, options.color, &options.output);
                if let Err(e) = result {
                    return Err(CliError::Runtime(e.to_string()));
                }
            }
        }
        if matches!(options.output_type, cli::OutputType::Json) {
            let result =
                output::write_json(&hurl_result, &content, filename, options.output.as_ref());
            if let Err(e) = result {
                return Err(CliError::Runtime(e.to_string()));
            }
        }

        let run = HurlRun {
            content,
            filename: filename.clone(),
            hurl_result,
        };
        runs.push(run);
    }

    Ok(runs)
}

/// Runs Hurl `files` in parallel, given a current directory and command-line options (see
/// [`crate::cli::options::CliOptions`]). This function returns a list of [`HurlRun`] results or
/// an error.
pub fn run_par(
    files: &[Input],
    current_dir: &Path,
    options: &CliOptions,
) -> Result<Vec<HurlRun>, CliError> {
    // TODO: default worker count to [`std::thread::available_parallelism`]
    let workers_count = min(files.len(), options.max_workers);
    let variables = &options.variables;

    let jobs = files
        .iter()
        .enumerate()
        .map(|(seq, input)| {
            let runner_options = options.to_runner_options(input, current_dir);
            let logger_options = options.to_logger_options(input, seq, files.len());
            Job::new(input, seq, &runner_options, variables, &logger_options)
        })
        .collect::<Vec<_>>();

    let mut runner = ParallelRunner::new(
        workers_count,
        options.test,
        options.progress_bar,
        options.color,
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
