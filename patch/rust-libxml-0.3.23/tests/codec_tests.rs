//! BOM parsing tests
//!
use libxml::parser::{Parser, XmlParseError};
use libxml::tree::Document;
use std::fs;
use std::io;
use std::io::prelude::*;

// HELPERS

///Read the entire file to a byte vector. Similar to read_to_string with
///no encoding assumption.
fn read_to_end(path: &str) -> io::Result<Vec<u8>> {
  let mut buffer = Vec::new();
  let mut file = fs::File::open(path)?;
  file.read_to_end(&mut buffer)?;
  Ok(buffer)
}

///Generate a unittest for a document result from parsing a variant of file01.
fn file01_test(doc_result: Result<Document, XmlParseError>) {
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let root = doc.get_root_element().unwrap();

  // Tests
  let root_children = root.get_child_nodes();
  assert_eq!(root_children.len(), 5, "file01 root has five child nodes");
  let mut element_children = root.get_child_elements();
  assert_eq!(
    element_children.len(),
    2,
    "file01 root has two child elements"
  );
  assert_eq!(element_children.pop().unwrap().get_name(), "child");
  assert_eq!(element_children.pop().unwrap().get_name(), "child");
  assert!(element_children.is_empty());
}

///Run a test for both the file and the path of file01.
fn run_test(path: &str) {
  let parser = Parser::default();
  file01_test(parser.parse_file(path));

  let input = read_to_end(path).unwrap();
  file01_test(parser.parse_string(&input));
}

// ENCODINGS

#[test]
fn utf8_test() {
  run_test("tests/resources/file01.xml");
}

#[test]
fn utf16le_test() {
  run_test("tests/resources/file01_utf16le.xml");
}

#[test]
fn utf16be_test() {
  run_test("tests/resources/file01_utf16be.xml");
}

// BOM

#[test]
fn utf8_bom_test() {
  run_test("tests/resources/file01_utf8_bom.xml");
}

#[test]
fn utf16le_bom_test() {
  run_test("tests/resources/file01_utf16le_bom.xml");
}

#[test]
fn utf16be_bom_test() {
  run_test("tests/resources/file01_utf16be_bom.xml");
}

// UNICODE PATHS

#[test]
fn nonbmp_path_test() {
  run_test("tests/resources/file01_🔥🔥🔥.xml");
}
