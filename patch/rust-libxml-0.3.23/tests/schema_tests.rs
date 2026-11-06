//!
//! Test Schema Loading, XML Validating
//!
use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;

use libxml::parser::Parser;

static NOTE_SCHEMA: &str = r#"<?xml version="1.0"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:element name="note">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="to" type="xs:string"/>
        <xs:element name="from" type="xs:string"/>
        <xs:element name="heading" type="xs:string"/>
        <xs:element name="body" type="xs:string"/>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
</xs:schema>
"#;

static STOCK_SCHEMA: &str = r#"<?xml version="1.0"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:element name="stock">
    <xs:complexType>
      <xs:sequence maxOccurs="unbounded">
        <xs:element name="sample">
          <xs:complexType>
            <xs:all>
              <xs:element name="date" type="xs:date"/>
              <xs:element name="price" type="xs:float"/>
            </xs:all>
          </xs:complexType>
        </xs:element>
      </xs:sequence>
      <xs:attribute name="ticker" type="xs:string" use="required"/>
      <xs:attribute name="exchange" type="xs:string" use="required"/>
    </xs:complexType>
  </xs:element>
</xs:schema>
"#;

static VALID_NOTE_XML: &str = r#"<?xml version="1.0"?>
<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#;

static INVALID_NOTE_XML: &str = r#"<?xml version="1.0"?>
<note>
  <bad>Tove</bad>
  <another>Jani</another>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#;

static INVALID_STOCK_XML: &str = r#"<?xml version="1.0"?>
<stock junkAttribute="foo">
  <sample>
    <date>2014-01-01</date>
    <price>NOT A NUMBER</price>
  </sample>
  <sample>
    <date>2014-01-02</date>
    <price>540.98</price>
  </sample>
  <sample>
    <date>NOT A DATE</date>
    <price>543.93</price>
  </sample>
</stock
"#;

// TODO: This test has revealed SchemaParserContext+SchemaValidationContext are not safe for
//       multi-threaded use in libxml >=2.12, at least not as currently implemented.
//       while it still reliably succeeds single-threaded, new implementation is needed to use
//       these in a parallel setting.
#[test]
fn schema_from_string() {
  let xml = Parser::default()
    .parse_string(VALID_NOTE_XML)
    .expect("Expected to be able to parse XML Document from string");

  let mut xsdparser = SchemaParserContext::from_buffer(NOTE_SCHEMA);
  let xsd = SchemaValidationContext::from_parser(&mut xsdparser);

  if let Err(errors) = xsd {
    for err in &errors {
      eprintln!("{}", err.message.as_ref().unwrap());
    }
    panic!("Failed to parse schema with {} errors", errors.len());
  }

  let mut xsdvalidator = xsd.unwrap();

  // loop over more than one validation to test for leaks in the error handling callback interactions
  for _ in 0..5 {
    if let Err(errors) = xsdvalidator.validate_document(&xml) {
      for err in &errors {
        eprintln!("{}", err.message.as_ref().unwrap());
      }

      panic!("Invalid XML accoding to XSD schema");
    }
  }
}

#[test]
fn schema_from_string_generates_errors() {
  let xml = Parser::default()
    .parse_string(INVALID_NOTE_XML)
    .expect("Expected to be able to parse XML Document from string");

  let mut xsdparser = SchemaParserContext::from_buffer(NOTE_SCHEMA);
  let xsd = SchemaValidationContext::from_parser(&mut xsdparser);

  if let Err(errors) = xsd {
    for err in &errors {
      eprintln!("{}", err.message.as_ref().unwrap());
    }
    panic!("Failed to parse schema with {} errors", errors.len());
  }

  let mut xsdvalidator = xsd.unwrap();
  for _ in 0..5 {
    if let Err(errors) = xsdvalidator.validate_document(&xml) {
      for err in &errors {
        assert_eq!(
          "Element 'bad': This element is not expected. Expected is ( to ).\n",
          err.message.as_ref().unwrap()
        );
      }
    }
  }
}

#[test]
fn schema_from_string_reports_unique_errors() {
  let xml = Parser::default()
    .parse_string(INVALID_STOCK_XML)
    .expect("Expected to be able to parse XML Document from string");

  let mut xsdparser = SchemaParserContext::from_buffer(STOCK_SCHEMA);
  let xsd = SchemaValidationContext::from_parser(&mut xsdparser);

  if let Err(errors) = xsd {
    for err in &errors {
      eprintln!("{}", err.message.as_ref().unwrap());
    }

    panic!("Failed to parse schema with {} errors", errors.len());
  }

  let mut xsdvalidator = xsd.unwrap();
  for _ in 0..5 {
    if let Err(errors) = xsdvalidator.validate_document(&xml) {
      assert_eq!(errors.len(), 5);
      let expected_errors = vec![
        "Element 'stock', attribute 'junkAttribute': The attribute 'junkAttribute' is not allowed.\n",
        "Element 'stock': The attribute 'ticker' is required but missing.\n",
        "Element 'stock': The attribute 'exchange' is required but missing.\n",
        "Element 'price': 'NOT A NUMBER' is not a valid value of the atomic type 'xs:float'.\n",
        "Element 'date': 'NOT A DATE' is not a valid value of the atomic type 'xs:date'.\n",
      ];
      for err_msg in expected_errors {
        assert!(
          errors
            .iter()
            .any(|err| err.message.as_ref().unwrap() == err_msg),
          "Expected error message {} was not found",
          err_msg
        );
      }
    }
  }
}

#[test]
fn schema_entity_reference_returns_error_instead_of_panicking() {
  let mut parser = SchemaParserContext::from_buffer(NOTE_SCHEMA);
  let mut context = SchemaValidationContext::from_parser(&mut parser).unwrap();
  let document = Parser::default().parse_string(
    r#"<!DOCTYPE note [<!ENTITY to "<to>Tove</to>">]><note>&to;<from>Jani</from><heading>Reminder</heading><body>Text</body></note>"#,
  ).unwrap();
  let errors = context.validate_document(&document).unwrap_err();
  assert!(!errors.is_empty());
  let valid = Parser::default().parse_string(VALID_NOTE_XML).unwrap();
  assert!(context.validate_document(&valid).is_ok());
}
