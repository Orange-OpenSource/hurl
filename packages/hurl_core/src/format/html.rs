/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

pub trait Htmlable {
    fn to_html(&self) -> String;
}

pub fn format_standalone(hurl_file: HurlFile) -> String {
    let css = include_str!("hurl.css");

    let mut buffer = String::from("");
    buffer.push_str("<!DOCTYPE html>\n");
    buffer.push_str("<html>");
    buffer.push_str("<head>");
    buffer.push_str("<title>Hurl File</title>");
    buffer.push_str("<style>\n");
    buffer.push_str(css);
    buffer.push_str("</style>");
    buffer.push_str("</head>");
    buffer.push_str("<body>\n");
    buffer.push_str(hurl_file.to_html().as_str());
    buffer.push_str("\n</body>");
    buffer.push_str("</html>");
    buffer
}

pub fn format(hurl_file: HurlFile, standalone: bool) -> String {
    if standalone {
        format_standalone(hurl_file)
    } else {
        hurl_file.to_html()
    }
}

impl Htmlable for HurlFile {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<pre><code class=\"language-hurl\">");
        for entry in self.clone().entries {
            buffer.push_str(entry.to_html().as_str());
        }
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("</code></pre>");
        buffer
    }
}

impl Htmlable for Entry {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<span class=\"hurl-entry\">");
        buffer.push_str(self.request.to_html().as_str());
        if let Some(response) = self.clone().response {
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());

        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.method.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(format!("<span class=\"url\">{}</span>", self.url.to_html()).as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());

        for header in self.headers.clone() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        if let Some(body) = self.body.clone() {
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.version.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.status.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        for header in self.headers.clone() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        if let Some(body) = self.body.clone() {
            buffer.push_str(body.to_html().as_str());
        }
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Method {
    fn to_html(&self) -> String {
        return format!("<span class=\"method\">{}</span>", self);
    }
}

impl Htmlable for Version {
    fn to_html(&self) -> String {
        return format!(
            "<span class=\"version\">HTTP/{}</span>",
            self.value.as_str()
        );
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());
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
        }
        buffer
    }
}

impl Htmlable for KeyValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(format!("<span class=\"string\">{}</span>", self.key.encoded).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(format!("<span class=\"string\">{}</span>", self.value.to_html()).as_str());
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(format!("<span class=\"string\">{}</span>", self.key.to_html()).as_str());
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
        buffer.push_str(
            format!("<span class=\"string\">{}</span>", self.filename.to_html()).as_str(),
        );
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer.push_str(self.space2.to_html().as_str());
        if let Some(content_type) = self.content_type.clone() {
            buffer.push_str(format!("<span class=\"string\">{}</span>", content_type).as_str());
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer
            .push_str(format!("<span class=\"name\">{}</span>", self.name.value.as_str()).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer
    }
}

impl Htmlable for Query {
    fn to_html(&self) -> String {
        let mut buffer = self.value.clone().to_html();
        if let Some((space, subquery)) = self.clone().subquery {
            buffer.push_str(space.to_html().as_str());
            buffer.push_str(subquery.to_html().as_str());
        }
        buffer
    }
}

impl Htmlable for QueryValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            QueryValue::Status {} => {
                buffer.push_str("<span class=\"query-type\">status</span>");
            }
            QueryValue::Header { space0, name } => {
                buffer.push_str("<span class=\"query-type\">header</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", name.to_html()).as_str(),
                );
            }
            QueryValue::Cookie { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">cookie</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", expr.to_html()).as_str(),
                );
            }
            QueryValue::Body {} => {
                buffer.push_str("<span class=\"query-type\">body</span>");
            }
            QueryValue::Xpath { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">xpath</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", expr.to_html()).as_str(),
                );
            }
            QueryValue::Jsonpath { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">jsonpath</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", expr.to_html()).as_str(),
                );
            }
            QueryValue::Regex { space0, value } => {
                buffer.push_str("<span class=\"query-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            QueryValue::Variable { space0, name } => {
                buffer.push_str("<span class=\"query-type\">variable</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", name.to_html()).as_str(),
                );
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
            RegexValue::Template(template) => {
                format!("<span class=\"string\">\"{}\"</span>", template.to_html())
            }
            RegexValue::Regex(regex) => regex.to_html(),
        }
    }
}

impl Htmlable for Subquery {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self.value.clone() {
            SubqueryValue::Regex { value, space0 } => {
                buffer.push_str("<span class=\"subquery-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                // buffer.push_str(
                //     format!("<span class=\"string\">\"{}\"</span>", value.to_html()).as_str(),
                // );
                buffer.push_str(value.to_html().as_str());
            }
            SubqueryValue::Count {} => {
                buffer.push_str("<span class=\"subquery-type\">count</span>")
            }
        }
        buffer
    }
}

impl Htmlable for CookiePath {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.name.to_html().as_str());
        if let Some(attribute) = self.attribute.clone() {
            buffer.push('[');
            buffer.push_str(attribute.to_html().as_str());
            buffer.push(']');
        }
        buffer
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
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
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
            PredicateValue::String(value) => {
                format!("<span class=\"string\">\"{}\"</span>", value.to_html())
            }
            PredicateValue::Raw(value) => value.to_html(),
            PredicateValue::Integer(value) => format!("<span class=\"number\">{}</span>", value),
            PredicateValue::Float(value) => {
                format!("<span class=\"number\">{}</span>", value)
            }
            PredicateValue::Bool(value) => format!("<span class=\"boolean\">{}</span>", value),
            PredicateValue::Hex(value) => value.to_html(),
            PredicateValue::Base64(value) => value.to_html(),
            PredicateValue::Expression(value) => value.to_html(),
            PredicateValue::Null {} => "<span class=\"null\">null</span>".to_string(),
            PredicateValue::Regex(value) => value.to_html(),
        }
    }
}

impl Htmlable for RawString {
    fn to_html(&self) -> String {
        let mut buffer = "".to_string();
        buffer.push_str("<span class=\"raw\">");
        buffer.push_str("<span class=\"line\">```");

        if !self.newline.value.as_str().is_empty() {
            buffer.push_str(
                format!(
                    "</span>{}<span class=\"line\">",
                    self.newline.value.as_str()
                )
                .as_str(),
            );
        }

        let mut lines: Vec<String> = regex::Regex::new(r"\n|\r\n")
            .unwrap()
            .split(self.value.to_string().trim())
            .map(|l| xml_escape(l.to_string()))
            .filter(|l| !l.is_empty())
            .collect();

        if lines.is_empty() {
            buffer.push_str("```</span>");
        } else if lines.len() == 1 {
            buffer.push_str(encode_html(lines.get(0).unwrap().to_string()).as_str());
            buffer.push_str("```</span>");
        } else {
            buffer.push_str(encode_html(lines.remove(0)).as_str());
            buffer.push_str("</span>\n");
            for line in lines {
                buffer.push_str("<span class=\"line\">");
                buffer.push_str(encode_html(line).as_str());
                buffer.push_str("</span>\n");
            }
            buffer.push_str("<span class=\"line\">```</span>");
        }
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Body {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
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
            Bytes::Json { value } => value.to_html(),
            Bytes::RawString(value) => value.to_html(),
            Bytes::Xml { value } => xml_html(value),
        }
    }
}

// you should probably define for XML value to be consistent with the other types
fn xml_html(value: &str) -> String {
    let mut buffer = String::from("<span class=\"xml\">");
    buffer.push_str(multilines(value.to_string()).as_str());
    buffer.push_str("</span>");
    buffer
}

fn xml_escape(s: String) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// Improvement: break into spans within the json value
impl Htmlable for JsonValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span class=\"json\">");
        buffer.push_str(multilines(self.encoded()).as_str());
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
        if let Some(v) = self.clone().comment {
            buffer.push_str(v.to_html().as_str());
        }
        buffer.push_str(self.newline.value.as_str());
        buffer
    }
}

impl Htmlable for Comment {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span class=\"comment\">");
        buffer.push_str(format!("#{}", xml_escape(self.value.clone())).as_str());
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
        format!("<span class=\"regex\">/{}/</span>", s)
    }
}
impl Htmlable for EncodedString {
    fn to_html(&self) -> String {
        format!("<span class=\"string\">{}</span>", self.encoded)
    }
}

impl Htmlable for Template {
    fn to_html(&self) -> String {
        let mut s = "".to_string();
        for element in self.elements.clone() {
            let elem_str = match element {
                TemplateElement::String { encoded, .. } => encoded,
                TemplateElement::Expression(expr) => format!("{{{{{}}}}}", expr),
            };
            s.push_str(elem_str.as_str())
        }
        xml_escape(s)
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

fn add_line_terminators(buffer: &mut String, line_terminators: Vec<LineTerminator>) {
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

fn multilines(s: String) -> String {
    regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(s.as_str())
        .map(|l| format!("<span class=\"line\">{}</span>", xml_escape(l.to_string())))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_string() {
        // ``````
        let raw_string = RawString {
            newline: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            value: Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "".to_string(),
                    encoded: "unused".to_string(),
                }],
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        };
        assert_eq!(
            raw_string.to_html(),
            "<span class=\"raw\"><span class=\"line\">``````</span></span>".to_string()
        );

        // ```hello```
        let raw_string = RawString {
            newline: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            value: Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    encoded: "unused".to_string(),
                }],
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        };
        assert_eq!(
            raw_string.to_html(),
            "<span class=\"raw\"><span class=\"line\">```hello```</span></span>".to_string()
        );

        // ```
        // line1
        // line2
        // ```
        let raw_string = RawString {
            newline: Whitespace {
                value: "\n".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            value: Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "line1\nline2\n".to_string(),
                    encoded: "unused".to_string(),
                }],
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        };
        assert_eq!(
            raw_string.to_html(),
            "<span class=\"raw\"><span class=\"line\">```</span>\n<span class=\"line\">line1</span>\n<span class=\"line\">line2</span>\n<span class=\"line\">```</span></span>".to_string()
        );
    }

    #[test]
    fn test_multilines() {
        assert_eq!(
            multilines("{\n   \"id\": 1\n}".to_string()),
            "<span class=\"line\">{</span>\n<span class=\"line\">   \"id\": 1</span>\n<span class=\"line\">}</span>"
        );
        assert_eq!(
            multilines("<?xml version=\"1.0\"?>\n<drink>café</drink>".to_string()),
            "<span class=\"line\">&lt;?xml version=\"1.0\"?&gt;</span>\n<span class=\"line\">&lt;drink&gt;café&lt;/drink&gt;</span>"
        );
    }

    #[test]
    fn test_json() {
        let value = JsonValue::Object {
            space0: "".to_string(),
            elements: vec![JsonObjectElement {
                space0: "\n   ".to_string(),
                name: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "id".to_string(),
                        encoded: "id".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
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
            quotes: true,
            elements: vec![TemplateElement::String {
                value: "\n".to_string(),
                encoded: "\\n".to_string(),
            }],
            source_info: SourceInfo::init(0, 0, 0, 0),
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
        assert_eq!(xml_escape("hello".to_string()), "hello");
        assert_eq!(
            xml_escape("<?xml version=\"1.0\"?>".to_string()),
            "&lt;?xml version=\"1.0\"?&gt;"
        );
    }
}
