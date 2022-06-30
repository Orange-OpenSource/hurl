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

/// Returns true if binary data with this `content_type` can be decoded as text.
pub fn is_kind_of_text(content_type: &str) -> bool {
    content_type.contains("text/")
        || content_type.contains("application/json")
        || content_type.contains("application/xml")
        || content_type.contains("application/x-www-form-urlencoded")
}

/// Returns true if this `content_type` is HTML.
pub fn is_html(content_type: &str) -> bool {
    content_type.starts_with("text/html")
}

/// Extracts charset from mime-type String
pub fn charset(mime_type: &str) -> Option<String> {
    mime_type
        .find("charset=")
        .map(|index| mime_type[(index + 8)..].to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_charset() {
        assert_eq!(
            charset("text/plain; charset=utf-8"),
            Some("utf-8".to_string())
        );
        assert_eq!(
            charset("text/plain; charset=ISO-8859-1"),
            Some("ISO-8859-1".to_string())
        );
        assert_eq!(charset("text/plain;"), None);
    }
}
