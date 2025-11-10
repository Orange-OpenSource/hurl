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
use crate::report::html::{HTMLResult, Testcase};
use crate::report::ReportError;
use chrono::{DateTime, Local};
use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

/// Creates and HTML report for this list of [`Testcase`] at `dir_path`/index.html.
///
/// If the report already exists, results are merged.
pub fn write_report(dir_path: &Path, testcases: &[Testcase]) -> Result<(), ReportError> {
    let index_path = dir_path.join("index.html");
    let mut results = parse_html(&index_path)?;
    for testcase in testcases.iter() {
        let html_result = HTMLResult::from(testcase);
        results.push(html_result);
    }
    let now = Local::now();
    let s = create_html_index(&now.to_rfc2822(), &results);

    let file_path = index_path;
    let mut file = std::fs::File::create(&file_path)
        .map_err(|e| ReportError::from_io_error(&e, &file_path, "Issue writing HTML report"))?;
    file.write_all(s.as_bytes())
        .map_err(|e| ReportError::from_io_error(&e, &file_path, "Issue writing HTML report"))?;
    Ok(())
}

/// Returns a standalone HTML report from the list of `hurl_results`.
fn create_html_index(now: &str, hurl_results: &[HTMLResult]) -> String {
    let count_total = hurl_results.len();
    let count_failure = hurl_results.iter().filter(|result| !result.success).count();
    let count_success = hurl_results.iter().filter(|result| result.success).count();
    let percentage_success = percentage(count_success, count_total);
    let percentage_failure = percentage(count_failure, count_total);
    let css = include_str!("resources/report.css");
    let rows = hurl_results
        .iter()
        .map(create_html_table_row)
        .collect::<Vec<String>>()
        .join("");
    format!(
        include_str!("resources/report.html"),
        now = now,
        css = css,
        count_total = count_total,
        count_success = count_success,
        count_failure = count_failure,
        percentage_success = percentage_success,
        percentage_failure = percentage_failure,
        rows = rows,
    )
}

fn parse_html(path: &Path) -> Result<Vec<HTMLResult>, ReportError> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let s = std::fs::read_to_string(path)
        .map_err(|e| ReportError::from_io_error(&e, path, "Issue reading HTML report"))?;
    Ok(parse_html_report(&s))
}

static TEST_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?x)
        data-duration="(?P<time_in_ms>\d+)"
        \s+
        data-status="(?P<status>[a-z]+)"
        \s+
        data-filename="(?P<filename>[\p{L}\p{N}\p{M}\p{S}\p{P}\p{Zs}_./-]+)"
        \s+
        data-id="(?P<id>[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"
        (\s+
        data-timestamp="(?P<timestamp>[0-9]{1,10})")?
    "#,
    )
    .unwrap()
});
/// Parses the HTML report `html` and returns a list of [`HTMLResult`].
fn parse_html_report(html: &str) -> Vec<HTMLResult> {
    // TODO: if the existing HTML report is not valid, we consider that there is no
    // existing report to append, without displaying any error or warning. Maybe a better option
    // would be to raise an error here and ask the user to explicitly deal with this error.
    TEST_REF
        .captures_iter(html)
        .map(|cap| {
            let filename = cap["filename"].to_string();
            let id = cap["id"].to_string();
            let time_in_ms = cap["time_in_ms"].to_string().parse().unwrap();
            let success = &cap["status"] == "success";

            // Older reports won't have this so make it optional
            let timestamp: i64 = cap
                .name("timestamp")
                .map_or(0, |m| m.as_str().parse().unwrap());

            HTMLResult {
                filename,
                id,
                time_in_ms,
                success,
                timestamp,
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
    let timestamp = result.timestamp;
    let displayed_time = if timestamp == 0 {
        "-".to_string()
    } else {
        DateTime::from_timestamp(timestamp, 0)
            .unwrap()
            .naive_local()
            .and_local_timezone(Local)
            .unwrap()
            .to_rfc3339()
    };
    format!(
        r#"<tr data-duration="{duration_in_ms}" data-status="{status}" data-filename="{filename}" data-id="{id}" data-timestamp="{timestamp}">
    <td><a href="store/{id}-source.html">{displayed_filename}</a></td>
    <td class="{status}"><a href="store/{id}-timeline.html">{status}</a></td>
    <td>{displayed_time}</td>
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
                <tr class="failure" data-duration="200" data-status="failure" data-filename="tests/failure.hurl" data-id="a6641ae3-8ce0-4d9f-80c5-3e23e032e055" data-timestamp="1696473444">
                  <td><a href="tests/failure.hurl.html">tests/failure.hurl</a></td>
                  <td>failure</td>
                  <td>2023-10-05T02:37:24Z</td>
                  <td>0.2s</td>
                </tr>
                <tr class="success" data-duration="50" data-status="success" data-filename="tests/café.hurl" data-id="a151aea6-2b02-465e-be47-45a2fa9cce02" data-timestamp="1796473444">
                  <td><a href="tests/café.hurl.html">tests/café.hurl</a></td>
                  <td>success</td>
                  <td>2023-10-05T02:37:24Z</td>
                  <td>0.4s</td>
                </tr>
                <tr class="failure" data-duration="366" data-status="failure" data-filename="abcd[1234]@2x.hurl" data-id="2008c777-025d-4708-8016-e2928b9ef538" data-timestamp="1796473666">
                  <td><a href="abcd[1234]@2x.hurl.html">abcd[1234]@2x.hurl</a></td>
                  <td>failure</td>
                  <td>2023-10-05T02:37:24Z</td>
                  <td>0.4s</td>
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
                    timestamp: 0,
                },
                HTMLResult {
                    filename: "tests/failure.hurl".to_string(),
                    id: "a6641ae3-8ce0-4d9f-80c5-3e23e032e055".to_string(),
                    time_in_ms: 200,
                    success: false,
                    timestamp: 1696473444,
                },
                HTMLResult {
                    filename: "tests/café.hurl".to_string(),
                    id: "a151aea6-2b02-465e-be47-45a2fa9cce02".to_string(),
                    time_in_ms: 50,
                    success: true,
                    timestamp: 1796473444,
                },
                HTMLResult {
                    filename: "abcd[1234]@2x.hurl".to_string(),
                    id: "2008c777-025d-4708-8016-e2928b9ef538".to_string(),
                    time_in_ms: 366,
                    success: false,
                    timestamp: 1796473666,
                },
            ]
        );
    }
}
