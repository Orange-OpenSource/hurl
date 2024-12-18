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
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

/// A redacted string.
///
/// This string redacts a list of secrets value.
pub struct RedactedString<'input> {
    value: String,
    secrets: &'input [&'input str],
}

impl<'input> RedactedString<'input> {
    /// Creates a new redacted string given a list of secrets.
    pub fn new(secrets: &'input [&str]) -> Self {
        let value = String::new();
        RedactedString { value, secrets }
    }
}

impl RedactedString<'_> {
    /// Appends a given string slice onto the end of this `RedactedString`.
    pub fn push_str(&mut self, string: &str) {
        let mut value = string.to_string();
        for s in self.secrets {
            value = value.replace(s, "***");
        }
        self.value.push_str(&value);
    }
}

impl fmt::Display for RedactedString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Deref for RedactedString<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        self.value.deref()
    }
}

impl AsRef<str> for RedactedString<'_> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::util::redacted::RedactedString;

    #[test]
    fn redacted_string_hides_secret() {
        // Inner function to trigger deref from &RedactedString to &str.
        fn assert_eq(left: &str, right: &str) {
            assert_eq!(left, right);
        }
        let secrets = ["foo", "bar", "baz"];
        let mut rs = RedactedString::new(&secrets);
        rs.push_str("Hello, here are secrets values: foo, ");
        rs.push_str("bar, ");
        rs.push_str("baz. ");
        rs.push_str("Baz is not secret");

        assert_eq(
            &rs,
            "Hello, here are secrets values: ***, ***, ***. Baz is not secret",
        )
    }
}
