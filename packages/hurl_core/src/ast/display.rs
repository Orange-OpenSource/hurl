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
        write!(f, "{}", self.value)
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
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for StatusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusValue::Any => write!(f, "*"),
            StatusValue::Specific(v) => write!(f, "{}", v),
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
            TemplateElement::Expression(value) => format!("{{{{{}}}}}", value),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.encoded)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable.name)
    }
}

impl fmt::Display for CookiePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = self.name.to_string();
        if let Some(attribute) = self.attribute.clone() {
            let s = format!("[{}]", attribute);
            buf.push_str(s.as_str());
        }
        write!(f, "{}", buf)
    }
}

impl fmt::Display for CookieAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.name {
            CookieAttributeName::MaxAge(_) => "Max-Age",
            CookieAttributeName::Value(_) => "Value",
            CookieAttributeName::Expires(_) => "Expires",
            CookieAttributeName::Domain(_) => "Domain",
            CookieAttributeName::Path(_) => "Path",
            CookieAttributeName::Secure(_) => "Secure",
            CookieAttributeName::HttpOnly(_) => "HttpOnly",
            CookieAttributeName::SameSite(_) => "SameSite",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hex,{}{}{};",
            self.space0.value, self.encoded, self.space1.value
        )
    }
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl PredicateFuncValue {
    pub fn name(&self) -> String {
        match self {
            PredicateFuncValue::Equal { operator, .. } => {
                if *operator {
                    "==".to_string()
                } else {
                    "equals".to_string()
                }
            }
            PredicateFuncValue::NotEqual { operator, .. } => {
                if *operator {
                    "!=".to_string()
                } else {
                    "notEquals".to_string()
                }
            }
            PredicateFuncValue::GreaterThan { operator, .. } => {
                if *operator {
                    ">".to_string()
                } else {
                    "greaterThan".to_string()
                }
            }
            PredicateFuncValue::GreaterThanOrEqual { operator, .. } => {
                if *operator {
                    ">=".to_string()
                } else {
                    "greaterThanOrEquals".to_string()
                }
            }
            PredicateFuncValue::LessThan { operator, .. } => {
                if *operator {
                    "<".to_string()
                } else {
                    "lessThan".to_string()
                }
            }
            PredicateFuncValue::LessThanOrEqual { operator, .. } => {
                if *operator {
                    "<=".to_string()
                } else {
                    "lessThanOrEquals".to_string()
                }
            }
            PredicateFuncValue::CountEqual { .. } => "countEquals".to_string(),
            PredicateFuncValue::StartWith { .. } => "startsWith".to_string(),
            PredicateFuncValue::EndWith { .. } => "endsWith".to_string(),
            PredicateFuncValue::Contain { .. } => "contains".to_string(),
            PredicateFuncValue::Include { .. } => "includes".to_string(),
            PredicateFuncValue::Match { .. } => "matches".to_string(),
            PredicateFuncValue::IsInteger { .. } => "isInteger".to_string(),
            PredicateFuncValue::IsFloat { .. } => "isFloat".to_string(),
            PredicateFuncValue::IsBoolean { .. } => "isBoolean".to_string(),
            PredicateFuncValue::IsString { .. } => "isString".to_string(),
            PredicateFuncValue::IsCollection { .. } => "isCollection".to_string(),
            PredicateFuncValue::Exist { .. } => "exists".to_string(),
        }
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
                value: 1.0,
                encoded: "1.0".to_string()
            }
            .to_string(),
            "1.0"
        );
        assert_eq!(
            Float {
                value: 1.01,
                encoded: "1.01".to_string()
            }
            .to_string(),
            "1.01"
        );
        assert_eq!(
            Float {
                value: 1.01,
                encoded: "1.010".to_string()
            }
            .to_string(),
            "1.010"
        );
        assert_eq!(
            Float {
                value: -1.333,
                encoded: "-1.333".to_string()
            }
            .to_string(),
            "-1.333"
        );
    }

    #[test]
    fn test_template() {
        assert_eq!(hello_template().to_string(), "Hello {{name}}!");
    }

    #[test]
    fn test_cookie_path() {
        assert_eq!(
            CookiePath {
                name: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "LSID".to_string(),
                        encoded: "unused".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                attribute: Some(CookieAttribute {
                    space0: whitespace(),
                    name: CookieAttributeName::MaxAge("Max-Age".to_string()),
                    space1: whitespace(),
                }),
            }
            .to_string(),
            "LSID[Max-Age]".to_string()
        );
    }
}
