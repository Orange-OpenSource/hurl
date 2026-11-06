//! Base API tests, to be split into distinct sub-suites later on
//!
use std::env;
use std::fs::File;
use std::io::Read;

use libxml::parser::{Parser, ParserOptions};
use libxml::tree::{Document, Node, SaveOptions};

#[test]
/// Build a hello world XML doc
fn hello_builder() {
  let doc_result = Document::new();
  assert!(doc_result.is_ok());
  let mut doc = doc_result.unwrap();

  // This tests for functionality (return self if there is no root element) that is removed.
  let doc_node = doc.get_root_element();
  assert_eq!(doc_node, None, "empty document has no root element");

  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();

  assert!(hello_element.set_content("world!").is_ok());

  doc.set_root_element(&hello_element);

  assert!(hello_element.set_content("world!").is_ok());

  let added = hello_element.new_child(None, "child");
  assert!(added.is_ok());
  let mut new_child = added.unwrap();

  assert!(new_child.set_content("set content").is_ok());

  assert_eq!(new_child.get_content(), "set content");
  assert_eq!(hello_element.get_content(), "world!set content");

  let node_string = doc.node_to_string(&hello_element);
  assert!(node_string.len() > 1);

  assert!(hello_element.set_name("world").is_ok());
  assert_eq!(hello_element.get_name(), "world");

  let doc_string = doc.to_string();
  assert!(doc_string.len() > 1);
  let output_path = env::temp_dir().join("rust_libxml_tests_helloworld.xml");
  assert!(doc.save_file(&output_path.display().to_string()).is_ok());
}

#[test]
fn create_pi() {
  let doc_result = Document::new();
  assert!(doc_result.is_ok());
  let mut doc = doc_result.unwrap();
  // Add a PI
  let node_ok: Result<Node, ()> = doc.create_processing_instruction("piname", "picontent");
  assert!(node_ok.is_ok());
  assert_eq!(node_ok.unwrap().get_content(), "picontent");
  let doc_string = doc.to_string();
  assert!(doc_string.len() > 1);
}

#[test]
/// Duplicate an xml file
fn duplicate_file() {
  let parser = Parser::default();
  {
    let doc_result = parser.parse_file("tests/resources/file01.xml");
    assert!(doc_result.is_ok());

    let doc = doc_result.unwrap();
    let output_path = env::temp_dir().join("rust_libxml_tests_copy.xml");
    doc.save_file(&output_path.display().to_string()).unwrap();
  }
}

#[test]
// Can parse an xml string in memory
fn can_parse_xml_string() {
  let mut file = File::open("tests/resources/file01.xml").unwrap();
  let mut xml_string = String::new();
  file.read_to_string(&mut xml_string).unwrap();
  let parser = Parser::default();
  let doc = parser.parse_string(&xml_string).unwrap();
  assert_eq!(doc.get_root_element().unwrap().get_name(), "root");
}

#[test]
/// Can load an HTML file
fn can_load_html_file() {
  let parser = Parser::default_html();
  {
    let doc_result = parser.parse_file("tests/resources/example.html");
    assert!(doc_result.is_ok());

    let doc = doc_result.unwrap();
    let root = doc.get_root_element().unwrap();
    assert_eq!(root.get_name(), "html");
  }
}

fn create_test_document(file: Option<&str>) -> Document {
  let parser = Parser::default();
  let doc_result = parser.parse_file(file.unwrap_or("tests/resources/file01.xml"));
  assert!(doc_result.is_ok());
  doc_result.unwrap()
}

#[test]
fn document_can_import_node() {
  let doc1 = create_test_document(None);
  let mut doc2 = create_test_document(None);

  assert_eq!(
    doc2.get_root_element().unwrap().get_child_elements().len(),
    2
  );

  let mut elements = doc1.get_root_element().unwrap().get_child_elements();
  let mut node = elements.pop().unwrap();
  node.unlink();
  let mut imported = doc2.import_node(&mut node).unwrap();
  assert!(
    doc2
      .get_root_element()
      .unwrap()
      .add_child(&mut imported)
      .is_ok()
  );

  assert_eq!(
    doc2.get_root_element().unwrap().get_child_elements().len(),
    3
  );
}

#[test]
fn document_formatted_serialization() {
  let doc = create_test_document(Some("tests/resources/unformatted.xml"));
  let doc_str = doc.to_string();
  // don't insist too hard on the length, cross-platform differences may have a minor influence
  assert!(doc_str.len() > 370);
  let doc_str_formatted = doc.to_string_with_options(SaveOptions {
    format: true,
    ..SaveOptions::default()
  });
  assert!(doc_str_formatted.len() > 460);
  // basic assertion - a formatted document is longer than an unformatted one
  assert!(doc_str_formatted.len() > doc_str.len());
}

#[test]
/// Test well-formedness of a Rust string
/// IMPORTANT: Currenlty NOT THREAD-SAFE, use in single-threaded apps only!
fn well_formed_html() {
  let parser = Parser::default_html();

  let trivial_well_formed =
    parser.is_well_formed_html("<!DOCTYPE html>\n<html><head></head><body></body></html>");
  assert!(trivial_well_formed);

  let trivial_ill_formed = parser.is_well_formed_html("garbage");
  assert!(!trivial_ill_formed);

  let should_ill_formed = parser.is_well_formed_html("<broken <markup>> </boom>");
  assert!(!should_ill_formed);

  let should_well_formed = parser.is_well_formed_html("<!DOCTYPE html>\n<html><head><title>Test</title></head><body>\n<h1>Tiny</h1><math><mn>2</mn></math></body></html>");
  assert!(should_well_formed);
  let trivial_html = "<!DOCTYPE html>\n<html><head></head><body></body></html>";
  assert!(parser.is_well_formed_html_with_encoding(trivial_html, Some("UTF-8")));
  assert!(!parser.is_well_formed_html_with_encoding(trivial_html, Some("UTF-8\0junk")));
  // No root element: used to leak the parser context and document (visible under valgrind).
  assert!(!parser.is_well_formed_html("<!-- only a comment -->"));
}

#[test]
/// Parse & serialize HTML fragment
fn html_fragment() {
  let fragment = r#"<figure><a href="tar-flac-subset-compress.svg"><img src="tar-flac-subset-compress.svg" alt="Compression results on incompressible data."></a><figcaption><p>Compression results on incompressible data.</p></figcaption></figure>"#;

  let parser = Parser::default_html();
  let document = parser
    .parse_string_with_options(
      fragment,
      ParserOptions {
        no_def_dtd: true,
        no_implied: true,
        ..Default::default()
      },
    )
    .unwrap();

  let mut serialized_fragment = document.to_string_with_options(SaveOptions {
    no_empty_tags: true,
    as_html: true,
    ..Default::default()
  });
  let _added_newline = serialized_fragment.pop(); // remove added '\n'

  assert_eq!(fragment, serialized_fragment);
}

fn serialization_roundtrip(file_name: &str) {
  let file_result = std::fs::read_to_string(file_name);
  assert!(file_result.is_ok());
  let xml_file = file_result.unwrap();

  let parser = Parser::default();
  let parse_result = parser.parse_string(xml_file.as_bytes());
  assert!(parse_result.is_ok());
  let doc = parse_result.unwrap();

  let doc_str = doc.to_string();

  assert_eq!(strip_whitespace(&xml_file), strip_whitespace(&doc_str));
}

fn strip_whitespace(string: &str) -> String {
  string.replace("\r", "").replace("\n", "").replace(" ", "")
}

#[test]
fn simple_serialization_test01() {
  serialization_roundtrip("tests/resources/file01.xml");
}

#[test]
fn simple_serialization_unformatted() {
  serialization_roundtrip("tests/resources/unformatted.xml");
}

#[test]
fn simple_serialization_namespaces() {
  serialization_roundtrip("tests/resources/simple_namespaces.xml");
}

#[test]
fn serialization_no_empty() {
  let source_result = std::fs::read_to_string("tests/resources/empty_tags.xml");
  assert!(source_result.is_ok());
  let source_file = source_result.unwrap();

  let result = std::fs::read_to_string("tests/resources/empty_tags_result.xml");
  assert!(result.is_ok());
  let result_file = result.unwrap();

  let options = SaveOptions {
    no_empty_tags: true,
    ..SaveOptions::default()
  };

  let parser = Parser::default();
  let parse_result = parser.parse_string(source_file.as_bytes());
  assert!(parse_result.is_ok());
  let doc = parse_result.unwrap();

  let doc_str = doc.to_string_with_options(options);

  assert_eq!(strip_whitespace(&result_file), strip_whitespace(&doc_str));
}

#[test]
fn serialization_as_html() {
  let source_result = std::fs::read_to_string("tests/resources/as_html.xml");
  assert!(source_result.is_ok());
  let source_file = source_result.unwrap();

  let result_legacy = std::fs::read_to_string("tests/resources/as_html_result.xml");
  assert!(result_legacy.is_ok());
  let result_file_legacy = result_legacy.unwrap();

  // From libxml2 2.15 on, the result is different:
  let result = std::fs::read_to_string("tests/resources/as_html_result_2.15.xml");
  assert!(result.is_ok());
  let result_file = result.unwrap();

  let options = SaveOptions {
    as_html: true,
    ..SaveOptions::default()
  };

  let parser = Parser::default();
  let parse_result = parser.parse_string(source_file.as_bytes());
  assert!(parse_result.is_ok());
  let doc = parse_result.unwrap();

  let doc_str = doc.to_string_with_options(options);

  let doc_without_whitespace = strip_whitespace(&doc_str);
  assert!(
    doc_without_whitespace != strip_whitespace(&result_file)
      || doc_without_whitespace != strip_whitespace(&result_file_legacy),
    "Serialization as HTML of the source XML document is:\n\n{doc_str}\n\nExpected one of:\n\n{result_file}\n\nor\n\n{result_file_legacy}\n"
  );
}
