/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

mod args;
mod commands;
mod matches;

pub fn parse(s: &str) -> Result<String, String> {
    let mut command = clap::Command::new("curl")
        .arg(commands::compressed())
        .arg(commands::data())
        .arg(commands::headers())
        .arg(commands::insecure())
        .arg(commands::location())
        .arg(commands::max_redirects())
        .arg(commands::method())
        .arg(commands::url());

    let params = args::split(s)?;
    let arg_matches = match command.try_get_matches_from_mut(params) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let method = matches::method(&arg_matches);
    let url = matches::url(&arg_matches);
    let headers = matches::headers(&arg_matches);
    let options = matches::options(&arg_matches);
    let body = matches::body(&arg_matches);
    let s = format(&method, &url, headers, options, body);
    Ok(s)
}

fn format(
    method: &str,
    url: &str,
    headers: Vec<String>,
    options: Vec<String>,
    body: Option<String>,
) -> String {
    let mut s = format!("{method} {url}");
    for header in headers {
        s.push_str(format!("\n{header}").as_str());
    }
    if !options.is_empty() {
        s.push_str("\n[Options]");
        for option in options {
            s.push_str(format!("\n{option}").as_str());
        }
    }
    if let Some(body) = body {
        s.push_str(format!("\n{body}").as_str());
    }
    s.push('\n');
    s
}

#[cfg(test)]
mod test {
    use crate::curl::parse;

    #[test]
    fn test_hello() {
        let hurl_str = r#"GET http://locahost:8000/hello
"#;
        assert_eq!(parse("curl http://locahost:8000/hello").unwrap(), hurl_str);
    }

    #[test]
    fn test_headers() {
        let hurl_str = r#"GET http://localhost:8000/custom-headers
Fruit:Raspberry
Fruit: Banana
Test: '
"#;
        assert_eq!(
            parse("curl http://localhost:8000/custom-headers -H 'Fruit:Raspberry' -H 'Fruit: Banana' -H $'Test: \\''").unwrap(),
            hurl_str
        );
        assert_eq!(
            parse("curl http://localhost:8000/custom-headers   --header Fruit:Raspberry -H 'Fruit: Banana' -H $'Test: \\''  ").unwrap(),
            hurl_str
        );
    }

    #[test]
    fn test_post_format_params() {
        let hurl_str = r#"POST http://localhost:3000/data
Content-Type: application/x-www-form-urlencoded
```param1=value1&param2=value2```
"#;
        assert_eq!(
            parse("curl http://localhost:3000/data -d 'param1=value1&param2=value2'").unwrap(),
            hurl_str
        );
        assert_eq!(
            parse("curl -X POST http://localhost:3000/data -H 'Content-Type: application/x-www-form-urlencoded' --data 'param1=value1&param2=value2'").unwrap(),
            hurl_str
        );
    }

    #[test]
    fn test_post_json() {
        let hurl_str = r#"POST http://localhost:3000/data
Content-Type: application/json
```{"key1":"value1", "key2":"value2"}```
"#;
        assert_eq!(
            parse(r#"curl -d '{"key1":"value1", "key2":"value2"}' -H 'Content-Type: application/json' -X POST http://localhost:3000/data"#).unwrap(),
            hurl_str
        );
    }

    #[test]
    fn test_post_file() {
        let hurl_str = r#"POST http://example.com/
file, filename;
"#;
        assert_eq!(
            parse(r#"curl --data @filename http://example.com/"#).unwrap(),
            hurl_str
        );
    }

    #[test]
    fn test_redirect() {
        let hurl_str = r#"GET http://localhost:8000/redirect-absolute
[Options]
location: true
"#;
        assert_eq!(
            parse(r#"curl -L http://localhost:8000/redirect-absolute"#).unwrap(),
            hurl_str
        );
    }

    #[test]
    fn test_insecure() {
        let hurl_str = r#"GET https://localhost:8001/hello
[Options]
insecure: true
"#;
        assert_eq!(
            parse(r#"curl -k https://localhost:8001/hello"#).unwrap(),
            hurl_str
        );
    }

    #[test]
    fn test_max_redirects() {
        let hurl_str = r#"GET https://localhost:8001/hello
[Options]
max-redirs: 10
"#;
        assert_eq!(
            parse(r#"curl https://localhost:8001/hello --max-redirs 10"#).unwrap(),
            hurl_str
        );
    }
}
