//! Tests for `Node::free_subtree` — immediate discard of a garbage subtree
//! with wrapper neutralization, the operation `unlink_node` +
//! `set_rust_owned` cannot provide safely when stray clones survive in
//! long-lived collections.

use libxml::parser::Parser;
use libxml::tree::{Document, Node, NodeType};

/// Freeing a subtree detaches it from the document and neutralizes every
/// live wrapper into it: held clones become inert (null node) instead of
/// dangling, and their later drop is a no-op — even after the document
/// itself is gone.
#[test]
fn free_subtree_neutralizes_held_wrappers() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a x=\"1\"><b>text</b></a><c/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let a = root.get_first_element_child().expect("a");
  let b = a.get_first_element_child().expect("b");
  let b_clone = b.clone(); // a stray holder, as in a bookkeeping Vec

  a.free_subtree();

  // The document no longer contains <a>; <c> survives.
  let remaining = root.get_first_element_child().expect("c stays");
  assert_eq!(remaining.get_name(), "c");
  // The held descendant wrapper is inert, not dangling.
  assert!(b_clone.get_type().is_none(), "neutralized wrapper has no type");
  assert_eq!(b_clone.get_name(), "", "null-guarded accessors go empty");
  // Dropping holders after the document is freed must be a no-op.
  drop(doc);
  drop(b_clone);
}

/// An already-unlinked (detached) subtree is freed too — the case where the
/// caller detached first and decides to discard afterwards.
#[test]
fn free_subtree_frees_detached_tree() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a><b/></a></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  let b = a.get_first_element_child().expect("b");
  a.unlink_node();
  a.free_subtree();
  assert!(b.get_type().is_none(), "detached descendant neutralized");
  assert!(root.get_first_element_child().is_none());
}

/// Fresh, never-linked nodes (built trees that end up discarded) free
/// cleanly, and the neutralization covers nodes added under them.
#[test]
fn free_subtree_frees_built_tree() {
  let mut doc = Document::new().expect("new doc");
  let root = Node::new("r", None, &doc).expect("root");
  doc.set_root_element(&root);
  let mut shell = Node::new("shell", None, &doc).expect("shell");
  let mut inner = Node::new("inner", None, &doc).expect("inner");
  shell.add_child(&mut inner).expect("add");
  let inner_clone = inner.clone();
  // Never linked into the document tree.
  shell.free_subtree();
  assert!(inner_clone.get_type().is_none());
  // Document tree unaffected.
  assert_eq!(root.get_name(), "r");
}

/// Calling `free_subtree` twice through separate clones must not
/// double-free: the second call sees a neutralized (null) handle.
#[test]
fn free_subtree_second_call_is_noop() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let a = root.get_first_element_child().expect("a");
  let a_clone = a.clone();
  a.free_subtree();
  a_clone.free_subtree(); // neutralized — must be a no-op
}

/// Documents and document-fragment shells are refused.
#[test]
fn free_subtree_refuses_documents() {
  let parser = Parser::default();
  let doc = parser.parse_string("<r/>".as_bytes()).expect("parse");
  let root = doc.get_root_element().expect("root");
  if let Some(doc_node) = root.get_parent() {
    assert_eq!(doc_node.get_type(), Some(NodeType::DocumentNode));
    doc_node.free_subtree(); // refused
  }
  // Still a live document with a root.
  assert_eq!(doc.get_root_element().expect("root").get_name(), "r");
}
