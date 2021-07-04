/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
//use hurlfmt::format::text::format_token;
//use hurlfmt::format::token::Tokenizable;
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
        buffer.push_str("<div class=\"hurl-file\">");
        for entry in self.clone().entries {
            buffer.push_str(entry.to_html().as_str());
        }
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("</div>");
        buffer
    }
}

impl Htmlable for Entry {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"hurl-entry\">");
        buffer.push_str(self.request.to_html().as_str());
        if let Some(response) = self.clone().response {
            buffer.push_str(response.to_html().as_str());
        }
        buffer.push_str("</div>");
        buffer
    }
}

impl Htmlable for Request {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"request\">");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.method.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(format!("<span class=\"url\">{}</span>", self.url.to_html()).as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str("</div>");
        for header in self.headers.clone() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        if let Some(body) = self.body.clone() {
            buffer.push_str(body.to_html().as_str());
        }
        buffer
    }
}

impl Htmlable for Response {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"response\">");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.version.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.status.to_html().as_str());
        buffer.push_str("</span>");
        for header in self.headers.clone() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        if let Some(body) = self.body.clone() {
            buffer.push_str(body.to_html().as_str());
        }
        buffer.push_str("</div>");
        buffer
    }
}

impl Htmlable for Method {
    fn to_html(&self) -> String {
        return format!("<span class=\"method\">{}</span>", self.to_string());
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
        format!("<span class=\"status\">{}</span>", self.value.to_string())
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
        buffer.push_str(format!("<span class=\"string\">{}</span>", self.key.value).as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(format!("<span class=\"string\">{}</span>", self.value.to_html()).as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
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
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
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
        let mut buffer = String::from("");
        buffer.push_str(self.value.as_str());
        buffer
    }
}

impl Htmlable for Cookie {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.value.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for CookieValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.value.as_str());
        buffer
    }
}

impl Htmlable for Capture {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.value.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
        buffer.push_str("</span>");
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
                buffer.push_str("<span class=\"query-type\">status</span>");
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
            QueryValue::Regex { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", expr.to_html()).as_str(),
                );
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
        }
        buffer
    }
}

impl Htmlable for Subquery {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self.value.clone() {
            SubqueryValue::Regex { expr, space0 } => {
                buffer.push_str("<span class=\"query-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!("<span class=\"string\">\"{}\"</span>", expr.to_html()).as_str(),
                );
            }
            SubqueryValue::Count {} => buffer.push_str("<span class=\"query-type\">count</span>"),
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
        buffer
    }
}

impl Htmlable for Predicate {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        if self.not {
            buffer.push_str("not");
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
                format!("<span class=\"number\">{}</span>", value.to_string())
            }
            PredicateValue::Bool(value) => format!("<span class=\"boolean\">{}</span>", value),
            PredicateValue::Hex(value) => {
                format!("<span class=\"hex\">{}</span>", value.to_string())
            }
            PredicateValue::Base64(value) => value.to_html(),
            PredicateValue::Expression(value) => value.to_html(),
            PredicateValue::Null {} => "<span class=\"null\">null</span>".to_string(),
        }
    }
}

impl Htmlable for RawString {
    fn to_html(&self) -> String {
        let mut buffer = String::from("```");
        if !self.newline.to_html().as_str().is_empty() {
            buffer.push_str("</span><span class=\"line\">");
        }

        let end_newline = self.value.to_string().ends_with('\n');
        let mut lines: Vec<String> = regex::Regex::new(r"\n|\r\n")
            .unwrap()
            .split(self.value.to_string().trim())
            .map(|l| l.to_string())
            .collect();

        buffer.push_str(xml_escape(lines.remove(0)).as_str());

        for line in lines {
            buffer.push_str("</span><span class=\"line\">");
            buffer.push_str(xml_escape(line).as_str());
        }
        if end_newline {
            buffer.push_str("</span><span class=\"line\">");
        }
        buffer.push_str("```");
        buffer
    }
}

impl Htmlable for Body {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Bytes {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            Bytes::Base64(value) => {
                buffer.push_str(value.to_html().as_str());
            }
            Bytes::Hex(value) => {
                buffer.push_str(value.to_html().as_str());
            }
            Bytes::File(value) => {
                buffer.push_str(value.to_html().as_str());
            }
            Bytes::RawString(value) => {
                buffer.push_str(value.to_html().as_str());
            }
            Bytes::Json { value } => buffer.push_str(value.to_html().as_str()),
            Bytes::Xml { value } => {
                let mut lines: Vec<String> = regex::Regex::new(r"\n|\r\n")
                    .unwrap()
                    .split(value.as_str())
                    .map(|l| l.to_string())
                    .collect();
                buffer.push_str(xml_escape(lines.remove(0)).as_str());
                for line in lines {
                    buffer.push_str("<span class=\"line\">");
                    buffer.push_str(xml_escape(line).as_str());
                    buffer.push_str("</span>");
                }
            }
        }
        buffer
    }
}

fn xml_escape(s: String) -> String {
    s.replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('&', "&amp;")
}

impl Htmlable for JsonValue {
    fn to_html(&self) -> String {
        let s = self.to_string();
        let mut lines: Vec<String> = regex::Regex::new(r"\n|\r\n")
            .unwrap()
            .split(s.as_str())
            .map(|l| l.to_string())
            .collect();
        let mut buffer = String::from("");
        buffer.push_str(lines.remove(0).as_str());
        for line in lines {
            buffer.push_str("<span class=\"line\">");
            buffer.push_str(line.as_str());
            buffer.push_str("</span>");
        }
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
            buffer.push_str("<span class=\"comment\">");
            buffer.push_str(format!("#{}", v.value.as_str()).as_str());
            buffer.push_str("</span>");
        }
        buffer
    }
}

impl Htmlable for File {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span>");
        buffer.push_str("file,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.filename.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for Base64 {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span>");
        buffer.push_str("base64,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.encoded.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer.push_str("</span>");
        buffer
    }
}
impl Htmlable for Hex {
    fn to_html(&self) -> String {
        let mut buffer = String::from("<span>");
        buffer.push_str("hex,");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.encoded.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push(';');
        buffer.push_str("</span>");
        buffer
    }
}

impl Htmlable for EncodedString {
    fn to_html(&self) -> String {
        format!("<span class=\"string\">{}</span>", self.encoded)
    }
}

impl Htmlable for Template {
    fn to_html(&self) -> String {
        xml_escape(self.to_string().replace("\n", "\\n"))
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
    for line_terminator in line_terminators.clone() {
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(line_terminator.to_html().as_str());
        if line_terminator.newline.value.is_empty() {
            buffer.push_str("<br>");
        }
        buffer.push_str("</span>");
    }
}

fn encode_html(s: String) -> String {
    s.replace(">", "&gt;").replace("<", "&lt;")
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
        assert_eq!(raw_string.to_html(), "``````".to_string());

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
        assert_eq!(raw_string.to_html(), "```hello```".to_string());

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
        assert_eq!(raw_string.to_html(), "```</span><span class=\"line\">line1</span><span class=\"line\">line2</span><span class=\"line\">```".to_string());
    }
}
