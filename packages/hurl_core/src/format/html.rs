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
use crate::ast::*;

trait Htmlable {
    fn to_html(&self) -> String;
}

/// Returns an HTML string of the Hurl file `hurl_file`.
///
/// If `standalone` is true, a complete HTML body with inline styling is returned.
/// Otherwise, a `<pre>` HTML tag is returned, without styling.
pub fn format(hurl_file: HurlFile, standalone: bool) -> String {
    if standalone {
        format_standalone(hurl_file)
    } else {
        hurl_file.to_html()
    }
}

fn format_standalone(hurl_file: HurlFile) -> String {
    let css = include_str!("hurl.css");
    let body = hurl_file.to_html();

    format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <title>Hurl File</title>
        <style>
{css}
        </style>
    </head>
    <body>
{body}
    </body>
</html>
"#
    )
}

impl Htmlable for HurlFile {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<pre><code class=\"language-hurl\">");
        for entry in self.entries.iter() {
            buffer.push_str(entry.to_html().as_str());
        }
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("</code></pre>");
        buffer
    }
}

impl Htmlable for Entry {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<span class=\"hurl-entry\">");
        buffer.push_str(self.request.to_html().as_str());
        if let Some(response) = &self.response {
            buffer.push_str(response.to_html().as_str());
        }
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Request {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<span class=\"request\">");
        add_line_terminators(&mut buffer, &self.line_terminators);

        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.method.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.url.span_with_class("url").as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());

        for header in self.headers.iter() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.iter() {
            buffer.push_str(section.to_html().as_str());
        }
        if let Some(body) = &self.body {
            buffer.push_str(body.to_html().as_str());
        }
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Response {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<span class=\"response\">");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.version.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.status.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        for header in self.headers.iter() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.iter() {
            buffer.push_str(section.to_html().as_str());
        }
        if let Some(body) = &self.body {
            buffer.push_str(body.to_html().as_str());
        }
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Method {
    fn to_html(&self) -> String {
        format!("<span class=\"method\">{self}</span>")
    }
}

impl Htmlable for Version {
    fn to_html(&self) -> String {
        format!("<span class=\"version\">{}</span>", self.value)
    }
}

impl Htmlable for Status {
    fn to_html(&self) -> String {
        format!("<span class=\"number\">{}</span>", self.value)
    }
}

impl Htmlable for Section {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(
            format!(
                "<span class=\"line section-header\">[{}]</span>",
                self.name()
            )
            .as_str(),
        );
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer
    }
}

impl Htmlable for SectionValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            SectionValue::Asserts(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::QueryParams(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::BasicAuth(item) => buffer.push_str(item.to_html().as_str()),
            SectionValue::FormParams(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::MultipartFormData(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::Cookies(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::Captures(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::Options(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
        }
        buffer
    }
}

impl Htmlable for KeyValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.key.encoded.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for EntryOption {
    fn to_html(&self) -> String {
        match self {
            EntryOption::CaCertificate(option) => option.to_html(),
            EntryOption::ClientCert(option) => option.to_html(),
            EntryOption::ClientKey(option) => option.to_html(),
            EntryOption::Compressed(option) => option.to_html(),
            EntryOption::Insecure(option) => option.to_html(),
            EntryOption::FollowLocation(option) => option.to_html(),
            EntryOption::MaxRedirect(option) => option.to_html(),
            EntryOption::Retry(option) => option.to_html(),
            EntryOption::RetryInterval(option) => option.to_html(),
            EntryOption::RetryMaxCount(option) => option.to_html(),
            EntryOption::Variable(option) => option.to_html(),
            EntryOption::Verbose(option) => option.to_html(),
            EntryOption::VeryVerbose(option) => option.to_html(),
        }
    }
}

impl Htmlable for CompressedOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("compressed".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for InsecureOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("insecure".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for CaCertificateOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("cacert".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.filename.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for ClientCertOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("cert".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.filename.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for ClientKeyOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("key".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.filename.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for FollowLocationOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("location".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for MaxRedirectOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("max-redirs".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for RetryOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("retry".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for RetryIntervalOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("retry-interval".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for RetryMaxCountOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("retry-max-count".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for VariableOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("variable".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for VariableDefinition {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.name.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>=</span>");
        buffer.push_str(self.value.to_html().as_str());
        buffer
    }
}

impl Htmlable for VariableValue {
    fn to_html(&self) -> String {
        match self {
            VariableValue::Null { .. } => "<span class=\"null\">null</span>".to_string(),
            VariableValue::Bool(v) => v.to_html(),
            VariableValue::Integer(v) => v.to_html(),
            VariableValue::Float(v) => v.to_html(),
            VariableValue::String(t) => t.to_html(),
        }
    }
}

impl Htmlable for VerboseOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("verbose".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for VeryVerboseOption {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str("very-verbose".to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for MultipartParam {
    fn to_html(&self) -> String {
        match self {
            MultipartParam::Param(keyvalue) => keyvalue.to_html(),
            MultipartParam::FileParam(file_param) => file_param.to_html(),
        }
    }
}

impl Htmlable for FileParam {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.key.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(':');
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for FileValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("file,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.filename.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer.push_str(self.space2.to_html().as_str());
        if let Some(content_type) = &self.content_type {
            buffer.push_str(content_type.to_html().as_str());
        }
        buffer
    }
}

impl Htmlable for Filename {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span class=\"filename\">");
        let s = self.value.replace(' ', "\\ ");
        buffer.push_str(s.as_str());
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Cookie {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer
            .push_str(format!("<span class=\"name\">{}</span>", self.name.value.as_str()).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for Capture {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer
            .push_str(format!("<span class=\"name\">{}</span>", self.name.value.as_str()).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
        for (space, filter) in self.filters.iter() {
            buffer.push_str(space.to_html().as_str());
            buffer.push_str(filter.to_html().as_str());
        }
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for Query {
    fn to_html(&self) -> String {
        self.value.to_html()
    }
}

impl Htmlable for QueryValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            QueryValue::Status {} => {
                buffer.push_str("<span class=\"query-type\">status</span>");
            }
            QueryValue::Url {} => {
                buffer.push_str("<span class=\"query-type\">url</span>");
            }
            QueryValue::Header { space0, name } => {
                buffer.push_str("<span class=\"query-type\">header</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(name.to_html().as_str());
            }
            QueryValue::Cookie { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">cookie</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(expr.to_html().as_str());
            }
            QueryValue::Body {} => {
                buffer.push_str("<span class=\"query-type\">body</span>");
            }
            QueryValue::Xpath { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">xpath</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(expr.to_html().as_str());
            }
            QueryValue::Jsonpath { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">jsonpath</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(expr.to_html().as_str());
            }
            QueryValue::Regex { space0, value } => {
                buffer.push_str("<span class=\"query-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            QueryValue::Variable { space0, name } => {
                buffer.push_str("<span class=\"query-type\">variable</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(name.to_html().as_str());
            }
            QueryValue::Duration {} => {
                buffer.push_str("<span class=\"query-type\">duration</span>");
            }
            QueryValue::Bytes {} => {
                buffer.push_str("<span class=\"query-type\">bytes</span>");
            }
            QueryValue::Sha256 {} => {
                buffer.push_str("<span class=\"query-type\">sha256</span>");
            }
            QueryValue::Md5 {} => {
                buffer.push_str("<span class=\"query-type\">md5</span>");
            }
        }
        buffer
    }
}

impl Htmlable for RegexValue {
    fn to_html(&self) -> String {
        match self {
            RegexValue::Template(template) => template.to_html(),
            RegexValue::Regex(regex) => regex.to_html(),
        }
    }
}

impl Htmlable for CookiePath {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push('"');
        buffer.push_str(self.name.to_encoded_string().as_str());
        if let Some(attribute) = &self.attribute {
            buffer.push('[');
            buffer.push_str(attribute.to_html().as_str());
            buffer.push(']');
        }
        buffer.push('"');
        buffer.to_html()
    }
}

impl Htmlable for CookieAttribute {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.value().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer
    }
}

impl Htmlable for Assert {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
        for (space, filter) in self.filters.iter() {
            buffer.push_str(space.to_html().as_str());
            buffer.push_str(filter.to_html().as_str());
        }
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.predicate.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for Predicate {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        if self.not {
            buffer.push_str("<span class=\"not\">not</span>");
            buffer.push_str(self.space0.to_html().as_str());
        }
        buffer.push_str(self.predicate_func.to_html().as_str());
        buffer
    }
}

impl Htmlable for PredicateFunc {
    fn to_html(&self) -> String {
        self.value.to_html()
    }
}

impl Htmlable for PredicateFuncValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            PredicateFuncValue::CountEqual { space0, value, .. } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::Equal { space0, value, .. } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::NotEqual { space0, value, .. } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::GreaterThan { space0, value, .. } => {
                buffer.push_str(
                    format!(
                        "<span class=\"predicate-type\">{}</span>",
                        encode_html(self.name())
                    )
                    .as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::GreaterThanOrEqual { space0, value, .. } => {
                buffer.push_str(
                    format!(
                        "<span class=\"predicate-type\">{}</span>",
                        encode_html(self.name())
                    )
                    .as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::LessThan { space0, value, .. } => {
                buffer.push_str(
                    format!(
                        "<span class=\"predicate-type\">{}</span>",
                        encode_html(self.name())
                    )
                    .as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::LessThanOrEqual { space0, value, .. } => {
                buffer.push_str(
                    format!(
                        "<span class=\"predicate-type\">{}</span>",
                        encode_html(self.name())
                    )
                    .as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::StartWith { space0, value } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::EndWith { space0, value } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::Contain { space0, value } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::Include { space0, value } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::Match { space0, value } => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::IsInteger {} => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
            }
            PredicateFuncValue::IsFloat {} => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
            }
            PredicateFuncValue::IsBoolean {} => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
            }
            PredicateFuncValue::IsString {} => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
            }
            PredicateFuncValue::IsCollection {} => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
            }
            PredicateFuncValue::Exist {} => {
                buffer.push_str(
                    format!("<span class=\"predicate-type\">{}</span>", self.name()).as_str(),
                );
            }
        }
        buffer
    }
}

impl Htmlable for PredicateValue {
    fn to_html(&self) -> String {
        match self {
            PredicateValue::String(value) => value.to_html(),
            PredicateValue::MultilineString(value) => value.to_html(),
            PredicateValue::Integer(value) => value.to_html(),
            PredicateValue::Float(value) => value.to_html(),
            PredicateValue::Bool(value) => value.to_html(),
            PredicateValue::Hex(value) => value.to_html(),
            PredicateValue::Base64(value) => value.to_html(),
            PredicateValue::Expression(value) => value.to_html(),
            PredicateValue::Null {} => "<span class=\"null\">null</span>".to_string(),
            PredicateValue::Regex(value) => value.to_html(),
        }
    }
}

impl Htmlable for MultilineString {
    fn to_html(&self) -> String {
        let lang = match self {
            MultilineString::OneLineText(_) => "".to_string(),
            MultilineString::Text(_)
            | MultilineString::Json(_)
            | MultilineString::Xml(_)
            | MultilineString::GraphQl(_) => {
                format!("{}\n", self.lang())
            }
        };
        let body = format!("```{lang}{self}```");
        let body = multilines(&body);
        format!("<span class=\"multiline\">{body}</span>")
    }
}

impl Htmlable for Body {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, &self.line_terminators);
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for Bytes {
    fn to_html(&self) -> String {
        match self {
            Bytes::Base64(value) => format!("<span class=\"line\">{}</span>", value.to_html()),
            Bytes::File(value) => format!("<span class=\"line\">{}</span>", value.to_html()),
            Bytes::Hex(value) => format!("<span class=\"line\">{}</span>", value.to_html()),
            Bytes::OnelineString(value) => {
                format!("<span class=\"line\">{}</span>", value.to_html())
            }
            Bytes::Json(value) => value.to_html(),
            Bytes::MultilineString(value) => value.to_html(),
            Bytes::Xml(value) => xml_html(value),
        }
    }
}

impl Htmlable for String {
    fn to_html(&self) -> String {
        format!("<span class=\"string\">{self}</span>")
    }
}

impl Htmlable for &str {
    fn to_html(&self) -> String {
        format!("<span class=\"string\">{self}</span>")
    }
}

impl Htmlable for bool {
    fn to_html(&self) -> String {
        format!("<span class=\"boolean\">{self}</span>")
    }
}

impl Htmlable for u64 {
    fn to_html(&self) -> String {
        format!("<span class=\"number\">{self}</span>")
    }
}

impl Htmlable for Float {
    fn to_html(&self) -> String {
        format!("<span class=\"number\">{}</span>", self.encoded)
    }
}

impl Htmlable for i64 {
    fn to_html(&self) -> String {
        format!("<span class=\"number\">{self}</span>")
    }
}

impl Htmlable for usize {
    fn to_html(&self) -> String {
        format!("<span class=\"number\">{self}</span>")
    }
}

// you should probably define for XML value to be consistent with the other types
fn xml_html(value: &str) -> String {
    let mut buffer = String::from("<span class=\"xml\">");
    buffer.push_str(multilines(value).as_str());
    buffer.push_str("</span>");
    buffer
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// Improvement: break into spans within the json value
impl Htmlable for JsonValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span class=\"json\">");
        buffer.push_str(multilines(&self.encoded()).as_str());
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Whitespace {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        let Whitespace { value, .. } = self;
        if !value.is_empty() {
            buffer.push_str(self.value.as_str());
        };
        buffer
    }
}

impl Htmlable for LineTerminator {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.space0.to_html().as_str());
        if let Some(v) = &self.comment {
            buffer.push_str(v.to_html().as_str());
        }
        buffer.push_str(self.newline.value.as_str());
        buffer
    }
}

impl Htmlable for Comment {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span class=\"comment\">");
        buffer.push_str(format!("#{}", xml_escape(&self.value)).as_str());
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for File {
    fn to_html(&self) -> String {
        let mut buffer = String::from("file,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.filename.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer
    }
}

impl Htmlable for Base64 {
    fn to_html(&self) -> String {
        let mut buffer = String::from("base64,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer
            .push_str(format!("<span class=\"base64\">{}</span>", self.encoded.as_str()).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer
    }
}

impl Htmlable for Hex {
    fn to_html(&self) -> String {
        let mut buffer = String::from("hex,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(format!("<span class=\"hex\">{}</span>", self.encoded.as_str()).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer
    }
}

impl Htmlable for Regex {
    fn to_html(&self) -> String {
        let s = str::replace(self.inner.as_str(), "/", "\\/");
        format!("<span class=\"regex\">/{s}/</span>")
    }
}

impl Htmlable for EncodedString {
    fn to_html(&self) -> String {
        self.encoded.to_html()
    }
}

impl Htmlable for Template {
    fn to_html(&self) -> String {
        self.span_with_class("string")
    }
}

impl Template {
    fn to_encoded_string(&self) -> String {
        let mut s = "".to_string();
        if let Some(d) = self.delimiter {
            s.push(d);
        }
        for element in self.elements.iter() {
            let elem_str = match element {
                TemplateElement::String { encoded, .. } => encoded.to_string(),
                TemplateElement::Expression(expr) => format!("{{{{{expr}}}}}"),
            };
            s.push_str(elem_str.as_str())
        }
        if let Some(d) = self.delimiter {
            s.push(d);
        }
        s
    }

    fn span_with_class(&self, class: &str) -> String {
        let s = self.to_encoded_string();
        format!("<span class=\"{}\">{}</span>", class, xml_escape(&s))
    }
}

impl Htmlable for Expr {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<span class=\"expr\">{{");
        buffer.push_str(self.to_string().as_str());
        buffer.push_str("}}</span>");
        buffer
    }
}

impl Htmlable for Filter {
    fn to_html(&self) -> String {
        self.value.to_html()
    }
}

impl Htmlable for FilterValue {
    fn to_html(&self) -> String {
        match self {
            FilterValue::Count => "<span class=\"filter-type\">count</span>".to_string(),
            FilterValue::Format { space0, fmt } => {
                let mut buffer = "<span class=\"filter-type\">format</span>".to_string();
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(fmt.to_html().as_str());
                buffer
            }
            FilterValue::HtmlEscape => "<span class=\"filter-type\">htmlEscape</span>".to_string(),
            FilterValue::HtmlUnescape => {
                "<span class=\"filter-type\">htmlUnescape</span>".to_string()
            }
            FilterValue::Nth { space0, n: value } => {
                let mut buffer = "<span class=\"filter-type\">nth</span>".to_string();
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
                buffer
            }
            FilterValue::Regex { space0, value } => {
                let mut buffer = "".to_string();
                buffer.push_str("<span class=\"filter-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
                buffer
            }
            FilterValue::Replace {
                space0,
                old_value,
                space1,
                new_value,
            } => {
                let mut buffer = "<span class=\"filter-type\">replace</span>".to_string();
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(old_value.to_html().as_str());
                buffer.push_str(space1.to_html().as_str());
                buffer.push_str(new_value.to_html().as_str());
                buffer
            }
            FilterValue::Split { space0, sep } => {
                let mut buffer = "<span class=\"filter-type\">split</span>".to_string();
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(sep.to_html().as_str());
                buffer
            }
            FilterValue::ToDate { space0, fmt } => {
                let mut buffer = "".to_string();
                buffer.push_str("<span class=\"filter-type\">toDate</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(fmt.to_html().as_str());
                buffer
            }
            FilterValue::ToInt => "<span class=\"filter-type\">toInt</span>".to_string(),
            FilterValue::UrlDecode => "<span class=\"filter-type\">urlDecode</span>".to_string(),
            FilterValue::UrlEncode => "<span class=\"filter-type\">urlEncode</span>".to_string(),
        }
    }
}

fn add_line_terminators(buffer: &mut String, line_terminators: &[LineTerminator]) {
    for line_terminator in line_terminators {
        buffer.push_str("<span class=\"line\">");
        if line_terminator.newline.value.is_empty() {
            buffer.push_str("<br>");
        }
        buffer.push_str("</span>");
        buffer.push_str(line_terminator.to_html().as_str());
    }
}

fn encode_html(s: String) -> String {
    s.replace('>', "&gt;").replace('<', "&lt;")
}

fn multilines(s: &str) -> String {
    regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(s)
        .map(|l| format!("<span class=\"line\">{}</span>", xml_escape(l)))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_string() {
        // ``````
        let multiline_string = MultilineString::OneLineText(Template {
            delimiter: None,
            elements: vec![TemplateElement::String {
                value: "".to_string(),
                encoded: "unused".to_string(),
            }],
            source_info: SourceInfo::new(0, 0, 0, 0),
        });
        assert_eq!(
            multiline_string.to_html(),
            "<span class=\"multiline\"><span class=\"line\">``````</span></span>".to_string()
        );

        // ```hello```
        let multiline_string = MultilineString::OneLineText(Template {
            delimiter: None,
            elements: vec![TemplateElement::String {
                value: "hello".to_string(),
                encoded: "unused".to_string(),
            }],
            source_info: SourceInfo::new(0, 0, 0, 0),
        });
        assert_eq!(
            multiline_string.to_html(),
            "<span class=\"multiline\"><span class=\"line\">```hello```</span></span>".to_string()
        );

        // ```
        // line1
        // line2
        // ```
        let multiline_string = MultilineString::Text(Text {
            space: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 4 },
                    end: Pos { line: 1, column: 4 },
                },
            },
            newline: Whitespace {
                value: "\n".to_string(),
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 4 },
                    end: Pos { line: 2, column: 1 },
                },
            },
            value: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "line1\nline2\n".to_string(),
                    encoded: "unused".to_string(),
                }],
                source_info: SourceInfo::new(0, 0, 0, 0),
            },
        });
        assert_eq!(
            multiline_string.to_html(),
            "<span class=\"multiline\"><span class=\"line\">```</span>\n<span class=\"line\">line1</span>\n<span class=\"line\">line2</span>\n<span class=\"line\">```</span></span>".to_string()
        );
    }

    #[test]
    fn test_multilines() {
        assert_eq!(
            multilines("{\n   \"id\": 1\n}"),
            "<span class=\"line\">{</span>\n<span class=\"line\">   \"id\": 1</span>\n<span class=\"line\">}</span>"
        );
        assert_eq!(
            multilines("<?xml version=\"1.0\"?>\n<drink>café</drink>"),
            "<span class=\"line\">&lt;?xml version=\"1.0\"?&gt;</span>\n<span class=\"line\">&lt;drink&gt;café&lt;/drink&gt;</span>"
        );

        assert_eq!(
            multilines("Hello\n"),
            "<span class=\"line\">Hello</span>\n<span class=\"line\"></span>"
        );
    }

    #[test]
    fn test_json() {
        let value = JsonValue::Object {
            space0: "".to_string(),
            elements: vec![JsonObjectElement {
                space0: "\n   ".to_string(),
                name: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "id".to_string(),
                        encoded: "id".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space1: "".to_string(),
                space2: " ".to_string(),
                value: JsonValue::Number("1".to_string()),
                space3: "\n".to_string(),
            }],
        };
        assert_eq!(
            value.to_html(),
            "<span class=\"json\"><span class=\"line\">{</span>\n<span class=\"line\">   \"id\": 1</span>\n<span class=\"line\">}</span></span>"
        );
    }

    #[test]
    fn test_json_encoded_newline() {
        let value = JsonValue::String(Template {
            delimiter: Some('"'),
            elements: vec![TemplateElement::String {
                value: "\n".to_string(),
                encoded: "\\n".to_string(),
            }],
            source_info: SourceInfo::new(0, 0, 0, 0),
        });
        assert_eq!(
            value.to_html(),
            "<span class=\"json\"><span class=\"line\">\"\\n\"</span></span>"
        )
    }

    #[test]
    fn test_xml() {
        let value = "<?xml version=\"1.0\"?>\n<drink>café</drink>";
        assert_eq!(
            xml_html(value),
            "<span class=\"xml\"><span class=\"line\">&lt;?xml version=\"1.0\"?&gt;</span>\n<span class=\"line\">&lt;drink&gt;café&lt;/drink&gt;</span></span>"
        )
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("hello"), "hello");
        assert_eq!(
            xml_escape("<?xml version=\"1.0\"?>"),
            "&lt;?xml version=\"1.0\"?&gt;"
        );
    }
}
