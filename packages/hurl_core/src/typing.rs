/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

/// Represents a count operation, either finite or infinite.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Count {
    Finite(usize),
    Infinite,
}

/// Represent a duration
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Duration {
    pub value: u64,
    pub unit: Option<DurationUnit>,
}

impl Duration {
    pub fn new(value: u64, unit: Option<DurationUnit>) -> Duration {
        Duration { value, unit }
    }
}

/// Represents a duration unit
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DurationUnit {
    MilliSecond,
    Second,
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
        }
    }
}
