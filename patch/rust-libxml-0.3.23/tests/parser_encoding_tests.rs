//! Tests for `ParserOptions::encoding`, the manually-specified input encoding.
//!
//! A use-after-free of the encoding name (fixed in #216) was not caught by
//! these assertions alone, as the freed bytes usually survive until libxml2
//! reads them; run under valgrind to check the encoding name's lifetime.
use libxml::parser::{Parser, ParserOptions, XmlParseError};

const LATIN1_BYTES: &[u8] = b"<root>caf\xe9</root>";
const LATIN1_FILE: &str = "tests/resources/file01_latin1_nodecl.xml";

fn latin1_options() -> ParserOptions<'static> {
  ParserOptions {
    encoding: Some("ISO-8859-1"),
    ..Default::default()
  }
}

fn nul_options() -> ParserOptions<'static> {
  ParserOptions {
    encoding: Some("UTF-8\0junk"),
    ..Default::default()
  }
}

#[test]
fn xml_string_with_encoding() {
  let doc = Parser::default()
    .parse_string_with_options(LATIN1_BYTES, latin1_options())
    .unwrap();
  assert_eq!(doc.get_root_element().unwrap().get_content(), "café");
}

#[test]
fn html_string_with_encoding() {
  let doc = Parser::default_html()
    .parse_string_with_options(LATIN1_BYTES, latin1_options())
    .unwrap();
  let root = doc.get_root_element().unwrap();
  assert_eq!(root.get_content(), "café");
}

#[test]
fn xml_file_with_encoding() {
  let doc = Parser::default()
    .parse_file_with_options(LATIN1_FILE, latin1_options())
    .unwrap();
  assert_eq!(doc.get_root_element().unwrap().get_content(), "café");
}

#[test]
fn html_file_with_encoding() {
  let doc = Parser::default_html()
    .parse_file_with_options(LATIN1_FILE, latin1_options())
    .unwrap();
  // the HTML parser keeps the file's trailing newline as body text
  assert_eq!(doc.get_root_element().unwrap().get_content(), "café\n");
}

#[test]
/// An encoding name with an interior NUL is an error, not a panic.
fn encoding_with_nul_is_rejected() {
  for parser in [Parser::default(), Parser::default_html()] {
    assert!(matches!(
      parser.parse_string_with_options(LATIN1_BYTES, nul_options()),
      Err(XmlParseError::GotNullPointer)
    ));
    assert!(matches!(
      parser.parse_file_with_options(LATIN1_FILE, nul_options()),
      Err(XmlParseError::GotNullPointer)
    ));
  }
}
