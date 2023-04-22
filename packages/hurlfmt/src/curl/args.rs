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

/// Split a `str` into a vec of String params
pub fn split(s: &str) -> Result<Vec<String>, String> {
    let mut params = vec![];
    let mut parser = Parser::new(s);
    while let Some(param) = parser.param()? {
        params.push(param);
    }
    Ok(params)
}

struct Parser {
    pub buffer: Vec<char>,
    pub index: usize,
}

impl Parser {
    fn new(s: &str) -> Parser {
        let buffer = s.chars().collect();
        let index = 0;
        Parser { buffer, index }
    }

    fn skip_spaces(&mut self) {
        while self.peek() == Some(' ') {
            self.read();
        }
    }

    fn read(&mut self) -> Option<char> {
        match self.buffer.get(self.index) {
            None => None,
            Some(c) => {
                self.index += 1;
                Some(*c)
            }
        }
    }
    fn peek(&mut self) -> Option<char> {
        self.buffer.get(self.index).copied()
    }

    fn end_of_string(&self) -> bool {
        self.index == self.buffer.len()
    }

    fn delimiter(&mut self) -> Option<(char, bool)> {
        if self.peek() == Some('\'') {
            self.read();
            Some(('\'', false))
        } else if self.peek() == Some('$') {
            let save = self.index;
            self.read();
            if self.peek() == Some('\'') {
                self.read();
                Some(('\'', true))
            } else {
                self.index = save;
                None
            }
        } else {
            None
        }
    }

    fn param(&mut self) -> Result<Option<String>, String> {
        self.skip_spaces();
        if self.end_of_string() {
            return Ok(None);
        }
        let mut value = "".to_string();
        if let Some((delimiter, escaping)) = self.delimiter() {
            while let Some(c1) = self.read() {
                if c1 == '\\' && escaping {
                    let c2 = match self.read() {
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some(c) => c,
                        _ => return Err(format!("Invalid escape at column {}", self.index + 1)),
                    };
                    value.push(c2);
                } else if c1 == delimiter {
                    return Ok(Some(value));
                } else {
                    value.push(c1);
                }
            }
            Err(format!(
                "Missing delimiter {delimiter} at column {}",
                self.index + 1
            ))
        } else {
            loop {
                match self.read() {
                    Some('\\') => {
                        if let Some(c) = self.read() {
                            value.push(c);
                        } else {
                            return Err(format!("Invalid escape at column {}", self.index + 1));
                        }
                    }
                    Some(' ') => return Ok(Some(value)),
                    Some(c) => {
                        value.push(c);
                    }
                    _ => return Ok(Some(value)),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::curl::args;
    use crate::curl::args::Parser;

    #[test]
    fn test_split() {
        let expected = vec!["AAA".to_string(), "BBB".to_string()];
        assert_eq!(args::split(r#"AAA BBB"#).unwrap(), expected);
        assert_eq!(args::split(r#"AAA  BBB"#).unwrap(), expected);
        assert_eq!(args::split(r#" AAA BBB "#).unwrap(), expected);
        assert_eq!(args::split(r#"AAA 'BBB'"#).unwrap(), expected);
        assert_eq!(args::split(r#"AAA $'BBB'"#).unwrap(), expected);

        let expected = vec!["'".to_string()];
        assert_eq!(args::split(r#"$'\''"#).unwrap(), expected);
    }

    #[test]
    fn test_split_error() {
        assert_eq!(
            args::split(r#"AAA 'BBB"#).err().unwrap(),
            "Missing delimiter ' at column 9".to_string()
        );
    }

    #[test]
    fn test_param_without_quote() {
        let mut parser = Parser::new("value");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.index, 5);

        let mut parser = Parser::new(" value  ");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.index, 7);
    }

    #[test]
    fn test_param_with_quote() {
        let mut parser = Parser::new("'value'");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.index, 7);

        let mut parser = Parser::new(" 'value'  ");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.index, 8);

        let mut parser = Parser::new("'\\n'");
        assert_eq!(parser.param().unwrap().unwrap(), "\\n".to_string());
        assert_eq!(parser.index, 4);
    }

    #[test]
    fn test_dollar_prefix() {
        let mut parser = Parser::new("$'Test: \\''");
        assert_eq!(parser.param().unwrap().unwrap(), "Test: '".to_string());
        assert_eq!(parser.index, 11);

        let mut parser = Parser::new("$'\\n'");
        assert_eq!(parser.param().unwrap().unwrap(), "\n".to_string());
        assert_eq!(parser.index, 5);
    }

    #[test]
    fn test_param_missing_closing_quote() {
        let mut parser = Parser::new("'value");
        assert_eq!(
            parser.param().err().unwrap(),
            "Missing delimiter ' at column 7".to_string()
        );
        assert_eq!(parser.index, 6);
    }

    #[test]
    fn test_no_more_param() {
        assert_eq!(Parser::new("").param().unwrap(), None);
        assert_eq!(Parser::new(" ").param().unwrap(), None);
    }

    #[test]
    fn test_delimiter() {
        let mut parser = Parser::new("value");
        assert_eq!(parser.delimiter(), None);
        assert_eq!(parser.index, 0);
        let mut parser = Parser::new("'value'");
        assert_eq!(parser.delimiter().unwrap(), ('\'', false));
        assert_eq!(parser.index, 1);
        let mut parser = Parser::new("$'value'");
        assert_eq!(parser.delimiter().unwrap(), ('\'', true));
        assert_eq!(parser.index, 2);
        let mut parser = Parser::new("$value");
        assert_eq!(parser.delimiter(), None);
        assert_eq!(parser.index, 0);
    }
}
