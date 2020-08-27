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
use super::super::core::ast::*;

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
    if standalone { format_standalone(hurl_file) } else { hurl_file.to_html() }
}

impl Htmlable for HurlFile {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"hurl-file\">");
        for entry in self.clone().entries {
            buffer.push_str(entry.to_html().as_str());
        }
        for line_terminator in self.line_terminators.clone() {
            buffer.push_str(line_terminator.to_html().as_str());
        }
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
        buffer.push_str(self.url.to_html().as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str("</div>");
        for header in self.headers.clone() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
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
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        buffer.push_str("</div>");
        buffer
    }
}

impl Htmlable for Method {
    fn to_html(&self) -> String {
        return format!("<span class=\"method\">{}</span>", self.as_str());
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

        buffer
            .push_str(format!("<span class=\"section-header\">[{}]</span>", self.name()).as_str());
        buffer.push_str("</span>");
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
        buffer.push_str(self.key.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
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
        buffer.push_str(self.key.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
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
        buffer.push_str(self.space0.to_html().as_str());
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
        buffer.push_str(self.line_terminator0.to_html().as_str());
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
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
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
                buffer.push_str("<span class=\"query-type\">status</span>");
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
            QueryValue::Regex { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">regex</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(expr.to_html().as_str());
            }
            QueryValue::Variable { space0, name } => {
                buffer.push_str("<span class=\"query-type\">variable</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(name.to_html().as_str());
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
            buffer.push_str(attribute.to_html().as_str());
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
            PredicateFuncValue::CountEqual { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::EqualString { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value.to_html()).as_str());
            }
            PredicateFuncValue::EqualInt { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::EqualFloat { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value.to_string()).as_str());
            }
            PredicateFuncValue::EqualExpression { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::StartWith { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">startsWith</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"string\">{}</span>", value.to_html()).as_str());
            }
            PredicateFuncValue::Contain { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">contains</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"string\">{}</span>", value.to_html()).as_str());
            }
            PredicateFuncValue::IncludeString { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">includes</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"string\">{}</span>", value.to_html()).as_str());
            }
            PredicateFuncValue::IncludeInt { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">includes</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::IncludeNull { space0 } => {
                buffer.push_str("<span class=\"predicate-type\">includes</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str("<span class=\"null\">null</span>");
            }
            PredicateFuncValue::IncludeBool { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">includes</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"boolean\">{}</span>", value).as_str());
            }
            PredicateFuncValue::IncludeFloat { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">includes</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value.to_string()).as_str());
            }
            PredicateFuncValue::IncludeExpression { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">includes</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(value.to_html().as_str());
            }
            PredicateFuncValue::Match { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">matches</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"string\">{}</span>", value.to_html()).as_str());
            }
            PredicateFuncValue::EqualNull { space0 } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str("<span class=\"null\">null</span>");
            }
            PredicateFuncValue::EqualBool { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"boolean\">{}</span>", value).as_str());
            }
            PredicateFuncValue::Exist {} => {
                buffer.push_str("<span class=\"predicate-type\">exists</span>");
            }
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

impl Htmlable for EncodedString {
    fn to_html(&self) -> String {
        format!("<span class=\"string\">\"{}\"</span>", self.encoded)
    }
}

impl Htmlable for Template {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        for element in self.elements.clone() {
            buffer.push_str(element.to_html().as_str());
        }
        buffer
    }
}

impl Htmlable for TemplateElement {
    fn to_html(&self) -> String {
        match self {
            TemplateElement::String { encoded, .. } => {
                format!("<span class=\"string\">{}</span>", encoded)
            }
            TemplateElement::Expression(value) => value.to_html(),
            /*            space0: _, variable: _, space1: _ } => {
                let mut buffer = String::from("");
                buffer.push_str("{{");
                buffer.push_str("}}");
                return buffer;
            }*/
        }
    }
}

impl Htmlable for Expr {
    fn to_html(&self) -> String {
        format!("<span class=\"variable\">{}</span>", self.variable.name)
    }
}

fn to_line(v: String) -> String {
    format!("<span class=\"line\">{}</span>", v)
}

fn add_line_terminators(buffer: &mut String, line_terminators: Vec<LineTerminator>) {
    for line_terminator in line_terminators.clone() {
        buffer.push_str(to_line(line_terminator.to_html()).as_str());
    }
}
