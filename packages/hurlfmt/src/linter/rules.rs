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
use hurl_core::ast::*;

use super::core::{Error, Lintable, LinterError};

impl Lintable<HurlFile> for HurlFile {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        for entry in self.entries.clone() {
            errors.append(&mut (entry.errors()));
        }
        errors
    }

    fn lint(&self) -> HurlFile {
        HurlFile {
            entries: self.entries.iter().map(|e| e.lint()).collect(),
            line_terminators: self.line_terminators.clone(),
        }
    }
}

impl Lintable<Entry> for Entry {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        errors.append(&mut (self.request.errors()));
        errors
    }

    fn lint(&self) -> Entry {
        Entry {
            request: self.request.lint(),
            response: self.clone().response.map(|response| response.lint()),
        }
    }
}

impl Lintable<Request> for Request {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        if !self.space0.value.is_empty() {
            errors.push(Error {
                source_info: self.clone().space0.source_info,
                inner: LinterError::UnneccessarySpace {},
            });
        }
        if self.space1.value != " " {
            errors.push(Error {
                source_info: self.clone().space1.source_info,
                inner: LinterError::OneSpace {},
            });
        }
        for error in self.line_terminator0.errors() {
            errors.push(error);
        }
        errors
    }

    fn lint(&self) -> Request {
        let line_terminators = self.clone().line_terminators;
        let space0 = empty_whitespace();
        let method = self.clone().method;
        let space1 = one_whitespace();

        let url = self.url.clone();
        let line_terminator0 = self.line_terminator0.lint();
        let headers = self.headers.iter().map(|e| e.lint()).collect();
        let b = self.clone().body.map(|body| body.lint());
        let mut sections: Vec<Section> = self.sections.iter().map(|e| e.lint()).collect();
        sections.sort_by_key(|k| section_value_index(k.value.clone()));

        let source_info = SourceInfo::init(0, 0, 0, 0);
        Request {
            line_terminators,
            space0,
            method,
            space1,
            url,
            line_terminator0,
            headers,
            sections,
            body: b,
            source_info,
        }
    }
}

impl Lintable<Response> for Response {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        if !self.space0.value.is_empty() {
            errors.push(Error {
                source_info: self.clone().space0.source_info,
                inner: LinterError::UnneccessarySpace {},
            });
        }
        errors
    }

    fn lint(&self) -> Response {
        let line_terminators = self.clone().line_terminators;
        let space0 = empty_whitespace();
        let _version = self.clone().version;
        let space1 = self.clone().space1;
        let _status = self.clone().status;
        let line_terminator0 = self.clone().line_terminator0;
        let headers = self.headers.iter().map(|e| e.lint()).collect();
        let mut sections: Vec<Section> = self.sections.iter().map(|e| e.lint()).collect();
        sections.sort_by_key(|k| section_value_index(k.value.clone()));

        let b = self.body.clone();
        Response {
            line_terminators,
            space0,
            version: _version,
            space1,
            status: _status,
            line_terminator0,
            headers,
            sections,
            body: b,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }
}

impl Lintable<Section> for Section {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Section {
        let line_terminators = self.clone().line_terminators;
        Section {
            line_terminators,
            space0: empty_whitespace(),
            value: self.value.lint(),
            line_terminator0: self.clone().line_terminator0,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }
}

impl Lintable<SectionValue> for SectionValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> SectionValue {
        match self {
            SectionValue::QueryParams(params) => {
                SectionValue::QueryParams(params.iter().map(|e| e.lint()).collect())
            }
            SectionValue::BasicAuth(param) => SectionValue::BasicAuth(param.lint()),
            SectionValue::Captures(captures) => {
                SectionValue::Captures(captures.iter().map(|e| e.lint()).collect())
            }
            SectionValue::Asserts(asserts) => {
                SectionValue::Asserts(asserts.iter().map(|e| e.lint()).collect())
            }
            SectionValue::FormParams(params) => {
                SectionValue::FormParams(params.iter().map(|e| e.lint()).collect())
            }
            SectionValue::MultipartFormData(params) => {
                SectionValue::MultipartFormData(params.iter().map(|e| e.lint()).collect())
            }
            SectionValue::Cookies(cookies) => {
                SectionValue::Cookies(cookies.iter().map(|e| e.lint()).collect())
            }
        }
    }
}

fn section_value_index(section_value: SectionValue) -> u32 {
    match section_value {
        SectionValue::QueryParams(_) => 0,
        SectionValue::BasicAuth(_) => 1,
        SectionValue::FormParams(_) => 2,
        SectionValue::MultipartFormData(_) => 3,
        SectionValue::Cookies(_) => 3,
        SectionValue::Captures(_) => 0,
        SectionValue::Asserts(_) => 1,
    }
}

impl Lintable<Assert> for Assert {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Assert {
        Assert {
            line_terminators: self.line_terminators.clone(),
            space0: empty_whitespace(),
            query: self.query.lint(),
            space1: one_whitespace(),
            predicate: self.predicate.lint(),
            line_terminator0: self.line_terminator0.clone(),
        }
    }
}

impl Lintable<Capture> for Capture {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Capture {
        Capture {
            line_terminators: self.clone().line_terminators,
            space0: empty_whitespace(),
            name: self.name.clone(),
            space1: empty_whitespace(),
            space2: one_whitespace(),
            query: self.query.lint(),
            line_terminator0: self.line_terminator0.lint(),
        }
    }
}

impl Lintable<Query> for Query {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Query {
        Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: self.value.lint(),
            subquery: self
                .subquery
                .clone()
                .map(|(_, s)| (one_whitespace(), s.lint())),
        }
    }
}

impl Lintable<QueryValue> for QueryValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> QueryValue {
        match self {
            QueryValue::Status {} => QueryValue::Status {},
            QueryValue::Header { name, .. } => QueryValue::Header {
                name: name.clone(),
                space0: one_whitespace(),
            },
            QueryValue::Cookie {
                expr: CookiePath { name, attribute },
                ..
            } => {
                let attribute = attribute.as_ref().map(|attribute| attribute.lint());
                QueryValue::Cookie {
                    space0: one_whitespace(),
                    expr: CookiePath {
                        name: name.clone(),
                        attribute,
                    },
                }
            }
            QueryValue::Body {} => QueryValue::Body {},
            QueryValue::Xpath { expr, .. } => QueryValue::Xpath {
                expr: expr.clone(),
                space0: one_whitespace(),
            },
            QueryValue::Jsonpath { expr, .. } => QueryValue::Jsonpath {
                expr: expr.clone(),
                space0: one_whitespace(),
            },
            QueryValue::Regex { value, .. } => QueryValue::Regex {
                value: value.lint(),
                space0: one_whitespace(),
            },
            QueryValue::Variable { name, .. } => QueryValue::Variable {
                name: name.clone(),
                space0: one_whitespace(),
            },
            QueryValue::Duration {} => QueryValue::Duration {},
            QueryValue::Bytes {} => QueryValue::Bytes {},
            QueryValue::Sha256 {} => QueryValue::Sha256 {},
            QueryValue::Md5 {} => QueryValue::Md5 {},
        }
    }
}

impl Lintable<Subquery> for Subquery {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Subquery {
        let source_info = SourceInfo::init(0, 0, 0, 0);
        let value = self.value.lint();
        Subquery { source_info, value }
    }
}

impl Lintable<RegexValue> for RegexValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> RegexValue {
        match self {
            RegexValue::Template(template) => RegexValue::Template(template.lint()),
            RegexValue::Regex(regex) => RegexValue::Regex(regex.clone()),
        }
    }
}
impl Lintable<SubqueryValue> for SubqueryValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> SubqueryValue {
        match self {
            SubqueryValue::Regex { value, .. } => SubqueryValue::Regex {
                space0: one_whitespace(),
                value: value.lint(),
            },
            SubqueryValue::Count {} => SubqueryValue::Count {},
        }
    }
}

impl Lintable<CookieAttribute> for CookieAttribute {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> CookieAttribute {
        let space0 = empty_whitespace();
        let name = self.name.lint();
        let space1 = empty_whitespace();
        CookieAttribute {
            space0,
            name,
            space1,
        }
    }
}

impl Lintable<CookieAttributeName> for CookieAttributeName {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> CookieAttributeName {
        match self {
            CookieAttributeName::Value(_) => CookieAttributeName::Value("Value".to_string()),
            CookieAttributeName::Expires(_) => CookieAttributeName::Expires("Expires".to_string()),
            CookieAttributeName::MaxAge(_) => CookieAttributeName::MaxAge("Max-Age".to_string()),
            CookieAttributeName::Domain(_) => CookieAttributeName::Domain("Domain".to_string()),
            CookieAttributeName::Path(_) => CookieAttributeName::Path("Path".to_string()),
            CookieAttributeName::Secure(_) => CookieAttributeName::Secure("Secure".to_string()),
            CookieAttributeName::HttpOnly(_) => {
                CookieAttributeName::HttpOnly("HttpOnly".to_string())
            }
            CookieAttributeName::SameSite(_) => {
                CookieAttributeName::SameSite("SameSite".to_string())
            }
        }
    }
}

impl Lintable<Predicate> for Predicate {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Predicate {
        Predicate {
            not: self.clone().not,
            space0: if self.not {
                one_whitespace()
            } else {
                empty_whitespace()
            },
            predicate_func: self.predicate_func.lint(),
        }
    }
}

impl Lintable<PredicateFunc> for PredicateFunc {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> PredicateFunc {
        PredicateFunc {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: self.value.lint(),
        }
    }
}

impl Lintable<PredicateFuncValue> for PredicateFuncValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    #[allow(clippy::clone_on_copy)]
    fn lint(&self) -> PredicateFuncValue {
        match self {
            PredicateFuncValue::Equal { value, .. } => PredicateFuncValue::Equal {
                space0: one_whitespace(),
                value: value.lint(),
                operator: true,
            },
            PredicateFuncValue::NotEqual { value, .. } => PredicateFuncValue::NotEqual {
                space0: one_whitespace(),
                value: value.lint(),
                operator: true,
            },
            PredicateFuncValue::GreaterThan { value, .. } => PredicateFuncValue::GreaterThan {
                space0: one_whitespace(),
                value: value.lint(),
                operator: true,
            },
            PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
                PredicateFuncValue::GreaterThanOrEqual {
                    space0: one_whitespace(),
                    value: value.lint(),
                    operator: true,
                }
            }
            PredicateFuncValue::LessThan { value, .. } => PredicateFuncValue::GreaterThan {
                space0: one_whitespace(),
                value: value.lint(),
                operator: true,
            },
            PredicateFuncValue::LessThanOrEqual { value, .. } => {
                PredicateFuncValue::GreaterThanOrEqual {
                    space0: one_whitespace(),
                    value: value.lint(),
                    operator: true,
                }
            }
            PredicateFuncValue::Contain { value, .. } => PredicateFuncValue::Contain {
                space0: one_whitespace(),
                value: value.clone().lint(),
            },

            PredicateFuncValue::Include { value, .. } => PredicateFuncValue::Include {
                space0: one_whitespace(),
                value: value.lint(),
            },

            PredicateFuncValue::Match { value, .. } => PredicateFuncValue::Match {
                space0: one_whitespace(),
                value: value.clone().lint(),
            },
            PredicateFuncValue::StartWith { value, .. } => PredicateFuncValue::StartWith {
                space0: one_whitespace(),
                value: value.clone().lint(),
            },
            PredicateFuncValue::EndWith { value, .. } => PredicateFuncValue::EndWith {
                space0: one_whitespace(),
                value: value.clone().lint(),
            },
            PredicateFuncValue::CountEqual { value, .. } => PredicateFuncValue::CountEqual {
                space0: one_whitespace(),
                value: value.clone(),
            },
            PredicateFuncValue::IsInteger {} => PredicateFuncValue::IsInteger {},
            PredicateFuncValue::IsFloat {} => PredicateFuncValue::IsFloat {},
            PredicateFuncValue::IsBoolean {} => PredicateFuncValue::IsBoolean {},
            PredicateFuncValue::IsString {} => PredicateFuncValue::IsString {},
            PredicateFuncValue::IsCollection {} => PredicateFuncValue::IsCollection {},
            PredicateFuncValue::Exist {} => PredicateFuncValue::Exist {},
        }
    }
}

impl Lintable<PredicateValue> for PredicateValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> PredicateValue {
        match self {
            PredicateValue::String(value) => PredicateValue::String(value.lint()),
            PredicateValue::Raw(value) => PredicateValue::Raw(value.lint()),
            PredicateValue::Integer(value) => PredicateValue::Integer(*value),
            PredicateValue::Float(value) => PredicateValue::Float(value.clone()),
            PredicateValue::Bool(value) => PredicateValue::Bool(*value),
            PredicateValue::Null {} => PredicateValue::Null {},
            PredicateValue::Hex(value) => PredicateValue::Hex(value.lint()),
            PredicateValue::Base64(value) => PredicateValue::Base64(value.lint()),
            PredicateValue::Expression(value) => PredicateValue::Expression(value.clone()),
            PredicateValue::Regex(value) => PredicateValue::Regex(value.clone()),
        }
    }
}
impl Lintable<RawString> for RawString {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> RawString {
        RawString {
            newline: self.newline.clone(),
            value: self.value.lint(),
        }
    }
}

impl Lintable<Cookie> for Cookie {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Cookie {
        self.clone()
    }
}

impl Lintable<Body> for Body {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> Body {
        let line_terminators = self.clone().line_terminators;
        let space0 = empty_whitespace();
        let value = self.value.lint();
        let line_terminator0 = self.clone().line_terminator0;
        Body {
            line_terminators,
            space0,
            value,
            line_terminator0,
        }
    }
}

impl Lintable<Bytes> for Bytes {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> Bytes {
        //let space0 = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        //let value = self.value.lint();
        //let line_terminator0 = self.clone().line_terminator0;
        match self {
            Bytes::File(value) => Bytes::File(value.lint()),
            Bytes::Base64(value) => Bytes::Base64(value.lint()),
            Bytes::Hex(value) => Bytes::Hex(value.lint()),
            Bytes::Json { value } => Bytes::Json {
                value: value.clone(),
            },
            Bytes::RawString(value) => Bytes::RawString(value.lint()),
            Bytes::Xml { value } => Bytes::Xml {
                value: value.clone(),
            },
        }
    }
}

impl Lintable<Base64> for Base64 {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> Base64 {
        Base64 {
            space0: one_whitespace(),
            value: self.value.clone(),
            encoded: self.encoded.clone(),
            space1: empty_whitespace(),
        }
    }
}

impl Lintable<Hex> for Hex {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> Hex {
        Hex {
            space0: one_whitespace(),
            value: self.value.clone(),
            encoded: self.encoded.clone(),
            space1: empty_whitespace(),
        }
    }
}

impl Lintable<File> for File {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> File {
        File {
            space0: one_whitespace(),
            filename: Filename {
                value: self.filename.clone().value,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: empty_whitespace(),
        }
    }
}

impl Lintable<KeyValue> for KeyValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> KeyValue {
        KeyValue {
            line_terminators: self.clone().line_terminators,
            space0: empty_whitespace(),
            key: self.clone().key,
            space1: empty_whitespace(),
            space2: if self.value.clone().elements.is_empty() {
                empty_whitespace()
            } else {
                one_whitespace()
            },
            value: self.clone().value,
            line_terminator0: self.clone().line_terminator0,
        }
    }
}

impl Lintable<MultipartParam> for MultipartParam {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> MultipartParam {
        match self {
            MultipartParam::Param(param) => MultipartParam::Param(param.lint()),
            MultipartParam::FileParam(file_param) => MultipartParam::FileParam(file_param.lint()),
        }
    }
    //        let line_terminators = self.line_terminators.clone();
    //        let space0 = empty_whitespace();
    //        let key = self.key.clone();
    //        let space1 = empty_whitespace();
    //        let space2 =  self.space2.clone();
    //        let value = self.clone().value;
    //        let line_terminator0 = self.clone().line_terminator0;
    //        MultipartParam { line_terminators, space0, key,space1, space2, value, line_terminator0}
    //    }
}

impl Lintable<FileParam> for FileParam {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> FileParam {
        let line_terminators = self.line_terminators.clone();
        let space0 = self.space0.clone();
        let key = self.key.clone();
        let space1 = self.space1.clone();
        let space2 = self.space2.clone();
        let value = self.value.clone();
        let line_terminator0 = self.line_terminator0.clone();
        FileParam {
            line_terminators,
            space0,
            key,
            space1,
            space2,
            value,
            line_terminator0,
        }
    }
}

fn empty_whitespace() -> Whitespace {
    Whitespace {
        value: "".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    }
}

fn one_whitespace() -> Whitespace {
    Whitespace {
        value: " ".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    }
}

impl Lintable<LineTerminator> for LineTerminator {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        match self.clone().comment {
            Some(value) => {
                for error in value.errors() {
                    errors.push(error);
                }
            }
            None => {
                if !self.space0.value.is_empty() {
                    errors.push(Error {
                        source_info: self.clone().space0.source_info,
                        inner: LinterError::UnneccessarySpace {},
                    });
                }
            }
        }
        errors
    }

    fn lint(&self) -> LineTerminator {
        let space0 = match self.comment {
            None => empty_whitespace(),
            Some(_) => Whitespace {
                value: self.clone().space0.value,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        };
        let comment = self.clone().comment.map(|comment| comment.lint());
        let newline = Whitespace {
            value: if self.newline.value.is_empty() {
                "".to_string()
            } else {
                "\n".to_string()
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        LineTerminator {
            space0,
            comment,
            newline,
        }
    }
}

impl Lintable<Comment> for Comment {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];

        errors
    }

    fn lint(&self) -> Comment {
        Comment {
            value: if self.value.starts_with(' ') {
                self.clone().value
            } else {
                format!(" {}", self.value)
            },
        }
    }
}

impl Lintable<Template> for Template {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        errors
    }

    fn lint(&self) -> Template {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hurl_file() {
        let hurl_file = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        let hurl_file_linted = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        assert_eq!(hurl_file.errors(), vec![]);
        assert_eq!(hurl_file.lint(), hurl_file_linted);
    }

    #[test]
    fn test_entry() {
        let entry = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        let entry_linted = HurlFile {
            entries: vec![],
            line_terminators: vec![],
        };
        assert_eq!(entry.errors(), vec![]);
        assert_eq!(entry.lint(), entry_linted);
    }
}
