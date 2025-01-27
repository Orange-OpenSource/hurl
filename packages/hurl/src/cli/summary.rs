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
use std::time::Duration;

use crate::HurlRun;

/// Returns the text summary of this Hurl `runs`.
///
/// This is used in `--test`mode.
pub fn summary(runs: &[HurlRun], duration: Duration) -> String {
    let total_files = runs.len();
    let total_requests = requests_count(runs);
    let duration_in_ms = duration.as_millis() as f64;
    let requests_rate = 1000.0 * (total_requests as f64) / duration_in_ms;

    let success_files = runs.iter().filter(|r| r.hurl_result.success).count();
    let success_percent = 100.0 * success_files as f32 / total_files as f32;
    let failed = total_files - success_files;
    let failed_percent = 100.0 * failed as f32 / total_files as f32;
    format!(
        "--------------------------------------------------------------------------------\n\
             Executed files:    {total_files}\n\
             Executed requests: {total_requests} ({requests_rate:.1}/s)\n\
             Succeeded files:   {success_files} ({success_percent:.1}%)\n\
             Failed files:      {failed} ({failed_percent:.1}%)\n\
             Duration:          {duration_in_ms} ms\n"
    )
}

/// Returns the total number of executed HTTP requests in this list of `runs`.
fn requests_count(runs: &[HurlRun]) -> usize {
    // Each entry has a list of calls. Each call is a pair of HTTP request / response
    // so, for a given entry, the number of executed requests is the number of calls. This count
    // also the retries.
    runs.iter()
        .map(|r| {
            r.hurl_result
                .entries
                .iter()
                .map(|e| e.calls.len())
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use hurl::http::CurlCmd;
    use hurl::runner::{EntryResult, HurlResult};
    use hurl_core::ast::SourceInfo;
    use hurl_core::input::Input;
    use hurl_core::reader::Pos;

    use super::*;

    #[test]
    fn create_run_summary() {
        fn new_run(success: bool, entries_count: usize) -> HurlRun {
            let dummy_entry = EntryResult {
                entry_index: 0,
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![],
                transfer_duration: Duration::from_millis(0),
                compressed: false,
                curl_cmd: CurlCmd::default(),
            };
            HurlRun {
                content: String::new(),
                filename: Input::new(""),
                hurl_result: HurlResult {
                    entries: vec![dummy_entry; entries_count],
                    success,
                    ..Default::default()
                },
            }
        }

        let runs = vec![new_run(true, 10), new_run(true, 20), new_run(true, 4)];
        let duration = Duration::from_millis(128);
        let s = summary(&runs, duration);
        assert_eq!(
            s,
            "--------------------------------------------------------------------------------\n\
             Executed files:    3\n\
             Executed requests: 0 (0.0/s)\n\
             Succeeded files:   3 (100.0%)\n\
             Failed files:      0 (0.0%)\n\
             Duration:          128 ms\n"
        );

        let runs = vec![new_run(true, 10), new_run(false, 10), new_run(true, 40)];
        let duration = Duration::from_millis(200);
        let s = summary(&runs, duration);
        assert_eq!(
            s,
            "--------------------------------------------------------------------------------\n\
            Executed files:    3\n\
            Executed requests: 0 (0.0/s)\n\
            Succeeded files:   2 (66.7%)\n\
            Failed files:      1 (33.3%)\n\
            Duration:          200 ms\n"
        );
    }
}
