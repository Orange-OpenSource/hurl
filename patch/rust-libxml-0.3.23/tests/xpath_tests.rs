//! xpath module tests
//!

use libxml::parser::Parser;
use libxml::xpath::Context;

#[test]
/// Test the evaluation of an xpath expression yields the correct number of nodes
fn xpath_result_number_correct() {
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let context = Context::new(&doc).unwrap();

  let result1 = context.evaluate("//child").unwrap();
  assert_eq!(result1.get_number_of_nodes(), 2);
  assert_eq!(result1.get_nodes_as_vec().len(), 2);

  let result2 = context.evaluate("//nonexistent").unwrap();
  assert_eq!(result2.get_number_of_nodes(), 0);
  assert_eq!(result2.get_nodes_as_vec().len(), 0);
}

#[test]
/// Test xpath with namespaces
fn xpath_with_namespaces() {
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/simple_namespaces.xml");
  assert!(doc_result.is_ok());

  let doc = doc_result.unwrap();
  let context = Context::new(&doc).unwrap();
  assert!(
    context
      .register_namespace("h", "http://example.com/ns/hello")
      .is_ok()
  );
  assert!(
    context
      .register_namespace("f", "http://example.com/ns/farewell")
      .is_ok()
  );
  assert!(
    context
      .register_namespace("r", "http://example.com/ns/root")
      .is_ok()
  );
  let result_h_td = context.evaluate("//h:td").unwrap();
  assert_eq!(result_h_td.get_number_of_nodes(), 3);
  assert_eq!(result_h_td.get_nodes_as_vec().len(), 3);

  let result_h_table = context.evaluate("//h:table").unwrap();
  assert_eq!(result_h_table.get_number_of_nodes(), 2);
  assert_eq!(result_h_table.get_nodes_as_vec().len(), 2);

  let result_f_footer = context.evaluate("//f:footer").unwrap();
  assert_eq!(result_f_footer.get_number_of_nodes(), 2);
  assert_eq!(result_f_footer.get_nodes_as_vec().len(), 2);

  let result_r = context.evaluate("//r:*").unwrap();
  assert_eq!(result_r.get_number_of_nodes(), 1);
  assert_eq!(result_r.get_nodes_as_vec().len(), 1);

  let result_h = context.evaluate("//h:*").unwrap();
  assert_eq!(result_h.get_number_of_nodes(), 7);
  assert_eq!(result_h.get_nodes_as_vec().len(), 7);

  let result_f = context.evaluate("//f:*").unwrap();
  assert_eq!(result_f.get_number_of_nodes(), 4);
  assert_eq!(result_f.get_nodes_as_vec().len(), 4);

  let result_all = context.evaluate("//*").unwrap();
  assert_eq!(result_all.get_number_of_nodes(), 12);
  assert_eq!(result_all.get_nodes_as_vec().len(), 12);

  let result_h_table = context.evaluate("//table").unwrap();
  assert_eq!(result_h_table.get_number_of_nodes(), 0);
  assert_eq!(result_h_table.get_nodes_as_vec().len(), 0);

  assert!(doc.as_node().recursively_remove_namespaces().is_ok());
  let result_h_table = context.evaluate("//table").unwrap();
  assert_eq!(result_h_table.get_number_of_nodes(), 2);
  assert_eq!(result_h_table.get_nodes_as_vec().len(), 2);
}

#[test]
/// Test that an xpath expression finds the correct node and
/// that the class names are interpreted correctly.
fn class_names() {
  let parser = Parser::default_html();
  let doc_result = parser.parse_file("tests/resources/file02.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let context = Context::new(&doc).unwrap();

  let p_result = context.evaluate("/html/body/p");
  assert!(p_result.is_ok());
  let p = p_result.unwrap();
  assert_eq!(p.get_number_of_nodes(), 1);

  let node = &p.get_nodes_as_vec()[0];
  let names = node.get_class_names();
  assert_eq!(names.len(), 2);
  assert!(names.contains("paragraph"));
  assert!(names.contains("important"));
  assert!(!names.contains("nonsense"));
}

#[test]
/// Test that an xpath string() function processed correctly
fn xpath_string_function() {
  let parser = Parser::default_html();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let context = Context::new(&doc).unwrap();

  let p_result = context.evaluate("string(//root//child[1]/@attribute)");
  assert!(p_result.is_ok());
  let p = p_result.unwrap();
  // Not a node really
  assert_eq!(p.get_number_of_nodes(), 0);
  let content = p.to_string();
  assert_eq!(content, "value");
}

#[test]
/// Test that the dual findnodes interfaces are operational
fn findnodes_interfaces() {
  let parser = Parser::default_html();
  let doc_result = parser.parse_file("tests/resources/file02.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();

  // Xpath interface
  let mut context = Context::new(&doc).unwrap();
  let body = context.evaluate("/html/body").unwrap().get_nodes_as_vec();
  let p_result = context.findnodes("p", body.first());
  assert!(p_result.is_ok());
  let p = p_result.unwrap();
  assert_eq!(p.len(), 1);

  // Node interface
  let body_node = body.first().unwrap();
  let p2_result = body_node.findnodes("p");
  assert!(p2_result.is_ok());
  let p2 = p2_result.unwrap();
  assert_eq!(p2.len(), 1);
}

#[test]
/// Clone is safe on Context objects
fn safe_context_clone() {
  let parser = Parser::default_html();
  let doc_result = parser.parse_file("tests/resources/file02.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();

  // Xpath interface
  let context = Context::new(&doc).unwrap();
  let body = context.evaluate("/html/body").unwrap().get_nodes_as_vec();
  assert_eq!(body.len(), 1);
  let context2 = context.clone();
  let body2 = context2.evaluate("/html/body").unwrap().get_nodes_as_vec();
  assert_eq!(body2.len(), 1);
}

#[test]
fn cleanup_safely_unlinked_xpath_nodes() {
  let p = Parser::default();
  let doc_result = p.parse_string(r##"<?xml version="1.0" standalone="no"?> <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.0//EN" "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd" > <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%">
<defs >
<font horiz-adv-x="874" ><font-face
    font-family="Luxi Serif"
    units-per-em="2048"
    panose-1="2 2 7 3 7 0 0 0 0 4"
    ascent="2073"
    descent="-432"
    alphabetic="0" />
<missing-glyph horiz-adv-x="512" d="M51 0V1480H461V0H51ZM410 51V1429H102V51H410Z" />
<glyph unicode=" " glyph-name="space" horiz-adv-x="512" />
<c g1="one" g2="X" k="32" />
</font>
</defs>
</svg>
"##);
  assert!(doc_result.is_ok(), "successfully parsed SVG snippet");
  let doc = doc_result.unwrap();
  let mut xpath = libxml::xpath::Context::new(&doc).unwrap();
  xpath
    .register_namespace("svg", "http://www.w3.org/2000/svg")
    .unwrap();
  for mut k in xpath.findnodes("//svg:c", None).unwrap() {
    k.unlink_node();
  }
  drop(xpath);
  drop(doc);
  // No assertion — reaching this point means the drops did not crash.
}

#[test]
fn xpath_find_string_values() {
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/ids.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let mut xpath = libxml::xpath::Context::new(&doc).unwrap();
  match doc.get_root_element() {
    Some(root) => {
      let tests = root.get_child_elements();
      let empty_test = &tests[0];
      let ids_test = &tests[1];
      let empty_values = xpath.findvalues(".//@xml:id", Some(empty_test));
      assert_eq!(empty_values, Ok(Vec::new()));
      let ids_values = xpath.findvalues(".//@xml:id", Some(ids_test));
      let expected_ids = Ok(vec![
        String::from("start"),
        String::from("mid"),
        String::from("end"),
      ]);
      assert_eq!(ids_values, expected_ids);
      let node_ids_values = ids_test.findvalues(".//@xml:id");
      assert_eq!(node_ids_values, expected_ids);
    }
    _ => {
      panic!("Document fails to obtain root!");
    }
  }
}

/// Tests for checking xpath well-formedness
mod compile_tests {
  use libxml::xpath::is_well_formed_xpath;

  #[test]
  fn can_compile_an_xpath() {
    let compiles = is_well_formed_xpath("//a");
    assert!(compiles);
  }

  #[test]
  fn invalid_xpath_does_not_compile() {
    let compiles = is_well_formed_xpath("//a[but invalid]");
    assert!(!compiles);
  }
}

/// Coverage for the `*_checked` evaluators and `XPathError` — the error-surfacing
/// API added for very large documents. The `Ok` path is exercised throughout the
/// tests above (the thin `evaluate`/`node_evaluate` wrappers delegate to their
/// `_checked` variant); these pin the previously-untested `Err`/classification path.
mod checked_evaluators {
  use libxml::parser::Parser;
  use libxml::xpath::{Context, XPathError};

  /// An invalid expression makes libxml2 return NULL; `evaluate_checked` turns
  /// that into a populated `XPathError` (message + non-zero code), while the thin
  /// `evaluate` still collapses it to `Err(())`.
  #[test]
  fn evaluate_checked_surfaces_invalid_expression() {
    let doc = Parser::default().parse_string("<r><c/></r>").unwrap();
    let ctx = Context::new(&doc).unwrap();

    let err = ctx
      .evaluate_checked("///")
      .expect_err("`///` is not a valid XPath expression");
    assert!(err.message.is_some(), "expected a libxml2 error message");
    assert_ne!(err.code, 0, "expected a non-zero libxml2 error code");
    assert!(
      !err.is_nodeset_limit(),
      "a syntax error is not the nodeset ceiling"
    );

    assert!(ctx.evaluate("///").is_err(), "thin wrapper still errors");
    // A valid expression still succeeds through the checked path.
    assert_eq!(
      ctx.evaluate_checked("//c").unwrap().get_number_of_nodes(),
      1
    );
  }

  /// `is_nodeset_limit` recognizes exactly libxml2's growing-nodeset / OOM
  /// messages and nothing else. Constructed directly (the fields are public)
  /// because materializing a real >10M node-set in a unit test is impractical.
  #[test]
  fn is_nodeset_limit_classifies_libxml2_messages() {
    let mk = |m: Option<&str>| XPathError {
      message: m.map(str::to_string),
      code: 0,
      domain: 0,
    };
    assert!(mk(Some("XPath error : growing nodeset hit limit")).is_nodeset_limit());
    assert!(mk(Some("Memory allocation failed : growing nodeset")).is_nodeset_limit());
    assert!(!mk(Some("Invalid expression\n")).is_nodeset_limit());
    assert!(!mk(None).is_nodeset_limit());
  }

  /// The node-relative checked evaluators resolve against the given context node
  /// (both the `&Node` and `RoNode` entry points) and surface errors the same way.
  #[test]
  fn node_checked_evaluators_ok_and_err() {
    let doc = Parser::default()
      .parse_file("tests/resources/file01.xml")
      .unwrap();
    let ctx = Context::new(&doc).unwrap();

    let root = doc.get_root_element().unwrap();
    assert_eq!(
      ctx
        .node_evaluate_checked("child", &root)
        .unwrap()
        .get_number_of_nodes(),
      2,
      "two <child> nodes are reachable from the root"
    );
    assert!(ctx.node_evaluate_checked("///", &root).is_err());

    let ro_root = doc.get_root_readonly().unwrap();
    assert_eq!(
      ctx
        .node_evaluate_readonly_checked("child", ro_root)
        .unwrap()
        .get_number_of_nodes(),
      2
    );
    assert!(ctx.node_evaluate_readonly_checked("///", ro_root).is_err());
  }
}

/// A compiled `xpath::Expression` evaluates as its source string does, reusable across nodes, contexts and documents.
mod compiled_expressions {
  use libxml::parser::Parser;
  use libxml::xpath::{Context, Expression};

  /// Every evaluator gives the same result for an expression compiled once as for its source string.
  #[test]
  fn compiled_matches_string_evaluation() {
    let doc = Parser::default()
      .parse_file("tests/resources/file01.xml")
      .unwrap();
    let mut ctx = Context::new(&doc).unwrap();
    let root = doc.get_root_element().unwrap();
    for xpath in [
      "/root",
      "//child",
      "child",
      "//*[@attribute]",
      "count(//child)",
      "*/text()",
    ] {
      let expression = Expression::compile(xpath).unwrap();
      for node in [None, Some(&root)] {
        assert_eq!(
          ctx.findnodes_compiled(&expression, node).unwrap(),
          ctx.findnodes(xpath, node).unwrap(),
          "findnodes {xpath:?} at {node:?}"
        );
        assert_eq!(
          ctx.findvalues_compiled(&expression, node).unwrap(),
          ctx.findvalues(xpath, node).unwrap(),
          "findvalues {xpath:?} at {node:?}"
        );
        assert_eq!(
          ctx.findvalue_compiled(&expression, node).unwrap(),
          ctx.findvalue(xpath, node).unwrap(),
          "findvalue {xpath:?} at {node:?}"
        );
      }
    }
  }

  /// As `node_evaluate`, a compiled node evaluation leaves its node as the context's current node, which a later
  /// evaluation at no node starts from.
  #[test]
  fn compiled_keeps_the_context_node_as_strings_do() {
    let doc = Parser::default()
      .parse_file("tests/resources/file01.xml")
      .unwrap();
    let ctx = Context::new(&doc).unwrap();
    let root = doc.get_root_element().unwrap();
    let child = Expression::compile("child").unwrap();
    ctx.node_evaluate_compiled(&child, &root).unwrap();
    let after_compiled = ctx.evaluate_compiled(&child).unwrap().get_number_of_nodes();
    ctx.node_evaluate("child", &root).unwrap();
    let after_string = ctx.evaluate("child").unwrap().get_number_of_nodes();
    assert_eq!(after_compiled, after_string);
    assert_eq!(after_compiled, 2);
  }

  /// A namespace prefix is resolved when the expression runs, from the context it runs in.
  #[test]
  fn compiled_resolves_prefixes_at_evaluation() {
    let doc = Parser::default()
      .parse_string(r#"<r xmlns="urn:a"><x/><x/></r>"#)
      .unwrap();
    let expression = Expression::compile("//a:x").unwrap();
    let plain = Context::new(&doc).unwrap();
    assert!(plain.findnodes_compiled(&expression, None).is_err());
    let registered = Context::new(&doc).unwrap();
    registered.register_namespace("a", "urn:a").unwrap();
    assert_eq!(
      registered
        .findnodes_compiled(&expression, None)
        .unwrap()
        .len(),
      2
    );
  }

  /// One expression evaluates against the contexts of different documents.
  #[test]
  fn compiled_is_reused_across_documents() {
    let expression = Expression::compile("count(//child)").unwrap();
    let one = Parser::default()
      .parse_file("tests/resources/file01.xml")
      .unwrap();
    let two = Parser::default().parse_string("<r><child/></r>").unwrap();
    let count = |doc: &libxml::tree::Document| {
      Context::new(doc)
        .unwrap()
        .findvalue_compiled(&expression, None)
        .unwrap()
    };
    assert_eq!(count(&one), "2");
    assert_eq!(count(&two), "1");
  }

  /// A malformed expression fails to compile with libxml2's structured error; a NUL byte is refused, not a panic.
  #[test]
  fn compile_reports_malformed_expressions() {
    let error = Expression::compile("//a[").unwrap_err();
    assert!(error.message.is_some(), "a structured message: {error}");
    assert!(Expression::compile("///").is_err());
    assert!(Expression::compile("a\0b").is_err());
  }

  /// A compiled node evaluation at a node of another document fails as `node_evaluate` does.
  #[test]
  fn compiled_refuses_a_node_of_another_document() {
    let one = Parser::default()
      .parse_file("tests/resources/file01.xml")
      .unwrap();
    let two = Parser::default().parse_string("<r/>").unwrap();
    let ctx = Context::new(&one).unwrap();
    let foreign = two.get_root_element().unwrap();
    let expression = Expression::compile("*").unwrap();
    assert!(ctx.node_evaluate_compiled(&expression, &foreign).is_err());
    assert!(ctx.node_evaluate("*", &foreign).is_err());
  }
}
