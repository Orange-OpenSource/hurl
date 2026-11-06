//! Tests for `Document::dup_node_into_new_doc` — duplicating a node
//! subtree into a fresh, independent `Document`.

use libxml::parser::Parser;
use libxml::tree::{Document, Node};

#[test]
/// `Document::dup_node_into_new_doc` returns an independent document for
/// a still-linked element, and the source / sub doc lifetimes are
/// independent (drop one, the other survives).
fn dup_node_into_new_doc_basic() {
  let parser = Parser::default();
  let src = parser
    .parse_string("<root xmlns=\"http://example.com/ns\"><a id=\"x\"><b/></a><c/></root>")
    .expect("parse src");
  let root = src.get_root_element().expect("src root");
  // pick the <a> child
  let a = root.get_first_child().expect("first child");
  let sub = Document::dup_node_into_new_doc(&a).expect("dup");
  let sub_root = sub.get_root_element().expect("sub root");
  assert_eq!(sub_root.get_name(), "a");
  // The sub doc's root is a deep copy — it has the <b> child too.
  assert!(sub_root.get_first_child().is_some());
  // Drop the source first; sub must still be valid for serialization.
  drop(src);
  let serialized = sub.to_string();
  assert!(serialized.contains("<a"));
  assert!(serialized.contains("<b"));
}

#[test]
/// Repeated extraction in the same source document — the failure mode
/// that motivated this API. `xmlDocCopyNode(src, dst, 1)` returns NULL
/// on the second call within one source doc; `dup_node_into_new_doc`
/// must succeed for every sibling.
fn dup_node_into_new_doc_multi_siblings() {
  let parser = Parser::default();
  let src = parser
    .parse_string(
      "<root xmlns=\"http://example.com/ns\">\
         <s id=\"s1\"><t>one</t></s>\
         <s id=\"s2\"><t>two</t></s>\
         <s id=\"s3\"><t>three</t></s>\
       </root>",
    )
    .expect("parse src");
  let root = src.get_root_element().expect("src root");
  let mut child = root.get_first_child();
  let mut subdocs = Vec::new();
  let mut count = 0;
  while let Some(n) = child {
    if n.get_name() == "s" {
      let sub = Document::dup_node_into_new_doc(&n)
        .expect("dup_node_into_new_doc must succeed for every sibling");
      assert_eq!(
        sub.get_root_element().unwrap().get_name(),
        "s",
        "sub-document #{count} should have <s> as root"
      );
      subdocs.push(sub);
      count += 1;
    }
    child = n.get_next_sibling();
  }
  assert_eq!(count, 3, "all three siblings extracted");
  // Drop the source while the subdocs are alive; serialization must
  // still work — proves the subdocs own their C-side memory.
  drop(src);
  for (i, s) in subdocs.iter().enumerate() {
    let xml = s.to_string();
    assert!(xml.contains("<s"), "subdoc {i} has <s>");
    assert!(xml.contains("<t"), "subdoc {i} has <t>");
  }
}

#[test]
/// Drop-order independence: dropping the source document before the
/// sub-document must not corrupt the sub-document. Specifically tests
/// that ns / dict pointers are owned by the sub-document, not still
/// referenced from the (now freed) source.
fn dup_node_into_new_doc_source_dropped_first() {
  let sub = {
    let parser = Parser::default();
    let src = parser
      .parse_string(
        "<root xmlns=\"http://example.com/ns\" xmlns:x=\"http://example.com/x\">\
           <a x:tag=\"hi\"><b>text</b></a></root>",
      )
      .expect("parse src");
    let root = src.get_root_element().unwrap();
    let a = root.get_first_child().unwrap();
    Document::dup_node_into_new_doc(&a).expect("dup")
  };
  // src goes out of scope here — its xmlFreeDoc has fired.
  let s = sub.to_string();
  assert!(s.contains("<a"));
  assert!(s.contains("<b"));
  assert!(s.contains("text"));
}

#[test]
/// Repeated extraction in the same source document, with explicit
/// unlink first — the failure mode that motivated this API. A plain
/// `xmlDocCopyNode(src, dst, 1)` returns NULL on the second sibling
/// in this pattern; `dup_node_into_new_doc` must succeed for every
/// detached subtree.
fn dup_node_into_new_doc_after_unlink_chain() {
  let parser = Parser::default();
  let src = parser
    .parse_string(
      "<root xmlns=\"http://example.com/ns\">\
         <s id=\"s1\"><t>one</t></s>\
         <s id=\"s2\"><t>two</t></s>\
         <s id=\"s3\"><t>three</t></s>\
       </root>",
    )
    .expect("parse src");
  let root = src.get_root_element().expect("src root");

  // Phase 1: collect sibling pages then unlink them from the parent
  // (mirrors the get_last_child / unlink_node loop in Split::process_pages).
  let mut pages: Vec<Node> = Vec::new();
  let mut cur = root.get_first_child();
  while let Some(n) = cur {
    let next = n.get_next_sibling();
    if n.get_name() == "s" {
      pages.push(n);
    }
    cur = next;
  }
  for p in pages.iter_mut() {
    p.unlink_node();
  }

  // Phase 2: dup each unlinked page into its own sub-document. Every
  // call must succeed — failure on the second sibling is the regression
  // we are guarding against.
  let mut subdocs = Vec::new();
  for (i, p) in pages.iter().enumerate() {
    let sub = Document::dup_node_into_new_doc(p)
      .unwrap_or_else(|_| panic!("dup #{i} failed for unlinked sibling"));
    let sub_root = sub.get_root_element().expect("sub root");
    assert_eq!(sub_root.get_name(), "s");
    subdocs.push(sub);
  }
  assert_eq!(subdocs.len(), 3);

  // Phase 3: drop source, then exercise each subdoc.
  drop(src);
  for s in &subdocs {
    let xml = s.to_string();
    // After namespace reconciliation, the default-ns prefix may be
    // synthesised as "default:"; check for the local-name with either
    // a "<" or "<prefix:" lead.
    assert!(
      xml.contains("<s ") || xml.contains("<s>") || xml.contains(":s "),
      "subdoc must contain element s: {xml}"
    );
    assert!(
      xml.contains("<t>") || xml.contains(":t>"),
      "subdoc must contain element t: {xml}"
    );
  }
}

#[test]
/// Repeated extraction with realistic between-dup work: mutate an
/// attribute on the page about to be duped and run XPath against the
/// detached subtree. Exercises every state-mutation knob a downstream
/// document-splitter is likely to touch between dups.
fn dup_node_into_new_doc_after_xpath_and_attr_mutation() {
  let parser = Parser::default();
  let src = parser
    .parse_string(
      "<root xmlns=\"http://example.com/ns\">\
         <s xml:id=\"s1\"><t>one</t></s>\
         <s xml:id=\"s2\"><t>two</t></s>\
         <s xml:id=\"s3\"><t>three</t></s>\
       </root>",
    )
    .expect("parse src");
  let root = src.get_root_element().unwrap();

  let mut pages: Vec<Node> = Vec::new();
  let mut cur = root.get_first_child();
  while let Some(n) = cur {
    let next = n.get_next_sibling();
    if n.get_name() == "s" {
      pages.push(n);
    }
    cur = next;
  }
  for p in pages.iter_mut() {
    p.unlink_node();
  }

  let mut subdocs = Vec::new();
  for (i, p) in pages.iter().enumerate() {
    eprintln!("[iter {i}] start");
    let mut p_mut = p.clone();
    p_mut.set_attribute("inlist", "toc").ok();
    eprintln!("[iter {i}] post set_attribute");
    let xpath_hits = p.findnodes("descendant-or-self::*[@*]").unwrap_or_default();
    eprintln!("[iter {i}] post findnodes (hits={})", xpath_hits.len());

    let sub = Document::dup_node_into_new_doc(p)
      .unwrap_or_else(|_| panic!("dup #{i} failed after xpath/attr mutation"));
    eprintln!("[iter {i}] post dup");
    subdocs.push(sub);
  }
  assert_eq!(subdocs.len(), 3);
  // Drop the page wrappers (still flagged unlinked by `unlink_node`)
  // BEFORE the source doc — otherwise `_Node::drop`'s xmlFreeNode
  // touches `node->doc` after `xmlFreeDoc(src)` has already freed it.
  // That's a pre-existing libxml-rs issue with unlinked-node-outliving
  // -its-doc, not specific to `dup_node_into_new_doc`.
  drop(pages);
  drop(src);
  for s in &subdocs {
    let _ = s.to_string();
  }
}

#[test]
/// Stress: many namespaces declared on the root, deep subtrees, and
/// repeated dup. Designed to surface dict / ns issues that low-content
/// tests miss.
fn dup_node_into_new_doc_many_ns_repeated() {
  let parser = Parser::default();
  let src_xml = "<root xmlns=\"http://example.com/ns\" \
                 xmlns:a=\"http://example.com/a\" \
                 xmlns:b=\"http://example.com/b\" \
                 xmlns:c=\"http://example.com/c\">\
                 <s id=\"s1\"><t a:k=\"v\"><u b:k=\"v\"/></t></s>\
                 <s id=\"s2\"><t a:k=\"v\"><u c:k=\"v\"/></t></s>\
                 <s id=\"s3\"><t b:k=\"v\"><u a:k=\"v\"/></t></s>\
                 <s id=\"s4\"><t c:k=\"v\"><u b:k=\"v\"/></t></s>\
                 <s id=\"s5\"><t a:k=\"v\"><u c:k=\"v\"/></t></s>\
                 </root>";
  let src = parser.parse_string(src_xml).expect("parse src");
  let root = src.get_root_element().unwrap();

  let mut pages: Vec<Node> = Vec::new();
  let mut cur = root.get_first_child();
  while let Some(n) = cur {
    let next = n.get_next_sibling();
    if n.get_name() == "s" {
      pages.push(n);
    }
    cur = next;
  }
  for p in pages.iter_mut() {
    p.unlink_node();
  }

  let mut subdocs = Vec::new();
  for (i, p) in pages.iter().enumerate() {
    let sub =
      Document::dup_node_into_new_doc(p).unwrap_or_else(|_| panic!("ns-stress dup #{i} failed"));
    subdocs.push(sub);
  }
  assert_eq!(subdocs.len(), 5);
  drop(src);
  for s in &subdocs {
    let _ = s.to_string();
  }
}

#[test]
/// Repro: large real-world doc, extract every section sibling. The
/// failure mode this guards against is xmlDocCopyNode returning NULL
/// on the second sibling specifically when the source document is a
/// large, deeply-nested tree (small synthetic XML does not reproduce).
fn dup_node_into_new_doc_large_doc_siblings() {
  let parser = Parser::default();
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    eprintln!("skipping: {path} not present");
    return;
  }
  let src = parser.parse_file(path).expect("parse large doc");
  let root = src.get_root_element().expect("root");
  // Find the chapter and pick its <section> children — mirrors the
  // pattern Split applies on a large LaTeXML chapter.
  let mut pages: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    // direct children of chapter only
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  assert!(
    pages.len() >= 2,
    "need at least 2 section siblings to repro"
  );
  // Detach each page from its parent before duping (mirrors a
  // document-splitter that pops siblings from the parent first).
  for p in pages.iter_mut() {
    p.unlink_node();
  }
  let mut subdocs = Vec::new();
  for (i, p) in pages.iter().enumerate() {
    eprintln!("[large_doc] dup #{i} of {}", p.get_name());
    let sub = Document::dup_node_into_new_doc(p)
      .unwrap_or_else(|_| panic!("dup #{i} of section returned NULL"));
    subdocs.push(sub);
  }
  eprintln!("[large_doc] all dups OK, count={}", subdocs.len());
  drop(pages);
  drop(src);
  for s in &subdocs {
    let _ = s.to_string();
  }
}

#[test]
/// Repro at scale: each detached page contains hundreds of xml:id-bearing
/// descendants, and we run XPath against the detached subtree before
/// each dup (mirrors the cleanup pattern a document-splitter uses to
/// drop ID-cache entries for the moved subtree). Small synthetic XMLs
/// don't trigger the second-dup-NULL; this test does.
fn dup_node_into_new_doc_xpath_then_dup_at_scale() {
  // Build a doc with 5 sibling sections, each carrying ~200 xml:id'd
  // descendants. Total: ~1000 ids on the source.
  let mut xml = String::from("<root xmlns=\"http://example.com/ns\">");
  for i in 0..5 {
    xml.push_str(&format!("<s xml:id=\"s{i}\">"));
    for j in 0..200 {
      xml.push_str(&format!(
        "<p xml:id=\"s{i}.p{j}\"><e xml:id=\"s{i}.p{j}.e\"/></p>"
      ));
    }
    xml.push_str("</s>");
  }
  xml.push_str("</root>");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let root = src.get_root_element().unwrap();

  let mut pages: Vec<Node> = Vec::new();
  let mut cur = root.get_first_child();
  while let Some(n) = cur {
    let next = n.get_next_sibling();
    if n.get_name() == "s" {
      pages.push(n);
    }
    cur = next;
  }
  for p in pages.iter_mut() {
    p.unlink_node();
  }

  let mut subdocs = Vec::new();
  for (i, p) in pages.iter().enumerate() {
    // Mirror the document-splitter pattern: walk the detached subtree
    // for xml:id-bearing descendants, then dup.
    let hits = p
      .findnodes("descendant-or-self::*[@*[local-name()='id']]")
      .unwrap_or_default();
    assert!(
      hits.len() > 100,
      "expected many xml:id hits, got {}",
      hits.len()
    );
    let sub = Document::dup_node_into_new_doc(p)
      .unwrap_or_else(|_| panic!("dup #{i} returned NULL after XPath descent"));
    subdocs.push(sub);
  }
  drop(pages);
  drop(src);
  for s in &subdocs {
    let _ = s.to_string();
  }
}

#[test]
/// Faithfully reproduce a document-splitter that, for each detached
/// page, also runs XPath on the SOURCE (still-live) document and
/// scans the detached subtree for xml:id descendants. This is the
/// pattern oxide hits — XPath fanout on the source between sub-doc
/// builds is what corrupts state for the next xmlDocCopyNode.
fn dup_node_into_new_doc_mixed_xpath_at_scale() {
  let mut xml = String::from("<root xmlns=\"http://example.com/ns\">");
  // Sprinkle "resource" elements at the top so the source XPath has
  // something to enumerate.
  for r in 0..5 {
    xml.push_str(&format!("<resource src=\"r{r}.css\"/>"));
  }
  for i in 0..7 {
    xml.push_str(&format!("<s xml:id=\"s{i}\">"));
    for j in 0..400 {
      xml.push_str(&format!(
        "<p xml:id=\"s{i}.p{j}\"><e xml:id=\"s{i}.p{j}.e\"/></p>"
      ));
    }
    xml.push_str("</s>");
  }
  xml.push_str("</root>");

  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let root = src.get_root_element().unwrap();

  let mut pages: Vec<Node> = Vec::new();
  let mut cur = root.get_first_child();
  while let Some(n) = cur {
    let next = n.get_next_sibling();
    if n.get_name() == "s" {
      pages.push(n);
    }
    cur = next;
  }
  for p in pages.iter_mut() {
    p.unlink_node();
  }

  let mut subdocs = Vec::new();
  for (i, p) in pages.iter().enumerate() {
    // (a) descendant scan on detached page (id-cache cleanup pattern)
    let id_hits = p
      .findnodes("descendant-or-self::*[@*[local-name()='id']]")
      .unwrap_or_default();
    assert!(id_hits.len() > 100, "iter {i}: too few id hits");
    // (b) XPath on the live source doc (resource enumeration pattern)
    let res_hits = root
      .findnodes("descendant::*[local-name()='resource']")
      .unwrap_or_default();
    assert_eq!(res_hits.len(), 5, "iter {i}: expected 5 resource hits");
    // (c) dup
    let sub = Document::dup_node_into_new_doc(p)
      .unwrap_or_else(|_| panic!("dup #{i} returned NULL after mixed XPath"));
    subdocs.push(sub);
  }
  drop(pages);
  drop(src);
  for s in &subdocs {
    let _ = s.to_string();
  }
}
