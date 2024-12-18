/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::ast::{
    BooleanOption, CookieAttribute, CookieAttributeName, CookiePath, CountOption, DurationOption,
    Expr, ExprKind, Float, Function, Hex, Method, MultilineString, MultilineStringAttribute,
    MultilineStringKind, NaturalOption, Number, Placeholder, PredicateFuncValue, Regex, Status,
    StatusValue, Template, TemplateElement, Variable, VariableDefinition, VariableValue, Version,
    VersionValue,
};
use core::fmt;

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
            VersionValue::Version1 => "HTTP/1.0",
            VersionValue::Version11 => "HTTP/1.1",
            VersionValue::Version2 => "HTTP/2",
            VersionValue::Version3 => "HTTP/3",
            VersionValue::VersionAny => "HTTP",
            VersionValue::VersionAnyLegacy => "HTTP/*",
        };
        write!(f, "{s}")
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
            StatusValue::Specific(v) => write!(f, "{v}"),
        }
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for element in self.elements.iter() {
            buffer.push_str(element.to_string().as_str());
        }
        write!(f, "{buffer}")
    }
}

impl fmt::Display for TemplateElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TemplateElement::String { value, .. } => value.clone(),
            TemplateElement::Placeholder(value) => format!("{{{{{value}}}}}"),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Float(value) => write!(f, "{}", value),
            Number::Integer(value) => write!(f, "{}", value),
            Number::BigInteger(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.encoded)
    }
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprKind::Variable(variable) => write!(f, "{}", variable),
            ExprKind::Function(function) => write!(f, "{}", function),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::NewDate => write!(f, "newDate"),
            Function::NewUuid => write!(f, "newUuid"),
        }
    }
}

impl fmt::Display for CookiePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = self.name.to_string();
        if let Some(attribute) = &self.attribute {
            let s = format!("[{attribute}]");
            buf.push_str(s.as_str());
        }
        write!(f, "{buf}")
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
        write!(f, "{s}")
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

impl fmt::Display for MultilineString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body = match &self.kind {
            MultilineStringKind::Text(text)
            | MultilineStringKind::Json(text)
            | MultilineStringKind::Xml(text) => text.value.to_string(),
            MultilineStringKind::GraphQl(graphql) => {
                let var = match &graphql.variables {
                    None => String::new(),
                    Some(var) => {
                        format!(
                            "variables{}{}{}",
                            var.space.value, var.value, var.whitespace.value
                        )
                    }
                };
                format!("{}{}", graphql.value, var)
            }
        };
        write!(f, "{body}")
    }
}

impl fmt::Display for MultilineStringAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultilineStringAttribute::Escape => write!(f, "escape"),
            MultilineStringAttribute::NoVariable => write!(f, "novariable"),
        }
    }
}

impl fmt::Display for BooleanOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanOption::Literal(v) => write!(f, "{}", v),
            BooleanOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for NaturalOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NaturalOption::Literal(v) => write!(f, "{}", v),
            NaturalOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for CountOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountOption::Literal(v) => write!(f, "{}", v),
            CountOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for DurationOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationOption::Literal(v) => write!(f, "{}", v),
            DurationOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for VariableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VariableValue::Null => "null".to_string(),
            VariableValue::Bool(value) => value.to_string(),
            VariableValue::Number(n) => n.to_string(),
            VariableValue::String(s) => s.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl PredicateFuncValue {
    pub fn name(&self) -> &str {
        match self {
            PredicateFuncValue::Equal { .. } => "==",
            PredicateFuncValue::NotEqual { .. } => "!=",
            PredicateFuncValue::GreaterThan { .. } => ">",
            PredicateFuncValue::GreaterThanOrEqual { .. } => ">=",
            PredicateFuncValue::LessThan { .. } => "<",
            PredicateFuncValue::LessThanOrEqual { .. } => "<=",
            PredicateFuncValue::StartWith { .. } => "startsWith",
            PredicateFuncValue::EndWith { .. } => "endsWith",
            PredicateFuncValue::Contain { .. } => "contains",
            PredicateFuncValue::Include { .. } => "includes",
            PredicateFuncValue::Match { .. } => "matches",
            PredicateFuncValue::IsInteger => "isInteger",
            PredicateFuncValue::IsFloat => "isFloat",
            PredicateFuncValue::IsBoolean => "isBoolean",
            PredicateFuncValue::IsString => "isString",
            PredicateFuncValue::IsCollection => "isCollection",
            PredicateFuncValue::IsDate => "isDate",
            PredicateFuncValue::IsIsoDate => "isIsoDate",
            PredicateFuncValue::Exist => "exists",
            PredicateFuncValue::IsEmpty => "isEmpty",
            PredicateFuncValue::IsNumber => "isNumber",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{CookieAttributeName, SourceInfo, Whitespace};
    use crate::reader::Pos;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn variable_placeholder() -> Placeholder {
        Placeholder {
            space0: whitespace(),
            expr: Expr {
                kind: ExprKind::Variable(Variable {
                    name: "name".to_string(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                }),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace(),
        }
    }

    fn hello_template() -> Template {
        Template {
            delimiter: None,
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    encoded: "Hello ".to_string(),
                },
                TemplateElement::Placeholder(variable_placeholder()),
                TemplateElement::String {
                    value: "!".to_string(),
                    encoded: "!".to_string(),
                },
            ],
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
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
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "LSID".to_string(),
                        encoded: "unused".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
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
