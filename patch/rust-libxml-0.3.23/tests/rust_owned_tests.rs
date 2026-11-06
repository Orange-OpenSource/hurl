//! Tests for `Node::set_rust_owned` and the `Linkage` state machine —
//! ownership transfer for detached subtrees, drop-order invariants, and
//! misuse-mode guards.

use libxml::parser::Parser;
use libxml::tree::{Document, Node};

// ---- set_rust_owned: ownership transfer for detached subtrees ---------

#[test]
/// `Node::is_rust_owned` defaults to false for newly-wrapped nodes.
fn rust_owned_default_is_false() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  assert!(!root.is_rust_owned());
  let a = root.get_first_element_child().expect("a");
  assert!(!a.is_rust_owned());
}

#[test]
/// `set_rust_owned` is idempotent — calling it twice does not cause
/// a double free; the C node is freed exactly once when the last
/// `Node` clone drops.
fn rust_owned_idempotent() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  a.unlink_node();
  // After unlink, source doc no longer reaches the subtree via tree
  // walks, but `node->doc` still points at the source. Mark it ours.
  a.set_rust_owned();
  a.set_rust_owned(); // idempotent — second call is a no-op
  assert!(a.is_rust_owned());
  drop(a);
  drop(doc);
  // No crash, no double-free.
}

#[test]
/// The `rust_owned` flag is sticky across clones: setting it on one
/// clone makes every clone observe it, and the C node is freed exactly
/// once when the last clone drops.
fn rust_owned_flag_visible_through_clones() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  a.unlink_node();

  let a_clone1 = a.clone();
  let a_clone2 = a.clone();

  // Set the flag through one clone; observe it through the others.
  a_clone1.set_rust_owned();
  assert!(a.is_rust_owned());
  assert!(a_clone1.is_rust_owned());
  assert!(a_clone2.is_rust_owned());

  // Drop in some order; only the last drop should free.
  drop(a);
  drop(a_clone1);
  drop(a_clone2);
  drop(doc);
}

#[test]
/// Drop ordering matters: a rust-owned orphan MUST drop while its
/// source document is still alive. The orphan's strings are typically
/// interned in `source_doc->dict`; libxml2's `xmlFreeNode` consults
/// `node->doc->dict` via `xmlDictOwns` to decide whether to free each
/// string. If the source doc has already been freed, that read is a
/// UAF — in libxml2, not in this wrapper. So the supported pattern is:
///   1. parse / build source doc
///   2. extract subtrees (`unlink_node` + `dup_node_into_new_doc`)
///   3. mark source-side detached nodes `set_rust_owned`
///   4. drop them (frees their C subtree)
///   5. drop source doc
///
/// This test exercises that order.
fn rust_owned_drops_before_source_doc() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a><b/></a></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  a.unlink_node();
  a.set_rust_owned();
  drop(a);
  drop(doc);
}

#[test]
/// After `dup_node_into_new_doc`, the original detached subtree is no
/// longer referenced by the source document's tree topology and can be
/// safely marked rust-owned. The duplicated copy in the new sub-doc
/// must remain valid after the original is dropped.
fn rust_owned_after_dup_node_into_new_doc() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a id=\"a1\"><b>hello</b></a><c/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  a.unlink_node();

  let sub = Document::dup_node_into_new_doc(&a).expect("dup");
  // The source-side `a` is no longer needed; transfer ownership to Rust
  // so that dropping its wrapper actually reclaims the C alloc rather
  // than leaking it (source doc's xmlFreeDoc won't reach it).
  a.set_rust_owned();
  drop(a);

  // The duplicated copy in `sub` is independent and must still be
  // intact.
  let sub_root = sub.get_root_element().expect("sub root");
  assert_eq!(sub_root.get_name(), "a");
  let sub_b = sub_root.get_first_element_child().expect("sub b");
  assert_eq!(sub_b.get_name(), "b");
  assert_eq!(sub_b.get_content(), "hello");

  // `c` (sibling that was *not* unlinked) is still in the source doc.
  let c = root.get_last_element_child().expect("c");
  assert_eq!(c.get_name(), "c");

  drop(sub);
  drop(doc);
}

#[test]
/// Stress: split-extract every section out of a multi-section source
/// doc, dup each one into its own sub-doc, and mark the source-side
/// extracted node rust-owned. Source doc and all sub-docs must drop
/// cleanly with no UAF and no double-free.
fn rust_owned_split_extract_stress() {
  let parser = Parser::default();
  let mut xml = String::from("<book>");
  for i in 0..16 {
    xml.push_str(&format!(
      "<section id=\"s{}\"><h>title {}</h><p>body {}</p></section>",
      i, i, i
    ));
  }
  xml.push_str("</book>");
  let doc = parser.parse_string(xml.as_bytes()).expect("parse");
  let root = doc.get_root_element().expect("root");

  let mut sections: Vec<Node> = root.get_child_elements();
  let mut subdocs: Vec<Document> = Vec::new();

  for n in sections.iter_mut() {
    n.unlink_node();
    let sub = Document::dup_node_into_new_doc(n).expect("dup");
    n.set_rust_owned();
    subdocs.push(sub);
  }

  // Drop the source-side extracted nodes first.
  drop(sections);
  // Source doc still has its root (now childless of element kids).
  let root2 = doc.get_root_element().expect("root still there");
  assert_eq!(root2.get_name(), "book");

  // Sub-docs remain independently valid.
  for (i, sub) in subdocs.iter().enumerate() {
    let sub_root = sub.get_root_element().expect("sub root");
    assert_eq!(sub_root.get_name(), "section");
    assert_eq!(sub_root.get_attribute("id"), Some(format!("s{}", i)));
  }

  drop(subdocs);
  drop(doc);
}

#[test]
/// When a rust-owned subtree is dropped, libxml2's `xmlFreeNode`
/// recursively frees every descendant in C. Any wrapper that points
/// at a descendant of that subtree must therefore drop *before* the
/// rust-owned ancestor — otherwise it would dereference a freed
/// pointer. This test exercises the supported order: descendant
/// wrappers drop first, then the rust-owned ancestor.
fn rust_owned_descendant_wrappers_drop_before_parent() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a><b><c/><d/></b><e/></a></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  a.unlink_node();
  a.set_rust_owned();

  // Hold wrappers to descendants. They must drop before `a` does.
  let b = a.get_first_element_child().expect("b");
  let c = b.get_first_element_child().expect("c");
  let d = c.get_next_element_sibling().expect("d");
  let e = a.get_last_element_child().expect("e");

  // Verify the wrappers see the expected names.
  assert_eq!(b.get_name(), "b");
  assert_eq!(c.get_name(), "c");
  assert_eq!(d.get_name(), "d");
  assert_eq!(e.get_name(), "e");

  // Drop descendant wrappers first (no-op: they are Linked under `a`).
  drop(c);
  drop(d);
  drop(b);
  drop(e);

  // Now drop `a` — fires xmlFreeNode and recursively reclaims the
  // whole subtree. No prior wrapper is alive to dereference the freed
  // descendants.
  drop(a);
  drop(doc);
}

#[test]
/// `Document::import_node` must reject `RustOwned` source nodes in
/// release builds (where the `set_linked` debug-assert is compiled
/// out). The early return preserves wrapper invariants — without it,
/// a release-build caller could re-link a rust-owned source via
/// `set_linked` and set up a later double-free.
fn import_node_rejects_rust_owned_source() {
  let parser = Parser::default();
  let src = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse src");
  let mut dest = parser
    .parse_string("<root/>".as_bytes())
    .expect("parse dest");

  let src_root = src.get_root_element().expect("src root");
  let mut a = src_root.get_first_element_child().expect("a");
  a.unlink_node();
  a.set_rust_owned();

  // Must not import a rust-owned node — would set up a double-free.
  let result = dest.import_node(&mut a);
  assert!(result.is_err(), "import_node must reject RustOwned source");
  // Wrapper state unchanged.
  assert!(a.is_rust_owned());

  drop(a);
  drop(dest);
  drop(src);
}

#[test]
/// `set_rust_owned` must not be called on a `Linked` node. In debug
/// builds we panic via `debug_assert!`; this test exercises the guard.
/// In release builds the call would be a silent UB, leaving the
/// wrapper poised to free a still-tree-attached node.
#[should_panic(expected = "set_rust_owned called on a Linked node")]
#[cfg(debug_assertions)]
fn rust_owned_panics_on_linked_node() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse");
  let root = doc.get_root_element().expect("root");
  let a = root.get_first_element_child().expect("a");
  // `a` is still in the tree — must NOT mark it rust-owned.
  a.set_rust_owned();
  drop(doc);
}

#[test]
/// Re-attaching a `RustOwned` node via `add_child` (or any other
/// linker) is a misuse pattern. In debug builds, the crate-private
/// `set_linked` transition fires a `debug_assert!` to catch this. We
/// exercise the guard via the public `add_child` path.
#[should_panic(expected = "set_linked called on a RustOwned node")]
#[cfg(debug_assertions)]
fn rust_owned_relink_via_add_child_panics_in_debug() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/></r>".as_bytes())
    .expect("parse");
  let mut root = doc.get_root_element().expect("root");
  let mut a = root.get_first_element_child().expect("a");
  a.unlink_node();
  a.set_rust_owned();
  // Intentional misuse: re-attach a rust-owned node. The crate-private
  // `set_linked` called by `add_child` must trip the debug assertion.
  let _ = root.add_child(&mut a);
  drop(doc);
}

#[test]
/// Mixed lifetime: some unlinked subtrees are re-attached to the
/// source doc, others are marked rust-owned and dropped. The wrapper
/// must NOT free re-attached nodes (the source doc still owns them),
/// and MUST free the rust-owned ones.
fn rust_owned_mixed_with_reattach() {
  let parser = Parser::default();
  let doc = parser
    .parse_string("<r><a/><b/><c/><d/></r>".as_bytes())
    .expect("parse");
  let mut root = doc.get_root_element().expect("root");

  let kids: Vec<Node> = root.get_child_elements();
  // Names: a, b, c, d
  let mut a = kids[0].clone();
  let mut b = kids[1].clone();
  let mut c = kids[2].clone();
  let mut d = kids[3].clone();
  drop(kids);

  // Detach all four.
  a.unlink_node();
  b.unlink_node();
  c.unlink_node();
  d.unlink_node();

  // Re-attach a and c; mark b and d rust-owned.
  root.add_child(&mut a).expect("re-attach a");
  root.add_child(&mut c).expect("re-attach c");
  b.set_rust_owned();
  d.set_rust_owned();

  drop(b);
  drop(d);

  // a and c should still be there and walkable.
  let names: Vec<String> = root
    .get_child_elements()
    .iter()
    .map(|n| n.get_name())
    .collect();
  assert_eq!(names, vec!["a", "c"]);

  drop(a);
  drop(c);
  drop(root);
  drop(doc);
}
