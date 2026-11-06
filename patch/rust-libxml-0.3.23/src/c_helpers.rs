#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bindings::*;
use libc::{c_char, c_int, size_t};
use std::os::raw::c_void;
use std::ptr;
use std::slice;
// error handling functions
// pub fn xmlSetGenericErrorFunc(ctx: *mut c_void, handler: *mut c_void);
// pub fn xmlThrDefSetGenericErrorFunc(ctx: *mut c_void, handler: *mut c_void);

// Taken from Nokogiri (https://github.com/sparklemotion/nokogiri/blob/24bb843327306d2d71e4b2dc337c1e327cbf4516/ext/nokogiri/xml_document.c#L64)
pub fn xmlNodeRecursivelyRemoveNs(node: xmlNodePtr) {
  if node.is_null() {
    return;
  }
  unsafe {
    let mut property: xmlAttrPtr;

    xmlSetNs(node, ptr::null_mut());
    let mut child: xmlNodePtr = (*node).children;
    while !child.is_null() {
      xmlNodeRecursivelyRemoveNs(child);
      child = (*child).next;
    }

    if (((*node).type_ == xmlElementType_XML_ELEMENT_NODE)
      || ((*node).type_ == xmlElementType_XML_XINCLUDE_START)
      || ((*node).type_ == xmlElementType_XML_XINCLUDE_END))
      && !(*node).nsDef.is_null()
    {
      xmlFreeNsList((*node).nsDef);
      (*node).nsDef = ptr::null_mut();
    }

    if (*node).type_ == xmlElementType_XML_ELEMENT_NODE && !(*node).properties.is_null() {
      property = (*node).properties;
      while !property.is_null() {
        if !(*property).ns.is_null() {
          (*property).ns = ptr::null_mut();
        }
        property = (*property).next;
      }
    }
  }
}
// Null-safety convention for the field accessors below.
//
// Each of these reads a single field out of a libxml2 struct via a raw
// dereference. A NULL input pointer therefore reads that field's offset off
// address 0 — e.g. `xmlGetNodeType(NULL)` reads `(*NULL).type_` at offset 8,
// the exact `segfault at 8` observed in production. A NULL node is a legal,
// reachable value (a placeholder `Node::null()`, an unlinked/adopted-away
// wrapper, an empty navigation result), so a raw deref here turns an ordinary
// "no node" into a hard SIGSEGV that bypasses `catch_unwind` and kills the
// whole process. Every accessor guards its pointer and returns the natural
// "absence" sentinel instead (a null pointer, or 0 for the element type, which
// `NodeType::from_int` maps to `None`). Callers already treat those sentinels
// as "no node", so this only ever converts a crash into the pre-existing
// no-node path — never a behavioural change on a valid pointer.
pub fn xmlGetDoc(cur: xmlNodePtr) -> xmlDocPtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).doc }
}
pub fn xmlNextNsSibling(ns: xmlNsPtr) -> xmlNsPtr {
  if ns.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*ns).next }
}
pub fn xmlNsPrefix(ns: xmlNsPtr) -> *const c_char {
  if ns.is_null() {
    return ptr::null();
  }
  unsafe { (*ns).prefix as *const c_char }
}
pub fn xmlNsHref(ns: xmlNsPtr) -> *const c_char {
  if ns.is_null() {
    return ptr::null();
  }
  unsafe { (*ns).href as *const c_char }
}
pub fn xmlNodeNsDeclarations(cur: xmlNodePtr) -> xmlNsPtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).nsDef }
}
pub fn xmlNodeNs(cur: xmlNodePtr) -> xmlNsPtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).ns }
}

pub fn xmlNextPropertySibling(attr: xmlAttrPtr) -> xmlAttrPtr {
  if attr.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*attr).next }
}
pub fn xmlAttrName(attr: xmlAttrPtr) -> *const c_char {
  if attr.is_null() {
    return ptr::null();
  }
  unsafe { (*attr).name as *const c_char }
}
pub fn xmlAttrNs(attr: xmlAttrPtr) -> xmlNsPtr {
  if attr.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*attr).ns }
}
pub fn xmlGetFirstProperty(node: xmlNodePtr) -> xmlAttrPtr {
  if node.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*node).properties }
}
pub fn xmlGetNodeType(cur: xmlNodePtr) -> xmlElementType {
  // 0 is not a valid xmlElementType; `NodeType::from_int(0)` returns `None`.
  if cur.is_null() {
    return 0;
  }
  unsafe { (*cur).type_ }
}

pub fn xmlGetParent(cur: xmlNodePtr) -> xmlNodePtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).parent }
}
pub fn xmlGetFirstChild(cur: xmlNodePtr) -> xmlNodePtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).children }
}
pub fn xmlPrevSibling(cur: xmlNodePtr) -> xmlNodePtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).prev }
}

// helper for tree
pub fn xmlNextSibling(cur: xmlNodePtr) -> xmlNodePtr {
  if cur.is_null() {
    return ptr::null_mut();
  }
  unsafe { (*cur).next }
}

pub fn xmlNodeGetName(cur: xmlNodePtr) -> *const c_char {
  if cur.is_null() {
    return ptr::null();
  }
  unsafe { (*cur).name as *const c_char }
}

// dummy function: no debug output at all
// helper for xpath
pub fn xmlXPathObjectNumberOfNodes(val: xmlXPathObjectPtr) -> c_int {
  unsafe {
    if val.is_null() {
      -1
    } else if (*val).nodesetval.is_null() {
      -2
    } else {
      (*(*val).nodesetval).nodeNr
    }
  }
}

pub fn xmlXPathObjectGetNodes(val: xmlXPathObjectPtr, size: size_t) -> Vec<xmlNodePtr> {
  unsafe {
    // Guard both the object and its node-set: a NULL either way (empty/failed
    // XPath) would deref off address 0 rather than yield an empty result.
    if val.is_null() || (*val).nodesetval.is_null() {
      return Vec::new();
    }
    slice::from_raw_parts((*(*val).nodesetval).nodeTab, size).to_vec()
  }
}

#[cfg(any(
  target_family = "unix",
  target_os = "macos",
  all(target_family = "windows", target_env = "gnu")
))]
pub fn bindgenFree(val: *mut c_void) {
  unsafe {
    if let Some(xml_free_fn) = xmlFree {
      xml_free_fn(val);
    } else {
      libc::free(val);
    }
  }
}
#[cfg(all(target_family = "windows", target_env = "msvc"))]
pub fn bindgenFree(val: *mut c_void) {
  unsafe {
    libc::free(val as *mut c_void);
  }
}
