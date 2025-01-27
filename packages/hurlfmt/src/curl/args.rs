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

use hurl_core::reader::Reader;

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
    reader: Reader,
}

impl Parser {
    fn new(s: &str) -> Parser {
        let reader = Reader::new(s);
        Parser { reader }
    }

    fn delimiter(&mut self) -> Option<(char, bool)> {
        if self.reader.peek() == Some('\'') {
            _ = self.reader.read();
            Some(('\'', false))
        } else if self.reader.peek() == Some('$') {
            let save = self.reader.cursor();
            _ = self.reader.read();
            if self.reader.peek() == Some('\'') {
                _ = self.reader.read();
                Some(('\'', true))
            } else {
                self.reader.seek(save);
                None
            }
        } else {
            None
        }
    }

    fn col(&self) -> usize {
        self.reader.cursor().pos.column
    }

    fn param(&mut self) -> Result<Option<String>, String> {
        _ = self.reader.read_while(|c| c == ' ');
        if self.reader.is_eof() {
            return Ok(None);
        }
        let mut value = String::new();
        if let Some((delimiter, escaping)) = self.delimiter() {
            while let Some(c1) = self.reader.read() {
                if c1 == '\\' && escaping {
                    let c2 = match self.reader.read() {
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some(c) => c,
                        _ => {
                            let col = self.col();
                            return Err(format!("Invalid escape at column {col}"));
                        }
                    };
                    value.push(c2);
                } else if c1 == delimiter {
                    return Ok(Some(value));
                } else {
                    value.push(c1);
                }
            }
            let col = self.col();
            Err(format!("Missing delimiter {delimiter} at column {col}"))
        } else {
            loop {
                match self.reader.read() {
                    Some('\\') => {
                        if let Some(c) = self.reader.read() {
                            value.push(c);
                        } else {
                            let col = self.col();
                            return Err(format!("Invalid escape at column {col}"));
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
        assert_eq!(args::split(r"$'\''").unwrap(), expected);
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
        assert_eq!(parser.col(), 6);

        let mut parser = Parser::new(" value  ");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.col(), 8);
    }

    #[test]
    fn test_param_with_quote() {
        let mut parser = Parser::new("'value'");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.col(), 8);

        let mut parser = Parser::new(" 'value'  ");
        assert_eq!(parser.param().unwrap().unwrap(), "value".to_string());
        assert_eq!(parser.col(), 9);

        let mut parser = Parser::new("'\\n'");
        assert_eq!(parser.param().unwrap().unwrap(), "\\n".to_string());
        assert_eq!(parser.col(), 5);
    }

    #[test]
    fn test_dollar_prefix() {
        let mut parser = Parser::new("$'Test: \\''");
        assert_eq!(parser.param().unwrap().unwrap(), "Test: '".to_string());
        assert_eq!(parser.col(), 12);

        let mut parser = Parser::new("$'\\n'");
        assert_eq!(parser.param().unwrap().unwrap(), "\n".to_string());
        assert_eq!(parser.col(), 6);
    }

    #[test]
    fn test_param_missing_closing_quote() {
        let mut parser = Parser::new("'value");
        assert_eq!(
            parser.param().err().unwrap(),
            "Missing delimiter ' at column 7".to_string()
        );
        assert_eq!(parser.col(), 7);
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
        assert_eq!(parser.col(), 1);
        let mut parser = Parser::new("'value'");
        assert_eq!(parser.delimiter().unwrap(), ('\'', false));
        assert_eq!(parser.col(), 2);
        let mut parser = Parser::new("$'value'");
        assert_eq!(parser.delimiter().unwrap(), ('\'', true));
        assert_eq!(parser.col(), 3);
        let mut parser = Parser::new("$value");
        assert_eq!(parser.delimiter(), None);
        assert_eq!(parser.col(), 1);
    }
}
