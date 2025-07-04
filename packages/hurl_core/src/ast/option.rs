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
use std::fmt;

use crate::ast::primitive::{
    LineTerminator, Number, Placeholder, SourceInfo, Template, Whitespace, U64,
};
use crate::typing::{Count, Duration, SourceString, ToSource};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryOption {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub kind: OptionKind,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionKind {
    AwsSigV4(Template),
    CaCertificate(Template),
    ClientCert(Template),
    ClientKey(Template),
    Compressed(BooleanOption),
    ConnectTo(Template),
    ConnectTimeout(DurationOption),
    Delay(DurationOption),
    Header(Template),
    Http10(BooleanOption),
    Http11(BooleanOption),
    Http2(BooleanOption),
    Http3(BooleanOption),
    Insecure(BooleanOption),
    IpV4(BooleanOption),
    IpV6(BooleanOption),
    FollowLocation(BooleanOption),
    FollowLocationTrusted(BooleanOption),
    LimitRate(NaturalOption),
    MaxRedirect(CountOption),
    MaxTime(DurationOption),
    NetRc(BooleanOption),
    NetRcFile(Template),
    NetRcOptional(BooleanOption),
    Output(Template),
    PathAsIs(BooleanOption),
    PinnedPublicKey(Template),
    Proxy(Template),
    Repeat(CountOption),
    Resolve(Template),
    Retry(CountOption),
    RetryInterval(DurationOption),
    Skip(BooleanOption),
    UnixSocket(Template),
    User(Template),
    Variable(VariableDefinition),
    Verbose(BooleanOption),
    VeryVerbose(BooleanOption),
}

impl OptionKind {
    /// Returns the Hurl string identifier of this option.
    pub fn identifier(&self) -> &'static str {
        match self {
            OptionKind::AwsSigV4(_) => "aws-sigv4",
            OptionKind::CaCertificate(_) => "cacert",
            OptionKind::ClientCert(_) => "cert",
            OptionKind::ClientKey(_) => "key",
            OptionKind::Compressed(_) => "compressed",
            OptionKind::ConnectTo(_) => "connect-to",
            OptionKind::ConnectTimeout(_) => "connect-timeout",
            OptionKind::Delay(_) => "delay",
            OptionKind::FollowLocation(_) => "location",
            OptionKind::FollowLocationTrusted(_) => "location-trusted",
            OptionKind::Header(_) => "header",
            OptionKind::Http10(_) => "http1.0",
            OptionKind::Http11(_) => "http1.1",
            OptionKind::Http2(_) => "http2",
            OptionKind::Http3(_) => "http3",
            OptionKind::Insecure(_) => "insecure",
            OptionKind::IpV4(_) => "ipv4",
            OptionKind::IpV6(_) => "ipv6",
            OptionKind::LimitRate(_) => "limit-rate",
            OptionKind::MaxRedirect(_) => "max-redirs",
            OptionKind::MaxTime(_) => "max-time",
            OptionKind::NetRc(_) => "netrc",
            OptionKind::NetRcFile(_) => "netrc-file",
            OptionKind::NetRcOptional(_) => "netrc-optional",
            OptionKind::Output(_) => "output",
            OptionKind::PathAsIs(_) => "path-as-is",
            OptionKind::PinnedPublicKey(_) => "pinnedpubkey",
            OptionKind::Proxy(_) => "proxy",
            OptionKind::Repeat(_) => "repeat",
            OptionKind::Resolve(_) => "resolve",
            OptionKind::Retry(_) => "retry",
            OptionKind::RetryInterval(_) => "retry-interval",
            OptionKind::Skip(_) => "skip",
            OptionKind::UnixSocket(_) => "unix-socket",
            OptionKind::User(_) => "user",
            OptionKind::Variable(_) => "variable",
            OptionKind::Verbose(_) => "verbose",
            OptionKind::VeryVerbose(_) => "very-verbose",
        }
    }
}

impl fmt::Display for OptionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            OptionKind::AwsSigV4(value) => value.to_string(),
            OptionKind::CaCertificate(filename) => filename.to_string(),
            OptionKind::ClientCert(filename) => filename.to_string(),
            OptionKind::ClientKey(filename) => filename.to_string(),
            OptionKind::Compressed(value) => value.to_string(),
            OptionKind::ConnectTo(value) => value.to_string(),
            OptionKind::ConnectTimeout(value) => value.to_string(),
            OptionKind::Delay(value) => value.to_string(),
            OptionKind::FollowLocation(value) => value.to_string(),
            OptionKind::FollowLocationTrusted(value) => value.to_string(),
            OptionKind::Header(value) => value.to_string(),
            OptionKind::Http10(value) => value.to_string(),
            OptionKind::Http11(value) => value.to_string(),
            OptionKind::Http2(value) => value.to_string(),
            OptionKind::Http3(value) => value.to_string(),
            OptionKind::Insecure(value) => value.to_string(),
            OptionKind::IpV4(value) => value.to_string(),
            OptionKind::IpV6(value) => value.to_string(),
            OptionKind::LimitRate(value) => value.to_string(),
            OptionKind::MaxRedirect(value) => value.to_string(),
            OptionKind::MaxTime(value) => value.to_string(),
            OptionKind::NetRc(value) => value.to_string(),
            OptionKind::NetRcFile(filename) => filename.to_string(),
            OptionKind::NetRcOptional(value) => value.to_string(),
            OptionKind::Output(filename) => filename.to_string(),
            OptionKind::PathAsIs(value) => value.to_string(),
            OptionKind::PinnedPublicKey(value) => value.to_string(),
            OptionKind::Proxy(value) => value.to_string(),
            OptionKind::Repeat(value) => value.to_string(),
            OptionKind::Resolve(value) => value.to_string(),
            OptionKind::Retry(value) => value.to_string(),
            OptionKind::RetryInterval(value) => value.to_string(),
            OptionKind::Skip(value) => value.to_string(),
            OptionKind::UnixSocket(value) => value.to_string(),
            OptionKind::User(value) => value.to_string(),
            OptionKind::Variable(value) => value.to_string(),
            OptionKind::Verbose(value) => value.to_string(),
            OptionKind::VeryVerbose(value) => value.to_string(),
        };
        write!(f, "{}: {}", self.identifier(), value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanOption {
    Literal(bool),
    Placeholder(Placeholder),
}

impl fmt::Display for BooleanOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanOption::Literal(v) => write!(f, "{v}"),
            BooleanOption::Placeholder(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NaturalOption {
    Literal(U64),
    Placeholder(Placeholder),
}

impl fmt::Display for NaturalOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NaturalOption::Literal(v) => write!(f, "{v}"),
            NaturalOption::Placeholder(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CountOption {
    Literal(Count),
    Placeholder(Placeholder),
}

impl fmt::Display for CountOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountOption::Literal(v) => write!(f, "{v}"),
            CountOption::Placeholder(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurationOption {
    Literal(Duration),
    Placeholder(Placeholder),
}

impl fmt::Display for DurationOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationOption::Literal(v) => write!(f, "{v}"),
            DurationOption::Placeholder(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableDefinition {
    pub source_info: SourceInfo,
    pub name: String,
    pub space0: Whitespace,
    pub space1: Whitespace,
    pub value: VariableValue,
}

impl fmt::Display for VariableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariableValue {
    Null,
    Bool(bool),
    Number(Number),
    String(Template),
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VariableValue::Null => "null".to_string(),
            VariableValue::Bool(value) => value.to_string(),
            VariableValue::Number(n) => n.to_string(),
            VariableValue::String(s) => s.to_string(),
        };
        write!(f, "{s}")
    }
}

impl ToSource for VariableValue {
    fn to_source(&self) -> SourceString {
        match self {
            VariableValue::Null => "null".to_source(),
            VariableValue::Bool(value) => value.to_string().to_source(),
            VariableValue::Number(value) => value.to_source(),
            VariableValue::String(value) => value.to_source(),
        }
    }
}
