//! Enforce Rust ownership pragmatics for the underlying libxml2 objects

use libxml::parser::Parser;

/// Mutating a node that merely has several live `Node` CLONES must SUCCEED.
///
/// Clones are the normal state of affairs — the owning document keeps one per
/// node in its cache, and callers hold their own handles — so a high
/// `Rc::strong_count` is NOT an aliasing conflict. `Node::node_ptr_mut` now
/// fails only on an *active* re-entrant borrow (via `RefCell::try_borrow_mut`).
/// This is the regression guard for the former `strong_count <= guard`
/// heuristic, which spuriously returned `Err` ("shared Node") for these benign
/// clones and corrupted otherwise-valid mutations.
#[test]
fn clones_do_not_block_mutation() {
  let parser = Parser::default();
  let doc = parser.parse_file("tests/resources/file01.xml").unwrap();
  let root = doc.get_root_element().unwrap();

  // Two live clones of the SAME node (Rc strong count >= 3: cache + a + b).
  let mut first_a = root.get_first_element_child().unwrap();
  let first_b = root.get_first_element_child().unwrap();

  assert_eq!(
    first_a.get_attribute("attribute"),
    Some(String::from("value"))
  );
  assert_eq!(
    first_b.get_attribute("attribute"),
    Some(String::from("value"))
  );

  // Previously this returned Err purely from the clone count; it must now succeed.
  assert!(first_a.set_attribute("attribute", "newa").is_ok());

  // Both handles alias the same underlying C node, so both observe the change.
  assert_eq!(
    first_a.get_attribute("attribute"),
    Some(String::from("newa"))
  );
  assert_eq!(
    first_b.get_attribute("attribute"),
    Some(String::from("newa"))
  );
}

/// The former tuning knob `set_node_rc_guard` is now a deprecated no-op, retained
/// only for API compatibility. Setting it to a tiny value must NOT re-introduce
/// the old false-positive (which would have blocked the write below).
#[test]
#[allow(deprecated)]
fn set_node_rc_guard_is_a_noop() {
  use libxml::tree::set_node_rc_guard;

  let parser = Parser::default();
  let doc = parser.parse_file("tests/resources/file01.xml").unwrap();
  let mut node = doc
    .get_root_element()
    .unwrap()
    .get_first_element_child()
    .unwrap();
  let _alias = node.clone(); // bump the strong count

  set_node_rc_guard(1); // no effect under the try_borrow_mut regime
  assert!(node.set_attribute("k", "v").is_ok());
}
