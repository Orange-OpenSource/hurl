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
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use super::header::{COOKIE, HeaderVec};
use super::request_cookie::RequestCookie;
use super::url::Url;

/// Represents a runtime HTTP request.
/// This is a real request, that has been executed by our HTTP client.
/// It's different from `crate::http::RequestSpec` which is the request asked to be executed by our
/// user. For instance, in the request spec, headers implicitly added by curl are not present, while
/// they will be present in the [`Request`] instances.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    /// Absolute URL.
    pub url: Url,
    /// Method.
    pub method: String,
    /// List of HTTP headers.
    pub headers: HeaderVec,
    /// Response body bytes.
    pub body: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum RequestedHttpVersion {
    /// We don't care what HTTP version to use; libcurl will choose the best possible one for us.
    #[default]
    Default,
    /// Request using HTTP/1.0 in the request
    Http10,
    /// Request using HTTP/1.1 in the request
    Http11,
    /// Request using HTTP/2 in the request (with HTTP/1.1 upgrade phase)
    Http2,
    /// Request using HTTP/2 without HTTP/1.1 upgrade
    Http2PriorKnowledge,
    /// Setting this value will make libcurl attempt to use HTTP/3 directly to
    /// server given in the URL but fallback to earlier HTTP versions if the HTTP/3
    /// connection establishment fails.
    Http3,
}

/// Transport Layer Security version (see <https://en.wikipedia.org/wiki/Transport_Layer_Security>)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TlsVersion {
    #[default]
    Default,
    Tls10,
    Tls11,
    Tls12,
    Tls13,
}

impl FromStr for TlsVersion {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "1.0" => Ok(TlsVersion::Tls10),
            "1.1" => Ok(TlsVersion::Tls11),
            "1.2" => Ok(TlsVersion::Tls12),
            "1.3" => Ok(TlsVersion::Tls13),
            _ => Err("expecting 1.0, 1.1, 1.2, 1.3"),
        }
    }
}

impl fmt::Display for TlsVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            TlsVersion::Default => "default",
            TlsVersion::Tls10 => "1.0",
            TlsVersion::Tls11 => "1.1",
            TlsVersion::Tls12 => "1.2",
            TlsVersion::Tls13 => "1.3",
        };
        write!(f, "{value}")
    }
}

impl fmt::Display for RequestedHttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            RequestedHttpVersion::Default => "HTTP (default)",
            RequestedHttpVersion::Http10 => "HTTP/1.0",
            RequestedHttpVersion::Http11 => "HTTP/1.1",
            RequestedHttpVersion::Http2 => "HTTP/2",
            RequestedHttpVersion::Http2PriorKnowledge => "HTTP/2 (prior knowledge)",
            RequestedHttpVersion::Http3 => "HTTP/3",
        };
        write!(f, "{value}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum IpResolve {
    /// Default, can use addresses of all IP versions that your system allows.
    #[default]
    Default,
    IpV4,
    IpV6,
}

/// A certificate file with an optional password.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CertFile {
    pub path: PathBuf,
    pub password: Option<String>,
}

impl From<&str> for CertFile {
    /// Parses a certificate file from a "curl" option string `PATH[:PASSWORD]`.
    fn from(value: &str) -> Self {
        CertFile::parse(value, cfg!(windows))
    }
}

impl CertFile {
    /// Parses a certificate file from a "curl" option string `PATH[:PASSWORD]`, following curl rules:
    ///
    /// - in the certificate part, `\:` and `\\` are escape sequences, any other `\` is kept
    ///   literally (so Windows path separators are preserved),
    /// - the first unescaped `:` separates the certificate from the password, the password is
    ///   taken as is,
    /// - if `is_windows` is `true`, a leading drive letter (`C:\` or `C:/`) is not considered as
    ///   a separator.
    fn parse(value: &str, is_windows: bool) -> Self {
        let chars = value.chars().collect::<Vec<_>>();
        let mut cert = String::new();
        let mut i = 0;

        if is_windows
            && chars.len() >= 3
            && chars[0].is_ascii_alphabetic()
            && chars[1] == ':'
            && (chars[2] == '\\' || chars[2] == '/')
        {
            cert.push(chars[0]);
            cert.push(':');
            i = 2;
        }

        while i < chars.len() {
            match chars[i] {
                '\\' if matches!(chars.get(i + 1), Some('\\') | Some(':')) => {
                    cert.push(chars[i + 1]);
                    i += 2;
                }
                ':' => {
                    let password = chars[i + 1..].iter().collect::<String>();
                    return CertFile {
                        path: PathBuf::from(cert),
                        password: Some(password),
                    };
                }
                c => {
                    cert.push(c);
                    i += 1;
                }
            }
        }
        CertFile {
            path: PathBuf::from(cert),
            password: None,
        }
    }
}

impl From<String> for CertFile {
    fn from(value: String) -> Self {
        CertFile::from(value.as_str())
    }
}

/// Do the HTTP client follow redirection, or not?
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum FollowLocation {
    /// No redirection is followed
    #[default]
    No,
    /// Redirection is followed, credentials are passed only to the initial host, or all hosts
    /// depending on the credential forwarding mode.
    Follow(CredentialForwarding),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CredentialForwarding {
    /// Credentials are only forwarded to the initial host
    #[default]
    OnlyInitialHost,
    /// Credentials are forwarded to all hosts involved in the redirection
    AllHosts,
}

impl Request {
    /// Creates a new request.
    pub fn new(method: &str, url: Url, headers: HeaderVec, body: Vec<u8>) -> Self {
        Request {
            url,
            method: method.to_string(),
            headers,
            body,
        }
    }

    /// Returns a list of request headers cookie.
    ///
    /// see <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie>
    pub fn cookies(&self) -> Vec<RequestCookie> {
        self.headers
            .get_all(COOKIE)
            .iter()
            .flat_map(|h| RequestCookie::from_header(h.value.as_str().trim()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Header, RequestCookie};

    fn hello_request() -> Request {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Host", "localhost:8000"));
        headers.push(Header::new("Accept", "*/*"));
        headers.push(Header::new("User-Agent", "hurl/1.0"));
        headers.push(Header::new("content-type", "application/json"));
        let url = "http://localhost:8000/hello".parse().unwrap();

        Request::new("GET", url, headers, vec![])
    }

    fn query_string_request() -> Request {
        let url = "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3".parse().unwrap();

        Request::new("GET", url, HeaderVec::new(), vec![])
    }

    fn cookies_request() -> Request {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Cookie", "cookie1=value1; cookie2=value2"));
        let url = "http://localhost:8000/cookies".parse().unwrap();
        Request::new("GET", url, headers, vec![])
    }

    #[test]
    fn test_content_type() {
        assert_eq!(
            hello_request().headers.content_type(),
            Some("application/json")
        );
        assert_eq!(query_string_request().headers.content_type(), None);
        assert_eq!(cookies_request().headers.content_type(), None);
    }

    #[test]
    fn test_cookies() {
        assert!(hello_request().cookies().is_empty());
        assert_eq!(
            cookies_request().cookies(),
            vec![
                RequestCookie::new("cookie1", "value1"),
                RequestCookie::new("cookie2", "value2"),
            ]
        );
    }

    #[test]
    fn test_tls_version() {
        assert_eq!("1.0".parse(), Ok(TlsVersion::Tls10));
        assert_eq!("1.1".parse(), Ok(TlsVersion::Tls11));
        assert_eq!("1.2".parse(), Ok(TlsVersion::Tls12));
        assert_eq!("1.3".parse(), Ok(TlsVersion::Tls13));
        assert!("1.4".parse::<TlsVersion>().is_err());
        assert_eq!(TlsVersion::Tls12.to_string(), "1.2");
    }

    fn cert(path: &str, password: Option<&str>) -> CertFile {
        CertFile {
            path: PathBuf::from(path),
            password: password.map(|p| p.to_string()),
        }
    }

    #[test]
    fn test_parse_cert() {
        assert_eq!(CertFile::from("foobar"), cert("foobar", None));
        assert_eq!(CertFile::from("foobar:toto"), cert("foobar", Some("toto")));
        assert_eq!(
            CertFile::from("foobar:toto:tata"),
            cert("foobar", Some("toto:tata"))
        );
        assert_eq!(CertFile::from("foobar:"), cert("foobar", Some("")));
        assert_eq!(CertFile::from("foobar\\"), cert("foobar\\", None));
        assert_eq!(
            CertFile::from("foo\\:bar\\:baz:toto:tata\\:tutu"),
            cert("foo:bar:baz", Some("toto:tata\\:tutu"))
        );
        assert_eq!(
            CertFile::from("foo\\\\:toto\\:tata:tutu"),
            cert("foo\\", Some("toto\\:tata:tutu"))
        );
    }

    #[test]
    fn parse_cert_keeps_non_escaping_backslash() {
        for is_windows in [false, true] {
            assert_eq!(
                CertFile::parse("certs\\client.p12", is_windows),
                cert("certs\\client.p12", None)
            );
            assert_eq!(
                CertFile::parse("certs\\client.p12:pwd", is_windows),
                cert("certs\\client.p12", Some("pwd"))
            );
            assert_eq!(
                CertFile::parse("\\\\server\\share\\a.p12:pwd", is_windows),
                cert("\\server\\share\\a.p12", Some("pwd"))
            );
        }
    }

    #[test]
    fn parse_cert_windows_drive_letter() {
        assert_eq!(
            CertFile::parse("C:\\certs\\a.p12", true),
            cert("C:\\certs\\a.p12", None)
        );
        assert_eq!(
            CertFile::parse("C:\\certs\\a.p12:pwd", true),
            cert("C:\\certs\\a.p12", Some("pwd"))
        );
        assert_eq!(
            CertFile::parse("c:/certs/a.p12:pwd", true),
            cert("c:/certs/a.p12", Some("pwd"))
        );
        assert_eq!(
            CertFile::parse("C:\\certs\\a\\:b.p12:pwd:x", true),
            cert("C:\\certs\\a:b.p12", Some("pwd:x"))
        );
        // Drive relative path, not detected as a drive letter, colon must be escaped.
        assert_eq!(
            CertFile::parse("C:a.p12:pwd", true),
            cert("C", Some("a.p12:pwd"))
        );
        assert_eq!(
            CertFile::parse("C\\:a.p12:pwd", true),
            cert("C:a.p12", Some("pwd"))
        );
        // Not a letter.
        assert_eq!(
            CertFile::parse("1:\\a.p12", true),
            cert("1", Some("\\a.p12"))
        );
    }

    #[test]
    fn parse_cert_no_drive_letter_on_unix() {
        assert_eq!(
            CertFile::parse("C:\\certs\\a.p12", false),
            cert("C", Some("\\certs\\a.p12"))
        );
        assert_eq!(
            CertFile::parse("C:/certs/a.p12:pwd", false),
            cert("C", Some("/certs/a.p12:pwd"))
        );
        assert_eq!(
            CertFile::parse("/etc/certs/a.p12:pwd", false),
            cert("/etc/certs/a.p12", Some("pwd"))
        );
    }
}
