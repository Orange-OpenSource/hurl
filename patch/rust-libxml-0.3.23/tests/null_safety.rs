//! Regression tests for NULL-node FFI safety.
//!
//! A `Node::null()` placeholder wraps a NULL `xmlNodePtr`. Every field accessor
//! must degrade to the natural "absence" value instead of dereferencing address
//! 0 — e.g. `get_type()` reads the node's `type_` at **offset 8**, so a raw
//! deref on NULL is the `segfault at 8` observed in production. A crash here
//! bypasses `catch_unwind` and takes down the whole process. These tests pin
//! the null guards in `src/c_helpers.rs`; before them, every assertion below
//! was a SIGSEGV rather than a value.

use libxml::tree::Node;

#[test]
fn null_node_get_type_is_none() {
  // The exact production fault: `xmlGetNodeType(NULL)` reading offset 8.
  assert_eq!(Node::null().get_type(), None);
}

#[test]
fn null_node_is_not_element() {
  assert!(!Node::null().is_element_node());
}

#[test]
fn null_node_navigation_is_none() {
  let n = Node::null();
  assert!(n.get_parent().is_none());
  assert!(n.get_first_child().is_none());
  assert!(n.get_last_child().is_none());
  assert!(n.get_next_sibling().is_none());
  assert!(n.get_prev_sibling().is_none());
  assert!(n.get_first_element_child().is_none());
  assert!(n.get_next_element_sibling().is_none());
  assert!(n.get_prev_element_sibling().is_none());
}

#[test]
fn null_node_name_and_content_are_empty() {
  let n = Node::null();
  assert_eq!(n.get_name(), String::new());
  assert_eq!(n.get_content(), String::new());
}
