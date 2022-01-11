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
use super::ast::*;

impl Html {
    pub fn render(self) -> String {
        format!(
            "<!DOCTYPE html>\n<html>{}{}</html>",
            self.head.render(),
            self.body.render()
        )
    }
}

impl Head {
    fn render(self) -> String {
        let mut s = "".to_string();
        s.push_str(format!("<title>{}</title>", self.title).as_str());
        if let Some(filename) = self.stylesheet {
            s.push_str(
                format!(
                    "<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">",
                    filename
                )
                .as_str(),
            );
        }
        format!("<head>{}</head>", s)
    }
}

impl Body {
    fn render(self) -> String {
        let children: Vec<String> = self.children.iter().map(|e| e.clone().render()).collect();
        format!("<body>{}</body>", children.join(""))
    }
}

impl Element {
    fn render(self) -> String {
        match self {
            Element::NodeElement {
                name,
                children,
                attributes,
            } => {
                let attributes = if attributes.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        " {}",
                        attributes
                            .iter()
                            .map(|a| a.clone().render())
                            .collect::<Vec<String>>()
                            .join(" ")
                    )
                };
                let children: Vec<String> = children.iter().map(|e| e.clone().render()).collect();
                format!("<{}{}>{}</{}>", name, attributes, children.join(""), name)
            }
            Element::TextElement(s) => s,
        }
    }
}

impl Attribute {
    fn render(self) -> String {
        match self {
            Attribute::Class(s) => format!("class=\"{}\"", s),
            //Attribute::Id(s) => format!("id=\"{}\"", s),
            Attribute::Href(s) => format!("href=\"{}\"", s),
            Attribute::Data(name, value) => format!("data-{}=\"{}\"", name, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn sample_html() -> Html {
        Html {
            head: Head {
                title: "This is a title".to_string(),
                stylesheet: None,
            },
            body: Body {
                children: vec![Element::NodeElement {
                    name: "p".to_string(),
                    attributes: vec![],
                    children: vec![Element::TextElement("Hello world!".to_string())],
                }],
            },
        }
    }

    #[test]
    fn test_render_html() {
        assert_eq!(sample_html().render(), "<!DOCTYPE html>\n<html><head><title>This is a title</title></head><body><p>Hello world!</p></body></html>");
    }

    pub fn sample_div() -> Element {
        Element::NodeElement {
            name: "div".to_string(),
            attributes: vec![Attribute::Class("request".to_string())],
            children: vec![],
        }
    }

    #[test]
    fn test_render_div() {
        assert_eq!(
            sample_div().render(),
            "<div class=\"request\"></div>".to_string()
        );
    }
}
