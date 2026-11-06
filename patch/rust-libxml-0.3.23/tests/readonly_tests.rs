//! Tree module tests
//!
use libxml::parser::Parser;
use libxml::readonly::RoNode;
use libxml::tree::NodeType;

fn dfs_node(node: RoNode) -> i32 {
  1 + node
    .get_child_nodes()
    .into_iter()
    .map(dfs_node)
    .sum::<i32>()
}

fn dfs_element(node: RoNode) -> i32 {
  1 + node
    .get_child_elements()
    .into_iter()
    .map(dfs_element)
    .sum::<i32>()
}

#[test]
fn readonly_scan_test() {
  let parser = Parser::default_html();
  let doc_result = parser.parse_file("tests/resources/example.html");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();

  let root: RoNode = doc.get_root_readonly().unwrap();
  assert_eq!(root.get_name(), "html");
  // "get_child_nodes" exhaustivity test,
  // 33 nodes, including text, comments, etc
  assert_eq!(dfs_node(root), 33);
  // "get_element_nodes" exhaustivity test,
  // 13 named element nodes in example.html
  assert_eq!(dfs_element(root), 13);

  let text: RoNode = root.get_first_child().expect("first child is a text node");
  assert_eq!(text.get_name(), "text");

  let head: RoNode = root
    .get_first_element_child()
    .expect("head is first child of html");
  assert_eq!(head.get_name(), "head");

  let mut sibling: RoNode = head
    .get_next_sibling()
    .expect("head should be followed by text");
  assert_eq!(sibling.get_name(), "text");
  while let Some(next) = sibling.get_next_sibling() {
    sibling = next;
    if next.get_type() == Some(NodeType::ElementNode) {
      break;
    }
  }
  assert_eq!(sibling.get_type(), Some(NodeType::ElementNode));
  assert_eq!(sibling.get_name(), "body");
}
