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

// TODO: maybe add a proper MimeType enum (see <https://github.com/hyperium/mime>)
// as implementation / api example.

/// Returns true if binary data with this `content_type` can be decoded as text.
pub fn is_kind_of_text(content_type: &str) -> bool {
    let content_types = [
        "text/",
        "application/json",
        "application/problem+json", // See https://datatracker.ietf.org/doc/html/rfc7807
        "application/xml",
        "application/x-www-form-urlencoded",
    ];

    content_types.iter().any(|c| content_type.contains(c))
}

/// Returns true if this `content_type` is HTML.
pub fn is_html(content_type: &str) -> bool {
    content_type.starts_with("text/html")
}

/// Returns true if this `content_type` is HTML.
pub fn is_xml(content_type: &str) -> bool {
    content_type.starts_with("text/xml") || content_type.starts_with("application/xml")
}

/// Returns true if this `content_type` is JSON.
pub fn is_json(content_type: &str) -> bool {
    content_type.starts_with("application/json")
        || content_type.starts_with("application/problem+json")
}

/// Extracts charset from mime-type String
pub fn charset(mime_type: &str) -> Option<String> {
    let parts = mime_type.trim().split(';');
    for part in parts {
        let param = part.trim().split('=').collect::<Vec<_>>();
        if param.len() == 2 && param[0].trim().eq_ignore_ascii_case("charset") {
            return Some(param[1].trim().to_string());
        }
    }
    None
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

        assert_eq!(
            charset("text/plain; CHARSET=ISO-8859-1"),
            Some("ISO-8859-1".to_string())
        );

        assert_eq!(
            charset("text/plain; version=0.0.4; charset=utf-8; escaping=values"),
            Some("utf-8".to_string())
        );
    }
}
