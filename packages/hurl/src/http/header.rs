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
use core::fmt;
use std::slice::Iter;

/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Encoding>
pub const ACCEPT_ENCODING: &str = "Accept-Encoding";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization>
pub const AUTHORIZATION: &str = "Authorization";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie>
pub const COOKIE: &str = "Cookie";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding>
pub const CONTENT_ENCODING: &str = "Content-Encoding";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type>
pub const CONTENT_TYPE: &str = "Content-Type";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expect>
pub const EXPECT: &str = "Expect";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location>
pub const LOCATION: &str = "Location";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie>
pub const SET_COOKIE: &str = "Set-Cookie";
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent>
pub const USER_AGENT: &str = "User-Agent";

/// Represents an HTTP header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl Header {
    /// Creates an HTTP header with this `name`and `value`.
    pub fn new(name: &str, value: &str) -> Self {
        Header {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// Returns `true` if this HTTP header name is equal to `name`.
    ///
    /// An HTTP header consists of a case-insensitive name.
    pub fn name_eq(&self, name: &str) -> bool {
        self.name.to_lowercase() == name.to_lowercase()
    }
}

/// Represents an ordered list of [`Header`].
/// The headers are sorted by insertion order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HeaderVec {
    headers: Vec<Header>,
}

impl HeaderVec {
    /// Creates an empty [`HeaderVec`].
    pub fn new() -> Self {
        HeaderVec::default()
    }

    /// Returns a reference to the header associated with `name`.
    ///
    /// If there are multiple headers associated with `name`, then the first one is returned.
    /// Use [HeaderVec::get_all] to get all values associated with a given key.
    pub fn get(&self, name: &str) -> Option<&Header> {
        self.headers.iter().find(|h| h.name_eq(name))
    }

    /// Returns a list of header associated with `name`.
    pub fn get_all(&self, name: &str) -> Vec<&Header> {
        self.headers.iter().filter(|h| h.name_eq(name)).collect()
    }

    /// Returns true if there is at least one header with the specified `name`.
    pub fn contains_key(&self, name: &str) -> bool {
        self.headers.iter().any(|h| h.name_eq(name))
    }

    /// Retains only the header specified by the predicate.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&Header) -> bool,
    {
        self.headers.retain(|h| f(h));
    }

    /// Returns an iterator over all the headers.
    pub fn iter(&self) -> impl Iterator<Item = &Header> {
        self.headers.iter()
    }

    /// Returns the number of headers stored in the list.
    ///
    /// This number represents the total numbers of header, including header with the same name and
    /// different values.
    pub fn len(&self) -> usize {
        self.headers.len()
    }

    /// Returns true if there is no header.
    pub fn is_empty(&self) -> bool {
        self.headers.len() == 0
    }

    /// Push a new `header` into the headers list.
    pub fn push(&mut self, header: Header) {
        self.headers.push(header);
    }

    /// Returns all headers values.
    pub fn values(&self, name: &str) -> Vec<&str> {
        self.get_all(name)
            .iter()
            .map(|h| h.value.as_str())
            .collect::<Vec<_>>()
    }
}

impl<'a> IntoIterator for &'a HeaderVec {
    type Item = &'a Header;
    type IntoIter = Iter<'a, Header>;

    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::header::HeaderVec;
    use crate::http::Header;

    #[test]
    fn test_simple_header_map() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("foo", "xxx"));
        headers.push(Header::new("bar", "yyy0"));
        headers.push(Header::new("bar", "yyy1"));
        headers.push(Header::new("bar", "yyy2"));
        headers.push(Header::new("baz", "zzz"));

        assert_eq!(headers.len(), 5);
        assert!(!headers.is_empty());

        assert_eq!(headers.get("foo"), Some(&Header::new("foo", "xxx")));
        assert_eq!(headers.get("FOO"), Some(&Header::new("foo", "xxx")));
        assert_eq!(headers.get("bar"), Some(&Header::new("bar", "yyy0")));
        assert_eq!(headers.get("qux"), None);

        assert_eq!(
            headers.get_all("bar"),
            vec![
                &Header::new("bar", "yyy0"),
                &Header::new("bar", "yyy1"),
                &Header::new("bar", "yyy2"),
            ]
        );
        assert_eq!(headers.get_all("BAZ"), vec![&Header::new("baz", "zzz")]);
        assert_eq!(headers.get_all("qux"), Vec::<&Header>::new());

        assert!(headers.contains_key("FOO"));
        assert!(!headers.contains_key("fuu"));

        headers.retain(|h| h.name_eq("Bar"));
        assert_eq!(headers.len(), 3);
    }

    #[test]
    fn test_iter() {
        let data = [("foo", "xxx"), ("bar", "yyy0"), ("baz", "yyy1")];
        let mut headers = HeaderVec::new();
        data.iter()
            .for_each(|(name, value)| headers.push(Header::new(name, value)));

        // Test iter()
        for (i, h) in headers.iter().enumerate() {
            assert_eq!(h.name, data[i].0);
            assert_eq!(h.value, data[i].1);
        }

        // Test into_iter()
        for (i, h) in (&headers).into_iter().enumerate() {
            assert_eq!(h.name, data[i].0);
            assert_eq!(h.value, data[i].1);
        }
    }
}
