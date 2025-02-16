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
//! Hurl common types.
use core::fmt;
use std::str::FromStr;

use crate::ast::U64;

/// Represents a count operation, either finite or infinite.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Count {
    Finite(usize),
    Infinite,
}

/// Represent a duration
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duration {
    pub value: U64,
    pub unit: Option<DurationUnit>,
}

impl Duration {
    pub fn new(value: U64, unit: Option<DurationUnit>) -> Duration {
        Duration { value, unit }
    }
}

/// Represents a duration unit
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DurationUnit {
    MilliSecond,
    Second,
    Minute,
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Count::Finite(n) => {
                write!(f, "{n}")
            }
            Count::Infinite => {
                write!(f, "-1")
            }
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = if let Some(value) = self.unit {
            value.to_string()
        } else {
            String::new()
        };
        write!(f, "{}{unit}", self.value)
    }
}

impl fmt::Display for DurationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationUnit::MilliSecond => write!(f, "ms"),
            DurationUnit::Second => write!(f, "s"),
            DurationUnit::Minute => write!(f, "m"),
        }
    }
}

impl FromStr for DurationUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ms" => Ok(DurationUnit::MilliSecond),
            "s" => Ok(DurationUnit::Second),
            "m" => Ok(DurationUnit::Minute),
            x => Err(format!("Invalid duration unit {x}")),
        }
    }
}

/// Represents bit rate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BytesPerSec(pub u64);

impl fmt::Display for BytesPerSec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a source string, in Hurl file format.
///
/// For instance, with this Hurl file:
///
/// ```hurl
/// GET https://example.org/api
/// HTTP 200
/// [Asserts]
/// jsonpath "$.slideshow.title" == "A beautiful \u{2708}!"
/// ```
///
/// `"A beautiful \u{2708}!"` is the source string, using a [Hurl Unicode literal](https://hurl.dev/docs/hurl-file.html#special-characters-in-strings).
/// It's string value is `A beautiful ✈!`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceString(String);

impl SourceString {
    /// Creates a new empty [`SourceString`].
    pub fn new() -> Self {
        SourceString(String::new())
    }

    /// Appends a given string slice onto the end of this [`SourceString`].
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string);
    }

    /// Appends the given char to the end of this [`SourceString`].
    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }

    /// Extracts a string slice containing the entire [`SourceString`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns `true` if this [`SourceString`] starts with the char `c`.
    pub fn starts_with(&self, c: char) -> bool {
        self.0.starts_with(c)
    }
}

pub trait ToSource {
    fn to_source(&self) -> SourceString;
}

impl ToSource for String {
    fn to_source(&self) -> SourceString {
        SourceString(self.clone())
    }
}

impl ToSource for &str {
    fn to_source(&self) -> SourceString {
        SourceString(self.to_string())
    }
}

impl fmt::Display for SourceString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
