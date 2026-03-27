/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
//! Hurl core types that can be used in different modules. These types are rather generic like `Count`
//! (a counter) or `Index` (a one-based index) and shared across `hurl`, `hurlfmt` and `hurl_core` crates.
use core::fmt;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::num::NonZero;
use std::ops::AddAssign;
use std::str::FromStr;

/// Represents a count operation, either finite or infinite.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Count {
    Finite(usize),
    Infinite,
}

impl From<i32> for Count {
    fn from(value: i32) -> Self {
        match value {
            -1 => Count::Infinite,
            n => Count::Finite(n as usize),
        }
    }
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

/// Represents a duration unit
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DurationUnit {
    MilliSecond,
    Second,
    Minute,
    Hour,
}

impl fmt::Display for DurationUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DurationUnit::MilliSecond => write!(f, "ms"),
            DurationUnit::Second => write!(f, "s"),
            DurationUnit::Minute => write!(f, "m"),
            DurationUnit::Hour => write!(f, "h"),
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
            "h" => Ok(DurationUnit::Hour),
            x => Err(format!("Invalid duration unit {x}")),
        }
    }
}

/// Represents bit rate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BytesPerSec(pub u64);

impl fmt::Display for BytesPerSec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Borrow<str> for SourceString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Represents a 1-based index.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index {
    value: NonZero<usize>,
}

impl Index {
    /// Creates a new index from 1-based usize index.
    pub fn new(value: usize) -> Index {
        let value = NonZero::new(value).unwrap();
        Index { value }
    }

    /// Creates a new index from 0-based usize index.
    pub fn from_zero_based(value: usize) -> Index {
        Index::new(value + 1)
    }

    pub fn get(&self) -> usize {
        self.value.get()
    }

    pub fn to_zero_based(&self) -> usize {
        self.value.get() - 1
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AddAssign<usize> for Index {
    fn add_assign(&mut self, other: usize) {
        // Adding a usize to a NonZero is always a NonZero.
        self.value = NonZero::new(self.value.get() + other).unwrap();
    }
}

impl PartialEq<usize> for Index {
    fn eq(&self, other: &usize) -> bool {
        self.value.get().eq(other)
    }
}

impl PartialOrd<usize> for Index {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.value.get().partial_cmp(other)
    }
}
