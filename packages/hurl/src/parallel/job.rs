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
use hurl_core::input::Input;
use hurl_core::typing::Count;

use crate::runner::{HurlResult, RunnerOptions, VariableSet};
use crate::util::logger::LoggerOptions;

/// Represents the job to run. A job instance groups the input data to execute, and has no methods
/// associated to it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    /// The Hurl source content
    pub filename: Input,
    /// The options to run this file.
    pub runner_options: RunnerOptions,
    /// Set of variables injected in the Hurl file
    pub variables: VariableSet,
    /// The logger options for this run
    pub logger_options: LoggerOptions,
    /// The job 0-based index in the jobs list
    pub seq: usize,
}

impl Job {
    /// Creates a new job.
    pub fn new(
        filename: &Input,
        seq: usize,
        runner_options: &RunnerOptions,
        variables: &VariableSet,
        logger_options: &LoggerOptions,
    ) -> Self {
        Job {
            filename: filename.clone(),
            runner_options: runner_options.clone(),
            variables: variables.clone(),
            logger_options: logger_options.clone(),
            seq,
        }
    }
}

pub struct JobResult {
    /// The job corresponding to this job result.
    pub job: Job,
    /// The source content of the job.
    pub content: String,
    /// The result of execution of the job.
    pub hurl_result: HurlResult,
}

impl JobResult {
    /// Creates a new job result.
    pub fn new(job: Job, content: String, hurl_result: HurlResult) -> Self {
        JobResult {
            job,
            content,
            hurl_result,
        }
    }
}

/// A job queue to manage a queue of [`Job`].
///
/// The job queue implements [`Iterator`] trait, and can return a new job to use each time its
/// `next` method is called. This queue can repeat its input sequence a certain number of times, or
/// can loop forever.
pub struct JobQueue<'job> {
    /// The input jobs list.
    jobs: &'job [Job],
    /// Current index of the job, referencing the input job list.
    index: usize,
    /// Repeat mode of this queue (finite or infinite).
    repeat: Count,
    /// Current index of the repeat.
    repeat_index: usize,
}

impl<'job> JobQueue<'job> {
    /// Create a new queue, with a list of `jobs` and a `repeat` mode.
    pub fn new(jobs: &'job [Job], repeat: Count) -> Self {
        JobQueue {
            jobs,
            index: 0,
            repeat,
            repeat_index: 0,
        }
    }

    /// Returns the effective number of jobs.
    ///
    /// If queue is created in loop forever mode ([`Repeat::Forever`]), returns `None`.
    pub fn jobs_count(&self) -> Option<usize> {
        match self.repeat {
            Count::Finite(n) => Some(self.jobs.len() * n),
            Count::Infinite => None,
        }
    }

    /// Returns a new job at the given `index`.
    fn job_at(&self, index: usize) -> Job {
        let mut job = self.jobs[index].clone();
        // When we're repeating a sequence, we clone an original job and give it a proper
        // sequence number relative to the current `repeat_index`.
        job.seq = self.jobs[index].seq + (self.jobs.len() * self.repeat_index);
        job
    }
}

impl Iterator for JobQueue<'_> {
    type Item = Job;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.jobs.len() {
            self.repeat_index = self.repeat_index.checked_add(1).unwrap_or(0);
            match self.repeat {
                Count::Finite(n) => {
                    if self.repeat_index >= n {
                        None
                    } else {
                        self.index = 1;
                        Some(self.job_at(0))
                    }
                }
                Count::Infinite => {
                    self.index = 1;
                    Some(self.job_at(0))
                }
            }
        } else {
            self.index += 1;
            Some(self.job_at(self.index - 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::input::Input;
    use hurl_core::typing::Count;

    use crate::parallel::job::{Job, JobQueue};
    use crate::runner::{RunnerOptionsBuilder, VariableSet};
    use crate::util::logger::LoggerOptionsBuilder;

    fn new_job(file: &str, index: usize) -> Job {
        let variables = VariableSet::new();
        let runner_options = RunnerOptionsBuilder::default().build();
        let logger_options = LoggerOptionsBuilder::default().build();
        Job::new(
            &Input::new(file),
            index,
            &runner_options,
            &variables,
            &logger_options,
        )
    }

    #[test]
    fn job_queue_is_finite() {
        let jobs = [
            new_job("a.hurl", 0),
            new_job("b.hurl", 1),
            new_job("c.hurl", 2),
        ];

        let mut queue = JobQueue::new(&jobs, Count::Finite(2));

        assert_eq!(queue.next(), Some(new_job("a.hurl", 0)));
        assert_eq!(queue.next(), Some(new_job("b.hurl", 1)));
        assert_eq!(queue.next(), Some(new_job("c.hurl", 2)));
        assert_eq!(queue.next(), Some(new_job("a.hurl", 3)));
        assert_eq!(queue.next(), Some(new_job("b.hurl", 4)));
        assert_eq!(queue.next(), Some(new_job("c.hurl", 5)));
        assert_eq!(queue.next(), None);

        assert_eq!(queue.jobs_count(), Some(6));
    }

    #[test]
    fn input_queue_is_infinite() {
        let jobs = [new_job("foo.hurl", 0)];

        let mut queue = JobQueue::new(&jobs, Count::Infinite);
        assert_eq!(queue.next(), Some(new_job("foo.hurl", 0)));
        assert_eq!(queue.next(), Some(new_job("foo.hurl", 1)));
        assert_eq!(queue.next(), Some(new_job("foo.hurl", 2)));
        assert_eq!(queue.next(), Some(new_job("foo.hurl", 3)));
        assert_eq!(queue.next(), Some(new_job("foo.hurl", 4)));
        // etc...

        assert_eq!(queue.jobs_count(), None);
    }
}
