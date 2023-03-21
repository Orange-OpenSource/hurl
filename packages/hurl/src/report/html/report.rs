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
use std::io::Write;
use std::path::Path;

use chrono::{DateTime, Local};

use crate::report::html::{HTMLResult, Testcase};
use crate::report::Error;

/// Creates and HTML report for this list of [`Testcase`] at `dir_path`/index.html.
///
/// If the report already exists, results are merged.
pub fn write_report(dir_path: &Path, testcases: &[Testcase]) -> Result<(), Error> {
    let index_path = dir_path.join("index.html");
    let mut results = parse_html(&index_path)?;
    for testcase in testcases.iter() {
        let html_result = HTMLResult::from(testcase);
        results.push(html_result);
    }
    let now: DateTime<Local> = Local::now();
    let s = create_html_index(&now.to_rfc2822(), &results);

    let file_path = index_path;
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            return Err(Error {
                message: format!("Issue writing to {}: {:?}", file_path.display(), why),
            });
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(s.as_bytes()) {
        return Err(Error {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        });
    }

    let file_path = dir_path.join("report.css");
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            return Err(Error {
                message: format!("Issue writing to {}: {:?}", file_path.display(), why),
            });
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(include_bytes!("resources/report.css")) {
        return Err(Error {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        });
    }
    Ok(())
}

/// Returns a standalone HTML report from the list of `hurl_results`.
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
        include_str!("resources/report.html"),
        now = now,
        count_total = count_total,
        count_success = count_success,
        count_failure = count_failure,
        percentage_success = percentage_success,
        percentage_failure = percentage_failure,
        rows = rows,
    )
}

fn parse_html(path: &Path) -> Result<Vec<HTMLResult>, Error> {
    if path.exists() {
        let s = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(why) => {
                return Err(Error {
                    message: format!("Issue reading {} to string to {:?}", path.display(), why),
                });
            }
        };
        Ok(parse_html_report(&s))
    } else {
        Ok(vec![])
    }
}

/// Parses the HTML report `html` an returns a list of [`HTMLResult`].
fn parse_html_report(html: &str) -> Vec<HTMLResult> {
    let re = regex::Regex::new(
        r#"(?x)
        data-duration="(?P<time_in_ms>\d+)"
        \s+
        data-status="(?P<status>[a-z]+)"
        \s+
        data-filename="(?P<filename>[A-Za-z0-9_./-]+)"
        \s+
        data-id="(?P<id>[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"
    "#,
    )
    .unwrap();
    re.captures_iter(html)
        .map(|cap| {
            let filename = cap["filename"].to_string();
            let id = cap["id"].to_string();
            let time_in_ms = cap["time_in_ms"].to_string().parse().unwrap();
            let success = &cap["status"] == "success";
            HTMLResult {
                filename,
                id,
                time_in_ms,
                success,
            }
        })
        .collect::<Vec<HTMLResult>>()
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
    let displayed_filename = if filename == "-" {
        "(standard input)"
    } else {
        filename
    };
    let id = &result.id;

    format!(
        r#"<tr class="{status}" data-duration="{duration_in_ms}" data-status="{status}" data-filename="{filename}" data-id="{id}">
    <td><a href="store/{id}.html">{displayed_filename}</a></td>
    <td>{status}</td>
    <td>{duration_in_s}</td>
</tr>
"#
    )
}

fn percentage(count: usize, total: usize) -> String {
    format!("{:.1}%", (count as f32 * 100.0) / total as f32)
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
                <tr class="success" data-duration="100" data-status="success" data-filename="tests/hello.hurl" data-id="08aad14a-8d10-4ecc-892e-a72703c5b494">
                  <td><a href="tests/hello.hurl.html">tests/hello.hurl</a></td>
                  <td>success</td>
                  <td>0.1s</td>
                </tr>
                <tr class="failure" data-duration="200" data-status="failure" data-filename="tests/failure.hurl" data-id="a6641ae3-8ce0-4d9f-80c5-3e23e032e055">
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
                    id: "08aad14a-8d10-4ecc-892e-a72703c5b494".to_string(),
                    time_in_ms: 100,
                    success: true,
                },
                HTMLResult {
                    filename: "tests/failure.hurl".to_string(),
                    id: "a6641ae3-8ce0-4d9f-80c5-3e23e032e055".to_string(),
                    time_in_ms: 200,
                    success: false,
                }
            ]
        );
    }
}
