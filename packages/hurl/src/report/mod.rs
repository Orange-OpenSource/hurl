/*
 * Hurl (https://hurl.dev)
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
use chrono::{DateTime, Local};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use super::cli::CliError;
use super::runner::HurlResult;

mod html;
mod junit;

pub use junit::create_report as create_junit_report;
pub use junit::Testcase;

pub fn parse_html(path: PathBuf) -> Result<Vec<HurlResult>, CliError> {
    if path.exists() {
        let s = match std::fs::read_to_string(path.clone()) {
            Ok(s) => s,
            Err(why) => {
                return Err(CliError {
                    message: format!("Issue reading {} to string to {:?}", path.display(), why),
                });
            }
        };
        Ok(parse_html_report(s.as_str()))
    } else {
        Ok(vec![])
    }
}

fn parse_html_report(html: &str) -> Vec<HurlResult> {
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
            let filename = format!("/{}", filename);
            let entries = vec![];
            let time_in_ms = cap["time_in_ms"].to_string().parse().unwrap();
            let success = &cap["status"] == "success";
            let cookies = vec![];
            HurlResult {
                filename,
                entries,
                time_in_ms,
                success,
                cookies,
            }
        })
        .collect::<Vec<HurlResult>>()
}

pub fn write_html_report(dir_path: PathBuf, hurl_results: Vec<HurlResult>) -> Result<(), CliError> {
    let index_path = dir_path.join("index.html");
    let mut results = parse_html(index_path)?;
    for result in hurl_results {
        results.push(result);
    }
    let now: DateTime<Local> = Local::now();
    let html = create_html_index(now.to_rfc2822(), results);
    let s = html.render();

    let file_path = dir_path.join("index.html");
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
    if let Err(why) = file.write_all(include_bytes!("report.css")) {
        return Err(CliError {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        });
    }
    Ok(())
}

fn percentage(count: usize, total: usize) -> String {
    format!("{:.1}%", (count as f32 * 100.0) / total as f32)
}

fn create_html_index(now: String, hurl_results: Vec<HurlResult>) -> html::Html {
    let head = html::Head {
        title: "Test Report".to_string(),
        stylesheet: Some("report.css".to_string()),
    };

    let count_total = hurl_results.len();
    let count_failure = hurl_results.iter().filter(|result| !result.success).count();
    let count_success = hurl_results.iter().filter(|result| result.success).count();

    let body = html::Body {
        children: vec![
            html::Element::NodeElement {
                name: "h2".to_string(),
                attributes: vec![],
                children: vec![html::Element::TextElement("Test Report".to_string())],
            },
            html::Element::NodeElement {
                name: "div".to_string(),
                attributes: vec![html::Attribute::Class("summary".to_string())],
                children: vec![
                    html::Element::NodeElement {
                        name: "div".to_string(),
                        attributes: vec![html::Attribute::Class("date".to_string())],
                        children: vec![html::Element::TextElement(now)],
                    },
                    html::Element::NodeElement {
                        name: "div".to_string(),
                        attributes: vec![html::Attribute::Class("count".to_string())],
                        children: vec![html::Element::TextElement(format!(
                            "Executed: {} (100%)",
                            count_total
                        ))],
                    },
                    html::Element::NodeElement {
                        name: "div".to_string(),
                        attributes: vec![html::Attribute::Class("count".to_string())],
                        children: vec![html::Element::TextElement(format!(
                            "Succeeded: {} ({})",
                            count_success,
                            percentage(count_success, count_total)
                        ))],
                    },
                    html::Element::NodeElement {
                        name: "div".to_string(),
                        attributes: vec![html::Attribute::Class("count".to_string())],
                        children: vec![html::Element::TextElement(format!(
                            "Failed: {} ({})",
                            count_failure,
                            percentage(count_failure, count_total)
                        ))],
                    },
                ],
            },
            html::Element::NodeElement {
                name: "table".to_string(),
                attributes: vec![],
                children: vec![
                    create_html_table_header(),
                    create_html_table_body(hurl_results),
                ],
            },
        ],
    };
    html::Html { head, body }
}

fn create_html_table_header() -> html::Element {
    html::Element::NodeElement {
        name: "thead".to_string(),
        attributes: vec![],
        children: vec![html::Element::NodeElement {
            name: "tr".to_string(),
            attributes: vec![],
            children: vec![
                html::Element::NodeElement {
                    name: "td".to_string(),
                    attributes: vec![],
                    children: vec![html::Element::TextElement("filename".to_string())],
                },
                html::Element::NodeElement {
                    name: "td".to_string(),
                    attributes: vec![],
                    children: vec![html::Element::TextElement("status".to_string())],
                },
                html::Element::NodeElement {
                    name: "td".to_string(),
                    attributes: vec![],
                    children: vec![html::Element::TextElement("duration".to_string())],
                },
            ],
        }],
    }
}

fn create_html_table_body(hurl_results: Vec<HurlResult>) -> html::Element {
    let children = hurl_results
        .iter()
        .map(|result| create_html_result(result.clone()))
        .collect();

    html::Element::NodeElement {
        name: "tbody".to_string(),
        attributes: vec![],
        children,
    }
}

///
/// return the canonical fullname relative to / (technically a relative path)
/// The function will panic if the input file does not exist
///
pub fn canonicalize_filename(input_file: &str) -> String {
    let relative_input_file = Path::new(input_file).canonicalize().expect("existing file");
    let relative_input_file = relative_input_file.to_string_lossy();
    relative_input_file.trim_start_matches('/').to_string()
}

fn create_html_result(result: HurlResult) -> html::Element {
    let status = if result.success {
        "success".to_string()
    } else {
        "failure".to_string()
    };
    let relative_input_file = canonicalize_filename(&result.filename);

    html::Element::NodeElement {
        name: "tr".to_string(),
        attributes: vec![
            html::Attribute::Class(status.clone()),
            html::Attribute::Data("duration".to_string(), result.time_in_ms.to_string()),
            html::Attribute::Data("status".to_string(), status.clone()),
            html::Attribute::Data("filename".to_string(), relative_input_file.to_string()),
        ],
        children: vec![
            html::Element::NodeElement {
                name: "td".to_string(),
                attributes: vec![],
                children: vec![html::Element::NodeElement {
                    name: "a".to_string(),
                    attributes: vec![html::Attribute::Href(format!(
                        "{}.html",
                        relative_input_file
                    ))],
                    children: vec![html::Element::TextElement(relative_input_file)],
                }],
            },
            html::Element::NodeElement {
                name: "td".to_string(),
                attributes: vec![],
                children: vec![html::Element::TextElement(status)],
            },
            html::Element::NodeElement {
                name: "td".to_string(),
                attributes: vec![],
                children: vec![html::Element::TextElement(format!(
                    "{}s",
                    result.time_in_ms as f64 / 1000.0
                ))],
            },
        ],
    }
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
                HurlResult {
                    filename: "/tests/hello.hurl".to_string(),
                    entries: vec![],
                    time_in_ms: 100,
                    success: true,
                    cookies: vec![],
                },
                HurlResult {
                    filename: "/tests/failure.hurl".to_string(),
                    entries: vec![],
                    time_in_ms: 200,
                    success: false,
                    cookies: vec![],
                }
            ]
        );
    }
}
