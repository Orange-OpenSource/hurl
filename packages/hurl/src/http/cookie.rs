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

///
/// This module defines a HTTP ResponseCookie,
/// namely the cookie returned from the response Set-Cookie header
///
///

///
/// Cookie return from HTTP Response
/// It contains arbitrary attributes.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseCookie {
    pub name: String,
    pub value: String,
    pub attributes: Vec<CookieAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookieAttribute {
    pub name: String,
    pub value: Option<String>,
}

impl ResponseCookie {
    /// Parses value from Set-Cookie header into a `ResponseCookie`.
    ///
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
    pub fn parse(s: String) -> Option<ResponseCookie> {
        if let Some(index) = s.find('=') {
            let (name, remaining) = s.split_at(index);
            let mut tokens: Vec<&str> = remaining[1..].split(';').collect();
            let value = tokens.remove(0);
            let attributes: Vec<CookieAttribute> = tokens
                .iter()
                .filter_map(|&s2| CookieAttribute::parse(s2.to_string()))
                .collect();

            Some(ResponseCookie {
                name: name.to_string(),
                value: value.to_string(),
                attributes,
            })
        } else {
            None
        }
    }

    /// Returns the optional Expires attribute as `String` type.
    pub fn expires(&self) -> Option<String> {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "Expires" {
                return attr.value;
            }
        }
        None
    }

    /// Returns the optional Max-Age attribute as `i64` type.
    ///
    /// If the value is not a valid integer, the attribute is simply ignored
    pub fn max_age(&self) -> Option<i64> {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "Max-Age" {
                if let Some(v) = attr.value {
                    if let Ok(v2) = v.as_str().parse::<i64>() {
                        return Some(v2);
                    }
                }
            }
        }
        None
    }

    /// Returns the optional Domain attribute as `String` type.
    pub fn domain(&self) -> Option<String> {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "Domain" {
                return attr.value;
            }
        }
        None
    }

    /// Returns the optional Path attribute as `String` type.
    pub fn path(&self) -> Option<String> {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "Path" {
                return attr.value;
            }
        }
        None
    }

    /// Return true if the Secure attribute is present.
    pub fn has_secure(&self) -> bool {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "Secure" && attr.value.is_none() {
                return true;
            }
        }
        false
    }

    /// Return true if the HttpOnly attribute is present.
    pub fn has_httponly(&self) -> bool {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "HttpOnly" && attr.value.is_none() {
                return true;
            }
        }
        false
    }

    /// Returns the optional SameSite attribute as `String` type.
    pub fn samesite(&self) -> Option<String> {
        for attr in self.attributes.clone() {
            if attr.name.as_str() == "SameSite" {
                return attr.value;
            }
        }
        None
    }
}

impl CookieAttribute {
    fn parse(s: String) -> Option<CookieAttribute> {
        if s.is_empty() {
            None
        } else {
            let tokens: Vec<&str> = s.split('=').collect();
            Some(CookieAttribute {
                name: tokens.get(0).unwrap().to_string().trim().to_string(),
                value: tokens.get(1).map(|e| e.to_string()),
            })
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_cookie_attribute() {
        assert_eq!(
            CookieAttribute::parse("Expires=Wed, 21 Oct 2015 07:28:00 GMT".to_string()).unwrap(),
            CookieAttribute {
                name: "Expires".to_string(),
                value: Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
            }
        );

        assert_eq!(
            CookieAttribute::parse("HttpOnly".to_string()).unwrap(),
            CookieAttribute {
                name: "HttpOnly".to_string(),
                value: None
            }
        );

        assert_eq!(CookieAttribute::parse("".to_string()), None);
    }

    #[test]
    fn test_session_cookie() {
        let cookie = ResponseCookie {
            name: "sessionId".to_string(),
            value: "38afes7a8".to_string(),
            attributes: vec![],
        };
        assert_eq!(
            ResponseCookie::parse("sessionId=38afes7a8".to_string()).unwrap(),
            cookie
        );
        assert_eq!(cookie.expires(), None);
        assert_eq!(cookie.max_age(), None);
        assert_eq!(cookie.domain(), None);
        assert_eq!(cookie.path(), None);
        assert!(!cookie.has_secure());
        assert!(!cookie.has_httponly());
        assert_eq!(cookie.samesite(), None);
    }

    #[test]
    fn test_permanent_cookie() {
        let cookie = ResponseCookie {
            name: "id".to_string(),
            value: "a3fWa".to_string(),
            attributes: vec![CookieAttribute {
                name: "Expires".to_string(),
                value: Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
            }],
        };
        assert_eq!(
            ResponseCookie::parse("id=a3fWa; Expires=Wed, 21 Oct 2015 07:28:00 GMT".to_string())
                .unwrap(),
            cookie
        );
        assert_eq!(
            cookie.expires(),
            Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
        );
        assert_eq!(cookie.max_age(), None);
        assert_eq!(cookie.domain(), None);
        assert_eq!(cookie.path(), None);
        assert!(!cookie.has_secure());
        assert!(!cookie.has_httponly());
        assert_eq!(cookie.samesite(), None);
    }

    #[test]
    fn test_permanent2_cookie() {
        let cookie = ResponseCookie {
            name: "id".to_string(),
            value: "a3fWa".to_string(),
            attributes: vec![CookieAttribute {
                name: "Max-Age".to_string(),
                value: Some("2592000".to_string()),
            }],
        };
        assert_eq!(
            ResponseCookie::parse("id=a3fWa; Max-Age=2592000".to_string()).unwrap(),
            cookie
        );
        assert_eq!(cookie.expires(), None);
        assert_eq!(cookie.max_age(), Some(2592000));
        assert_eq!(cookie.domain(), None);
        assert_eq!(cookie.path(), None);
        assert!(!cookie.has_secure());
        assert!(!cookie.has_httponly());
        assert_eq!(cookie.samesite(), None);
    }

    #[test]
    fn test_lsid_cookie() {
        let cookie = ResponseCookie {
            name: "LSID".to_string(),
            value: "DQAAAK…Eaem_vYg".to_string(),
            attributes: vec![
                CookieAttribute {
                    name: "Path".to_string(),
                    value: Some("/accounts".to_string()),
                },
                CookieAttribute {
                    name: "Expires".to_string(),
                    value: Some("Wed, 13 Jan 2021 22:23:01 GMT".to_string()),
                },
                CookieAttribute {
                    name: "Secure".to_string(),
                    value: None,
                },
                CookieAttribute {
                    name: "HttpOnly".to_string(),
                    value: None,
                },
            ],
        };
        assert_eq!(
            ResponseCookie::parse("LSID=DQAAAK…Eaem_vYg; Path=/accounts; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly".to_string()).unwrap(),
            cookie
        );
        assert_eq!(
            cookie.expires(),
            Some("Wed, 13 Jan 2021 22:23:01 GMT".to_string())
        );
        assert_eq!(cookie.max_age(), None);
        assert_eq!(cookie.domain(), None);
        assert_eq!(cookie.path(), Some("/accounts".to_string()));
        assert!(cookie.has_secure());
        assert!(cookie.has_httponly());
        assert_eq!(cookie.samesite(), None);
    }

    #[test]
    fn test_hsid_cookie() {
        let cookie = ResponseCookie {
            name: "HSID".to_string(),
            value: "AYQEVn…DKrdst".to_string(),
            attributes: vec![
                CookieAttribute {
                    name: "Domain".to_string(),
                    value: Some(".foo.com".to_string()),
                },
                CookieAttribute {
                    name: "Path".to_string(),
                    value: Some("/".to_string()),
                },
                CookieAttribute {
                    name: "Expires".to_string(),
                    value: Some("Wed, 13 Jan 2021 22:23:01 GMT".to_string()),
                },
                CookieAttribute {
                    name: "HttpOnly".to_string(),
                    value: None,
                },
            ],
        };
        assert_eq!(
            ResponseCookie::parse("HSID=AYQEVn…DKrdst; Domain=.foo.com; Path=/; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly".to_string()).unwrap(),
            cookie
        );
        assert_eq!(
            cookie.expires(),
            Some("Wed, 13 Jan 2021 22:23:01 GMT".to_string())
        );
        assert_eq!(cookie.max_age(), None);
        assert_eq!(cookie.domain(), Some(".foo.com".to_string()));
        assert_eq!(cookie.path(), Some("/".to_string()));
        assert!(!cookie.has_secure());
        assert!(cookie.has_httponly());
        assert_eq!(cookie.samesite(), None);
    }

    #[test]
    fn test_trailing_semicolon() {
        assert_eq!(
            ResponseCookie::parse("xx=yy;".to_string()).unwrap(),
            ResponseCookie {
                name: "xx".to_string(),
                value: "yy".to_string(),
                attributes: vec![]
            }
        );
    }

    #[test]
    fn test_invalid_cookie() {
        assert_eq!(ResponseCookie::parse("xx".to_string()), None);
    }

    #[test]
    fn test_cookie_with_invalid_attributes() {
        let cookie = ResponseCookie {
            name: "id".to_string(),
            value: "a3fWa".to_string(),
            attributes: vec![
                CookieAttribute {
                    name: "Secure".to_string(),
                    value: Some("0".to_string()),
                },
                CookieAttribute {
                    name: "Max-Age".to_string(),
                    value: Some("".to_string()),
                },
            ],
        };
        assert_eq!(
            ResponseCookie::parse("id=a3fWa; Secure=0; Max-Age=".to_string()).unwrap(),
            cookie
        );
        assert_eq!(cookie.expires(), None);
        assert_eq!(cookie.max_age(), None);
        assert_eq!(cookie.domain(), None);
        assert_eq!(cookie.path(), None);
        assert!(!cookie.has_secure());
        assert!(!cookie.has_httponly());
        assert_eq!(cookie.samesite(), None);
    }
}
