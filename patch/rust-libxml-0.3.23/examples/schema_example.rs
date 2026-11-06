//!
//! Example Usage of XSD Schema Validation
//!
use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;

use libxml::parser::Parser;

fn main() {
  let xml = Parser::default()
    .parse_file("tests/resources/schema.xml")
    .expect("Expected to be able to parse XML Document from file");

  let mut xsdparser = SchemaParserContext::from_file("tests/resources/schema.xsd");
  let xsd = SchemaValidationContext::from_parser(&mut xsdparser);

  if let Err(errors) = xsd {
    for err in &errors {
      println!("{}", err.message.as_ref().unwrap());
    }

    panic!("Failed to parse schema");
  }

  let mut xsd = xsd.unwrap();

  if let Err(errors) = xsd.validate_document(&xml) {
    for err in &errors {
      println!("{}", err.message.as_ref().unwrap());
    }

    panic!("Invalid XML accoding to XSD schema");
  }
}
