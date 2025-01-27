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
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};

use crate::report::junit::xml::reader::ParserError::{GenericError, InvalidXml};
use crate::report::junit::xml::{Attribute, Element, XmlDocument, XmlNode};

/// Errors raised when deserializing a buffer.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ParserError {
    InvalidXml(String),
    GenericError(String),
}

/// Deserializes a XML document from [`std::io::Read`] source. The XML
/// document returned is a in-memory tree representation of the whole document.
impl XmlDocument {
    /// Read a XML `source` and parses it to a [`XmlDocument`].
    pub fn parse<R>(source: R) -> Result<XmlDocument, ParserError>
    where
        R: std::io::Read,
    {
        let mut reader = EventReader::new(source);
        let mut initialized = false;
        let mut root: Option<Element> = None;
        loop {
            match reader.next() {
                Ok(XmlEvent::StartDocument { .. }) => initialized = true,
                Ok(XmlEvent::EndDocument) => {
                    if !initialized {
                        return Err(InvalidXml("Invalid end of document".to_string()));
                    }
                    return Ok(XmlDocument { root });
                }
                Ok(XmlEvent::ProcessingInstruction { .. }) => {}
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    // At this point of the parsing, we must have an initialized document
                    // and no root.
                    if !initialized || root.is_some() {
                        return Err(InvalidXml("Invalid start of document".to_string()));
                    }
                    let element = Element::try_parse(&name, &attributes, &mut reader)?;
                    root = Some(element);
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    return Err(InvalidXml("Invalid end of element".to_string()))
                }
                Ok(XmlEvent::CData(_)) => {}
                Ok(XmlEvent::Comment(_)) => {}
                Ok(XmlEvent::Characters(_)) => {}
                Ok(XmlEvent::Whitespace(_)) => {}
                Err(e) => return Err(GenericError(e.to_string())),
            }
        }
    }
}

impl Element {
    fn try_parse<R: std::io::Read>(
        name: &OwnedName,
        attributes: &[OwnedAttribute],
        reader: &mut EventReader<R>,
    ) -> Result<Element, ParserError> {
        let mut element = Element::new(&name.local_name);
        element.attrs = attributes
            .iter()
            .map(|a| Attribute::new(&a.name.to_string(), &a.value))
            .collect();

        loop {
            match reader.next() {
                Ok(XmlEvent::StartDocument { .. }) => {
                    return Err(InvalidXml("Invalid start of document".to_string()))
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(InvalidXml("Invalid stop of document".to_string()))
                }
                Ok(XmlEvent::ProcessingInstruction { name, data }) => {
                    let child = XmlNode::ProcessingInstruction(name, data);
                    element.children.push(child);
                }
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let child = Element::try_parse(&name, &attributes, reader)?;
                    element.children.push(XmlNode::Element(child));
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    return if element.name == name.local_name {
                        Ok(element)
                    } else {
                        Err(InvalidXml(format!("Bag closing element {name}")))
                    }
                }
                Ok(XmlEvent::CData(value)) => {
                    let child = XmlNode::CData(value);
                    element.children.push(child);
                }
                Ok(XmlEvent::Comment(value)) => {
                    let child = XmlNode::Comment(value);
                    element.children.push(child);
                }
                Ok(XmlEvent::Characters(value)) => {
                    let child = XmlNode::Text(value);
                    element.children.push(child);
                }
                Ok(XmlEvent::Whitespace(_)) => {}
                Err(e) => return Err(GenericError(e.to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::report::junit::xml::reader::ParserError;
    use crate::report::junit::xml::{Element, XmlDocument};

    /// Convenient function to read and parse a XML string `source`.
    fn parse_str(source: &str) -> Result<XmlDocument, ParserError> {
        let bytes = source.as_bytes();
        XmlDocument::parse(bytes)
    }

    #[test]
    fn read_xml_0_succeed() {
        let xml = r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<names>
    <name first="bob" last="jones" />
    <name first="elizabeth" last="smith" />
</names>
"#;
        let doc = parse_str(xml).unwrap();
        assert_eq!(
            doc.root.unwrap(),
            Element::new("names")
                .add_child(
                    Element::new("name")
                        .attr("first", "bob")
                        .attr("last", "jones")
                )
                .add_child(
                    Element::new("name")
                        .attr("first", "elizabeth")
                        .attr("last", "smith")
                )
        );
    }

    #[test]
    fn read_xml_1_succeed() {
        let xml = r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<project name="project-name">
    <libraries>
        <library groupId="org.example" artifactId="&lt;name&gt;" version="0.1"/>
        <library groupId="com.example" artifactId="&quot;cool-lib&amp;" version="999"/>
    </libraries>
    <module name="module-1">
        <files>
            <file name="somefile.java" type="java">
                Some &lt;java&gt; class
            </file>
            <file name="another_file.java" type="java">
                Another &quot;java&quot; class
            </file>
            <file name="config.xml" type="xml">
                Weird &apos;XML&apos; config
            </file>
        </files>
        <libraries>
            <library groupId="junit" artifactId="junit" version="1.9.5"/>
        </libraries>
    </module>
    <module name="module-2">
        <files>
            <file name="program.js" type="javascript">
                JavaScript &amp; program
            </file>
            <file name="style.css" type="css">
                Cascading style sheet: &#xA9; - &#1161;
            </file>
        </files>
    </module>
</project>
        "#;

        let doc = parse_str(xml).unwrap();
        assert_eq!(
            doc.root.unwrap(),
            Element::new("project")
                        .attr("name", "project-name")
                        .add_child(
                            Element::new("libraries")
                                .add_child(
                                    Element::new("library")
                                        .attr("groupId", "org.example")
                                        .attr("artifactId", "<name>")
                                        .attr("version", "0.1")
                                )
                                .add_child(
                                    Element::new("library")
                                        .attr("groupId", "com.example")
                                        .attr("artifactId", "\"cool-lib&")
                                        .attr("version", "999")
                                )
                        )
                        .add_child(
                            Element::new("module")
                                .attr("name", "module-1")
                                .add_child(
                                    Element::new("files")
                                        .add_child(
                                            Element::new("file")
                                                .attr("name", "somefile.java")
                                                .attr("type", "java")
                                                .text("\n                Some <java> class\n            ")
                                        )
                                        .add_child(
                                            Element::new("file")
                                                .attr("name", "another_file.java")
                                                .attr("type", "java")
                                                .text("\n                Another \"java\" class\n            ")
                                        )
                                        .add_child(
                                            Element::new("file")
                                                .attr("name", "config.xml")
                                                .attr("type", "xml")
                                                .text("\n                Weird 'XML' config\n            ")
                                        )
                                )
                                .add_child(
                                    Element::new("libraries")
                                        .add_child(Element::new("library")
                                            .attr("groupId", "junit")
                                            .attr("artifactId", "junit")
                                            .attr("version", "1.9.5")
                                        )
                                )
                        )
                        .add_child(
                            Element::new("module")
                                .attr("name", "module-2")
                                .add_child(
                                    Element::new("files")
                                        .add_child(
                                            Element::new("file")
                                                .attr("name", "program.js")
                                                .attr("type", "javascript")
                                                .text("\n                JavaScript & program\n            ")
                                        )
                                        .add_child(
                                            Element::new("file")
                                                .attr("name", "style.css")
                                                .attr("type", "css")
                                                .text("\n                Cascading style sheet: © - \u{489}\n            ")
                                        )
                                )
                        )
                );
    }

    #[test]
    fn read_xml_with_namespaces() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<p:data xmlns:p="urn:example:namespace" xmlns:d="urn:example:double" xmlns:h="urn:example:header">
  <p:datum id="34">
    <p:name>Name</p:name>
    <d:name>Another name</d:name>
    <d:arg>0.3</d:arg>
    <d:arg>0.2</d:arg>
    <p:arg>0.1</p:arg>
    <p:arg>0.01</p:arg>
    <h:header name="Header-1">header 1 value</h:header>
    <h:header name="Header-2">
      Some bigger value
    </h:header>
  </p:datum>
</p:data>"#;

        let doc = parse_str(xml).unwrap();
        assert_eq!(
            doc.root.unwrap(),
            Element::new("data").add_child(
                Element::new("datum")
                    .attr("id", "34")
                    .add_child(Element::new("name").text("Name"))
                    .add_child(Element::new("name").text("Another name"))
                    .add_child(Element::new("arg").text("0.3"))
                    .add_child(Element::new("arg").text("0.2"))
                    .add_child(Element::new("arg").text("0.1"))
                    .add_child(Element::new("arg").text("0.01"))
                    .add_child(
                        Element::new("header")
                            .attr("name", "Header-1")
                            .text("header 1 value")
                    )
                    .add_child(
                        Element::new("header")
                            .attr("name", "Header-2")
                            .text("\n      Some bigger value\n    ")
                    )
            )
        );
    }

    #[test]
    fn read_junit_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><testsuites><testsuite tests="3" errors="0" failures="1"><testcase id="/tmp/a.hurl" name="/tmp/a.hurl" time="0.438" /><testcase id="/tmp/b.hurl" name="/tmp/b.hurl" time="0.234"><failure>Assert status code
  --> /tmp/b.hurl:4:6
   |
 4 | HTTP 300
   |      ^^^ actual value is &lt;200>
   |</failure></testcase><testcase id="/tmp/c.hurl" name="/tmp/c.hurl" time="0.236" /></testsuite><testsuite tests="2" errors="0" failures="1"><testcase id="/tmp/a.hurl" name="/tmp/a.hurl" time="0.370" /><testcase id="/tmp/c.hurl" name="/tmp/c.hurl" time="0.247"><failure>Assert failure
  --> /tmp/c.hurl:6:0
   |
 6 | xpath "normalize-space(//title)" == "Hello World!"
   |   actual:   string &lt;Hurl - Run and Test HTTP Requests>
   |   expected: string &lt;Hello World!>
   |</failure></testcase></testsuite></testsuites>"#;
        let doc = parse_str(xml).unwrap();
        assert_eq!(
            doc.root.unwrap(),
            Element::new("testsuites")
                .add_child(
                    Element::new("testsuite")
                        .attr("tests", "3")
                        .attr("errors", "0")
                        .attr("failures", "1")
                        .add_child(
                            Element::new("testcase")
                                .attr("id", "/tmp/a.hurl")
                                .attr("name", "/tmp/a.hurl")
                                .attr("time", "0.438")
                        )
                        .add_child(
                            Element::new("testcase")
                                .attr("id", "/tmp/b.hurl")
                                .attr("name", "/tmp/b.hurl")
                                .attr("time", "0.234")
                                .add_child(Element::new("failure").text(
                                    r#"Assert status code
  --> /tmp/b.hurl:4:6
   |
 4 | HTTP 300
   |      ^^^ actual value is <200>
   |"#
                                ))
                        )
                        .add_child(
                            Element::new("testcase")
                                .attr("id", "/tmp/c.hurl")
                                .attr("name", "/tmp/c.hurl")
                                .attr("time", "0.236")
                        )
                )
                .add_child(
                    Element::new("testsuite")
                        .attr("tests", "2")
                        .attr("errors", "0")
                        .attr("failures", "1")
                        .add_child(
                            Element::new("testcase")
                                .attr("id", "/tmp/a.hurl")
                                .attr("name", "/tmp/a.hurl")
                                .attr("time", "0.370")
                        )
                        .add_child(
                            Element::new("testcase")
                                .attr("id", "/tmp/c.hurl")
                                .attr("name", "/tmp/c.hurl")
                                .attr("time", "0.247")
                                .add_child(Element::new("failure").text(
                                    r#"Assert failure
  --> /tmp/c.hurl:6:0
   |
 6 | xpath "normalize-space(//title)" == "Hello World!"
   |   actual:   string <Hurl - Run and Test HTTP Requests>
   |   expected: string <Hello World!>
   |"#
                                ))
                        )
                )
        );
    }
}
