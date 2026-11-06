//! `Node::with_property` reads what `Node::get_property` reads, without allocating for the value.

use libxml::parser::Parser;

/// Every shape of value reads the same through both accessors: plain, empty, absent, a predefined entity (`&amp;`, one text node), an
/// unsubstituted entity reference (the copying fallback), a DTD default (the fallback), a namespaced attribute, and values
/// at and past the 128 bytes that fit the stack buffer.
#[test]
fn with_property_matches_get_property() {
  let xml = format!(
    r#"<!DOCTYPE r [<!ENTITY e "EE"><!ATTLIST r d CDATA "dflt">]>
<r a="1234567890123456789" empty="" amp="x&amp;y" ent="x&e;y" xmlns:p="urn:p" p:q="nsval" b128="{}" b129="{}"/>"#,
    "s".repeat(128),
    "h".repeat(129)
  );
  let doc = Parser::default().parse_string(xml).unwrap();
  let root = doc.get_root_element().unwrap();
  for name in [
    "a", "empty", "amp", "ent", "d", "q", "b128", "b129", "absent",
  ] {
    let c_name = std::ffi::CString::new(name).unwrap();
    assert_eq!(
      root.with_property(&c_name, |value| value.to_string()),
      root.get_property(name),
      "property {name:?}"
    );
    assert_eq!(
      root.with_attribute(&c_name, str::len),
      root.get_attribute(name).map(|value| value.len())
    );
  }
  assert_eq!(
    root.with_property(c"a", |v| v.parse::<u64>().ok()),
    Some(Some(1234567890123456789))
  );
  assert_eq!(
    root.with_property(c"ent", str::to_string).as_deref(),
    Some("xEEy")
  );
  // (a DTD default is read through the copying fallback)
  assert_eq!(
    root.with_property(c"d", str::to_string).as_deref(),
    Some("dflt")
  );
}

/// Overwrite every block libxml2 frees, so that reading a freed value sees other bytes instead of the old ones. Only
/// glibc reports the size of a block (`malloc_usable_size`).
#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn poison_freed_memory() {
  use libxml::bindings::{xmlMemGet, xmlMemSetup};
  use std::os::raw::c_void;

  unsafe extern "C" fn poisoning_free(ptr: *mut c_void) {
    if !ptr.is_null() {
      unsafe { std::ptr::write_bytes(ptr as *mut u8, 0xA5, libc::malloc_usable_size(ptr)) };
    }
    unsafe { libc::free(ptr) }
  }

  static POISON: std::sync::Once = std::sync::Once::new();
  POISON.call_once(|| unsafe {
    let (mut free, mut malloc, mut realloc, mut strdup) = (None, None, None, None);
    assert_eq!(
      xmlMemGet(&mut free, &mut malloc, &mut realloc, &mut strdup),
      0
    );
    assert_eq!(
      xmlMemSetup(Some(poisoning_free), malloc, realloc, strdup),
      0
    );
  });
}

/// `f` reads its own copy of the value. A change made while `f` runs, here through the attribute's own node (which frees
/// libxml2's text node), leaves the value `f` holds intact, whether it was copied on the stack or into a `String`.
#[test]
fn with_property_value_survives_a_change_during_the_callback() {
  #[cfg(all(target_os = "linux", target_env = "gnu"))]
  poison_freed_memory();
  for length in [40, 400] {
    let value = "v".repeat(length);
    let doc = Parser::default()
      .parse_string(format!(r#"<r a="{value}"/>"#))
      .unwrap();
    let root = doc.get_root_element().unwrap();
    let (read, added) = root
      .with_property(c"a", |read| {
        let mut attr = root.get_property_node("a").unwrap();
        attr.set_content("changed").unwrap();
        // (bytes: a freed value need not be UTF-8 any more)
        let read = read.as_bytes().to_vec();
        let mut element = root.clone();
        (read, element.set_property("b", "added").is_ok())
      })
      .unwrap();
    assert!(
      read == value.as_bytes(),
      "length {length}: the value read was freed"
    );
    assert_eq!(root.get_property("a").as_deref(), Some("changed"));
    assert!(added, "the element itself may change too");
    assert_eq!(root.get_property("b").as_deref(), Some("added"));
  }
}
