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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Method(String),
    Version(String),
    Status(String),
    SectionHeader(String),
    QueryType(String),
    PredicateType(String),
    Not(String),
    Keyword(String),

    // Primitives
    Whitespace(String),
    Comment(String),
    Value(String),
    Colon(String),
    Quote(String),
    Boolean(String),
    Number(String),
    String(String),
    CodeDelimiter(String),
    CodeVariable(String),
}

pub trait Tokenizable {
    fn tokenize(&self) -> Vec<Token>;
}

fn add_tokens(tokens1: &mut Vec<Token>, tokens2: Vec<Token>) {
    for token in tokens2 {
        tokens1.push(token);
    }
}

impl Tokenizable for HurlFile {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.entries.iter().flat_map(|e| e.tokenize()).collect(),
        );
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens
    }
}

impl Tokenizable for Entry {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(&mut tokens, self.request.tokenize());
        if let Some(response) = self.clone().response {
            add_tokens(&mut tokens, response.tokenize())
        }
        tokens
    }
}

impl Tokenizable for Request {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::Method(self.method.to_string()));
        add_tokens(&mut tokens, self.space1.tokenize());
        add_tokens(&mut tokens, self.url.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        add_tokens(
            &mut tokens,
            self.headers.iter().flat_map(|e| e.tokenize()).collect(),
        );
        add_tokens(
            &mut tokens,
            self.sections.iter().flat_map(|e| e.tokenize()).collect(),
        );
        if let Some(body) = self.clone().body {
            add_tokens(&mut tokens, body.tokenize())
        }
        tokens
    }
}

impl Tokenizable for Response {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.version.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        add_tokens(&mut tokens, self.status.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        add_tokens(
            &mut tokens,
            self.headers.iter().flat_map(|e| e.tokenize()).collect(),
        );
        add_tokens(
            &mut tokens,
            self.sections.iter().flat_map(|e| e.tokenize()).collect(),
        );
        if let Some(body) = self.clone().body {
            add_tokens(&mut tokens, body.tokenize())
        }
        tokens
    }
}

impl Tokenizable for Status {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self.value.clone() {
            StatusValue::Any => tokens.push(Token::Status("*".to_string())),
            StatusValue::Specific(v) => tokens.push(Token::Status(v.to_string())),
        }
        tokens
    }
}

impl Tokenizable for Version {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::Version(format!(
            "HTTP/{}",
            match self.value {
                VersionValue::Version1 => String::from("1.0"),
                VersionValue::Version11 => String::from("1.1"),
                VersionValue::Version2 => String::from("2"),
                VersionValue::VersionAny => String::from("*"),
            }
        ))]
    }
}

impl Tokenizable for Body {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Bytes {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            Bytes::Json { value } => tokens.append(&mut value.tokenize()),
            Bytes::Xml { value } => {
                tokens.push(Token::String(value.to_string()));
            }
            //            Bytes::MultilineString { value: _ } => {}
            Bytes::RawString(value) => {
                tokens.append(&mut value.tokenize());
            }
            Bytes::Base64(value) => {
                tokens.append(&mut value.tokenize());
            }
            Bytes::Hex(value) => {
                tokens.append(&mut value.tokenize());
            }
            Bytes::File(value) => {
                tokens.append(&mut value.tokenize());
            }
        }
        tokens
    }
}

impl Tokenizable for Section {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::SectionHeader(format!("[{}]", self.name())));
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        //        add_tokens(&mut tokens, self.space0.tokenize());
        //        tokens.push(Token::SectionHeader(format!("[{}]", self.name)));
        tokens
    }
}

impl Tokenizable for SectionValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            SectionValue::Asserts(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::QueryParams(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::BasicAuth(item) => {
                tokens.append(&mut item.tokenize());
            }
            SectionValue::FormParams(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::MultipartFormData(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::Cookies(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::Captures(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
        }
        tokens
    }
}

impl Tokenizable for Base64 {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword(String::from("base64,"))];
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::String(self.encoded.to_string()));
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Keyword(String::from(";")));
        tokens
    }
}

impl Tokenizable for Hex {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword(String::from("hex,"))];
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::String(self.encoded.to_string()));
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Keyword(String::from(";")));
        tokens
    }
}

impl Tokenizable for File {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword(String::from("file,"))];
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.filename.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Keyword(String::from(";")));
        tokens
    }
}

impl Tokenizable for KeyValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.key.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for MultipartParam {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            MultipartParam::Param(key_value) => key_value.tokenize(),
            MultipartParam::FileParam(file_param) => file_param.tokenize(),
        }
    }
}

impl Tokenizable for FileParam {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.key.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        tokens.append(&mut self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for FileValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword("file,".to_string())];
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.filename.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Keyword(";".to_string()));
        tokens.append(&mut self.space2.tokenize());
        if let Some(content_type) = self.content_type.clone() {
            tokens.push(Token::String(content_type));
        }
        tokens
    }
}

impl Tokenizable for Cookie {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.name.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Capture {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.name.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.query.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Assert {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.query.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        // TODO reconvert back your first predicate for jsonpath
        // so that you can use your firstX predicate for other query
        add_tokens(&mut tokens, self.predicate.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Query {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = self.value.clone().tokenize();
        if let Some((space, subquery)) = &self.subquery {
            add_tokens(&mut tokens, space.clone().tokenize());
            tokens.append(&mut subquery.tokenize());
        }
        tokens
    }
}

impl Tokenizable for QueryValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self.clone() {
            QueryValue::Status {} => tokens.push(Token::QueryType(String::from("status"))),
            QueryValue::Header { space0, name } => {
                tokens.push(Token::QueryType(String::from("header")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, name.tokenize());
            }
            QueryValue::Cookie { space0, expr } => {
                tokens.push(Token::QueryType(String::from("cookie")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::CodeDelimiter("\"".to_string()));
                add_tokens(&mut tokens, expr.tokenize());
                tokens.push(Token::CodeDelimiter("\"".to_string()));
            }
            QueryValue::Body {} => tokens.push(Token::QueryType(String::from("body"))),
            QueryValue::Xpath { space0, expr } => {
                tokens.push(Token::QueryType(String::from("xpath")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, expr.tokenize());
            }
            QueryValue::Jsonpath { space0, expr } => {
                tokens.push(Token::QueryType(String::from("jsonpath")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, expr.tokenize());
            }
            QueryValue::Regex { space0, value } => {
                tokens.push(Token::QueryType(String::from("regex")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            QueryValue::Variable { space0, name } => {
                tokens.push(Token::QueryType(String::from("variable")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, name.tokenize());
            }
            QueryValue::Duration {} => tokens.push(Token::QueryType(String::from("duration"))),
            QueryValue::Bytes {} => tokens.push(Token::QueryType(String::from("bytes"))),
            QueryValue::Sha256 {} => tokens.push(Token::QueryType(String::from("sha256"))),
            QueryValue::Md5 {} => tokens.push(Token::QueryType(String::from("md5"))),
        }
        tokens
    }
}

impl Tokenizable for RegexValue {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            RegexValue::Template(template) => template.tokenize(),
            RegexValue::Regex(regex) => regex.tokenize(),
        }
    }
}

impl Tokenizable for CookiePath {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(&mut tokens, self.name.tokenize());
        if let Some(attribute) = self.attribute.clone() {
            add_tokens(&mut tokens, attribute.tokenize());
        }
        tokens
    }
}

impl Tokenizable for CookieAttribute {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::CodeDelimiter("[".to_string())];
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::String(self.name.value()));
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::CodeDelimiter("]".to_string()));
        tokens
    }
}

impl Tokenizable for Subquery {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self.value.clone() {
            SubqueryValue::Regex { space0, value } => {
                tokens.push(Token::QueryType(String::from("regex")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            SubqueryValue::Count { .. } => {
                tokens.push(Token::QueryType(String::from("count")));
            }
        }
        tokens
    }
}

impl Tokenizable for Predicate {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.not {
            tokens.push(Token::Not(String::from("not")));
            add_tokens(&mut tokens, self.space0.tokenize());
        }
        add_tokens(&mut tokens, self.predicate_func.tokenize());
        tokens
    }
}

impl Tokenizable for PredicateFunc {
    fn tokenize(&self) -> Vec<Token> {
        self.value.tokenize()
    }
}

impl Tokenizable for PredicateFuncValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            PredicateFuncValue::Equal { space0, value, .. } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::NotEqual { space0, value, .. } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::GreaterThan { space0, value, .. } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::GreaterThanOrEqual { space0, value, .. } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::LessThan { space0, value, .. } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::LessThanOrEqual { space0, value, .. } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::CountEqual { space0, value } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::StartWith { space0, value } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            PredicateFuncValue::EndWith { space0, value } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            PredicateFuncValue::Contain { space0, value } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            PredicateFuncValue::Include { space0, value } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::Match { space0, value } => {
                tokens.push(Token::PredicateType(self.name()));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }

            PredicateFuncValue::IsInteger {} => {
                tokens.push(Token::PredicateType(self.name()));
            }
            PredicateFuncValue::IsFloat {} => {
                tokens.push(Token::PredicateType(self.name()));
            }
            PredicateFuncValue::IsBoolean {} => {
                tokens.push(Token::PredicateType(self.name()));
            }
            PredicateFuncValue::IsString {} => {
                tokens.push(Token::PredicateType(self.name()));
            }
            PredicateFuncValue::IsCollection {} => {
                tokens.push(Token::PredicateType(self.name()));
            }
            PredicateFuncValue::Exist {} => {
                tokens.push(Token::PredicateType(self.name()));
            }
        }
        tokens
    }
}

impl Tokenizable for PredicateValue {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            PredicateValue::String(value) => value.tokenize(),
            PredicateValue::Raw(value) => value.tokenize(),
            PredicateValue::Integer(value) => vec![Token::Number(value.to_string())],
            PredicateValue::Float(value) => vec![Token::Number(value.to_string())],
            PredicateValue::Bool(value) => vec![Token::Boolean(value.to_string())],
            PredicateValue::Null {} => vec![Token::Keyword("null".to_string())],
            PredicateValue::Hex(value) => vec![Token::String(value.to_string())],
            PredicateValue::Base64(value) => value.tokenize(),
            PredicateValue::Expression(value) => value.tokenize(),
            PredicateValue::Regex(value) => value.tokenize(),
        }
    }
}

impl Tokenizable for RawString {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![
            Token::Keyword("```".to_string()),
            Token::Whitespace(self.newline.value.clone()),
        ];
        tokens.append(&mut self.value.tokenize());
        tokens.push(Token::Keyword("```".to_string()));
        tokens
    }
}

impl Tokenizable for EncodedString {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.quotes {
            tokens.push(Token::Quote(
                if self.clone().quotes { "\"" } else { "" }.to_string(),
            ));
        }
        tokens.push(Token::String(self.encoded.clone()));

        if self.quotes {
            tokens.push(Token::Quote(
                if self.clone().quotes { "\"" } else { "" }.to_string(),
            ));
        }
        tokens
    }
}

impl Tokenizable for Template {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.quotes {
            tokens.push(Token::Quote(
                if self.clone().quotes { "\"" } else { "" }.to_string(),
            ));
        }
        for element in self.elements.clone() {
            add_tokens(&mut tokens, element.tokenize());
        }

        if self.quotes {
            tokens.push(Token::Quote(
                if self.clone().quotes { "\"" } else { "" }.to_string(),
            ));
        }
        tokens
    }
}

impl Tokenizable for TemplateElement {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            TemplateElement::String { encoded, .. } => {
                vec![Token::String(encoded.to_string())]
            }
            TemplateElement::Expression(value) => {
                let mut tokens: Vec<Token> = vec![];
                add_tokens(&mut tokens, value.tokenize());
                tokens
            }
        }
    }
}

impl Tokenizable for Expr {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::CodeDelimiter(String::from("{{"))];
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::CodeVariable(self.variable.name.clone()));
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::CodeDelimiter(String::from("}}")));
        tokens
    }
}

impl Tokenizable for Regex {
    fn tokenize(&self) -> Vec<Token> {
        let s = str::replace(self.inner.as_str(), "/", "\\/");
        vec![Token::String(format!("/{}/", s))]
    }
}

impl Tokenizable for LineTerminator {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(&mut tokens, self.space0.tokenize());
        if let Some(comment) = self.clone().comment {
            add_tokens(&mut tokens, comment.tokenize());
        }
        add_tokens(&mut tokens, self.newline.tokenize());
        tokens
    }
}

impl Tokenizable for Whitespace {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if !self.value.is_empty() {
            tokens.push(Token::Whitespace(self.clone().value));
        }
        tokens
    }
}

impl Tokenizable for Comment {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::Comment(format!("#{}", self.clone().value))]
    }
}

impl Tokenizable for Filename {
    fn tokenize(&self) -> Vec<Token> {
        let s = self.clone().value.replace(' ', "\\ ");
        return vec![Token::String(s)];
    }
}

impl Tokenizable for JsonValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            JsonValue::String(s) => {
                //tokens.push(Token::CodeDelimiter("\"".to_string()));
                tokens.append(&mut s.tokenize());
                //tokens.push(Token::CodeDelimiter("\"".to_string()));
            }
            JsonValue::Number(value) => {
                tokens.push(Token::Number(value.to_string()));
            }
            JsonValue::Boolean(value) => {
                tokens.push(Token::Boolean(value.to_string()));
            }
            JsonValue::List { space0, elements } => {
                tokens.push(Token::CodeDelimiter("[".to_string()));
                tokens.push(Token::Whitespace(space0.clone()));
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        tokens.push(Token::CodeDelimiter(",".to_string()));
                    }
                    tokens.append(&mut element.tokenize());
                }
                tokens.push(Token::CodeDelimiter("]".to_string()));
            }
            JsonValue::Object { space0, elements } => {
                tokens.push(Token::CodeDelimiter("{".to_string()));
                tokens.push(Token::Whitespace(space0.clone()));
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        tokens.push(Token::CodeDelimiter(",".to_string()));
                    }
                    tokens.append(&mut element.tokenize());
                }
                tokens.push(Token::CodeDelimiter("}".to_string()));
            }
            JsonValue::Null {} => {
                tokens.push(Token::Keyword("null".to_string()));
            }
            JsonValue::Expression(exp) => {
                tokens.append(&mut exp.tokenize());
            }
        }
        tokens
    }
}

impl Tokenizable for JsonListElement {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Whitespace(self.space0.clone())];
        tokens.append(&mut self.value.tokenize());
        tokens.push(Token::Whitespace(self.space1.clone()));
        tokens
    }
}

impl Tokenizable for JsonObjectElement {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Whitespace(self.space0.clone())];
        tokens.push(Token::Quote("\"".to_string()));
        tokens.push(Token::String(self.name.to_string()));
        tokens.push(Token::Quote("\"".to_string()));
        tokens.push(Token::Whitespace(self.space1.clone()));
        tokens.push(Token::CodeDelimiter(":".to_string()));
        tokens.push(Token::Whitespace(self.space2.clone()));
        tokens.append(&mut self.value.tokenize());
        tokens.push(Token::Whitespace(self.space3.clone()));
        tokens
    }
}
