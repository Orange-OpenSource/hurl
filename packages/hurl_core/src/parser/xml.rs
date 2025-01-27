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
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use libxml::bindings::{
    xmlChar, xmlCreatePushParserCtxt, xmlFreeParserCtxt, xmlParseChunk, xmlSAXHandlerPtr,
};

use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

/// Parses a text buffer until a valid XML has been found.
/// We're using a SAX XML parser because we need to stop the parsing at the byte position where
/// an XML text is detected.
/// For example, when we have this kind of Hurl file:
///
/// ```hurl
/// POST https://foo.com
/// <?xml version="1.0"?>
/// <catalog>
///   <book id="bk101">
///     <author>Gambardella, Matthew</author>
///     <title>XML Developer's Guide</title>
///   </book>
/// </catalog>
/// HTTP 201
/// ```
///
/// As there is no "formal" end of body, we need to parse the string until we detect at the precise
/// byte a possible valid XML body.
///
pub fn parse(reader: &mut Reader) -> ParseResult<String> {
    // We test if our first character is a start of an XML text.
    // If not, we return immediately a recoverable error.
    // Otherwise, we start parsing the supposed XML buffer. Any subsequent error will be a
    // non-recoverable error.
    let c = reader.peek();
    match c {
        Some('<') => {}
        _ => {
            return Err(ParseError::new(
                reader.cursor().pos,
                true,
                ParseErrorKind::Xml,
            ))
        }
    }

    let mut buf = String::new();
    let mut parser = new_sax_parser();
    let mut parser_context = ParserContext::new();

    // We use libxml SAX parser to identify the end of the XML body.
    // We feed the SAX parser chars by chars (Rust char), so chunks are UFT-8 bytes,
    // 1 byte to 4 bytes long. The detection of the body end is done when receiving a closing
    // element event by testing the depth of the XML tree.
    unsafe {
        let context = xmlCreatePushParserCtxt(
            &mut parser as xmlSAXHandlerPtr,
            &mut parser_context as *mut ParserContext as *mut c_void,
            ptr::null(),
            0,
            ptr::null(),
        );

        // We keep track of the previous char reader position, to accurately raise eventual error.
        let mut prev_pos = reader.cursor().pos;

        while let Some(c) = reader.read() {
            buf.push(c);

            // We feed the parser chars by chars.
            // A buffer of length four is large enough to encode any char.
            let mut bytes = [0u8; 4];
            let end = reader.is_eof() as c_int;
            let bytes = c.encode_utf8(&mut bytes);
            let count = bytes.len() as c_int;
            let bytes = bytes.as_ptr() as *const c_char;
            let ret = xmlParseChunk(context, bytes, count, end);
            if ret != 0 {
                xmlFreeParserCtxt(context);
                return Err(ParseError::new(prev_pos, false, ParseErrorKind::Xml));
            }

            // End of the XML body is detected with a closing element event and depth of the tree.
            // There is also a closing document event but it's not always raised at the exact
            // closing `>` position.
            if std::matches!(parser_context.state, ParserState::EndElement)
                && parser_context.depth == 0
            {
                break;
            }
            prev_pos = reader.cursor().pos;
        }

        xmlFreeParserCtxt(context);
    }

    Ok(buf)
}

/// A context for the SAX parser, containing a `state` and the current tree `depth`.
struct ParserContext {
    depth: usize,
    state: ParserState,
}

impl ParserContext {
    fn new() -> ParserContext {
        ParserContext {
            depth: 0,
            state: ParserState::Created,
        }
    }
}

enum ParserState {
    Created,
    StartDocument,
    EndDocument,
    StartElement,
    EndElement,
}

fn new_sax_parser() -> libxml::bindings::xmlSAXHandler {
    libxml::bindings::xmlSAXHandler {
        internalSubset: None,
        isStandalone: None,
        hasInternalSubset: None,
        hasExternalSubset: None,
        resolveEntity: None,
        getEntity: None,
        entityDecl: None,
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: None,
        startDocument: Some(on_start_document),
        endDocument: Some(on_end_document),
        startElement: None,
        endElement: None,
        reference: None,
        characters: None,
        ignorableWhitespace: None,
        processingInstruction: None,
        comment: None,
        warning: None,
        error: None,
        fatalError: None,
        getParameterEntity: None,
        cdataBlock: None,
        externalSubset: None,
        initialized: libxml::bindings::XML_SAX2_MAGIC,
        _private: ptr::null_mut(),
        startElementNs: Some(on_start_element),
        endElementNs: Some(on_end_element),
        serror: None,
    }
}

/// Called when the document start being processed.
unsafe extern "C" fn on_start_document(ctx: *mut c_void) {
    let context: &mut ParserContext = unsafe { &mut *(ctx as *mut ParserContext) };
    context.state = ParserState::StartDocument;
}

/// Called when the document end has been detected.
unsafe extern "C" fn on_end_document(ctx: *mut c_void) {
    let context: &mut ParserContext = unsafe { &mut *(ctx as *mut ParserContext) };
    context.state = ParserState::EndDocument;
}

/// Called when an opening tag has been processed.
unsafe extern "C" fn on_start_element(
    ctx: *mut c_void,
    _local_name: *const xmlChar,
    _prefix: *const xmlChar,
    _uri: *const xmlChar,
    _nb_namespaces: c_int,
    _namespaces: *mut *const xmlChar,
    _nb_attributes: c_int,
    _nb_defaulted: c_int,
    _attributes: *mut *const xmlChar,
) {
    let context: &mut ParserContext = unsafe { &mut *(ctx as *mut ParserContext) };
    context.state = ParserState::StartElement;
    context.depth += 1;
}

/// Called when the end of an element has been detected.
unsafe extern "C" fn on_end_element(
    ctx: *mut c_void,
    _local_name: *const xmlChar,
    _prefix: *const xmlChar,
    _uri: *const xmlChar,
) {
    let context: &mut ParserContext = unsafe { &mut *(ctx as *mut ParserContext) };
    context.state = ParserState::EndElement;
    context.depth -= 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Pos;

    #[test]
    fn parse_xml_brute_force_errors() {
        let mut reader = Reader::new("");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Xml);
        assert!(error.recoverable);

        let mut reader = Reader::new("x");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Xml);
        assert!(error.recoverable);

        let mut reader = Reader::new("<<");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(error.kind, ParseErrorKind::Xml);
        assert!(!error.recoverable);

        let mut reader = Reader::new("<users><user /></users");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 22
            }
        );
        assert_eq!(error.kind, ParseErrorKind::Xml);

        let mut reader = Reader::new("<users aa><user /></users");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 10
            }
        );
        assert_eq!(error.kind, ParseErrorKind::Xml);
    }

    #[test]
    fn parse_xml_brute_force_ok() {
        let mut reader = Reader::new("<users><user /></users>");
        assert_eq!(
            parse(&mut reader).unwrap(),
            String::from("<users><user /></users>")
        );
        assert_eq!(reader.cursor().index, 23);

        let mut reader = Reader::new("<users><user /></users>xx");
        assert_eq!(
            parse(&mut reader).unwrap(),
            String::from("<users><user /></users>")
        );
        assert_eq!(reader.cursor().index, 23);
        assert_eq!(reader.peek_n(2), String::from("xx"));

        let mut reader = Reader::new("<?xml version=\"1.0\"?><users/>xxx");
        assert_eq!(
            parse(&mut reader).unwrap(),
            String::from("<?xml version=\"1.0\"?><users/>")
        );
        assert_eq!(reader.cursor().index, 29);
    }

    #[test]
    fn parse_xml_soap_utf8() {
        let xml = r#"<?xml version='1.0' encoding='UTF-8'?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
    <soapenv:Body>
        <ns31:UploadInboundResponseElement xmlns:ns31="http://www.example.com/schema/xyzWS">
            <ns31:UploadInboundResult>&lt;?xml version="1.0" encoding="UTF-8" ?>&lt;ATKCST>&lt;Head>&lt;FunCode>9000&lt;/FunCode>&lt;Remark>接收数据成功&lt;/Remark>&lt;/Head>&lt;/ATKCST></ns31:UploadInboundResult>
        </ns31:UploadInboundResponseElement>
    </soapenv:Body>
</soapenv:Envelope>"#;

        // A valid complete XML
        let input = xml;
        let output = xml;
        let mut reader = Reader::new(input);
        assert_eq!(parse(&mut reader).unwrap(), String::from(output),);
        assert_eq!(reader.cursor().index, 520);

        // A XML with data padding
        let input = format!("{xml} xx xx xx xx");
        let output = xml;
        let mut reader = Reader::new(&input);
        assert_eq!(parse(&mut reader).unwrap(), String::from(output),);
        assert_eq!(reader.cursor().index, 520);

        // Two consecutive XML
        let input = format!("{xml}{xml}");
        let output = xml;
        let mut reader = Reader::new(&input);
        assert_eq!(parse(&mut reader).unwrap(), String::from(output),);
        assert_eq!(reader.cursor().index, 520);

        let mut reader = Reader::new(&input);
        assert_eq!(parse(&mut reader).unwrap(), String::from(output),);
        assert_eq!(reader.cursor().index, 520);
    }

    #[test]
    fn parse_xml_books_with_entry_response_start() {
        let xml = r#"<?xml version="1.0"?>
<catalog>
   <book id="bk101">
      <author>Gambardella, Matthew</author>
      <title>XML Developer's Guide</title>
      <genre>Computer</genre>
      <price>44.95</price>
      <publish_date>2000-10-01</publish_date>
      <description>An in-depth look at creating applications
      with XML.</description>
   </book>
   <book id="bk102">
      <author>Ralls, Kim</author>
      <title>Midnight Rain</title>
      <genre>Fantasy</genre>
      <price>5.95</price>
      <publish_date>2000-12-16</publish_date>
      <description>A former architect battles corporate zombies,
      an evil sorceress, and her own childhood to become queen
      of the world.</description>
   </book>
   <book id="bk103">
      <author>Corets, Eva</author>
      <title>Maeve Ascendant</title>
      <genre>Fantasy</genre>
      <price>5.95</price>
      <publish_date>2000-11-17</publish_date>
      <description>After the collapse of a nanotechnology
      society in England, the young survivors lay the
      foundation for a new society.</description>
   </book>
   <book id="bk104">
      <author>Corets, Eva</author>
      <title>Oberon's Legacy</title>
      <genre>Fantasy</genre>
      <price>5.95</price>
      <publish_date>2001-03-10</publish_date>
      <description>In post-apocalypse England, the mysterious
      agent known only as Oberon helps to create a new life
      for the inhabitants of London. Sequel to Maeve
      Ascendant.</description>
   </book>
   <book id="bk105">
      <author>Corets, Eva</author>
      <title>The Sundered Grail</title>
      <genre>Fantasy</genre>
      <price>5.95</price>
      <publish_date>2001-09-10</publish_date>
      <description>The two daughters of Maeve, half-sisters,
      battle one another for control of England. Sequel to
      Oberon's Legacy.</description>
   </book>
   <book id="bk106">
      <author>Randall, Cynthia</author>
      <title>Lover Birds</title>
      <genre>Romance</genre>
      <price>4.95</price>
      <publish_date>2000-09-02</publish_date>
      <description>When Carla meets Paul at an ornithology
      conference, tempers fly as feathers get ruffled.</description>
   </book>
   <book id="bk107">
      <author>Thurman, Paula</author>
      <title>Splish Splash</title>
      <genre>Romance</genre>
      <price>4.95</price>
      <publish_date>2000-11-02</publish_date>
      <description>A deep sea diver finds true love twenty
      thousand leagues beneath the sea.</description>
   </book>
   <book id="bk108">
      <author>Knorr, Stefan</author>
      <title>Creepy Crawlies</title>
      <genre>Horror</genre>
      <price>4.95</price>
      <publish_date>2000-12-06</publish_date>
      <description>An anthology of horror stories about roaches,
      centipedes, scorpions  and other insects.</description>
   </book>
   <book id="bk109">
      <author>Kress, Peter</author>
      <title>Paradox Lost</title>
      <genre>Science Fiction</genre>
      <price>6.95</price>
      <publish_date>2000-11-02</publish_date>
      <description>After an inadvertent trip through a Heisenberg
      Uncertainty Device, James Salway discovers the problems
      of being quantum.</description>
   </book>
   <book id="bk110">
      <author>O'Brien, Tim</author>
      <title>Microsoft .NET: The Programming Bible</title>
      <genre>Computer</genre>
      <price>36.95</price>
      <publish_date>2000-12-09</publish_date>
      <description>Microsoft's .NET initiative is explored in
      detail in this deep programmer's reference.</description>
   </book>
   <book id="bk111">
      <author>O'Brien, Tim</author>
      <title>MSXML3: A Comprehensive Guide</title>
      <genre>Computer</genre>
      <price>36.95</price>
      <publish_date>2000-12-01</publish_date>
      <description>The Microsoft MSXML3 parser is covered in
      detail, with attention to XML DOM interfaces, XSLT processing,
      SAX and more.</description>
   </book>
   <book id="bk112">
      <author>Galos, Mike</author>
      <title>Visual Studio 7: A Comprehensive Guide</title>
      <genre>Computer</genre>
      <price>49.95</price>
      <publish_date>2001-04-16</publish_date>
      <description>Microsoft Visual Studio 7 is explored in depth,
      looking at how Visual Basic, Visual C++, C#, and ASP+ are
      integrated into a comprehensive development
      environment.</description>
   </book>
</catalog>"#;

        let chunk = format!("{xml}\nHTTP 200");
        let mut reader = Reader::new(&chunk);
        assert_eq!(parse(&mut reader).unwrap(), String::from(xml),);
        assert_eq!(reader.cursor().index, 4411);
    }
}
