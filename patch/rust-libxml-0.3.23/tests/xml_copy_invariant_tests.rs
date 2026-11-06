//! Low-level libxml2 invariant tests: `xmlCopyDoc` / `xmlCopyNode`
//! behavior across our wrapper's lifetime model. Each test imports the
//! raw FFI symbols inline.

use libxml::parser::Parser;
use libxml::tree::{Document, Node};

#[test]
/// Source-doc corruption test: after a single xmlCopyDoc + xmlFreeDoc
/// of the copy, can we still read xml:id attributes off the source?
/// If this test fails, there's per-doc dict sharing between
/// xmlCopyDoc's source and result that violates libxml2's documented
/// independence guarantee.
fn xml_copy_doc_does_not_corrupt_source() {
  use libxml::bindings::{xmlCopyDoc, xmlFreeDoc};
  let parser = Parser::default();
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let src = parser.parse_file(path).expect("parse");
  let root = src.get_root_element().expect("root");
  // Find every section, snapshot its xml:id BEFORE the copy.
  let sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  let pre: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();

  // Copy the source doc, then immediately free the copy.
  unsafe {
    let copy = xmlCopyDoc(src.doc_ptr(), 1);
    assert!(!copy.is_null());
    xmlFreeDoc(copy);
  }

  // After the copy round-trip, source xml:ids must still be readable.
  let post: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();
  for (i, (p, q)) in pre.iter().zip(post.iter()).enumerate() {
    assert_eq!(
      p, q,
      "section {i} xml:id changed across xmlCopyDoc round-trip: {p:?} -> {q:?}"
    );
  }
}

#[test]
/// Same source-corruption check, but mirroring the host-app's path:
/// parse from a STRING (not file), run an `//*[@xml:id]` and a
/// `processing-instruction` XPath against the doc (the LaTeXML
/// PostDocument init walk), THEN xmlCopyDoc + xmlFreeDoc.
fn xml_copy_doc_does_not_corrupt_source_after_init_walks() {
  use libxml::bindings::{xmlCopyDoc, xmlFreeDoc};
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let root = src.get_root_element().unwrap();
  // Mirror PostDocument::set_document_internal init walks.
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _id_walk = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();
  let _pi_walk = ctx
    .findnodes(".//processing-instruction('latexml')", None)
    .unwrap_or_default();

  let sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  let pre: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();

  unsafe {
    let copy = xmlCopyDoc(src.doc_ptr(), 1);
    assert!(!copy.is_null());
    xmlFreeDoc(copy);
  }

  let post: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();
  for (i, (p, q)) in pre.iter().zip(post.iter()).enumerate() {
    assert_eq!(
      p, q,
      "section {i} xml:id changed across copy/free: {p:?} -> {q:?}"
    );
  }
}

#[test]
/// Mirror oxide more precisely: parse_string + init XPath walks +
/// detach sections + xmlCopyDoc + xmlFreeDoc + check source xml:id
/// readable.
fn xml_copy_doc_no_corrupt_after_unlink() {
  use libxml::bindings::{xmlCopyDoc, xmlFreeDoc};
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let root = src.get_root_element().unwrap();
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _ = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();

  let mut sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();

  // Read xml:id BEFORE unlinking.
  let pre: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();

  // Detach each section from its parent.
  for s in sections.iter_mut() {
    s.unlink_node();
  }

  // After unlink, xml:id reads MUST still match pre.
  let mid: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();
  for (i, (p, q)) in pre.iter().zip(mid.iter()).enumerate() {
    assert_eq!(p, q, "section {i} xml:id changed BY unlink: {p:?} -> {q:?}");
  }

  // Now xmlCopyDoc + xmlFreeDoc.
  unsafe {
    let copy = xmlCopyDoc(src.doc_ptr(), 1);
    assert!(!copy.is_null());
    xmlFreeDoc(copy);
  }

  let post: Vec<Option<String>> = sections.iter().map(|s| s.get_attribute("xml:id")).collect();
  for (i, (p, q)) in pre.iter().zip(post.iter()).enumerate() {
    assert_eq!(
      p, q,
      "section {i} xml:id changed AFTER copy/free: {p:?} -> {q:?}"
    );
  }
}

#[test]
/// Two xmlCopyNode calls on the SAME source node, with realistic
/// LaTeXML doc as the source and the node detached after the first
/// call. Checks whether the failure is per-source-doc state or
/// per-source-node.
fn xml_copy_node_twice_same_source() {
  use libxml::bindings::{xmlCopyNode, xmlFreeNode};
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let root = src.get_root_element().unwrap();
  // PostDocument-like init.
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _ = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();
  let _ = ctx
    .findnodes(".//processing-instruction('latexml')", None)
    .unwrap_or_default();

  let sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  assert!(sections.len() >= 2);

  // Detach first section.
  let mut s1 = sections[0].clone();
  s1.unlink_node();

  // First copy.
  unsafe {
    let c1 = xmlCopyNode(s1.node_ptr(), 1);
    assert!(!c1.is_null(), "first xmlCopyNode of S1 returned NULL");
    xmlFreeNode(c1);
  }

  // Second copy of the SAME node.
  unsafe {
    let c2 = xmlCopyNode(s1.node_ptr(), 1);
    assert!(!c2.is_null(), "second xmlCopyNode of S1 returned NULL");
    xmlFreeNode(c2);
  }
}

#[test]
/// Two xmlCopyNode calls on DIFFERENT sibling sections, both detached
/// before the first copy. Mirrors the exact sequence in
/// `Split::process_pages`.
fn xml_copy_node_twice_different_sources_after_unlink_all() {
  use libxml::bindings::{xmlCopyNode, xmlFreeNode};
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let root = src.get_root_element().unwrap();
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _ = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();
  let _ = ctx
    .findnodes(".//processing-instruction('latexml')", None)
    .unwrap_or_default();

  let mut sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  assert!(sections.len() >= 2);

  // Detach ALL sections from the chapter (pop-all).
  for s in sections.iter_mut() {
    s.unlink_node();
  }

  unsafe {
    let c1 = xmlCopyNode(sections[0].node_ptr(), 1);
    assert!(!c1.is_null(), "first copy returned NULL");
    let c2 = xmlCopyNode(sections[1].node_ptr(), 1);
    assert!(
      !c2.is_null(),
      "SECOND copy returned NULL — this is the oxide failure mode"
    );
    xmlFreeNode(c1);
    xmlFreeNode(c2);
  }
}

#[test]
/// Reproduce oxide's exact pre-dup operations: parse_string + init
/// XPath walks + Split.process's own `set_attribute("xml:id",
/// "TEMPORARY_DOCUMENT_ID")` on the root + getPages XPath descent.
/// Then detach + dup pair.
fn xml_copy_node_pair_with_split_preamble() {
  use libxml::bindings::{xmlCopyNode, xmlFreeNode};
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let mut root = src.get_root_element().unwrap();
  // PostDocument init walks.
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _ = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();
  let _ = ctx
    .findnodes(".//processing-instruction('latexml')", None)
    .unwrap_or_default();
  // Split.process: assign TEMPORARY_DOCUMENT_ID if root has no xml:id.
  if root.get_attribute("xml:id").is_none() {
    root.set_attribute("xml:id", "TEMPORARY_DOCUMENT_ID").ok();
  }
  // Split.getPages XPath (a richer path than just descendant::section).
  let _pages = root
    .findnodes(
      "//ltx:section | //ltx:chapter | //ltx:part | //ltx:bibliography \
       | //ltx:appendix | //ltx:index",
    )
    .unwrap_or_default();

  let mut sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  for s in sections.iter_mut() {
    s.unlink_node();
  }

  unsafe {
    let c1 = xmlCopyNode(sections[0].node_ptr(), 1);
    assert!(!c1.is_null(), "first copy returned NULL");
    let c2 = xmlCopyNode(sections[1].node_ptr(), 1);
    assert!(
      !c2.is_null(),
      "SECOND copy returned NULL — repro of oxide bug"
    );
    xmlFreeNode(c1);
    xmlFreeNode(c2);
  }
}

#[test]
/// dup S1 → XPath on source for resources/PIs → dup S2. Mirrors the
/// `findnodes('descendant::ltx:resource')` and PI-scan that oxide's
/// PostDocument::new_document runs BETWEEN successive dups.
fn xml_copy_node_pair_with_intermediate_xpath_on_source() {
  use libxml::bindings::{xmlCopyNode, xmlFreeNode};
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let mut root = src.get_root_element().unwrap();
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _ = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();
  if root.get_attribute("xml:id").is_none() {
    root.set_attribute("xml:id", "TEMPORARY_DOCUMENT_ID").ok();
  }

  let mut sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  for s in sections.iter_mut() {
    s.unlink_node();
  }

  unsafe {
    let c1 = xmlCopyNode(sections[0].node_ptr(), 1);
    assert!(!c1.is_null(), "first copy returned NULL");
    xmlFreeNode(c1);
  }

  // Mid-pair: XPath descents on source, mimicking oxide.
  let _r = root
    .findnodes("descendant::*[local-name()='resource']")
    .unwrap_or_default();
  let _p = root
    .findnodes(".//processing-instruction('latexml')")
    .unwrap_or_default();
  let _i = root.findnodes("//*[@xml:id]").unwrap_or_default();

  unsafe {
    let c2 = xmlCopyNode(sections[1].node_ptr(), 1);
    assert!(
      !c2.is_null(),
      "SECOND copy returned NULL after intermediate XPath — repro of oxide bug"
    );
    xmlFreeNode(c2);
  }
}

#[test]
/// Closest possible repro: parse, set TEMPORARY_DOCUMENT_ID, get
/// pages, detach all, then for each page do dup + create a wrapping
/// Document via Document::new_ptr (mirroring what libxml-rs's higher-
/// level callers do) + run XPath on the subdoc + run XPath on source.
fn xml_copy_node_pair_full_oxide_style() {
  use libxml::bindings::{
    xmlCopyNode, xmlDocSetRootElement, xmlNewDoc, xmlReconciliateNs, xmlSetTreeDoc,
  };
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let parser = Parser::default();
  let src = parser.parse_string(&xml).expect("parse");
  let mut root = src.get_root_element().unwrap();
  let mut ctx = libxml::xpath::Context::new(&src).unwrap();
  let _ = ctx.findnodes("//*[@xml:id]", None).unwrap_or_default();
  if root.get_attribute("xml:id").is_none() {
    root.set_attribute("xml:id", "TEMPORARY_DOCUMENT_ID").ok();
  }

  let mut sections: Vec<Node> = root
    .findnodes("descendant::*[local-name()='section']")
    .unwrap_or_default()
    .into_iter()
    .filter(|n| {
      n.get_parent()
        .map(|p| p.get_name() == "chapter")
        .unwrap_or(false)
    })
    .collect();
  for s in sections.iter_mut() {
    s.unlink_node();
  }

  let mut subdocs: Vec<Document> = Vec::new();
  for (i, s) in sections.iter().enumerate() {
    let sub: Document = unsafe {
      let copy = xmlCopyNode(s.node_ptr(), 1);
      assert!(!copy.is_null(), "dup #{i} xmlCopyNode returned NULL");
      let doc_ptr = xmlNewDoc(c"1.0".as_ptr() as *const u8);
      xmlDocSetRootElement(doc_ptr, copy);
      xmlSetTreeDoc(copy, doc_ptr);
      let _ = xmlReconciliateNs(doc_ptr, copy);
      Document::new_ptr(doc_ptr)
    };
    // Mirror PostDocument::new_document post-dup work:
    let _id_walk = sub
      .get_root_element()
      .unwrap()
      .findnodes("//*[@xml:id]")
      .unwrap_or_default();
    let _src_pis = root
      .findnodes(".//processing-instruction('latexml')")
      .unwrap_or_default();
    let _src_res = root
      .findnodes("descendant::*[local-name()='resource']")
      .unwrap_or_default();
    subdocs.push(sub);
  }
  drop(sections);
  drop(src);
  for s in &subdocs {
    let _ = s.to_string();
  }
}

#[test]
/// Parse the source with XML_PARSE_NODICT to disable string interning,
/// then run the dup pair. If this works while the default parse fails,
/// the bug is dict-related in libxml2's xmlStaticCopyNode for this
/// document shape.
fn xml_copy_node_pair_nodict_parse() {
  use libxml::bindings::{
    xmlCopyNode, xmlFreeDoc, xmlFreeNode, xmlParserOption_XML_PARSE_NODICT, xmlReadMemory,
  };
  let path = "tests/resources/large_doc.xml";
  if std::fs::metadata(path).is_err() {
    return;
  }
  let xml = std::fs::read_to_string(path).expect("read");
  let xml_bytes = xml.as_bytes();
  unsafe {
    let doc_ptr = xmlReadMemory(
      xml_bytes.as_ptr() as *const ::std::os::raw::c_char,
      xml_bytes.len() as ::std::os::raw::c_int,
      c"file.xml".as_ptr(),
      std::ptr::null(),
      xmlParserOption_XML_PARSE_NODICT as ::std::os::raw::c_int,
    );
    assert!(!doc_ptr.is_null(), "xmlReadMemory returned NULL");
    let doc = Document::new_ptr(doc_ptr);
    let root = doc.get_root_element().unwrap();
    // Find sections.
    let sections: Vec<Node> = root
      .findnodes("descendant::*[local-name()='section']")
      .unwrap_or_default()
      .into_iter()
      .filter(|n| {
        n.get_parent()
          .map(|p| p.get_name() == "chapter")
          .unwrap_or(false)
      })
      .collect();
    let mut detached: Vec<Node> = sections.to_vec();
    for s in detached.iter_mut() {
      s.unlink_node();
    }
    let mut copies = Vec::new();
    for (i, s) in detached.iter().enumerate() {
      let c = xmlCopyNode(s.node_ptr(), 1);
      eprintln!("[NODICT] dup #{} ret={:p}", i, c);
      assert!(!c.is_null(), "dup #{} returned NULL even with NODICT", i);
      copies.push(c);
    }
    for c in copies {
      xmlFreeNode(c);
    }
    drop(detached);
    drop(doc);
    let _ = doc_ptr; // just for symmetry
    // (xmlFreeDoc already called by Document::drop via doc-ptr ownership)
    let _ = xmlFreeDoc;
  }
}

#[test]
/// Repro using same allocator path as the host application: this test
/// is only meaningful when run via a binary that has mimalloc as the
/// global allocator (e.g. a fresh integration test compiled with
/// mimalloc). For now, kept as documentation; libxml-rs's test runner
/// uses the default allocator and will pass.
fn _doc_allocator_repro_marker() {
  // Intentionally empty.
}
