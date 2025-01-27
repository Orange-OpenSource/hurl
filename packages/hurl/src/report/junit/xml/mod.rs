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
mod reader;
mod writer;

/// An XML element document.
///
/// This struct provides support for serialization to and from standard XML.
/// This is a lightweight object wrapper around [xml-rs crate](https://github.com/netvl/xml-rs)
/// to simplify XML in-memory tree manipulation.
///
/// XML namespaces are not supported, as the main usage of this class is
/// to support JUnit report.
///
pub struct XmlDocument {
    pub root: Option<Element>,
}

impl XmlDocument {
    pub fn new(root: Element) -> XmlDocument {
        XmlDocument { root: Some(root) }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum XmlNode {
    /// An XML element.
    Element(Element),
    /// A CDATA section.
    CData(String),
    /// A comment.
    Comment(String),
    /// A text content.
    Text(String),
    /// Processing instruction.
    ProcessingInstruction(String, Option<String>),
}

/// A XML attribute.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    fn new(name: &str, value: &str) -> Self {
        Attribute {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Element {
    // We are only using local name, namespaces are not managed
    pub name: String,
    /// This element's attributes.
    pub attrs: Vec<Attribute>,
    /// This element's children.
    pub children: Vec<XmlNode>,
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            name: name.to_string(),
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Add a new attribute to `self`.
    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attrs.push(Attribute::new(name, value));
        self
    }

    /// Add a new child `element`.
    pub fn add_child(mut self, element: Element) -> Self {
        self.children.push(XmlNode::Element(element));
        self
    }

    /// Add a text content to `self`.
    pub fn text(mut self, text: &str) -> Self {
        self.children.push(XmlNode::Text(text.to_string()));
        self
    }

    /// Add a comment `self`.
    pub fn comment(mut self, comment: &str) -> Self {
        self.children.push(XmlNode::Comment(comment.to_string()));
        self
    }

    /// Add a CDATA section `self`.
    pub fn cdata(mut self, cdata: &str) -> Self {
        self.children.push(XmlNode::CData(cdata.to_string()));
        self
    }
}
