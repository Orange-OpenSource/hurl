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
use hurl_core::ast::HurlFile;

use crate::http::Call;
use crate::report::html::nav::Tab;
use crate::report::html::Testcase;
use crate::runner::EntryResult;
use crate::util::redacted::Redact;

impl Testcase {
    /// Creates an HTML view of a run (HTTP status code, response header etc...)
    pub fn get_run_html(
        &self,
        hurl_file: &HurlFile,
        content: &str,
        entries: &[EntryResult],
        secrets: &[&str],
    ) -> String {
        let nav = self.get_nav_html(content, Tab::Run, secrets);
        let nav_css = include_str!("resources/nav.css");
        let run_css = include_str!("resources/run.css");

        let mut run = String::new();
        for (entry_index, e) in entries.iter().enumerate() {
            let entry_src_index = e.entry_index - 1;
            let entry_src = hurl_file.entries.get(entry_src_index).unwrap();
            let line = entry_src.source_info().start.line;
            let source = self.source_filename();

            run.push_str("<details open>");
            let info = get_entry_html(e, entry_index + 1, secrets);
            run.push_str(&info);

            for (call_index, c) in e.calls.iter().enumerate() {
                let info = get_call_html(
                    c,
                    entry_index + 1,
                    call_index + 1,
                    &self.filename,
                    &source,
                    line,
                    secrets,
                );
                run.push_str(&info);
            }

            run.push_str("</details>");
        }

        format!(
            include_str!("resources/run.html"),
            filename = self.filename,
            nav = nav,
            nav_css = nav_css,
            run = run,
            run_css = run_css,
        )
    }
}

/// Returns an HTML view of an `entry` information as HTML (title, `entry_index` and captures).
fn get_entry_html(entry: &EntryResult, entry_index: usize, secrets: &[&str]) -> String {
    let mut text = String::new();
    text.push_str(&format!("<summary>Entry {entry_index}</summary>"));

    let cmd = entry.curl_cmd.to_string().redact(secrets);
    let table = new_table("Debug", &[("Command", &cmd)]);
    text.push_str(&table);

    if !entry.captures.is_empty() {
        let mut values = entry
            .captures
            .iter()
            .map(|c| (&c.name, c.value.to_string().redact(secrets)))
            .collect::<Vec<(&String, String)>>();
        values.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        let table = new_table("Captures", &values);
        text.push_str(&table);
    }

    text
}

/// Returns an HTML view of a `call` (source file, request and response headers, certificate etc...)
fn get_call_html(
    call: &Call,
    entry_index: usize,
    call_index: usize,
    filename: &str,
    source: &str,
    line: usize,
    secrets: &[&str],
) -> String {
    let mut text = String::new();
    let id = format!("e{entry_index}:c{call_index}");
    text.push_str(&format!("<h4 id=\"{id}\">Call {call_index}</h3>"));

    // General
    let status = call.response.status.to_string();
    let version = call.response.version.to_string();
    let url = &call.request.url.to_string().redact(secrets);
    let url = format!("<a href=\"{url}\">{url}</a>");
    let source = format!("<a href=\"{source}#l{line}\">{filename}:{line}</a>");
    let values = vec![
        ("Request URL", url.as_str()),
        ("Request Method", call.request.method.as_str()),
        ("Version", version.as_str()),
        ("Status code", status.as_str()),
        ("Source", source.as_str()),
    ];
    let table = new_table("General", &values);
    text.push_str(&table);

    // Certificate
    if let Some(certificate) = &call.response.certificate {
        let start_date = certificate.start_date.to_string();
        let end_date = certificate.expire_date.to_string();
        let values = vec![
            ("Subject", certificate.subject.as_str()),
            ("Issuer", certificate.issuer.as_str()),
            ("Start Date", start_date.as_str()),
            ("Expire Date", end_date.as_str()),
            ("Serial Number", certificate.serial_number.as_str()),
        ];
        let table = new_table("Certificate", &values);
        text.push_str(&table);
    }

    let mut values = call
        .request
        .headers
        .iter()
        .map(|h| (h.name.as_str(), h.value.redact(secrets)))
        .collect::<Vec<(&str, String)>>();
    values.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    let table = new_table("Request Headers", &values);
    text.push_str(&table);

    let mut values = call
        .response
        .headers
        .iter()
        .map(|h| (h.name.as_str(), h.value.redact(secrets)))
        .collect::<Vec<(&str, String)>>();
    values.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    let table = new_table("Response Headers", &values);
    text.push_str(&table);

    text
}

/// Returns an HTML table with a `title` and a list of key/values. Values are redacted using `secrets`.
fn new_table<T: AsRef<str>, U: AsRef<str> + std::fmt::Display>(
    title: &str,
    data: &[(T, U)],
) -> String {
    let mut text = String::new();
    text.push_str(&format!(
        "<table><thead><tr><th colspan=\"2\">{title}</tr></th></thead><tbody>"
    ));
    data.iter().for_each(|(name, value)| {
        text.push_str(&format!(
            "<tr><td class=\"name\">{}</td><td class=\"value\">{}</td></tr>",
            name.as_ref(),
            value
        ));
    });
    text.push_str("</tbody></table>");
    text
}
