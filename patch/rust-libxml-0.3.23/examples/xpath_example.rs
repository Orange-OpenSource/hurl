use libxml::parser::Parser;
use libxml::xpath::Context;

fn main() {
  let parser = Parser::default();
  let doc = parser.parse_file("tests/resources/file01.xml").unwrap();
  let context = Context::new(&doc).unwrap();
  let result = context.evaluate("//child/text()").unwrap();

  for node in &result.get_nodes_as_vec() {
    println!("Found: {}", node.get_content());
  }
}
