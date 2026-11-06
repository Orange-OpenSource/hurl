use libxml::parser::Parser;
use libxml::tree::c14n::{CanonicalizationMode, CanonicalizationOptions};

fn assert_eq_lines(seen: &str, expected: &str) {
  let lines_iter = seen.lines().zip(expected.lines());

  for (seen_line, expected_line) in lines_iter {
    assert_eq!(seen_line, expected_line);
  }

  assert_eq!(seen.lines().count(), expected.lines().count());
}

fn canonicalize_xml(input: &str, opts: CanonicalizationOptions) -> String {
  let parser = Parser::default();
  let doc = parser.parse_string(input).unwrap();

  doc.canonicalize(opts, None).unwrap()
}

#[test]
fn canonical_1_1_example_3_1_no_comment() {
  let input = include_str!("resources/canonical_1_1/3_1_input.xml");
  let expected = include_str!("resources/canonical_1_1/3_1_output_no_comment.xml");

  let canonicalized = canonicalize_xml(
    input,
    CanonicalizationOptions {
      mode: CanonicalizationMode::Canonical1_1,
      with_comments: false,
      inclusive_ns_prefixes: vec![],
    },
  );

  assert_eq_lines(&canonicalized, expected);
}

#[test]
fn canonical_1_1_example_3_2() {
  let input = include_str!("resources/canonical_1_1/3_2_input.xml");
  let expected = include_str!("resources/canonical_1_1/3_2_output.xml");

  let canonicalized = canonicalize_xml(
    input,
    CanonicalizationOptions {
      mode: CanonicalizationMode::Canonical1_1,
      with_comments: true,
      inclusive_ns_prefixes: vec![],
    },
  );

  // for some reason, we get a stray \n at end of file :/
  assert_eq_lines(&canonicalized, expected.trim())
}

#[test]
fn canonical_exclusive_example_1() {
  let input = include_str!("resources/canonical_exclusive/1_input.xml");
  let expected = include_str!("resources/canonical_exclusive/1_output.xml");

  let canonicalized = canonicalize_xml(
    input,
    CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      with_comments: true,
      inclusive_ns_prefixes: vec![],
    },
  );

  // for some reason, we get a stray \n at end of file :/
  assert_eq_lines(&canonicalized, expected.trim())
}

#[test]
fn canonical_exclusive_example_2() {
  let input = include_str!("resources/canonical_exclusive/2_input.xml");
  let expected = include_str!("resources/canonical_exclusive/2_output.xml");

  let canonicalized = canonicalize_xml(
    input,
    CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      with_comments: true,
      inclusive_ns_prefixes: ["stay1".to_string(), "stay2".to_string()].to_vec(),
    },
  );

  // for some reason, we get a stray \n at end of file :/
  assert_eq_lines(&canonicalized, expected.trim())
}

#[test]
fn test_c14n_node() {
  let xml = "<a><b><c></c></b></a>";
  let doc = Parser::default().parse_string(xml).unwrap();
  let mut node = doc.as_node().findnodes("//b").unwrap().pop().unwrap();

  let c14n = node.canonicalize(opts()).unwrap();

  assert_eq!("<b><c></c></b>", c14n)
}

#[test]
fn test_c14n_modes() {
  // http://www.w3.org/TR/xml-exc-c14n/#sec-Enveloping

  let doc1 = Parser::default()
    .parse_string(
      r#"
          <n0:local xmlns:n0="http://foobar.org" xmlns:n3="ftp://example.org">
            <n1:elem2 xmlns:n1="http://example.net" xml:lang="en">
              <n3:stuff xmlns:n3="ftp://example.org"/>
            </n1:elem2>
          </n0:local>

      "#,
    )
    .unwrap();

  let mut node1 = doc1
    .as_node()
    .at_xpath("//n1:elem2", &[("n1", "http://example.net")])
    .unwrap()
    .unwrap();

  let doc2 = Parser::default()
    .parse_string(
      r#"
    <n2:pdu xmlns:n1="http://example.com"
               xmlns:n2="http://foo.example"
               xmlns:n4="http://foo.example"
               xml:lang="fr"
               xml:space="retain">
      <n1:elem2 xmlns:n1="http://example.net" xml:lang="en">
        <n3:stuff xmlns:n3="ftp://example.org"/>
        <n4:stuff />
      </n1:elem2>
    </n2:pdu>
    "#,
    )
    .unwrap();
  let mut node2 = doc2
    .as_node()
    .at_xpath("//n1:elem2", &[("n1", "http://example.net")])
    .unwrap()
    .unwrap();

  let expected = r#"
    <n1:elem2 xmlns:n0="http://foobar.org" xmlns:n1="http://example.net" xmlns:n3="ftp://example.org" xml:lang="en">
              <n3:stuff></n3:stuff>
            </n1:elem2>
  "#.trim();
  let c14n = node1.canonicalize(opts()).unwrap();
  assert_eq_lines(expected, &c14n);

  let expected = r#"
    <n1:elem2 xmlns:n1="http://example.net" xmlns:n2="http://foo.example" xmlns:n4="http://foo.example" xml:lang="en" xml:space="retain">
        <n3:stuff xmlns:n3="ftp://example.org"></n3:stuff>
        <n4:stuff></n4:stuff>
      </n1:elem2>
  "#.trim();
  let c14n = node2.canonicalize(opts()).unwrap();
  assert_eq_lines(expected, &c14n);

  let opts = CanonicalizationOptions {
    mode: CanonicalizationMode::Canonical1_0,
    ..Default::default()
  };
  let c14n = node2.canonicalize(opts).unwrap();
  assert_eq_lines(expected, &c14n);

  let expected = r#"
    <n1:elem2 xmlns:n1="http://example.net" xml:lang="en">
              <n3:stuff xmlns:n3="ftp://example.org"></n3:stuff>
            </n1:elem2>
  "#
  .trim();
  let c14n = node1
    .canonicalize(CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      ..Default::default()
    })
    .unwrap();

  assert_eq_lines(expected, &c14n);

  let expected = r#"
    <n1:elem2 xmlns:n1="http://example.net" xml:lang="en">
        <n3:stuff xmlns:n3="ftp://example.org"></n3:stuff>
        <n4:stuff xmlns:n4="http://foo.example"></n4:stuff>
      </n1:elem2>
  "#
  .trim();
  let c14n = node2
    .canonicalize(CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      ..Default::default()
    })
    .unwrap();
  assert_eq_lines(expected, &c14n);

  let expected = r#"
    <n1:elem2 xmlns:n1="http://example.net" xmlns:n2="http://foo.example" xml:lang="en">
        <n3:stuff xmlns:n3="ftp://example.org"></n3:stuff>
        <n4:stuff xmlns:n4="http://foo.example"></n4:stuff>
      </n1:elem2>
  "#
  .trim();
  let c14n = node2
    .canonicalize(CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      inclusive_ns_prefixes: vec!["n2".into()],
      ..Default::default()
    })
    .unwrap();
  assert_eq_lines(expected, &c14n);

  let expected = r#"
    <n1:elem2 xmlns:n1="http://example.net" xmlns:n2="http://foo.example" xmlns:n4="http://foo.example" xml:lang="en">
        <n3:stuff xmlns:n3="ftp://example.org"></n3:stuff>
        <n4:stuff></n4:stuff>
      </n1:elem2>
  "#.trim();
  let c14n = node2
    .canonicalize(CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      inclusive_ns_prefixes: vec!["n2".into(), "n4".into()],
      ..Default::default()
    })
    .unwrap();
  assert_eq_lines(expected, &c14n);

  let expected = r#"
    <n1:elem2 xmlns:n1="http://example.net" xmlns:n2="http://foo.example" xmlns:n4="http://foo.example" xml:lang="en" xml:space="retain">
        <n3:stuff xmlns:n3="ftp://example.org"></n3:stuff>
        <n4:stuff></n4:stuff>
      </n1:elem2>
  "#.trim();
  let c14n = node2
    .canonicalize(CanonicalizationOptions {
      mode: CanonicalizationMode::Canonical1_1,
      ..Default::default()
    })
    .unwrap();
  assert_eq_lines(expected, &c14n);
}

fn opts() -> CanonicalizationOptions {
  CanonicalizationOptions {
    mode: CanonicalizationMode::Canonical1_1,
    with_comments: false,
    inclusive_ns_prefixes: vec![],
  }
}
