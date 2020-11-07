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
use super::core::*;
use core::fmt;

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl fmt::Display for VersionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VersionValue::Version1 => "1.0",
            VersionValue::Version11 => "1.1",
            VersionValue::Version2 => "2",
            VersionValue::VersionAny => "*",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl fmt::Display for StatusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusValue::Any => write!(f, "*"),
            StatusValue::Specific(v) => write!(f, "{}", v.to_string()),
        }
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::from("");
        for element in self.elements.clone() {
            buffer.push_str(element.to_string().as_str());
        }
        write!(f, "{}", buffer)
    }
}

impl fmt::Display for TemplateElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TemplateElement::String { value, .. } => value.clone(),
            TemplateElement::Expression(value) => format!("{{{{{}}}}}", value.to_string()),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decimal_str: String = format!("{:018}", self.decimal)
            .chars()
            .take(self.decimal_digits)
            .collect();
        write!(f, "{}.{}", self.int, decimal_str)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    fn variable_expr() -> Expr {
        Expr {
            space0: whitespace(),
            variable: Variable {
                name: "name".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: whitespace(),
        }
    }

    fn hello_template() -> Template {
        Template {
            quotes: false,
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    encoded: "Hello ".to_string(),
                },
                TemplateElement::Expression(variable_expr()),
                TemplateElement::String {
                    value: "!".to_string(),
                    encoded: "!".to_string(),
                },
            ],
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    #[test]
    fn test_float() {
        assert_eq!(
            Float {
                int: 1,
                decimal: 0,
                decimal_digits: 1,
            }
            .to_string(),
            "1.0"
        );
        assert_eq!(
            Float {
                int: 1,
                decimal: 10_000_000_000_000_000,
                decimal_digits: 2,
            }
            .to_string(),
            "1.01"
        );
        assert_eq!(
            Float {
                int: 1,
                decimal: 10_000_000_000_000_000,
                decimal_digits: 3,
            }
            .to_string(),
            "1.010"
        );
        assert_eq!(
            Float {
                int: -1,
                decimal: 333_333_333_333_333_333,
                decimal_digits: 3,
            }
            .to_string(),
            "-1.333"
        );
    }

    #[test]
    fn test_template() {
        assert_eq!(hello_template().to_string(), "Hello {{name}}!");
    }
}
