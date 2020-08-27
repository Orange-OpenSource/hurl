/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use chrono::NaiveDateTime;
use cookie::Cookie as ExternalCookie;

use super::core::*;

//use std::collections::HashMap;

// cookies
// keep cookies same name different domains
// send the most specific?? send the 2 of them?
// more flexible to keep list of cookies internally

pub type Domain = String;
pub type Name = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseCookie {
    pub name: String,
    pub value: String,
    pub max_age: Option<i64>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub expires: Option<String>,
    pub same_site: Option<String>,

}

pub struct ParseCookieError {}

impl std::str::FromStr for ResponseCookie {
    type Err = ParseCookieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = ExternalCookie::parse(s).unwrap();
        let name = c.name().to_string();
        let value = c.value().to_string();
        let max_age = match c.max_age() {
            None => None,
            Some(d) => Some(d.num_seconds())
        };
        let domain = match c.domain() {
            None => None,
            Some(v) => Some(v.to_string())
        };
        let path = match c.path() {
            None => None,
            Some(v) => Some(v.to_string())
        };
        let secure = match c.secure() {
            None => None,
            Some(value) => Some(value)
        };
        let http_only = match c.http_only() {
            None => None,
            Some(value) => Some(value)
        };
        let expires = match c.expires() {
            None => None,
            Some(time) => Some(time.rfc822().to_string())
        };
        let same_site = match c.same_site() {
            None => None,
            Some(s) => Some(s.to_string())
        };
        Ok(ResponseCookie { name, value, max_age, domain, path, secure, expires, http_only, same_site })
    }
}

impl fmt::Display for ResponseCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_age = match self.clone().max_age {
            None => String::from(""),
            Some(v) => format!("; Max-Age:{}", v)
        };
        let domain = match self.clone().domain {
            None => String::from(""),
            Some(v) => format!("; Domain:{}", v)
        };
        let path = match self.clone().path {
            None => String::from(""),
            Some(v) => format!("; Path:{}", v)
        };
        write!(f, "{}={}{}{}{}",
               self.name,
               self.value,
               max_age,
               domain,
               path)
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}; domain={}; path={}",
               self.name,
               self.value,
               self.domain,
               self.path,
        )
    }
}

impl ResponseCookie {


//    pub fn to_header(&self) -> Header {
//        return Header {
//            name: String::from("Cookie"),
//            value: format!("{}={}", self.name, self.value),
//        };
//        //format!("Cookie: {}", self.to_string());
//    }


    pub fn encode_cookie(header_name: String, header_value: String) -> Header {
        let name = String::from("Cookie");
        let value = format!("{}={};", header_name, header_value);
        Header { name, value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct CookieJar {
    inner: Vec<Cookie>
}

impl CookieJar {
    pub fn init(cookies: Vec<Cookie>) -> CookieJar {
        CookieJar { inner: cookies }
    }

    pub fn cookies(self) -> Vec<Cookie> {
        self.inner
    }


    pub fn get_cookies(self, domain: String, path: String) -> Vec<ResponseCookie> {
        self.inner
            .iter()
            .filter(|c| c.is_usable(domain.clone(), path.clone()))
            .map(|c| ResponseCookie {
                name: c.clone().name,
                value: c.clone().value,
                max_age: None,
                domain: Some(c.domain.clone()),
                path: Some(c.path.clone()),
                secure: Some(c.secure),
                http_only: None,
                expires: None,
                same_site: None,
            })
            .collect()
    }

    pub fn update_cookies(&mut self, default_domain: String, _default_path: String, cookie: ResponseCookie) {
        match cookie.max_age {
            Some(0) => {
                //eprintln!("delete cookie {:?}", cookie);
                self.inner.retain(|c| c.name != cookie.name);
            }
            _ => {

                // replace value if same name+domain
                let domain = match cookie.clone().domain {
                    None => default_domain,
                    Some(d) => d,
                };
                let path = match cookie.clone().path {
                    None => String::from("/"), // do not use default path for the time-beingdefault_path,
                    Some(p) => p,
                };

                // find existing cookie
                for c in self.inner.iter_mut() {
                    if c.name == cookie.name && c.domain == domain {
                        c.value = cookie.value;
                        return;
                    }
                }

                let secure = if let Some(v) = cookie.secure { v } else { false };

                // push new cookie
                self.inner.push(Cookie {
                    name: cookie.clone().name,
                    value: cookie.clone().value,
                    domain,
                    path,
                    subdomains: cookie.domain.is_some(),
                    secure,
                    expires: None,
                });
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub subdomains: bool,
    pub secure: bool,
    pub expires: Option<NaiveDateTime>,
}

impl Cookie {
    fn is_usable(&self, domain: String, path: String) -> bool {

        // domain
        if !is_subdomain(self.clone().domain, domain.clone()) {
            return false;
        }
        if !self.subdomains && domain != self.clone().domain {
            return false;
        }

        // path
        if !is_subpath(self.clone().path, path) {
            return false;
        }
        true
    }
}

fn is_subdomain(domain: String, subdomain: String) -> bool {
    if domain.as_str() == "" {
        return false;
    }

    let mut domain_segments: Vec<&str> = domain.split('.').collect();
    if domain_segments.get(0).unwrap() == &"" {
        domain_segments.remove(0);
    }
    domain_segments.reverse();

    let mut subdomain_segments: Vec<&str> = subdomain.split('.').collect();
    if subdomain_segments.get(0).unwrap() == &"" {
        subdomain_segments.remove(0);
    }
    subdomain_segments.reverse();
    if domain_segments.len() > subdomain_segments.len() {
        return false;
    }

    for i in 0..domain_segments.len() {
        if domain_segments.get(i).unwrap() != subdomain_segments.get(i).unwrap() {
            return false;
        }
    }

    true
}

fn is_subpath(path: String, subpath: String) -> bool {
    if path.as_str() == "" {
        return false;
    }

    let mut path_segments: Vec<&str> = path.split('/').collect();
    if path_segments.get(0).unwrap() == &"" {
        path_segments.remove(0);
    }
    path_segments.reverse();
    if path_segments.get(0).unwrap() == &"" {
        path_segments.remove(0);
    }

    let mut subpath_segments: Vec<&str> = subpath.split('/').collect();
    if subpath_segments.get(0).unwrap() == &"" {
        subpath_segments.remove(0);
    }
    subpath_segments.reverse();
    if path_segments.len() > subpath_segments.len() {
        return false;
    }


    for i in 0..path_segments.len() {
        if path_segments.get(i).unwrap() != subpath_segments.get(i).unwrap() {
            return false;
        }
    }

    true
}

impl Cookie {
    pub fn to_netscape(&self) -> String {
        let domain_name = self.domain.to_string();
        let include_domains = if self.subdomains { "TRUE" } else { "FALSE" }.to_string();
        let path = self.path.clone();
        let https_only = if self.secure { "TRUE" } else { "FALSE" }.to_string();
        let expires = if let Some(expires) = self.expires {
            expires.timestamp().to_string()
        } else {
            "0".to_string()
        };
        let name = self.name.clone();
        let value = self.value.clone();
        format!("{}\t{}\t{}\t{}\t{}\t{}\t{}",
                domain_name,
                include_domains,
                path,
                https_only,
                expires,
                name,
                value
        )
    }

    pub fn from_netscape(s: &str) -> Option<Cookie> {
        let tokens = s.split('\t').collect::<Vec<&str>>();
        if tokens.len() != 7 { return None; }

        let domain = (*tokens.get(0).unwrap()).to_string();
        let subdomains = (*tokens.get(1).unwrap()).to_string().as_str() == "TRUE";
        let path = (*tokens.get(2).unwrap()).to_string();
        let secure = (*tokens.get(3).unwrap()).to_string().as_str() == "TRUE";
        let expires = None;
        let name = (*tokens.get(5).unwrap()).to_string();
        let value = (*tokens.get(6).unwrap()).to_string();

        Some(Cookie { name, value, domain, path, subdomains, secure, expires })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn cookie_lsid() -> Cookie {
        Cookie {
            name: String::from("LSID"),
            value: String::from("DQAAAK…Eaem_vYg"),
            domain: String::from("docs.foo.com"),
            path: String::from("/accounts"),
            subdomains: false,
            secure: false,
            expires: None,
        }
    }

    fn cookie_hsid() -> Cookie {
        Cookie {
            name: String::from("HSID"),
            value: String::from("AYQEVn…DKrdst"),
            domain: String::from(".foo.com"),
            path: String::from("/"),
            subdomains: true,
            secure: false,
            expires: None,
        }
    }

    fn cookie_ssid() -> Cookie {
        Cookie {
            name: String::from("SSID"),
            value: String::from("Ap4P…GTEq"),
            domain: String::from("foo.com"),
            path: String::from("/"),
            subdomains: true,
            secure: false,
            expires: None,
        }
    }

    fn sample_cookiejar() -> CookieJar {
        CookieJar {
            inner: vec![
                cookie_lsid(),
                cookie_hsid(),
                cookie_ssid(),
            ]
        }
    }

    #[test]
    fn test_is_usable() {
        let domain = String::from("example.org");
        let path = String::from("/");
        assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), false);
        assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), false);
        assert_eq!(cookie_ssid().is_usable(domain, path.clone()), false);

        let domain = String::from("foo.com");
        let path = String::from("/");
        assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), false);
        assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), true);
        assert_eq!(cookie_ssid().is_usable(domain, path.clone()), true);

        let domain = String::from("foo.com");
        let path = String::from("/accounts");
        assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), false);
        assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), true);
        assert_eq!(cookie_ssid().is_usable(domain, path), true);

        let domain = String::from("docs.foo.com");
        let path = String::from("/accounts");
        assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), true);
        assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), true);
        assert_eq!(cookie_ssid().is_usable(domain, path), true);
    }

    #[test]
    fn test_get_cookies() {
        let domain = String::from("docs.foo.com");
        let path = String::from("/accounts");
        assert_eq!(sample_cookiejar().get_cookies(domain, path).len(), 3);

        let domain = String::from("toto.docs.foo.com");
        let path = String::from("/accounts");
        assert_eq!(sample_cookiejar().get_cookies(domain, path).len(), 2);
    }

    #[test]
    fn test_is_subdomain() {
        assert_eq!(is_subdomain(String::from("foo.example.org"), String::from("example.org")), false);
        assert_eq!(is_subdomain(String::from("example.org"), String::from("toto.org")), false);

        assert_eq!(is_subdomain(String::from("example.org"), String::from("example.org")), true);
        assert_eq!(is_subdomain(String::from("example.org"), String::from("foo.example.org")), true);
        assert_eq!(is_subdomain(String::from(".example.org"), String::from("foo.example.org")), true);
    }

    #[test]
    fn test_is_subpath() {
        assert_eq!(is_subpath(String::from("/toto"), String::from("/toto")), true);
        assert_eq!(is_subpath(String::from("/"), String::from("/toto")), true);
        assert_eq!(is_subpath(String::from("/to"), String::from("/toto")), false);
    }

    #[test]
    fn test_from_netscape() {
        assert_eq!(Cookie::from_netscape("localhost\tFALSE\t/\tFALSE\t0\tcookie2\tvalueA").unwrap(),
                   Cookie {
                       name: "cookie2".to_string(),
                       value: "valueA".to_string(),
                       domain: "localhost".to_string(),
                       path: "/".to_string(),
                       subdomains: false,
                       secure: false,
                       expires: None,
                   }
        );
    }
}
