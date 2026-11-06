//! Node canonicalization logic
//!
use std::ffi::c_void;

use crate::{
  bindings::{xmlC14NIsVisibleCallback, xmlElementType, xmlNodePtr},
  c_helpers::xmlGetNodeType,
  tree::{Node, c14n::*},
};

use super::node_ancestors;

impl Node {
  /// Canonicalize a document and return the results.
  pub fn canonicalize(&mut self, options: CanonicalizationOptions) -> Result<String, ()> {
    let doc_ref = self.get_docref().upgrade().unwrap();
    let document = crate::tree::Document(doc_ref);

    let user_data = self.node_ptr_mut().unwrap();
    let callback: xmlC14NIsVisibleCallback = Some(callback_wrapper);

    document.canonicalize(options, Some((user_data, callback)))
  }
}

unsafe extern "C" fn callback_wrapper(
  c14n_root_ptr: *mut c_void,
  node_ptr: xmlNodePtr,
  parent_ptr: xmlNodePtr,
) -> ::std::os::raw::c_int {
  let c14n_root_ptr = c14n_root_ptr as xmlNodePtr;
  let node_type = xmlGetNodeType(node_ptr);

  let tn_ptr = if C14_NODE_TYPES.contains(&node_type) {
    node_ptr
  } else {
    parent_ptr
  };

  let tn_ancestors = node_ancestors(tn_ptr);

  let ret = (tn_ptr == c14n_root_ptr) || tn_ancestors.contains(&c14n_root_ptr);
  if ret { 1 } else { 0 }
}

const C14_NODE_TYPES: [xmlElementType; 7] = [
  super::xmlElementType_XML_ELEMENT_NODE,
  super::xmlElementType_XML_ATTRIBUTE_NODE,
  super::xmlElementType_XML_DOCUMENT_TYPE_NODE,
  super::xmlElementType_XML_TEXT_NODE,
  super::xmlElementType_XML_DTD_NODE,
  super::xmlElementType_XML_PI_NODE,
  super::xmlElementType_XML_COMMENT_NODE,
];
