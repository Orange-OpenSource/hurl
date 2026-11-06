//! The property accessors read what libxml2's own getters return, whichever path a name or a value takes (a name
//! copied on the stack or through a `CString`, a value read in place or through libxml2's copy), for every shape of
//! value: those a parser makes, and those only the tree API makes (no children, a text child with no content, a CDATA
//! child). The child lists hold the children in document order.

use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::{Once, OnceLock};

#[cfg(not(all(target_family = "windows", target_env = "msvc")))]
use libxml::bindings::xmlFree;
use libxml::bindings::{
  xmlAddChild, xmlGetNoNsProp, xmlGetNsProp, xmlGetProp, xmlMemGet, xmlMemSetup, xmlNewCDataBlock,
  xmlNewDocText, xmlNewNs, xmlNewNsProp, xmlNewProp, xmlNodeGetContent, xmlNodePtr,
};
use libxml::parser::Parser;
use libxml::tree::{Node, NodeType};

/// libxml2's own reading of a value: the string its getter returns, freed after the copy (as the crate frees it: on
/// MSVC with the C runtime's `free`).
fn libxml2_value(value_ptr: *mut u8) -> Option<String> {
  if value_ptr.is_null() {
    return None;
  }
  let value = unsafe { CStr::from_ptr(value_ptr as *const c_char) }
    .to_string_lossy()
    .into_owned();
  #[cfg(not(all(target_family = "windows", target_env = "msvc")))]
  unsafe {
    match xmlFree {
      Some(free) => free(value_ptr as *mut _),
      None => libc::free(value_ptr as *mut _),
    }
  }
  #[cfg(all(target_family = "windows", target_env = "msvc"))]
  unsafe {
    libc::free(value_ptr as *mut _)
  };
  Some(value)
}

fn long_name() -> String {
  format!("n{}", "x".repeat(199))
}

/// Every shape a value comes in: plain, empty, a predefined entity (one text node), an unsubstituted entity reference
/// (libxml2's copying path), a DTD default, a namespaced attribute, values at and past 128 bytes, and a 200-byte name.
fn shapes_document() -> libxml::tree::Document {
  let xml = format!(
    r#"<!DOCTYPE r [<!ENTITY e "EE"><!ATTLIST r d CDATA "dflt">]>
<r a="1234567890" empty="" amp="x&amp;y" ent="x&e;y" xmlns:p="urn:p" p:q="nsval" b128="{}" b129="{}" {}="long"/>"#,
    "s".repeat(128),
    "h".repeat(129),
    long_name()
  );
  Parser::default().parse_string(xml).unwrap()
}

#[test]
fn get_property_reads_what_libxml2_reads() {
  let doc = shapes_document();
  let root = doc.get_root_element().unwrap();
  let node_ptr = root.node_ptr() as xmlNodePtr;
  let long = long_name();
  let names = [
    "a", "empty", "amp", "ent", "d", "q", "b128", "b129", "absent", &long,
  ];
  for name in names {
    let c_name = CString::new(name).unwrap();
    let c_name = c_name.as_bytes_with_nul().as_ptr();
    let expected = libxml2_value(unsafe { xmlGetProp(node_ptr, c_name) });
    assert_eq!(root.get_property(name), expected, "get_property {name:?}");
    assert_eq!(root.get_attribute(name), expected, "get_attribute {name:?}");
    assert_eq!(
      root.has_property(name),
      expected.is_some(),
      "has_property {name:?}"
    );
    let expected_no_ns = libxml2_value(unsafe { xmlGetNoNsProp(node_ptr, c_name) });
    assert_eq!(
      root.get_property_no_ns(name),
      expected_no_ns,
      "get_property_no_ns {name:?}"
    );
    assert_eq!(root.has_property_no_ns(name), expected_no_ns.is_some());
    let c_ns = c"urn:p".as_ptr() as *const u8;
    let expected_ns = libxml2_value(unsafe { xmlGetNsProp(node_ptr, c_name, c_ns) });
    assert_eq!(
      root.get_property_ns(name, "urn:p"),
      expected_ns,
      "get_property_ns {name:?}"
    );
    assert_eq!(root.has_property_ns(name, "urn:p"), expected_ns.is_some());
  }
  assert_eq!(root.get_property("empty").as_deref(), Some(""));
  assert_eq!(root.get_property("ent").as_deref(), Some("xEEy"));
  assert_eq!(root.get_property("d").as_deref(), Some("dflt"));
  assert_eq!(root.get_property_ns("q", "urn:p").as_deref(), Some("nsval"));
  assert_eq!(root.get_property(&long).as_deref(), Some("long"));
}

/// The shapes only the tree API makes: an attribute with no children (`xmlNewProp` with no value), one whose text child
/// has no content (`xmlNewDocText` with none), and one holding a CDATA child; each with and without a namespace.
/// `get_property` and its forms, `with_property` and `get_properties` read each as libxml2's getters and
/// `xmlNodeGetContent` read it.
#[test]
fn tree_built_value_shapes_read_as_libxml2_reads_them() {
  let doc = Parser::default().parse_string("<r/>").unwrap();
  let root = doc.get_root_element().unwrap();
  let node_ptr = root.node_ptr() as xmlNodePtr;
  let doc_ptr = doc.doc_ptr();
  let names = [c"none", c"nul", c"cd"];
  let mut attrs = Vec::new();
  unsafe {
    let ns = xmlNewNs(
      node_ptr,
      c"urn:p".as_ptr() as *const u8,
      c"p".as_ptr() as *const u8,
    );
    for (prefix, ns) in [("", std::ptr::null_mut()), ("p", ns)] {
      for name in names {
        let qualified = CString::new(format!("{prefix}{}", name.to_str().unwrap())).unwrap();
        let attr = if ns.is_null() {
          xmlNewProp(node_ptr, qualified.as_ptr() as *const u8, std::ptr::null())
        } else {
          xmlNewNsProp(
            node_ptr,
            ns,
            qualified.as_ptr() as *const u8,
            std::ptr::null(),
          )
        };
        assert!(!attr.is_null() && (*attr).children.is_null());
        match name.to_bytes() {
          b"nul" => {
            let text = xmlNewDocText(doc_ptr, std::ptr::null());
            assert!((*text).content.is_null());
            xmlAddChild(attr as xmlNodePtr, text);
          }
          b"cd" => {
            let cdata = xmlNewCDataBlock(doc_ptr, c"a<b".as_ptr() as *const u8, 3);
            xmlAddChild(attr as xmlNodePtr, cdata);
          }
          _ => {}
        }
        attrs.push((qualified.into_string().unwrap(), !ns.is_null(), attr));
      }
    }
  }
  let properties = root.get_properties();
  assert_eq!(properties.len(), attrs.len());
  for (name, namespaced, attr) in &attrs {
    let c_name = CString::new(name.as_str()).unwrap();
    let c_name = c_name.as_bytes_with_nul().as_ptr();
    let c_ns = c"urn:p".as_ptr() as *const u8;
    let content = libxml2_value(unsafe { xmlNodeGetContent(*attr as xmlNodePtr) });
    if *namespaced {
      let expected = libxml2_value(unsafe { xmlGetNsProp(node_ptr, c_name, c_ns) });
      assert_eq!(
        root.get_property_ns(name, "urn:p"),
        expected,
        "get_property_ns {name:?}"
      );
      assert_eq!(
        root.get_property_no_ns(name),
        None,
        "get_property_no_ns {name:?}"
      );
    } else {
      let expected = libxml2_value(unsafe { xmlGetProp(node_ptr, c_name) });
      assert_eq!(root.get_property(name), expected, "get_property {name:?}");
      assert_eq!(
        root.with_property(&CString::new(name.as_str()).unwrap(), str::to_string),
        expected,
        "with_property {name:?}"
      );
      let expected_no_ns = libxml2_value(unsafe { xmlGetNoNsProp(node_ptr, c_name) });
      assert_eq!(
        root.get_property_no_ns(name),
        expected_no_ns,
        "get_property_no_ns {name:?}"
      );
    }
    // (`get_properties` reads each attribute it walks as `xmlNodeGetContent` reads it, a NULL as "")
    assert_eq!(
      properties.get(name.as_str()),
      Some(&content.unwrap_or_default()),
      "get_properties {name:?}"
    );
  }
  assert_eq!(root.get_property("none").as_deref(), Some(""));
  assert_eq!(root.get_property("cd").as_deref(), Some("a<b"));
}

#[test]
fn get_properties_reads_each_value_as_get_property() {
  let doc = shapes_document();
  let root = doc.get_root_element().unwrap();
  let properties = root.get_properties();
  // (the DTD default and the `xmlns:p` declaration are no attributes of the element; `p:q` is keyed by its local name)
  assert_eq!(properties.len(), 8);
  // (each value as libxml2 reads the attribute node itself, `xmlNodeGetContent`, a NULL as "")
  let mut attr = unsafe { (*(root.node_ptr() as xmlNodePtr)).properties };
  let mut walked = 0;
  while !attr.is_null() {
    let name = unsafe { CStr::from_ptr((*attr).name as *const c_char) }
      .to_str()
      .unwrap();
    let content = libxml2_value(unsafe { xmlNodeGetContent(attr as xmlNodePtr) });
    assert_eq!(
      properties.get(name),
      Some(&content.unwrap_or_default()),
      "property {name:?}"
    );
    walked += 1;
    attr = unsafe { (*attr).next };
  }
  assert_eq!(walked, properties.len());
  for ((name, ns), value) in root.get_properties_ns() {
    let expected = match ns {
      Some(ns) => root.get_property_ns(&name, &ns.get_href()),
      None => root.get_property_no_ns(&name),
    };
    assert_eq!(Some(value), expected, "property {name:?}");
  }
}

thread_local! {
  /// libxml2's allocations on this thread, counted once `libxml2_allocations` has first run.
  static LIBXML2_ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

/// The `xmlMalloc` that `counting_malloc` passes each allocation on to.
static LIBXML2_MALLOC: OnceLock<unsafe extern "C" fn(usize) -> *mut c_void> = OnceLock::new();

unsafe extern "C" fn counting_malloc(size: usize) -> *mut c_void {
  let _ = LIBXML2_ALLOCATIONS.try_with(|count| count.set(count.get() + 1));
  unsafe { LIBXML2_MALLOC.get().unwrap()(size) }
}

/// How many times libxml2 calls `xmlMalloc` (or `xmlMallocAtomic`) while `op` runs on this thread. The count is per
/// thread, so tests running alongside do not disturb it.
fn libxml2_allocations<R>(op: impl FnOnce() -> R) -> usize {
  static COUNT: Once = Once::new();
  COUNT.call_once(|| unsafe {
    let (mut free, mut malloc, mut realloc, mut strdup) = (None, None, None, None);
    assert_eq!(
      xmlMemGet(&mut free, &mut malloc, &mut realloc, &mut strdup),
      0
    );
    LIBXML2_MALLOC.set(malloc.unwrap()).unwrap();
    assert_eq!(xmlMemSetup(free, Some(counting_malloc), realloc, strdup), 0);
  });
  let before = LIBXML2_ALLOCATIONS.with(Cell::get);
  drop(std::hint::black_box(op()));
  LIBXML2_ALLOCATIONS.with(Cell::get) - before
}

/// libxml2's getters return NULL whenever their lookup (`xmlHasProp`, `xmlHasNsProp`) does, so an absent property is
/// looked up once, as the matching `has_` method looks it up. A lookup that finds no attribute on the element goes on
/// to the DTD, and here allocates there: it splits the prefixed name, or joins the prefixed element name, and lists
/// the namespaces in scope for a namespaced name. Counting those allocations tells one lookup from two.
#[test]
fn an_absent_property_is_looked_up_once() {
  let doc = Parser::default()
    .parse_string(r#"<!DOCTYPE p:r [<!ATTLIST p:r d CDATA "dflt">]><p:r xmlns:p="urn:p" a="1"/>"#)
    .unwrap();
  let root = doc.get_root_element().unwrap();
  let lookups = [
    (
      "get_property",
      libxml2_allocations(|| root.get_property("p:absent")),
      libxml2_allocations(|| root.has_property("p:absent")),
    ),
    (
      "get_property_ns",
      libxml2_allocations(|| root.get_property_ns("absent", "urn:p")),
      libxml2_allocations(|| root.has_property_ns("absent", "urn:p")),
    ),
    (
      "get_property_no_ns",
      libxml2_allocations(|| root.get_property_no_ns("absent")),
      libxml2_allocations(|| root.has_property_no_ns("absent")),
    ),
  ];
  for (accessor, _, has) in lookups {
    assert!(has > 0, "{accessor}: the lookup allocates nothing to count");
  }
  let looked_up_again: Vec<String> = lookups
    .iter()
    .filter(|(_, get, has)| get != has)
    .map(|(accessor, get, has)| format!("{accessor}: {get} allocations, {has} for one lookup"))
    .collect();
  assert!(
    looked_up_again.is_empty(),
    "an absent property is looked up again: {looked_up_again:?}"
  );
}

/// Names and values on both sides of the 128-byte stack copy are written, read, tested and removed alike.
#[test]
fn set_and_remove_property_with_short_and_long_names_and_values() {
  let doc = Parser::default().parse_string("<r/>").unwrap();
  let mut root = doc.get_root_element().unwrap();
  for name_length in [1, 127, 128, 300] {
    for value_length in [0, 127, 128, 300] {
      let name = format!("n{}", "x".repeat(name_length - 1));
      let value = "v".repeat(value_length);
      root.set_property(&name, &value).unwrap();
      assert_eq!(root.get_property(&name).as_deref(), Some(value.as_str()));
      assert!(root.has_property(&name));
      assert!(root.get_property_node(&name).is_some());
      root.remove_property(&name).unwrap();
      assert!(!root.has_property(&name));
      assert_eq!(root.get_property(&name), None);
    }
  }
  let ns = libxml::tree::Namespace::new("p", "urn:p", &mut root).unwrap();
  let name = "q".repeat(200);
  root.set_property_ns(&name, "nsval", &ns).unwrap();
  assert_eq!(
    root.get_property_ns(&name, "urn:p").as_deref(),
    Some("nsval")
  );
  root.remove_property_ns(&name, "urn:p").unwrap();
  assert!(!root.has_property_ns(&name, "urn:p"));
}

/// A name holding a NUL byte panics, as these methods always have (`CString::new(name).unwrap()`).
#[test]
#[should_panic(expected = "NulError")]
fn get_property_with_a_nul_in_the_name_panics() {
  let doc = Parser::default().parse_string(r#"<r a="1"/>"#).unwrap();
  let root = doc.get_root_element().unwrap();
  let _ = root.get_property("a\0b");
}

#[test]
#[should_panic(expected = "NulError")]
fn set_property_with_a_nul_in_the_value_panics() {
  let doc = Parser::default().parse_string("<r/>").unwrap();
  let mut root = doc.get_root_element().unwrap();
  let _ = root.set_property("a", "x\0y");
}

#[test]
fn child_lists_keep_document_order() {
  let doc = Parser::default()
    .parse_string("<r>t1<a/><!--c--><b>in</b>t2<c/></r>")
    .unwrap();
  let root = doc.get_root_element().unwrap();
  let nodes: Vec<Node> = root.get_child_nodes();
  let mut walked = Vec::new();
  let mut child = root.get_first_child();
  while let Some(node) = child {
    child = node.get_next_sibling();
    walked.push(node);
  }
  assert_eq!(nodes, walked);
  let elements: Vec<String> = root
    .get_child_elements()
    .iter()
    .map(|element| element.get_name())
    .collect();
  assert_eq!(elements, ["a", "b", "c"]);
  let filtered: Vec<Node> = nodes
    .into_iter()
    .filter(|node| node.get_type() == Some(NodeType::ElementNode))
    .collect();
  assert_eq!(root.get_child_elements(), filtered);
  let leaf = root.get_first_element_child().unwrap();
  assert!(leaf.get_child_nodes().is_empty());
  assert!(leaf.get_child_elements().is_empty());
}
