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
use std::fmt;
use std::slice::Iter;

/// Represents a SVG element. This list is __partial__, and contains only
/// elements necessary to Hurl waterfall export.
/// See <https://developer.mozilla.org/en-US/docs/Web/SVG/Element>.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ElementKind {
    A,            // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/a
    Defs,         // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/defs
    FeDropShadow, // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow
    Filter,       // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/filter
    Group,        // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g
    Line,         // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/line
    Path,         // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path
    Rect,         // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/rect
    Style,        // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/style
    Svg,          // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg
    Symbol,       // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/symbol
    Text,         // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text
    Use,          // https://developer.mozilla.org/en-US/docs/Web/SVG/Element/use
}

impl ElementKind {
    /// Returns the XML tag name of this SVG element.
    pub fn name(&self) -> &'static str {
        match self {
            ElementKind::A => "a",
            ElementKind::Defs => "defs",
            ElementKind::Filter => "filter",
            ElementKind::FeDropShadow => "feDropShadow",
            ElementKind::Group => "g",
            ElementKind::Line => "line",
            ElementKind::Path => "path",
            ElementKind::Rect => "rect",
            ElementKind::Style => "style",
            ElementKind::Svg => "svg",
            ElementKind::Symbol => "symbol",
            ElementKind::Text => "text",
            ElementKind::Use => "use",
        }
    }
}

/// Represents a SVG element of `kind` type.
/// SVG elements can have attributes (a list of [`Attribute`]), and children (a list of [`Element`]).
/// Optionally, an SVG element can have `content`.
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    kind: ElementKind,
    attrs: Vec<Attribute>,
    children: Vec<Element>,
    content: Option<String>,
}

impl Element {
    /// Returns a new SVG element of type `kind`.
    pub fn new(kind: ElementKind) -> Element {
        Element {
            kind,
            attrs: vec![],
            children: vec![],
            content: None,
        }
    }

    /// Adds an attribute `attr` to this element.
    pub fn add_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }

    /// Returns an iterator over these element's attributes.
    pub fn attrs(&self) -> Iter<'_, Attribute> {
        self.attrs.iter()
    }

    /// Adds a `child` to this element.
    pub fn add_child(&mut self, child: Element) {
        self.children.push(child);
    }

    /// Returns an iterator over these element's children.
    pub fn children(&self) -> Iter<'_, Element> {
        self.children.iter()
    }

    /// Returns [true] if this element has any child, [false] otherwise.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns this element's kind.
    pub fn kind(&self) -> ElementKind {
        self.kind
    }

    /// Sets the `content` of this element.
    pub fn set_content(&mut self, content: &str) {
        self.content = Some(content.to_string());
    }

    /// Returns [true] if this element has content, [false] otherwise.
    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    /// Returns the content if this element or an empty string if this element has no content.
    pub fn content(&self) -> &str {
        match &self.content {
            None => "",
            Some(e) => e,
        }
    }

    /// Serializes this element to a SVG string.
    fn to_svg(&self, buffer: &mut String) {
        let name = self.kind().name();

        buffer.push('<');
        buffer.push_str(name);

        if self.kind() == ElementKind::Svg {
            // Attributes specific to svg
            push_attr(buffer, "xmlns", "http://www.w3.org/2000/svg");
        }

        for att in self.attrs() {
            buffer.push(' ');
            buffer.push_str(&att.to_string());
        }

        if self.has_children() || self.has_content() {
            buffer.push('>');
            for child in self.children() {
                child.to_svg(buffer);
            }
            buffer.push_str(self.content());
            buffer.push_str("</");
            buffer.push_str(name);
            buffer.push('>');
        } else {
            buffer.push_str(" />");
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();
        self.to_svg(&mut text);
        f.write_str(&text)
    }
}

fn push_attr(f: &mut String, key: &str, value: &str) {
    f.push_str(&format!(" {key}=\"{value}\""));
}

/// SVG elements can be modified using attributes.
/// This list of attributes is __partial__ and only includes attributes necessary for Hurl waterfall
/// export. See <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute>
// TODO: fond a better way to represent unit. For the moment X attribute
// take a float but X could be "10", "10px", "10%".
#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Class(String),      // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/class
    D(String),          // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d
    DX(f64),            // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dx
    DY(f64),            // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dy
    Fill(String),       // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill
    Filter(String),     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filter
    FloodOpacity(f64),  // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-opacity
    FontFamily(String), // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-family
    FontSize(String),   // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size
    FontWeight(String), // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-weight
    Height(String),     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height
    Href(String),       // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/href
    Id(String),         // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/id
    Opacity(f64),       // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/opacity
    StdDeviation(f64),  // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stdDeviation
    Stroke(String),     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke
    StrokeWidth(f64),   // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-width
    TextDecoration(String), // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-decoration
    ViewBox(f64, f64, f64, f64), // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox
    Width(String),               // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width
    X(f64),                      // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x
    X1(f64),                     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x1
    X2(f64),                     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x2
    Y(f64),                      // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/y
    Y1(f64),                     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/y1
    Y2(f64),                     // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/y2
}

impl Attribute {
    fn name(&self) -> &'static str {
        match self {
            Attribute::Class(_) => "class",
            Attribute::D(_) => "d",
            Attribute::DX(_) => "dx",
            Attribute::DY(_) => "dy",
            Attribute::Fill(_) => "fill",
            Attribute::Filter(_) => "filter",
            Attribute::FloodOpacity(_) => "flood-opacity",
            Attribute::FontFamily(_) => "font-family",
            Attribute::FontSize(_) => "font-size",
            Attribute::FontWeight(_) => "font-weight",
            Attribute::Height(_) => "height",
            Attribute::Href(_) => "href",
            Attribute::Id(_) => "id",
            Attribute::Opacity(_) => "opacity",
            Attribute::StdDeviation(_) => "stdDeviation",
            Attribute::Stroke(_) => "stroke",
            Attribute::StrokeWidth(_) => "stroke-width",
            Attribute::TextDecoration(_) => "text-decoration",
            Attribute::ViewBox(_, _, _, _) => "viewBox",
            Attribute::Width(_) => "width",
            Attribute::X(_) => "x",
            Attribute::X1(_) => "x1",
            Attribute::X2(_) => "x2",
            Attribute::Y(_) => "y",
            Attribute::Y1(_) => "y1",
            Attribute::Y2(_) => "y2",
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Attribute::Class(value) => value.clone(),
            Attribute::D(value) => value.clone(),
            Attribute::DX(value) => value.to_string(),
            Attribute::DY(value) => value.to_string(),
            Attribute::Fill(value) => value.clone(),
            Attribute::Filter(value) => value.clone(),
            Attribute::FloodOpacity(value) => value.to_string(),
            Attribute::FontFamily(value) => value.clone(),
            Attribute::FontSize(value) => value.clone(),
            Attribute::FontWeight(value) => value.clone(),
            Attribute::Height(value) => value.to_string(),
            Attribute::Href(value) => value.to_string(),
            Attribute::Id(value) => value.clone(),
            Attribute::Opacity(value) => value.to_string(),
            Attribute::StdDeviation(value) => value.to_string(),
            Attribute::Stroke(value) => value.to_string(),
            Attribute::StrokeWidth(value) => value.to_string(),
            Attribute::TextDecoration(value) => value.clone(),
            Attribute::ViewBox(min_x, min_y, width, height) => {
                format!("{min_x} {min_y} {width} {height}")
            }
            Attribute::Width(value) => value.to_string(),
            Attribute::X(value) => value.to_string(),
            Attribute::X1(value) => value.to_string(),
            Attribute::X2(value) => value.to_string(),
            Attribute::Y(value) => value.to_string(),
            Attribute::Y1(value) => value.to_string(),
            Attribute::Y2(value) => value.to_string(),
        };
        f.write_str(&format!("{}=\"{}\"", self.name(), value))
    }
}

/// Returns a new `<a>` element.
pub fn new_a(href: &str) -> Element {
    let mut elt = Element::new(ElementKind::A);
    elt.add_attr(Attribute::Href(href.to_string()));
    elt
}

/// Returns a new `<svg>` element.
pub fn new_svg() -> Element {
    Element::new(ElementKind::Svg)
}

/// Returns a new `<g>` element.
pub fn new_group() -> Element {
    Element::new(ElementKind::Group)
}

/// Returns a new `<style>` element.
pub fn new_style(content: &str) -> Element {
    let mut elt = Element::new(ElementKind::Style);
    elt.set_content(content);
    elt
}

/// Returns a new `<text>` element.
pub fn new_text(x: f64, y: f64, content: &str) -> Element {
    let mut elt = Element::new(ElementKind::Text);
    elt.add_attr(Attribute::X(x));
    elt.add_attr(Attribute::Y(y));
    elt.set_content(content);
    elt
}

/// Returns a new `<line>` element.
pub fn new_line(x1: f64, y1: f64, x2: f64, y2: f64) -> Element {
    let mut elt = Element::new(ElementKind::Line);
    elt.add_attr(Attribute::X1(x1));
    elt.add_attr(Attribute::Y1(y1));
    elt.add_attr(Attribute::X2(x2));
    elt.add_attr(Attribute::Y2(y2));
    elt
}

/// Returns a new `<rect>` element.
pub fn new_rect(x: f64, y: f64, width: f64, height: f64, fill: &str) -> Element {
    let mut elt = Element::new(ElementKind::Rect);
    elt.add_attr(Attribute::X(x));
    elt.add_attr(Attribute::Y(y));
    elt.add_attr(Attribute::Width(width.to_string()));
    elt.add_attr(Attribute::Height(height.to_string()));
    elt.add_attr(Attribute::Fill(fill.to_string()));
    elt
}

/// Returns a new `<defs>` element.
pub fn new_defs() -> Element {
    Element::new(ElementKind::Defs)
}

/// Returns a new `<filter>` element.
pub fn new_filter() -> Element {
    Element::new(ElementKind::Filter)
}

/// Returns a new `<feDropShadow>` element.
pub fn new_fe_drop_shadow() -> Element {
    Element::new(ElementKind::FeDropShadow)
}

/// Returns a new `<symbol>` element.
pub fn new_symbol() -> Element {
    Element::new(ElementKind::Symbol)
}

/// Returns a new `<path>` element.
pub fn new_path(d: &str) -> Element {
    let mut elt = Element::new(ElementKind::Path);
    elt.add_attr(Attribute::D(d.to_string()));
    elt
}

/// Returns a new `<use>` element.
pub fn new_use() -> Element {
    Element::new(ElementKind::Use)
}

#[cfg(test)]
mod tests {
    use super::Attribute::*;
    use super::*;

    #[test]
    fn simple_line_svg() {
        let mut elt = new_line(0.0, 80.0, 100.0, 20.0);
        elt.add_attr(Stroke("black".to_string()));
        assert_eq!(
            elt.to_string(),
            r#"<line x1="0" y1="80" x2="100" y2="20" stroke="black" />"#
        );
    }

    #[test]
    fn group_svg() {
        let mut root = new_svg();
        root.add_attr(ViewBox(0.0, 0.0, 100.0, 100.0));

        let mut group = new_group();
        group.add_attr(Fill("white".to_string()));
        group.add_attr(Stroke("green".to_string()));
        group.add_attr(StrokeWidth(5.0));

        let elt = new_rect(0.0, 0.0, 40.0, 60.0, "#fff");
        group.add_child(elt);
        let elt = new_rect(20.0, 10.0, 3.5, 15.0, "red");
        group.add_child(elt);
        root.add_child(group);

        assert_eq!(
            root.to_string(),
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">\
                <g fill=\"white\" stroke=\"green\" stroke-width=\"5\">\
                    <rect x=\"0\" y=\"0\" width=\"40\" height=\"60\" fill=\"#fff\" />\
                    <rect x=\"20\" y=\"10\" width=\"3.5\" height=\"15\" fill=\"red\" />\
                </g>\
            </svg>"
        );
    }
}
