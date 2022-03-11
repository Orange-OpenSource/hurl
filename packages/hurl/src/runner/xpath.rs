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

// Unique entry point to libxml2

use std::ffi::CStr;

use super::value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XpathError {
    InvalidXml,
    InvalidHtml,
    Eval,
    Unsupported,
}

/// Eval a XPath 1.0 expression against a XML text.
///
/// # Arguments
///
/// * `xml` - A string slice that holds a XML body
/// * `expr`- A string slice for a XPath expression
///
pub fn eval_xml(xml: &str, expr: &str) -> Result<Value, XpathError> {
    let parser = libxml::parser::Parser::default();
    match parser.parse_string(xml) {
        Ok(doc) => {
            if doc.get_root_element() == None {
                Err(XpathError::InvalidXml {})
            } else {
                eval(&doc, expr, true)
            }
        }
        Err(_) => Err(XpathError::InvalidXml {}),
    }
}

/// Eval a XPath 1.0 expression against an HTML text
///
/// # Arguments
///
/// * `html` - A string slice that holds an HTML body
/// * `expr`- A string slice for a XPath expression
///
pub fn eval_html(html: &str, expr: &str) -> Result<Value, XpathError> {
    let parser = libxml::parser::Parser::default_html();
    match parser.parse_string(html) {
        Ok(doc) => {
            // You can have a doc structure even if the input xml is not valid
            // check that the root element exists
            if doc.get_root_element() == None {
                Err(XpathError::InvalidHtml {})
            } else {
                eval(&doc, expr, false)
            }
        }
        Err(_) => Err(XpathError::InvalidHtml {}),
    }
}

extern "C" {
    pub fn silentErrorFunc(
        ctx: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        ...
    );
}

/// Register all XML namespaces from a document to a context.
///
/// # Arguments
///
/// * `doc`- A libxml2 document reference
/// * `context` - A libxml2 context reference
///
fn register_namespaces(doc: &libxml::tree::Document, context: &libxml::xpath::Context) {
    // We walk through the xml document to register each namespace,
    // so we can eval xpath queries with namespace. For convenience, we register the
    // first default namespace with _ prefix. Other default namespaces are not registered
    // and should be referenced vi `local-name` or `name` XPath functions.
    let namespaces = get_document_namespaces(doc);
    let mut default_registered = false;

    for n in namespaces {
        if n.prefix.is_empty() {
            if !default_registered {
                context.register_namespace("_", &n.href).unwrap();
                default_registered = true;
            }
        } else {
            context.register_namespace(&n.prefix, &n.href).unwrap();
        }
    }
}

/// Eval a XPath 1.0 expression against an libxml2 document.
///
/// # Arguments
///
/// * `doc` - A libxml2 document reference
/// * `expr` - A string slice for an XPath expression
/// * `support_ns` - `true` if we need to support XML namespaces, `false` otherwise
///
fn eval(doc: &libxml::tree::Document, expr: &str, support_ns: bool) -> Result<Value, XpathError> {
    let context = libxml::xpath::Context::new(doc).expect("error setting context in xpath module");

    // libxml2 prints to sdtout warning and errors, so we mut it.
    unsafe {
        libxml::bindings::initGenericErrorDefaultFunc(&mut Some(silentErrorFunc));
    }

    if support_ns {
        register_namespaces(doc, &context);
    }

    let result = match context.evaluate(expr) {
        Ok(object) => object,
        Err(_) => return Err(XpathError::Eval {}),
    };

    match unsafe { *result.ptr }.type_ {
        libxml::bindings::xmlXPathObjectType_XPATH_NUMBER => {
            Ok(Value::from_f64(unsafe { *result.ptr }.floatval))
        }
        libxml::bindings::xmlXPathObjectType_XPATH_BOOLEAN => {
            Ok(Value::Bool(unsafe { *result.ptr }.boolval != 0))
        }
        libxml::bindings::xmlXPathObjectType_XPATH_STRING => {
            // TO BE CLEANED
            let c_s = unsafe { *result.ptr }.stringval;
            let c_s2 = c_s as *const std::os::raw::c_char;
            let x = unsafe { CStr::from_ptr(c_s2) };
            //let x = unsafe { CStr::from_ptr(u8::from(c_s2)) };
            let s = x.to_string_lossy().to_string();

            Ok(Value::String(s))
        }
        libxml::bindings::xmlXPathObjectType_XPATH_NODESET => {
            Ok(Value::Nodeset(result.get_number_of_nodes()))
        }
        _ => Err(XpathError::Unsupported {}),
    }
}

/// A XML namespace
#[derive(Debug, PartialEq, Eq)]
struct Namespace {
    prefix: String,
    href: String,
}

impl Namespace {
    /// Create a Namespace given a libxml2 namespace reference.
    ///
    /// # Arguments
    ///
    /// * `namespace` - A libxml2 namespace reference
    ///
    fn from(namespace: &libxml::tree::Namespace) -> Namespace {
        Namespace {
            prefix: namespace.get_prefix(),
            href: namespace.get_href(),
        }
    }
}

/// Returns all XML namespaces for a libxml2 document.
fn get_document_namespaces(doc: &libxml::tree::Document) -> Vec<Namespace> {
    let root = doc.get_root_element();
    let root = match root {
        None => return vec![],
        Some(r) => r,
    };
    get_namespaces(&root)
}

/// Returns all XML namespaces for a given libxml2 node.
fn get_namespaces(node: &libxml::tree::Node) -> Vec<Namespace> {
    let mut all_ns = Vec::new();

    // Get namespaces from the current node
    let ns: Vec<Namespace> = node
        .get_namespace_declarations()
        .into_iter()
        .map(|n| Namespace::from(&n))
        .collect();
    all_ns.extend(ns);

    // Get children namespaces
    let ns: Vec<Namespace> = node
        .get_child_nodes()
        .into_iter()
        .flat_map(|c| get_namespaces(&c))
        .collect();
    all_ns.extend(ns);

    all_ns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<food>
  <banana type="fruit" price="1.1"/>
  <apple type="fruit"/>
  <beef type="meat"/>
</food>
"#;
        let xpath = "count(//food/*)";
        assert_eq!(eval_xml(xml, xpath).unwrap(), Value::from_f64(3.0));

        let xpath = "//food/*";
        assert_eq!(eval_xml(xml, xpath).unwrap(), Value::Nodeset(3));

        let xpath = "count(//*[@type='fruit'])";
        assert_eq!(eval_xml(xml, xpath).unwrap(), Value::from_f64(2.0));

        let xpath = "number(//food/banana/@price)";
        assert_eq!(eval_xml(xml, xpath).unwrap(), Value::from_f64(1.1));
    }

    #[test]
    fn test_error_eval() {
        assert_eq!(eval_xml("<a/>", "^^^").err().unwrap(), XpathError::Eval {});
        assert_eq!(eval_xml("<a/>", "//").err().unwrap(), XpathError::Eval {});
        // assert_eq!(1,2);
    }

    // TBC!!!
    // Invalid XML not detected at parsing??? => goes into an eval error
    #[test]
    fn test_invalid_xml() {
        assert_eq!(
            eval_xml("??", "//person").err().unwrap(),
            XpathError::InvalidXml
        );
    }

    #[test]
    fn test_cafe() {
        assert_eq!(
            eval_xml("<data>café</data>", "normalize-space(//data)").unwrap(),
            Value::String(String::from("café"))
        );
    }

    #[test]
    fn test_html() {
        let html = r#"<html>
  <head>
    <meta charset="UTF-8"\>
  </head>
  <body>
    <br>
  </body>
</html>"#;
        let xpath = "normalize-space(/html/head/meta/@charset)";
        assert_eq!(
            eval_html(html, xpath).unwrap(),
            Value::String(String::from("UTF-8"))
        );
    }

    #[test]
    fn test_bug() {
        let html = r#"<html></html>"#;
        //let xpath = String::from("boolean(count(//a[contains(@href,'xxx')]))");
        let xpath = "boolean(count(//a[contains(@href,'xxx')]))";
        assert_eq!(eval_html(html, xpath).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_unregistered_function() {
        let html = r#"<html></html>"#;
        let xpath = "strong(//head/title)";
        assert_eq!(eval_html(html, xpath).err().unwrap(), XpathError::Eval);
    }

    #[test]
    fn test_namespaces_with_prefix() {
        let xml = r#"<?xml version ="1.0"?>
<a:books xmlns:a="foo:" xmlns:b="bar:">
    <b:book xmlns:c="baz:">
        <b:title>Dune</b:title>
        <c:author>Franck Herbert</c:author>
    </b:book>
</a:books>"#;

        let expr = "string(//a:books/b:book/b:title)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("Dune".to_string())
        );

        let expr = "string(//a:books/b:book/c:author)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("Franck Herbert".to_string())
        );

        let expr = "string(//*[name()='a:books']/*[name()='b:book']/*[name()='c:author'])";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("Franck Herbert".to_string())
        );

        let expr =
            "string(//*[local-name()='books']/*[local-name()='book']/*[local-name()='author'])";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("Franck Herbert".to_string())
        );
    }

    #[test]
    fn test_default_namespaces() {
        let xml = r#"<svg version="1.1" width="300" height="200" xmlns="http://www.w3.org/2000/svg">
    <rect width="100%" height="100%" fill="red" />
    <circle cx="150" cy="100" r="80" fill="green" />
    <text x="150" y="125" font-size="60" text-anchor="middle" fill="white">SVG</text>
</svg>"#;

        let expr = "string(//_:svg/_:text)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("SVG".to_string())
        );

        let expr = "string(//*[name()='svg']/*[name()='text'])";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("SVG".to_string())
        );

        let expr = "string(//*[local-name()='svg']/*[local-name()='text'])";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("SVG".to_string())
        );
    }

    #[test]
    fn test_soap() {
        let xml = r#"<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <soap:Body xmlns:ns1="http://www.opentravel.org/OTA/2003/05">
        <ns1:OTA_AirAvailRS
            EchoToken="11868765275150-1300257934"
            PrimaryLangID="en-us"
            RetransmissionIndicator="false"
            SequenceNmbr="1"
            TransactionIdentifier="TID$16459590516432752971.demo2144"
            Version="2006.01">
        </ns1:OTA_AirAvailRS>
    </soap:Body>
</soap:Envelope>"#;

        let expr = "string(//soap:Envelope/soap:Body/ns1:OTA_AirAvailRS/@TransactionIdentifier)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("TID$16459590516432752971.demo2144".to_string())
        );

        let expr = "string(//*[name()='soap:Envelope']/*[name()='soap:Body']/*[name()='ns1:OTA_AirAvailRS']/@TransactionIdentifier)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("TID$16459590516432752971.demo2144".to_string())
        );

        let expr = "string(//*[local-name()='Envelope']/*[local-name()='Body']/*[local-name()='OTA_AirAvailRS']/@TransactionIdentifier)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("TID$16459590516432752971.demo2144".to_string())
        );
    }

    #[test]
    fn test_namespaces_scoping() {
        let xml = r#"<?xml version="1.0"?>
<!-- initially, the default namespace is "books" -->
<book xmlns='urn:loc.gov:books'
      xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <title>Cheaper by the Dozen</title>
    <isbn:number>1568491379</isbn:number>
    <notes>
      <!-- make HTML the default namespace for some commentary -->
      <p xmlns='http://www.w3.org/1999/xhtml'>
          This is a <i>funny</i> book!
      </p>
    </notes>
</book>
        "#;

        let expr = "string(//_:book/_:title)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("Cheaper by the Dozen".to_string())
        );

        let expr = "string(//_:book/isbn:number)";
        assert_eq!(
            eval_xml(xml, expr).unwrap(),
            Value::String("1568491379".to_string())
        );

        let expr = "//*[name()='book']/*[name()='notes']";
        assert_eq!(eval_xml(xml, expr).unwrap(), Value::Nodeset(1));

        let expr = "//_:book/_:notes/*[local-name()='p']";
        assert_eq!(eval_xml(xml, expr).unwrap(), Value::Nodeset(1));
    }
}
