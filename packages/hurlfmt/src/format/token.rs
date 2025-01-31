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
use hurl_core::ast::{
    Assert, Base64, Body, BooleanOption, Bytes, Capture, CertificateAttributeName, Comment, Cookie,
    CookieAttribute, CookiePath, CountOption, DurationOption, Entry, EntryOption, Expr, ExprKind,
    File, FileParam, FileValue, Filter, FilterValue, Function, GraphQl, GraphQlVariables, Hex,
    HurlFile, JsonListElement, JsonObjectElement, JsonValue, KeyValue, LineTerminator, Method,
    MultilineString, MultilineStringAttribute, MultilineStringKind, MultipartParam, NaturalOption,
    OptionKind, Placeholder, Predicate, PredicateFunc, PredicateFuncValue, PredicateValue, Query,
    QueryValue, Regex, RegexValue, Request, Response, Section, SectionValue, Status, StatusValue,
    Template, TemplateElement, Text, Variable, VariableDefinition, VariableValue, Version,
    Whitespace, I64, U64,
};
use hurl_core::typing::{Count, Duration};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Method(String),
    Version(String),
    Status(String),
    SectionHeader(String),
    QueryType(String),
    PredicateType(String),
    FilterType(String),
    Not(String),
    Keyword(String),

    // Primitives
    Whitespace(String),
    Comment(String),
    Value(String),
    Colon(String),
    StringDelimiter(String),
    Boolean(String),
    Number(String),
    String(String),
    CodeDelimiter(String),
    CodeVariable(String),
    Lang(String),
    Unit(String),
}

pub trait Tokenizable {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizable for HurlFile {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(&mut self.entries.iter().flat_map(|e| e.tokenize()).collect());
        tokens.append(
            &mut self
                .line_terminators
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
        tokens.append(&mut self.request.tokenize());
        if let Some(response) = &self.response {
            tokens.append(&mut response.tokenize());
        }
        tokens
    }
}

impl Tokenizable for Request {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.method.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.append(&mut self.url.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens.append(&mut self.headers.iter().flat_map(|e| e.tokenize()).collect());
        tokens.append(&mut self.sections.iter().flat_map(|e| e.tokenize()).collect());
        if let Some(body) = &self.body {
            tokens.append(&mut body.tokenize());
        }
        tokens
    }
}

impl Tokenizable for Method {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::Method(self.to_string())]
    }
}

impl Tokenizable for Response {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.version.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.append(&mut self.status.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens.append(&mut self.headers.iter().flat_map(|e| e.tokenize()).collect());
        tokens.append(&mut self.sections.iter().flat_map(|e| e.tokenize()).collect());
        if let Some(body) = self.clone().body {
            tokens.append(&mut body.tokenize());
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
        vec![Token::Version(self.value.to_string())]
    }
}

impl Tokenizable for Body {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Bytes {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            Bytes::Json(value) => tokens.append(&mut value.tokenize()),
            Bytes::Xml(value) => tokens.push(Token::String(value.to_string())),
            Bytes::MultilineString(value) => tokens.append(&mut value.tokenize()),
            Bytes::OnelineString(value) => tokens.append(&mut value.tokenize()),
            Bytes::Base64(value) => tokens.append(&mut value.tokenize()),
            Bytes::Hex(value) => tokens.append(&mut value.tokenize()),
            Bytes::File(value) => tokens.append(&mut value.tokenize()),
        }
        tokens
    }
}

impl Tokenizable for Section {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.push(Token::SectionHeader(format!("[{}]", self.identifier())));
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens
    }
}

impl Tokenizable for SectionValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            SectionValue::Asserts(items) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
            SectionValue::QueryParams(items, _) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
            SectionValue::BasicAuth(item) => {
                if let Some(kv) = item {
                    tokens.append(&mut kv.tokenize());
                }
            }
            SectionValue::FormParams(items, _) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
            SectionValue::MultipartFormData(items, _) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
            SectionValue::Cookies(items) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
            SectionValue::Captures(items) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
            SectionValue::Options(items) => {
                tokens.append(&mut items.iter().flat_map(|e| e.tokenize()).collect());
            }
        }
        tokens
    }
}

impl Tokenizable for Base64 {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword(String::from("base64,"))];
        tokens.append(&mut self.space0.tokenize());
        tokens.push(Token::String(self.source.to_string()));
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Keyword(String::from(";")));
        tokens
    }
}

impl Tokenizable for Hex {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword(String::from("hex,"))];
        tokens.append(&mut self.space0.tokenize());
        tokens.push(Token::String(self.source.to_string()));
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Keyword(String::from(";")));
        tokens
    }
}

impl Tokenizable for File {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::Keyword(String::from("file,"))];
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.filename.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Keyword(String::from(";")));
        tokens
    }
}

impl Tokenizable for KeyValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.key.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        tokens.append(&mut self.space2.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
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
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.key.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        tokens.append(&mut self.space2.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
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
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.name.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        tokens.append(&mut self.space2.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Capture {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.name.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        tokens.append(&mut self.space2.tokenize());
        tokens.append(&mut self.query.tokenize());
        for (space, filter) in &self.filters {
            tokens.append(&mut space.tokenize());
            tokens.append(&mut filter.tokenize());
        }
        if self.redact {
            tokens.append(&mut self.space3.tokenize());
            tokens.push(Token::Keyword(String::from("redact")));
        }
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Assert {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.query.tokenize());
        for (space, filter) in &self.filters {
            tokens.append(&mut space.tokenize());
            tokens.append(&mut filter.tokenize());
        }
        tokens.append(&mut self.space1.tokenize());
        // TODO reconvert back your first predicate for jsonpath
        // so that you can use your firstX predicate for other query
        tokens.append(&mut self.predicate.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for Query {
    fn tokenize(&self) -> Vec<Token> {
        self.value.tokenize()
    }
}

impl Tokenizable for QueryValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = vec![];
        let token = Token::QueryType(self.identifier().to_string());
        tokens.push(token);

        match self {
            QueryValue::Header { space0, name } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut name.tokenize());
            }
            QueryValue::Cookie { space0, expr } => {
                tokens.append(&mut space0.tokenize());
                tokens.push(Token::CodeDelimiter("\"".to_string()));
                tokens.append(&mut expr.tokenize());
                tokens.push(Token::CodeDelimiter("\"".to_string()));
            }
            QueryValue::Xpath { space0, expr } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut expr.tokenize());
            }
            QueryValue::Jsonpath { space0, expr } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut expr.tokenize());
            }
            QueryValue::Regex { space0, value } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            QueryValue::Variable { space0, name } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut name.tokenize());
            }
            QueryValue::Certificate {
                space0,
                attribute_name: field,
            } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut field.tokenize());
            }
            _ => {}
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
        tokens.append(&mut self.name.tokenize());
        if let Some(attribute) = self.attribute.clone() {
            tokens.append(&mut attribute.tokenize());
        }
        tokens
    }
}

impl Tokenizable for CookieAttribute {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::CodeDelimiter("[".to_string())];
        tokens.append(&mut self.space0.tokenize());
        tokens.push(Token::String(self.name.value()));
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::CodeDelimiter("]".to_string()));
        tokens
    }
}

impl Tokenizable for CertificateAttributeName {
    fn tokenize(&self) -> Vec<Token> {
        vec![
            Token::StringDelimiter("\"".to_string()),
            Token::String(self.identifier().to_string()),
            Token::StringDelimiter("\"".to_string()),
        ]
    }
}

impl Tokenizable for Predicate {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.not {
            tokens.push(Token::Not(String::from("not")));
            tokens.append(&mut self.space0.tokenize());
        }
        tokens.append(&mut self.predicate_func.tokenize());
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
        let name = self.identifier().to_string();
        match self {
            PredicateFuncValue::Equal { space0, value, .. } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::NotEqual { space0, value, .. } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::GreaterThan { space0, value, .. } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::GreaterThanOrEqual { space0, value, .. } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::LessThan { space0, value, .. } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::LessThanOrEqual { space0, value, .. } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::StartWith { space0, value } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::EndWith { space0, value } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::Contain { space0, value } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::Include { space0, value } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::Match { space0, value } => {
                tokens.push(Token::PredicateType(name));
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            PredicateFuncValue::IsInteger => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsFloat => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsBoolean => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsString => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsCollection => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsDate => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsIsoDate => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::Exist => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsEmpty => {
                tokens.push(Token::PredicateType(name));
            }
            PredicateFuncValue::IsNumber => {
                tokens.push(Token::PredicateType(name));
            }
        }
        tokens
    }
}

impl Tokenizable for PredicateValue {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            PredicateValue::String(value) => value.tokenize(),
            PredicateValue::MultilineString(value) => value.tokenize(),
            PredicateValue::Bool(value) => vec![Token::Boolean(value.to_string())],
            PredicateValue::Null => vec![Token::Keyword("null".to_string())],
            PredicateValue::Number(value) => vec![Token::Number(value.to_string())],
            PredicateValue::File(value) => value.tokenize(),
            PredicateValue::Hex(value) => vec![Token::String(value.to_string())],
            PredicateValue::Base64(value) => value.tokenize(),
            PredicateValue::Placeholder(value) => value.tokenize(),
            PredicateValue::Regex(value) => value.tokenize(),
        }
    }
}

impl Tokenizable for MultilineString {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::StringDelimiter("```".to_string())];
        tokens.push(Token::Lang(self.lang().to_string()));
        for (i, attribute) in self.attributes.iter().enumerate() {
            if i > 0 || !self.lang().is_empty() {
                tokens.push(Token::StringDelimiter(",".to_string()));
            }
            tokens.append(&mut attribute.tokenize());
        }
        match self {
            MultilineString {
                kind: MultilineStringKind::Text(text),
                ..
            }
            | MultilineString {
                kind: MultilineStringKind::Json(text),
                ..
            }
            | MultilineString {
                kind: MultilineStringKind::Xml(text),
                ..
            } => tokens.append(&mut text.tokenize()),
            MultilineString {
                kind: MultilineStringKind::GraphQl(graphql),
                ..
            } => tokens.append(&mut graphql.tokenize()),
        }
        tokens.push(Token::StringDelimiter("```".to_string()));
        tokens
    }
}

impl Tokenizable for MultilineStringAttribute {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            MultilineStringAttribute::Escape => vec![Token::String("escape".to_string())],
            MultilineStringAttribute::NoVariable => vec![Token::String("novariable".to_string())],
        }
    }
}

impl Tokenizable for Text {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(&mut self.space.tokenize());
        tokens.append(&mut self.newline.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens
    }
}

impl Tokenizable for GraphQl {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(&mut self.space.tokenize());
        tokens.append(&mut self.newline.tokenize());
        tokens.append(&mut self.value.tokenize());
        if let Some(vars) = &self.variables {
            tokens.append(&mut vars.tokenize());
        }
        tokens
    }
}

impl Tokenizable for GraphQlVariables {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.push(Token::String("variables".to_string()));
        tokens.append(&mut self.space.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens.append(&mut self.whitespace.tokenize());
        tokens
    }
}

impl Tokenizable for Template {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if let Some(d) = self.delimiter {
            tokens.push(Token::StringDelimiter(d.to_string()));
        }
        for element in &self.elements {
            tokens.append(&mut element.tokenize());
        }
        if let Some(d) = self.delimiter {
            tokens.push(Token::StringDelimiter(d.to_string()));
        }
        tokens
    }
}

impl Tokenizable for TemplateElement {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            TemplateElement::String { source, .. } => {
                vec![Token::String(source.to_string())]
            }
            TemplateElement::Placeholder(value) => {
                let mut tokens: Vec<Token> = vec![];
                tokens.append(&mut value.tokenize());
                tokens
            }
        }
    }
}

impl Tokenizable for Placeholder {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::CodeDelimiter(String::from("{{"))];
        tokens.append(&mut self.space0.tokenize());
        tokens.append(&mut self.expr.tokenize());
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::CodeDelimiter(String::from("}}")));
        tokens
    }
}

impl Tokenizable for Expr {
    fn tokenize(&self) -> Vec<Token> {
        self.kind.tokenize()
    }
}

impl Tokenizable for ExprKind {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            ExprKind::Variable(variable) => variable.tokenize(),
            ExprKind::Function(function) => function.tokenize(),
        }
    }
}

impl Tokenizable for Variable {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::CodeVariable(self.name.clone())]
    }
}

impl Tokenizable for Function {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            Function::NewDate => vec![Token::CodeVariable("newDate".to_string())],
            Function::NewUuid => vec![Token::CodeVariable("newUuid".to_string())],
        }
    }
}

impl Tokenizable for Regex {
    fn tokenize(&self) -> Vec<Token> {
        let s = str::replace(self.inner.as_str(), "/", "\\/");
        vec![Token::String(format!("/{s}/"))]
    }
}

impl Tokenizable for LineTerminator {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(&mut self.space0.tokenize());
        if let Some(comment) = &self.comment {
            tokens.append(&mut comment.tokenize());
        }
        tokens.append(&mut self.newline.tokenize());
        tokens
    }
}

impl Tokenizable for Whitespace {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if !self.value.is_empty() {
            tokens.push(Token::Whitespace(self.value.clone()));
        }
        tokens
    }
}

impl Tokenizable for Comment {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::Comment(format!("#{}", self.value.clone()))]
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
            JsonValue::Null => {
                tokens.push(Token::Keyword("null".to_string()));
            }
            JsonValue::Placeholder(exp) => {
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
        tokens.push(Token::StringDelimiter("\"".to_string()));
        tokens.push(Token::String(self.name.to_string()));
        tokens.push(Token::StringDelimiter("\"".to_string()));
        tokens.push(Token::Whitespace(self.space1.clone()));
        tokens.push(Token::CodeDelimiter(":".to_string()));
        tokens.push(Token::Whitespace(self.space2.clone()));
        tokens.append(&mut self.value.tokenize());
        tokens.push(Token::Whitespace(self.space3.clone()));
        tokens
    }
}

impl Tokenizable for EntryOption {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.append(
            &mut self
                .line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        tokens.append(&mut self.space0.tokenize());
        tokens.push(Token::String(self.kind.identifier().to_string()));
        tokens.append(&mut self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        tokens.append(&mut self.space2.tokenize());
        tokens.append(&mut self.kind.tokenize());
        tokens.append(&mut self.line_terminator0.tokenize());
        tokens
    }
}

impl Tokenizable for OptionKind {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            OptionKind::AwsSigV4(value) => value.tokenize(),
            OptionKind::CaCertificate(filename) => filename.tokenize(),
            OptionKind::ClientCert(filename) => filename.tokenize(),
            OptionKind::ClientKey(filename) => filename.tokenize(),
            OptionKind::Compressed(value) => value.tokenize(),
            OptionKind::ConnectTo(value) => value.tokenize(),
            OptionKind::ConnectTimeout(value) => value.tokenize(),
            OptionKind::Delay(value) => value.tokenize(),
            OptionKind::FollowLocation(value) => value.tokenize(),
            OptionKind::FollowLocationTrusted(value) => value.tokenize(),
            OptionKind::Header(value) => value.tokenize(),
            OptionKind::Http10(value) => value.tokenize(),
            OptionKind::Http11(value) => value.tokenize(),
            OptionKind::Http2(value) => value.tokenize(),
            OptionKind::Http3(value) => value.tokenize(),
            OptionKind::Insecure(value) => value.tokenize(),
            OptionKind::IpV4(value) => value.tokenize(),
            OptionKind::IpV6(value) => value.tokenize(),
            OptionKind::LimitRate(value) => value.tokenize(),
            OptionKind::MaxRedirect(value) => value.tokenize(),
            OptionKind::NetRc(value) => value.tokenize(),
            OptionKind::NetRcFile(filename) => filename.tokenize(),
            OptionKind::NetRcOptional(value) => value.tokenize(),
            OptionKind::Output(filename) => filename.tokenize(),
            OptionKind::PathAsIs(value) => value.tokenize(),
            OptionKind::Proxy(value) => value.tokenize(),
            OptionKind::Repeat(value) => value.tokenize(),
            OptionKind::Resolve(value) => value.tokenize(),
            OptionKind::Retry(value) => value.tokenize(),
            OptionKind::RetryInterval(value) => value.tokenize(),
            OptionKind::Skip(value) => value.tokenize(),
            OptionKind::UnixSocket(value) => value.tokenize(),
            OptionKind::User(value) => value.tokenize(),
            OptionKind::Variable(value) => value.tokenize(),
            OptionKind::Verbose(value) => value.tokenize(),
            OptionKind::VeryVerbose(value) => value.tokenize(),
        }
    }
}

impl Tokenizable for BooleanOption {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            BooleanOption::Literal(value) => vec![Token::Boolean(value.to_string())],
            BooleanOption::Placeholder(expr) => expr.tokenize(),
        }
    }
}

impl Tokenizable for NaturalOption {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            NaturalOption::Literal(value) => value.tokenize(),
            NaturalOption::Placeholder(expr) => expr.tokenize(),
        }
    }
}

impl Tokenizable for U64 {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::Number(self.to_string())]
    }
}

impl Tokenizable for I64 {
    fn tokenize(&self) -> Vec<Token> {
        vec![Token::Number(self.to_string())]
    }
}

impl Tokenizable for CountOption {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            CountOption::Literal(retry) => retry.tokenize(),
            CountOption::Placeholder(expr) => expr.tokenize(),
        }
    }
}

impl Tokenizable for Count {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            Count::Finite(n) => vec![Token::Number(n.to_string())],
            Count::Infinite => vec![Token::Number("-1".to_string())],
        }
    }
}

impl Tokenizable for DurationOption {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            DurationOption::Literal(value) => value.tokenize(),
            DurationOption::Placeholder(expr) => expr.tokenize(),
        }
    }
}

impl Tokenizable for Duration {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = vec![Token::Number(self.value.to_string())];
        if let Some(unit) = self.unit {
            tokens.push(Token::Unit(unit.to_string()));
        }
        tokens
    }
}

impl Tokenizable for VariableDefinition {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![Token::String(self.name.clone())];
        tokens.append(&mut self.space0.tokenize());
        tokens.push(Token::Keyword("=".to_string()));
        tokens.append(&mut self.space1.tokenize());
        tokens.append(&mut self.value.tokenize());
        tokens
    }
}

impl Tokenizable for VariableValue {
    fn tokenize(&self) -> Vec<Token> {
        match self {
            VariableValue::Null => vec![Token::Keyword("null".to_string())],
            VariableValue::Bool(v) => vec![Token::Boolean(v.to_string())],
            VariableValue::Number(v) => vec![Token::Number(v.to_string())],
            VariableValue::String(v) => v.tokenize(),
        }
    }
}

impl Tokenizable for Filter {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = vec![Token::FilterType(self.value.identifier().to_string())];
        match &self.value {
            FilterValue::Decode { space0, encoding } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut encoding.tokenize());
            }
            FilterValue::Format { space0, fmt } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut fmt.tokenize());
            }
            FilterValue::JsonPath { space0, expr } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut expr.tokenize());
            }
            FilterValue::Nth { space0, n } => {
                tokens.append(&mut space0.tokenize());
                tokens.push(Token::Number(n.to_string()));
            }
            FilterValue::Regex { space0, value } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut value.tokenize());
            }
            FilterValue::Replace {
                space0,
                old_value,
                space1,
                new_value,
            } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut old_value.tokenize());
                tokens.append(&mut space1.tokenize());
                tokens.append(&mut new_value.tokenize());
            }
            FilterValue::Split { space0, sep } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut sep.tokenize());
            }
            FilterValue::ToDate { space0, fmt } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut fmt.tokenize());
            }
            FilterValue::XPath { space0, expr } => {
                tokens.append(&mut space0.tokenize());
                tokens.append(&mut expr.tokenize());
            }
            _ => {}
        }
        tokens
    }
}
