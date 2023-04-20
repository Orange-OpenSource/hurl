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
        .arg(commands::headers())
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
    let s = format(&method, &url, headers);
    Ok(s)
}

fn format(method: &str, url: &str, headers: Vec<String>) -> String {
    let mut s = format!("{method} {url}");
    for header in headers {
        s.push_str(format!("\n{header}").as_str());
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
}
