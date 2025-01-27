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
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::string::FromUtf8Error;

use xml::attribute::Attribute;
use xml::name::Name;
use xml::namespace::Namespace;
use xml::writer::{Error, XmlEvent};
use xml::EventWriter;

use crate::report::junit::xml::{Element, XmlDocument, XmlNode};

/// Errors raised when serializing an XML document.
#[derive(Debug)]
pub enum WriterError {
    Io(std::io::Error),
    FromUtf8Error(FromUtf8Error),
    GenericError(String),
}

impl fmt::Display for WriterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WriterError::Io(err) => write!(f, "{err}"),
            WriterError::FromUtf8Error(err) => write!(f, "{err}"),
            WriterError::GenericError(err) => write!(f, "{err}"),
        }
    }
}

impl From<Error> for WriterError {
    fn from(value: Error) -> Self {
        match value {
            Error::Io(error) => WriterError::Io(error),
            Error::DocumentStartAlreadyEmitted
            | Error::LastElementNameNotAvailable
            | Error::EndElementNameIsNotEqualToLastStartElementName
            | Error::EndElementNameIsNotSpecified => WriterError::GenericError(value.to_string()),
        }
    }
}

impl From<FromUtf8Error> for WriterError {
    fn from(value: FromUtf8Error) -> Self {
        WriterError::FromUtf8Error(value)
    }
}

impl XmlDocument {
    /// Convenient method to serialize an XML document to a string.
    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String, WriterError> {
        let buffer = vec![];
        let buffer = self.write(buffer)?;
        let str = String::from_utf8(buffer)?;
        Ok(str)
    }

    /// Serializes an XML document to a `buffer`.
    pub fn write<W>(&self, buffer: W) -> Result<W, WriterError>
    where
        W: std::io::Write,
    {
        let mut writer = EventWriter::new(buffer);
        if let Some(root) = &self.root {
            root.write(&mut writer)?;
        }
        Ok(writer.into_inner())
    }
}

impl XmlNode {
    fn write<W>(&self, writer: &mut EventWriter<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        match self {
            XmlNode::Element(elem) => elem.write(writer)?,
            XmlNode::CData(cdata) => writer.write(XmlEvent::CData(cdata))?,
            XmlNode::Comment(comment) => writer.write(XmlEvent::Comment(comment))?,
            XmlNode::Text(text) => writer.write(XmlEvent::Characters(text))?,
            XmlNode::ProcessingInstruction(name, data) => match data {
                Some(string) => writer.write(XmlEvent::ProcessingInstruction {
                    name,
                    data: Some(string),
                })?,
                None => writer.write(XmlEvent::ProcessingInstruction { name, data: None })?,
            },
        }
        Ok(())
    }
}

impl Element {
    fn write<W>(&self, writer: &mut EventWriter<W>) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        let name = Name::local(&self.name);
        let attributes = self
            .attrs
            .iter()
            .map(|attr| Attribute {
                name: Name::local(&attr.name),
                value: &attr.value,
            })
            .collect();

        // TODO: manage namespaces
        let empty_ns = Namespace::empty();
        writer.write(XmlEvent::StartElement {
            name,
            attributes: Cow::Owned(attributes),
            namespace: Cow::Owned(empty_ns),
        })?;

        for child in self.children.iter() {
            child.write(writer)?;
        }

        writer.write(XmlEvent::EndElement { name: Some(name) })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::report::junit::xml::{Element, XmlDocument};

    #[test]
    fn write_xml_0() {
        let root = Element::new("catalog")
            .add_child(
                Element::new("book")
                    .attr("id", "bk101")
                    .add_child(
                        Element::new("author")
                            .text("Gambardella, Matthew")
                    )
                    .add_child(
                        Element::new("title")
                            .text("XML Developer's Guide")
                    )
                    .add_child(
                        Element::new("genre")
                            .text("Computer")
                    )
                    .add_child(
                        Element::new("price")
                            .text("44.95")
                    )
                    .add_child(
                        Element::new("publish_date")
                            .text("2000-10-01")
                    )
                    .add_child(
                        Element::new("description")
                            .text("An in-depth look at creating applications with XML.")
                    )
            )
            .add_child(
                Element::new("book")
                    .attr("id", "bk102")
                    .add_child(
                        Element::new("author")
                            .text("Ralls, Kim")
                    )
                    .add_child(
                        Element::new("title")
                            .text("Midnight Rain")
                    )
                    .add_child(
                        Element::new("genre")
                            .text("Fantasy")
                    )
                    .add_child(
                        Element::new("price")
                            .text("5.95")
                    )
                    .add_child(
                        Element::new("publish_date")
                            .text("2000-12-16")
                    )
                    .add_child(
                        Element::new("description")
                            .text("A former architect battles corporate zombies, an evil sorceress, and her own childhood to become queen of the world.")
                    )
            )
            ;
        let doc = XmlDocument { root: Some(root) };
        assert_eq!(
            doc.to_string().unwrap(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <catalog>\
                <book id=\"bk101\">\
                    <author>Gambardella, Matthew</author>\
                    <title>XML Developer's Guide</title>\
                    <genre>Computer</genre>\
                    <price>44.95</price>\
                    <publish_date>2000-10-01</publish_date>\
                    <description>An in-depth look at creating applications with XML.</description>\
                </book>\
                <book id=\"bk102\">\
                    <author>Ralls, Kim</author>\
                    <title>Midnight Rain</title>\
                    <genre>Fantasy</genre>\
                    <price>5.95</price>\
                    <publish_date>2000-12-16</publish_date>\
                    <description>A former architect battles corporate zombies, an evil sorceress, and her own childhood to become queen of the world.</description>\
                </book>\
            </catalog>"
        );
    }
}
