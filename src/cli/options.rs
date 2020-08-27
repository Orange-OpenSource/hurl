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
use std::fs;

use crate::http;

use super::Error;

pub fn cookies_output_file(filename: String, n: usize) -> Result<std::path::PathBuf, Error> {
    if n > 1 {
        Err(Error {
            message: "Only save cookies for a unique session".to_string()
        })
    } else {
        let path = std::path::Path::new(&filename);
        Ok(path.to_path_buf())
    }
}

pub fn cookies(filename: &str) -> Result<Vec<http::cookie::Cookie>, Error> {
    let path = std::path::Path::new(filename);
    if !path.exists() {
        return Err(Error {
            message: format!("file {} does not exist", filename)
        });
    }
    let s = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&s)
        .collect();

    let mut cookies = vec![];
    for line in lines {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(cookie) = http::cookie::Cookie::from_netscape(line) {
            cookies.push(cookie);
        } else {
            return Err(Error {
                message: format!("Cookie {} can not be parsed", line)
            });
        };
    }
    Ok(cookies)
}

pub fn output_color(color_present: bool, no_color_present: bool, stdout: bool) -> bool {
    if color_present {
        true
    } else if no_color_present {
        false
    } else {
        stdout
    }
}

pub fn redirect(redirect_present: bool, max_redirect: &str) -> Result<http::client::Redirect, Error> {
    if redirect_present {
        if max_redirect == "-1" {
            Ok(http::client::Redirect::Unlimited)
        } else if let Ok(n) = max_redirect.parse::<usize>() {
            Ok(http::client::Redirect::Limited(n))
        } else {
            Err(Error { message: "Invalid value for option --max-redirs".to_string() })
        }
    } else {
        Ok(http::client::Redirect::None)
    }
}


pub fn validate_proxy(url: String) -> Result<String, Error> {
    // validate proxy value at parsing
    // use code from reqwest for the timebeing
    let url = if url.starts_with("http") {
        url
    } else {
        format!("http://{}", url)
    };
    match reqwest::Proxy::http(url.as_str()) {
        Ok(_) => Ok(url),
        Err(_) => Err(Error { message: format!("Invalid proxy url <{}>", url) })
    }
}

pub fn proxy(option_value: Option<&str>, env_value: Option<String>) -> Result<Option<String>, Error> {
    match option_value {
        Some(url) => if url.is_empty() {
            Ok(None)
        } else {
            let url = validate_proxy(url.to_string())?;
            Ok(Some(url))
        },
        None => match env_value {
            Some(url) => if url.is_empty() {
                Ok(None)
            } else {
                let url = validate_proxy(url)?;
                Ok(Some(url))
            },
            None => Ok(None),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_color() {
        assert_eq!(output_color(true, false, true), true);
        assert_eq!(output_color(false, false, true), true);
    }

    #[test]
    fn test_redirect() {
        assert_eq!(redirect(false, "10").unwrap(), http::client::Redirect::None);
        assert_eq!(redirect(true, "10").unwrap(), http::client::Redirect::Limited(10));
        assert_eq!(redirect(true, "-1").unwrap(), http::client::Redirect::Unlimited);
        assert_eq!(redirect(true, "A").err().unwrap().message, "Invalid value for option --max-redirs");
    }

    #[test]
    fn test_http_proxy() {
        assert_eq!(proxy(None, None).unwrap(), None);
        assert_eq!(proxy(Some("http://localhost:8001"), None).unwrap(), Some("http://localhost:8001".to_string()));
        assert_eq!(proxy(Some("http://localhost:8001"), Some("http://localhost:8002".to_string())).unwrap(), Some("http://localhost:8001".to_string()));
        assert_eq!(proxy(Some(""), Some("http://localhost:8002".to_string())).unwrap(), None);
        assert_eq!(proxy(None, Some("http://localhost:8002".to_string())).unwrap(), Some("http://localhost:8002".to_string()));
    }
}
