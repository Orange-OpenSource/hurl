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

/// A trait for transforming a characters sequence to HTML-safe sequence.
pub trait HtmlEscape {
    fn html_escape(&self) -> String;
}

impl<T> HtmlEscape for T
where
    T: AsRef<str> + ToString,
{
    fn html_escape(&self) -> String {
        html_escape(self.as_ref())
    }
}

/// Replaces special characters "&", "<" and ">" to HTML-safe sequences.
///
/// Both double quote (") and single quote (') characters are also translated.
fn html_escape(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    // Every character that must be escaped is ASCII so if we match it, we know we are in a char
    // boundary and never inside a multi-byte UTF-8. We can copy safely slices instead of copying
    // chars to chars.
    let mut last = 0;
    for (i, b) in text.bytes().enumerate() {
        let escaped = match b {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#x27;",
            _ => continue,
        };
        output.push_str(&text[last..i]);
        output.push_str(escaped);
        last = i + 1;
    }
    output.push_str(&text[last..]);
    output
}

#[cfg(test)]
mod tests {
    use super::{HtmlEscape, html_escape};

    #[test]
    fn eval_html_escape() {
        let tests = [
            ("foo", "foo"),
            ("<tag>", "&lt;tag&gt;"),
            ("foo & bar", "foo &amp; bar"),
            (
                "string with double quote: \"baz\"",
                "string with double quote: &quot;baz&quot;",
            ),
            (
                "string with single quote: 'baz'",
                "string with single quote: &#x27;baz&#x27;",
            ),
            ("", ""),
            ("&&", "&amp;&amp;"),
            ("caf\u{e9} <\u{1f600}>", "caf\u{e9} &lt;\u{1f600}&gt;"),
        ];
        for (input, output) in tests.iter() {
            assert_eq!(html_escape(input), output.to_string());
            assert_eq!(input.html_escape(), output.to_string());
        }
    }
}
