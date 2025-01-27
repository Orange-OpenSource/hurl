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

pub trait Redact {
    /// Redacts this given a list of secrets.
    fn redact(&self, secrets: &[impl AsRef<str>]) -> String;
}

impl<T> Redact for T
where
    T: AsRef<str> + ToString,
{
    fn redact(&self, secrets: &[impl AsRef<str>]) -> String {
        let mut value = self.to_string();
        for s in secrets {
            value = value.replace(s.as_ref(), "***");
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::util::redacted::Redact;

    #[test]
    fn redacted_string_hides_secret() {
        // Inner function to trigger deref from &RedactedString to &str.
        fn assert_eq(left: &str, right: &str) {
            assert_eq!(left, right);
        }
        let secrets = ["foo", "bar", "baz"];
        assert_eq(
            &"Hello, here are secrets values: foo".redact(&secrets),
            "Hello, here are secrets values: ***",
        );
        assert_eq(&"bar".redact(&secrets), "***");
        assert_eq(&"Baz is not secret".redact(&secrets), "Baz is not secret");
    }
}
