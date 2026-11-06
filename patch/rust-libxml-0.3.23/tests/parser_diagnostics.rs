use libxml::error::{StructuredError, XmlErrorLevel};
use libxml::parser::{MAX_DIAGNOSTICS, Parser, ParserOptions, XmlParseFailure};

fn strict() -> ParserOptions<'static> {
  ParserOptions {
    recover: false,
    no_net: true,
    ..Default::default()
  }
}

fn messages(diagnostics: &[StructuredError]) -> Vec<&str> {
  diagnostics
    .iter()
    .map(|error| error.message.as_deref().unwrap_or("").trim_end())
    .collect()
}

fn failed(
  result: Result<(libxml::tree::Document, Vec<StructuredError>), XmlParseFailure>,
) -> Vec<StructuredError> {
  match result {
    Err(XmlParseFailure::ParseFailed(diagnostics)) => diagnostics,
    Err(other) => panic!("expected ParseFailed, got {other:?}"),
    Ok(_) => panic!("expected ParseFailed, got a document"),
  }
}

#[test]
fn owned_diagnostic_survives_later_parses() {
  let parser = Parser::default();
  let diagnostics =
    failed(parser.parse_string_with_diagnostics("\n<?xml version=\"1.0\"?><root/>", strict()));
  let error = &diagnostics[0];
  assert_eq!(error.line, Some(2));
  assert!(error.col.is_some());
  let message = error.message.as_ref().unwrap();
  assert!(message.contains("declaration"), "{message}");
  let later = parser
    .parse_string_with_diagnostics("<root>&missing;</root>", strict())
    .err()
    .unwrap();
  assert!(later.to_string().contains("missing"));
  assert_ne!(message, &later.to_string());
  let (doc, diagnostics) = parser
    .parse_string_with_diagnostics("<root>ok</root>", strict())
    .unwrap();
  assert!(diagnostics.is_empty());
  assert_eq!(doc.get_root_element().unwrap().get_content(), "ok");
  assert!(message.contains("declaration"));
}

#[test]
/// All errors are kept, in order, not only the last one.
fn every_error_is_collected() {
  let diagnostics =
    failed(Parser::default().parse_string_with_diagnostics("<r>&first;&second;</r>", strict()));
  let messages = messages(&diagnostics);
  assert_eq!(messages.len(), 2, "{messages:?}");
  assert!(messages[0].contains("'first'"), "{messages:?}");
  assert!(messages[1].contains("'second'"), "{messages:?}");
  // Display reports the first error, without libxml2's trailing newline.
  let failure = XmlParseFailure::ParseFailed(diagnostics);
  assert!(failure.to_string().contains("'first'"));
  assert!(!failure.to_string().ends_with('\n'));
}

#[test]
/// With the default options (recover, no_error), the recovered document comes
/// with the errors that were recovered from.
fn recovered_xml_returns_document_and_errors() {
  let (doc, diagnostics) = Parser::default()
    .parse_string_with_diagnostics("<a><b></a>", ParserOptions::default())
    .unwrap();
  assert!(doc.get_root_element().is_some());
  let messages = messages(&diagnostics);
  assert!(messages[0].contains("mismatch"), "{messages:?}");
}

#[test]
fn html_errors_are_collected() {
  let (doc, diagnostics) = Parser::default_html()
    .parse_string_with_diagnostics("<p><b>x</p></i>", ParserOptions::default())
    .unwrap();
  assert!(doc.get_root_element().is_some());
  let messages = messages(&diagnostics);
  assert_eq!(messages.len(), 2, "{messages:?}");
  assert!(messages[1].contains("Unexpected end tag"), "{messages:?}");
  assert!(
    diagnostics
      .iter()
      .all(|e| matches!(e.level, XmlErrorLevel::Error))
  );
}

#[test]
fn clean_input_has_no_diagnostics() {
  for parser in [Parser::default(), Parser::default_html()] {
    let (_, diagnostics) = parser
      .parse_string_with_diagnostics("<html><body>ok</body></html>", strict())
      .unwrap();
    assert!(diagnostics.is_empty(), "{:?}", messages(&diagnostics));
  }
}

#[test]
fn contexts_do_not_share_errors_across_threads() {
  let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
  let threads: Vec<_> = (0..8)
    .map(|i| {
      let barrier = barrier.clone();
      std::thread::spawn(move || {
        let entity = format!("missing_{i}");
        let xml = format!("<root>&{entity};</root>");
        barrier.wait();
        for _ in 0..50 {
          let diagnostics = failed(Parser::default().parse_string_with_diagnostics(&xml, strict()));
          assert_eq!(diagnostics.len(), 1);
          assert!(diagnostics[0].message.as_ref().unwrap().contains(&entity));
        }
      })
    })
    .collect();
  for thread in threads {
    thread.join().unwrap();
  }
}

#[test]
fn invalid_encoding_names_return_errors_without_panicking() {
  let options = ParserOptions {
    encoding: Some("UTF-8\0bad"),
    ..strict()
  };
  let failure = Parser::default()
    .parse_string_with_diagnostics("<root/>", options)
    .err()
    .unwrap();
  assert!(std::error::Error::source(&failure).is_some());
  let XmlParseFailure::InvalidEncoding(error) = failure else {
    panic!("expected an encoding setup error");
  };
  assert_eq!(error.nul_position(), 5);
}

#[test]
fn xml_and_html_documents_outlive_their_parser_contexts() {
  for parser in [Parser::default(), Parser::default_html()] {
    let (document, _) = parser
      .parse_string_with_diagnostics(
        b"<html><body>caf\xe9</body></html>",
        ParserOptions {
          encoding: Some("ISO-8859-1"),
          ..strict()
        },
      )
      .unwrap();
    assert_eq!(document.get_root_element().unwrap().get_content(), "café");
  }
}

#[test]
/// Warnings are collected too, even with the default `no_warning: true`.
fn warnings_are_collected() {
  let (_, diagnostics) = Parser::default()
    .parse_string_with_diagnostics(r#"<r xmlns="not-absolute"/>"#, ParserOptions::default())
    .unwrap();
  assert_eq!(diagnostics.len(), 1, "{:?}", messages(&diagnostics));
  assert!(matches!(diagnostics[0].level, XmlErrorLevel::Warning));
  assert!(messages(&diagnostics)[0].contains("not absolute"));
}

#[test]
/// Hostile input cannot grow the diagnostics without bound: libxml2 before 2.13
/// reports one error per `&a;`.
fn diagnostics_are_capped() {
  let input = format!("<r>{}</r>", "&a;".repeat(10 * MAX_DIAGNOSTICS));
  let (_, diagnostics) = Parser::default()
    .parse_string_with_diagnostics(input, ParserOptions::default())
    .unwrap();
  assert_eq!(diagnostics.len(), MAX_DIAGNOSTICS);
}
