//! Tree module tests
//!

use libxml::parser::Parser;
use libxml::tree::{Document, Namespace, Node, NodeType};

#[test]
/// Root node and first child of root node are different
/// (There is a tiny chance this might fail for a correct program)
fn child_of_root_has_different_hash() {
  let parser = Parser::default();
  {
    let doc_result = parser.parse_file("tests/resources/file01.xml");
    assert!(doc_result.is_ok());
    let doc = doc_result.unwrap();
    let root = doc.get_root_element().unwrap();
    assert!(!root.is_text_node());
    match root.get_first_child() {
      Some(child) => {
        assert!(root != child);
      }
      _ => {
        unreachable!("test failed - first child doesn't exist");
      }
    }
    // same check with last child
    match root.get_last_child() {
      Some(child) => {
        assert!(root != child);
      }
      _ => {
        unreachable!("test failed - last child doesn't exist");
      }
    }
  }
}

#[test]
/// Siblings basic unit tests
fn node_sibling_accessors() {
  let mut doc = Document::new().unwrap();
  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();
  doc.set_root_element(&hello_element);

  let mut new_sibling = Node::new("sibling", None, &doc).unwrap();
  assert!(hello_element.add_prev_sibling(&mut new_sibling).is_ok());
}

#[test]
/// Constructing a comment node, attaching it, and serializing it
fn node_new_comment() {
  let mut doc = Document::new().unwrap();
  let mut root = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root);

  let mut comment = Node::new_comment(" hello ", &doc).unwrap();
  assert_eq!(comment.get_type(), Some(NodeType::CommentNode));
  assert!(root.add_child(&mut comment).is_ok());

  let serialized = doc.to_string();
  assert!(
    serialized.contains("<!-- hello -->"),
    "expected comment in serialized doc, got: {serialized}"
  );

  // Embedded NUL should return Err rather than panic.
  assert!(Node::new_comment("bad\0content", &doc).is_err());
}

#[test]
/// `Document::remove_internal_subset` strips the DOCTYPE preamble
/// from a parsed document, mirroring XML::LibXML's
/// `removeInternalSubset`.
fn document_remove_internal_subset() {
  let parser = Parser::default();
  let xml = r#"<?xml version="1.0"?>
<!DOCTYPE root SYSTEM "example.dtd">
<root><child>hi</child></root>"#;
  let mut doc = parser.parse_string(xml).expect("parse");

  // Before removal: serialisation includes the DOCTYPE.
  let before = doc.to_string();
  assert!(
    before.contains("<!DOCTYPE"),
    "expected DOCTYPE in serialized doc before remove, got: {before}"
  );

  doc.remove_internal_subset();

  // After: no DOCTYPE preamble, root element untouched.
  let after = doc.to_string();
  assert!(
    !after.contains("<!DOCTYPE"),
    "DOCTYPE still present after remove_internal_subset: {after}"
  );
  assert!(after.contains("<root>"));
  assert!(after.contains("<child>hi</child>"));

  // Idempotent: second call on a subset-less doc is a no-op.
  doc.remove_internal_subset();
  let after2 = doc.to_string();
  assert_eq!(after, after2);
}

#[test]
/// `Document::remove_internal_subset` on a doc without a DTD is a no-op.
fn document_remove_internal_subset_no_doctype_noop() {
  let parser = Parser::default();
  let mut doc = parser
    .parse_string(r#"<?xml version="1.0"?><root/>"#)
    .expect("parse");
  let before = doc.to_string();
  doc.remove_internal_subset();
  let after = doc.to_string();
  assert_eq!(before, after);
}

#[test]
fn node_children_accessors() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
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

#[test]
fn node_attributes_accessor() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let root = doc.get_root_element().unwrap();
  let mut root_elements = root.get_child_elements();
  let child_opt = root_elements.first_mut();
  assert!(child_opt.is_some());
  let child = child_opt.unwrap();

  // All attributes
  let attributes = child.get_attributes();
  assert_eq!(attributes.len(), 1);
  assert_eq!(attributes.get("attribute"), Some(&"value".to_string()));

  // Has
  assert!(child.has_attribute("attribute"));
  // Get
  assert_eq!(child.get_attribute("attribute"), Some("value".to_string()));
  // Get as node
  let attr_node_opt = child.get_attribute_node("attribute");
  assert!(attr_node_opt.is_some());
  let attr_node = attr_node_opt.unwrap();
  assert_eq!(attr_node.get_name(), "attribute");
  assert_eq!(attr_node.get_type(), Some(NodeType::AttributeNode));

  // Set
  assert!(child.set_attribute("attribute", "setter_value").is_ok());
  assert_eq!(
    child.get_attribute("attribute"),
    Some("setter_value".to_string())
  );
  // Remove
  assert!(child.remove_attribute("attribute").is_ok());
  assert_eq!(child.get_attribute("attribute"), None);
  assert!(!child.has_attribute("attribute"));
  // Recount
  let attributes = child.get_attributes();
  assert_eq!(attributes.len(), 0);
}

#[test]
fn node_attributes_ns_accessor() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01_ns.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let root = doc.get_root_element().unwrap();
  let mut root_elements = root.get_child_elements();
  let child_opt = root_elements.first_mut();
  assert!(child_opt.is_some());
  let child = child_opt.unwrap();

  // All attributes
  let attributes = child.get_attributes_ns();
  assert_eq!(attributes.len(), 3);
  assert_eq!(
    attributes.get(&("attribute".to_string(), None)),
    Some(&"value1".to_string())
  );
  let namespaces = child.get_namespaces(&doc);
  assert_eq!(namespaces.len(), 2);
  let foo_ns = namespaces[0].clone();
  let bar_ns = namespaces[1].clone();

  assert_eq!(
    attributes.get(&("attribute".to_string(), Some(foo_ns.clone()))),
    Some(&"foo1".to_string())
  );
  assert_eq!(
    attributes.get(&("attr".to_string(), Some(bar_ns.clone()))),
    Some(&"bar1".to_string())
  );

  // Has
  assert!(child.has_attribute("attribute"));
  assert!(child.has_attribute_no_ns("attribute"));
  assert!(child.has_attribute_ns("attribute", "http://www.example.com/myns"),);
  assert!(child.has_attribute("attr"));
  assert!(!child.has_attribute_no_ns("attr"));
  assert!(child.has_attribute_ns("attr", "http://www.example.com/myns"));

  // Get
  assert_eq!(
    child.get_attribute_no_ns("attribute"),
    Some("value1".to_string())
  );
  assert_eq!(
    child.get_attribute_ns("attribute", "http://www.example.com/myns"),
    Some("foo1".to_string())
  );
  assert_eq!(
    child.get_attribute_ns("attr", "http://www.example.com/myns"),
    Some("bar1".to_string())
  );

  // Get as node
  let attr_node_opt = child.get_attribute_node_no_ns("attribute");
  assert!(attr_node_opt.is_some());
  let attr_node = attr_node_opt.unwrap();
  assert_eq!(attr_node.get_name(), "attribute");
  assert_eq!(attr_node.get_type(), Some(NodeType::AttributeNode));
  let attr_node_opt = child.get_attribute_node_no_ns("attr");
  assert!(attr_node_opt.is_none());
  let attr_node_opt = child.get_attribute_node_ns("attr", "http://www.example.com/myns");
  assert!(attr_node_opt.is_some());
  let attr_node = attr_node_opt.unwrap();
  assert_eq!(attr_node.get_name(), "attr");
  assert_eq!(attr_node.get_type(), Some(NodeType::AttributeNode));

  // Set
  assert!(child.set_attribute("attribute", "setter_value").is_ok());
  assert_eq!(
    child.get_attribute_no_ns("attribute"),
    Some("setter_value".to_string())
  );
  assert!(
    child
      .set_attribute_ns("attribute", "foo_value", &foo_ns)
      .is_ok()
  );
  assert_eq!(
    child.get_attribute_no_ns("attribute"),
    Some("setter_value".to_string())
  );
  // Remove
  assert!(child.has_attribute_no_ns("attribute"));
  assert!(child.remove_attribute_no_ns("attribute").is_ok());
  assert_eq!(child.get_attribute_no_ns("attribute"), None);
  assert!(!child.has_attribute_no_ns("attribute"));
  // Recount
  let attributes = child.get_attributes_ns();
  assert_eq!(attributes.len(), 2);
}

#[test]
fn namespace_partial_eq() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01_ns.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let root = doc.get_root_element().unwrap();
  let mut root_elements = root.get_child_elements();
  let child1_opt = root_elements.first_mut();
  assert!(child1_opt.is_some());
  let child1 = child1_opt.unwrap();

  // Child 1 namespaces
  let namespaces1 = child1.get_namespaces(&doc);
  assert_eq!(namespaces1.len(), 2);
  let foo_ns1 = namespaces1[0].clone();
  assert_eq!(foo_ns1.get_prefix(), "foo");
  assert_eq!(foo_ns1.get_href(), "http://www.example.com/myns");
  let bar_ns1 = namespaces1[1].clone();
  assert_eq!(bar_ns1.get_prefix(), "bar");
  assert_eq!(bar_ns1.get_href(), "http://www.example.com/myns");
  // The current implementation of PartialEq for Namespace compares the prefix
  // and href
  assert!(foo_ns1 != bar_ns1);

  //  Compare with child2 namespace
  let child2_opt = child1.get_next_element_sibling();
  assert!(child2_opt.is_some());
  let child2 = child2_opt.unwrap();
  let attributes2 = child2.get_attributes_ns();
  assert_eq!(attributes2.len(), 2);
  let namespaces2 = child2.get_namespaces(&doc);
  assert_eq!(namespaces2.len(), 1);
  let foo_ns2 = namespaces2[0].clone();
  // The current implementation of PartialEq for Namespace compares the prefix
  // and href not the pointer
  assert!(foo_ns1 == foo_ns2);
  assert_eq!(foo_ns1.get_href(), foo_ns2.get_href());
  assert_eq!(foo_ns1.get_prefix(), foo_ns2.get_prefix());
  assert_ne!(foo_ns1.ns_ptr(), foo_ns2.ns_ptr());
}

#[test]
fn attribute_namespace_accessors() {
  let mut doc = Document::new().unwrap();
  let element_result = Node::new("example", None, &doc);
  assert!(element_result.is_ok());

  let mut element = element_result.unwrap();
  doc.set_root_element(&element);

  let ns_result = Namespace::new(
    "myxml",
    "http://www.w3.org/XML/1998/namespace",
    &mut element,
  );
  assert!(ns_result.is_ok());
  let ns = ns_result.unwrap();
  assert!(element.set_attribute_ns("id", "testing", &ns).is_ok());

  let id_attr = element.get_attribute_ns("id", "http://www.w3.org/XML/1998/namespace");
  assert!(id_attr.is_some());
  assert_eq!(id_attr.unwrap(), "testing");

  let id_regular = element.get_attribute("id");
  assert!(id_regular.is_some());
  assert_eq!(id_regular.unwrap(), "testing");

  let id_false_ns = element.get_attribute_ns("id", "http://www.foobar.org");
  assert!(id_false_ns.is_none());
  let fb_ns_result = Namespace::new("fb", "http://www.foobar.org", &mut element);
  assert!(fb_ns_result.is_ok());
  let fb_ns = fb_ns_result.unwrap();
  assert!(element.set_attribute_ns("fb", "fb", &fb_ns).is_ok());
  assert_eq!(
    element.get_attribute_ns("fb", "http://www.foobar.org"),
    Some("fb".to_string())
  );
  assert!(
    element
      .remove_attribute_ns("fb", "http://www.foobar.org")
      .is_ok()
  );
  assert_eq!(
    element.get_attribute_ns("fb", "http://www.foobar.org"),
    None
  );

  let ns_prefix = element.lookup_namespace_prefix("http://www.w3.org/XML/1998/namespace");
  assert_eq!(ns_prefix, Some("xml".to_string())); // system ns has the global prefix when doing global lookup
  let fb_prefix = element.lookup_namespace_prefix("http://www.foobar.org");
  assert_eq!(fb_prefix, Some("fb".to_string())); // system ns has the global prefix when doing global lookup

  let ns_uri = element.lookup_namespace_uri("myxml");
  assert_eq!(
    ns_uri,
    Some("http://www.w3.org/XML/1998/namespace".to_string())
  ); // system ns has the global uri when doing global lookup
  let fb_uri = element.lookup_namespace_uri("fb");
  assert_eq!(fb_uri, Some("http://www.foobar.org".to_string())); // system ns has the global prefix when doing global lookup
}

#[test]
fn attribute_no_namespace() {
  let mut doc = Document::new().unwrap();
  let element_result = Node::new("example", None, &doc);
  assert!(element_result.is_ok());

  let mut element = element_result.unwrap();
  doc.set_root_element(&element);

  let ns_result = Namespace::new("myns", "https://www.example.com/myns", &mut element);
  assert!(ns_result.is_ok());
  let ns = ns_result.unwrap();
  assert!(element.set_attribute_ns("foo", "ns", &ns).is_ok());

  let foo_ns_attr = element.get_attribute_ns("foo", "https://www.example.com/myns");
  assert!(foo_ns_attr.is_some());
  assert_eq!(foo_ns_attr.unwrap(), "ns");

  let foo_no_ns_attr = element.get_attribute_no_ns("foo");
  assert!(foo_no_ns_attr.is_none());

  assert!(element.set_attribute("foo", "no_ns").is_ok());

  let foo_no_ns_attr = element.get_attribute_no_ns("foo");
  assert!(foo_no_ns_attr.is_some());
  assert_eq!(foo_no_ns_attr.unwrap(), "no_ns");

  assert!(element.remove_attribute_no_ns("foo").is_ok());
  let foo_no_ns_attr = element.get_attribute_no_ns("foo");
  assert!(foo_no_ns_attr.is_none());

  assert!(element.set_attribute("bar", "bar").is_ok());
  let bar_no_ns_attr = element.get_attribute_no_ns("bar");
  assert!(bar_no_ns_attr.is_some());
  assert_eq!(bar_no_ns_attr.unwrap(), "bar");
}

#[test]
fn node_can_unbind() {
  let mut doc = Document::new().unwrap();
  let element_result = Node::new("example", None, &doc);
  assert!(element_result.is_ok());

  let mut element = element_result.unwrap();
  doc.set_root_element(&element);

  let mut first_child = Node::new("first", None, &doc).unwrap();
  let mut second_child = Node::new("second", None, &doc).unwrap();
  let mut third_child = Node::new("third", None, &doc).unwrap();

  assert!(element.add_child(&mut first_child).is_ok());
  assert!(element.add_child(&mut second_child).is_ok());
  assert!(element.add_child(&mut third_child).is_ok());

  assert_eq!(element.get_child_nodes().len(), 3);
  first_child.unbind_node();
  assert_eq!(element.get_child_nodes().len(), 2);
  second_child.unlink_node();
  assert_eq!(element.get_child_nodes().len(), 1);
  third_child.unlink();
  assert_eq!(element.get_child_nodes().len(), 0);

  // Test reparenting via unlink
  let mut transfer = Node::new("transfer", None, &doc).unwrap();
  assert!(element.add_child(&mut transfer).is_ok());
  assert!(transfer.append_text("test text").is_ok());
  let mut receiver = Node::new("receiver", None, &doc).unwrap();
  assert!(element.add_child(&mut receiver).is_ok());
  assert_eq!(element.get_child_nodes().len(), 2);
  assert_eq!(transfer.get_child_nodes().len(), 1);
  assert_eq!(receiver.get_child_nodes().len(), 0);

  transfer.unlink();
  assert_eq!(element.get_child_nodes().len(), 1);
  assert_eq!(receiver.get_child_nodes().len(), 0);
  assert!(receiver.add_child(&mut transfer).is_ok());
  assert_eq!(receiver.get_child_nodes().len(), 1);
  assert_eq!(transfer.get_content(), "test text".to_owned());
  assert_eq!(transfer.get_parent(), Some(receiver));
}

#[test]
/// Can mock a node object (useful for defaults that will be overridden)
fn can_mock_node() {
  let doc_mock = Document::new().unwrap();
  let node_mock = Node::mock(&doc_mock);
  assert!(!node_mock.is_text_node());
}

#[test]
/// Can make a mock node hashable
fn can_hash_mock_node() {
  let doc_mock = Document::new().unwrap();
  let node_mock = Node::mock(&doc_mock);
  assert!(node_mock.to_hashable() > 0);
}

#[test]
/// Can make null nodes and documents, to avoid memory allocations
fn can_null_node() {
  let null_node = Node::null();
  let second_null_node = Node::null();
  assert!(null_node.is_null());
  assert!(second_null_node.is_null());
  assert_eq!(null_node, second_null_node);
}

#[test]
/// Can set and get attributes
fn can_manage_attributes() {
  let mut doc = Document::new().unwrap();
  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();
  doc.set_root_element(&hello_element);

  let key = "examplekey";
  let value = "examplevalue";
  let pre_value = hello_element.get_attribute(key);
  assert_eq!(pre_value, None);
  let pre_prop_check = hello_element.has_property(key);
  assert!(!pre_prop_check);
  let pre_prop_value = hello_element.get_property(key);
  assert_eq!(pre_prop_value, None);

  assert!(hello_element.set_attribute(key, value).is_ok());
  let new_check = hello_element.has_attribute(key);
  assert!(new_check);
  let new_value = hello_element.get_attribute(key);
  assert_eq!(new_value, Some(value.to_owned()));
}

#[test]
/// Can set and get text node content
fn can_set_get_text_node_content() {
  let mut doc = Document::new().unwrap();
  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();
  doc.set_root_element(&hello_element);

  assert!(hello_element.get_content().is_empty());
  assert!(hello_element.append_text("hello ").is_ok());
  assert_eq!(hello_element.get_content(), "hello ");
  assert!(hello_element.append_text("world!").is_ok());
  assert_eq!(hello_element.get_content(), "hello world!");
}

#[test]
/// Basic namespace workflow
fn can_work_with_namespaces() {
  let mut doc = Document::new().unwrap();
  let mut root_node = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root_node);

  let initial_namespace_list = root_node.get_namespaces(&doc);
  assert_eq!(initial_namespace_list.len(), 0);

  let mock_ns_result = Namespace::new("mock", "http://example.com/ns/mock", &mut root_node);
  assert!(mock_ns_result.is_ok());
  let second_ns_result = Namespace::new("second", "http://example.com/ns/second", &mut root_node);
  assert!(second_ns_result.is_ok());

  // try to attach this namespace to a node
  assert!(root_node.get_namespace().is_none());
  assert!(root_node.set_namespace(&mock_ns_result.unwrap()).is_ok());
  let active_ns_opt = root_node.get_namespace();
  assert!(active_ns_opt.is_some());
  let active_ns = active_ns_opt.unwrap();
  assert_eq!(active_ns.get_prefix(), "mock");
  assert_eq!(active_ns.get_href(), "http://example.com/ns/mock");

  // now get all namespaces for the node and check we have ours
  let mut namespace_list = root_node.get_namespaces(&doc);
  assert_eq!(namespace_list.len(), 2);

  let second_ns = namespace_list.pop().unwrap();
  assert_eq!(second_ns.get_prefix(), "second");
  assert_eq!(second_ns.get_href(), "http://example.com/ns/second");

  let first_ns = namespace_list.pop().unwrap();
  assert_eq!(first_ns.get_prefix(), "mock");
  assert_eq!(first_ns.get_href(), "http://example.com/ns/mock");
}

#[test]
fn can_work_with_ns_declarations() {
  let mut doc = Document::new().unwrap();
  let mut root_node = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root_node);

  let mock_ns_result = Namespace::new("mock1", "http://example.com/ns/mock1", &mut root_node);
  assert!(mock_ns_result.is_ok());
  let second_ns_result = Namespace::new("mock2", "http://example.com/ns/mock2", &mut root_node);
  assert!(second_ns_result.is_ok());

  let declarations = root_node.get_namespace_declarations();
  assert_eq!(declarations.len(), 2);
}

#[test]
/// Can view documents as nodes
fn can_cast_doc_to_node() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());

  let doc = doc_result.unwrap();
  let doc_node = doc.as_node();
  assert_eq!(doc_node.get_type(), Some(NodeType::DocumentNode));
  let root_node_opt = doc_node.get_first_child();
  assert!(root_node_opt.is_some());
  let root_node = root_node_opt.unwrap();
  assert_eq!(root_node.get_name(), "root");
}

#[test]
fn can_replace_child() {
  let mut doc = Document::new().unwrap();
  let mut root_node = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root_node);
  let mut a = Node::new("a", None, &doc).unwrap();
  let mut b = Node::new("b", None, &doc).unwrap();
  let mut c = Node::new("c", None, &doc).unwrap();
  let mut d = Node::new("d", None, &doc).unwrap();
  let mut e = Node::new("e", None, &doc).unwrap();

  assert!(root_node.add_child(&mut a).is_ok());
  assert!(root_node.add_child(&mut b).is_ok());
  assert!(root_node.add_child(&mut c).is_ok());
  assert!(root_node.add_child(&mut d).is_ok());
  assert!(root_node.add_child(&mut e).is_ok());
  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><a/><b/><c/><d/><e/></root>\n",
    "document initialized correctly."
  );

  // replace first child with new F
  let f = Node::new("F", None, &doc).unwrap();
  let a_result = root_node.replace_child_node(f, a);
  assert!(a_result.is_ok());

  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><c/><d/><e/></root>\n",
    "document initialized correctly."
  );

  // replace last child with new G
  let g = Node::new("G", None, &doc).unwrap();
  assert!(root_node.replace_child_node(g, e).is_ok());
  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><c/><d/><G/></root>\n",
    "document initialized correctly."
  );

  // replace middle child with new H
  let h = Node::new("H", None, &doc).unwrap();
  assert!(root_node.replace_child_node(h, c).is_ok());
  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly."
  );

  // fail to replace a, as it is already removed.
  let none = Node::new("none", None, &doc).unwrap();
  assert!(
    root_node
      .replace_child_node(none, a_result.unwrap())
      .is_err()
  );
  // no change.
  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly."
  );

  // replacing with self succeeds without change.
  assert!(root_node.replace_child_node(b.clone(), b).is_ok());
  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly."
  );
  // replacing with parent succeeds without change.
  assert!(root_node.replace_child_node(root_node.clone(), d).is_ok());
  assert_eq!(
    doc.to_string(),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly."
  );
}
