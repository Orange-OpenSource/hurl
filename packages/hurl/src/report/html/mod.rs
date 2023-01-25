/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

//! HTML report

use crate::cli::CliError;
use crate::runner::HurlResult;
use crate::util::path;
use chrono::{DateTime, Local};
use std::io::Write;
use std::path::Path;

/// The test result to be displayed in an HTML page
///
/// The filename has been [canonicalized] (https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.canonicalize)
/// and does not need to exist in the filesystem
#[derive(Clone, Debug, PartialEq, Eq)]
struct HTMLResult {
    pub filename: String,
    pub time_in_ms: u128,
    pub success: bool,
}

/// Creates and HTML report for this list of [`HurlResult`] at `dir_path`/index.html.
///
/// If the report already exists, results are merged.
pub fn write_report(dir_path: &Path, hurl_results: &[&HurlResult]) -> Result<(), CliError> {
    let index_path = dir_path.join("index.html");
    let mut results = parse_html(&index_path)?;
    for result in hurl_results.iter() {
        let html_result = HTMLResult {
            filename: path::canonicalize_filename(&result.filename),
            time_in_ms: result.time_in_ms,
            success: result.success,
        };
        results.push(html_result);
    }
    let now: DateTime<Local> = Local::now();
    let s = create_html_index(&now.to_rfc2822(), &results);

    let file_path = index_path;
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            return Err(CliError {
                message: format!("Issue writing to {}: {:?}", file_path.display(), why),
            });
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(s.as_bytes()) {
        return Err(CliError {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        });
    }

    let file_path = dir_path.join("report.css");
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            return Err(CliError {
                message: format!("Issue writing to {}: {:?}", file_path.display(), why),
            });
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(include_bytes!("../report.css")) {
        return Err(CliError {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        });
    }
    Ok(())
}

fn parse_html(path: &Path) -> Result<Vec<HTMLResult>, CliError> {
    if path.exists() {
        let s = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(why) => {
                return Err(CliError {
                    message: format!("Issue reading {} to string to {:?}", path.display(), why),
                });
            }
        };
        Ok(parse_html_report(&s))
    } else {
        Ok(vec![])
    }
}

fn parse_html_report(html: &str) -> Vec<HTMLResult> {
    let re = regex::Regex::new(
        r#"(?x)
        data-duration="(?P<time_in_ms>\d+)"
        \s+
        data-status="(?P<status>[a-z]+)"
        \s+
        data-filename="(?P<filename>[A-Za-z0-9_./-]+)"
    "#,
    )
    .unwrap();
    re.captures_iter(html)
        .map(|cap| {
            let filename = cap["filename"].to_string();
            // The HTML filename is using a relative path relatively in the report
            // to make the report portable
            // But the original Hurl file is really an absolute file
            let time_in_ms = cap["time_in_ms"].to_string().parse().unwrap();
            let success = &cap["status"] == "success";
            HTMLResult {
                filename,
                time_in_ms,
                success,
            }
        })
        .collect::<Vec<HTMLResult>>()
}

fn percentage(count: usize, total: usize) -> String {
    format!("{:.1}%", (count as f32 * 100.0) / total as f32)
}

fn create_html_index(now: &str, hurl_results: &[HTMLResult]) -> String {
    let count_total = hurl_results.len();
    let count_failure = hurl_results.iter().filter(|result| !result.success).count();
    let count_success = hurl_results.iter().filter(|result| result.success).count();
    let percentage_success = percentage(count_success, count_total);
    let percentage_failure = percentage(count_failure, count_total);

    let rows = hurl_results
        .iter()
        .map(create_html_table_row)
        .collect::<Vec<String>>()
        .join("");

    format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <title>Test Report</title>
        <link rel="stylesheet" type="text/css" href="report.css">
    </head>
    <body>
        <h2>Test Report</h2>
        <div class="summary">
            <div class="date">{now}</div>
            <div class="count">Executed: {count_total} (100%)</div>
            <div class="count">Succeeded: {count_success} ({percentage_success})</div>
            <div class="count">Failed: {count_failure} ({percentage_failure})</div>
        </div>
        <table>
            <thead>
                <td>File</td>
                <td>Status</td>
                <td>Duration</td>
            </thead>
            <tbody>
{rows}
            </tbody>
        </table>
    </body>
</html>
"#
    )
}

fn create_html_table_row(result: &HTMLResult) -> String {
    let status = if result.success {
        "success".to_string()
    } else {
        "failure".to_string()
    };
    let duration_in_ms = result.time_in_ms;
    let duration_in_s = result.time_in_ms as f64 / 1000.0;
    let filename = &result.filename;

    format!(
        r#"<tr class="{status}" data-duration="{duration_in_ms}" data-status="{status}" data-filename="{filename}">
    <td><a href="{filename}.html">{filename}</a></td>
    <td>{status}</td>
    <td>{duration_in_s}</td>
</tr>
"#
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_percentage() {
        assert_eq!(percentage(100, 100), "100.0%".to_string());
        assert_eq!(percentage(66, 99), "66.7%".to_string());
        assert_eq!(percentage(33, 99), "33.3%".to_string());
    }

    #[test]
    fn test_parse_html_report() {
        let html = r#"<html>
          <body>
            <h2>Hurl Report</h2>
            <table>
              <tbody>
                <tr class="success" data-duration="100" data-status="success" data-filename="tests/hello.hurl">
                  <td><a href="tests/hello.hurl.html">tests/hello.hurl</a></td>
                  <td>success</td>
                  <td>0.1s</td>
                </tr>
                <tr class="failure" data-duration="200" data-status="failure" data-filename="tests/failure.hurl">
                  <td><a href="tests/failure.hurl.html">tests/failure.hurl</a></td>
                  <td>failure</td>
                  <td>0.2s</td>
                </tr>
                </tbody>
              <table>
           </body>
        </html>"#;

        assert_eq!(
            parse_html_report(html),
            vec![
                HTMLResult {
                    filename: "tests/hello.hurl".to_string(),
                    time_in_ms: 100,
                    success: true,
                },
                HTMLResult {
                    filename: "tests/failure.hurl".to_string(),
                    time_in_ms: 200,
                    success: false,
                }
            ]
        );
    }
}
