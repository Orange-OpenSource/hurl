//! Document feature set
//!
use libc::{c_char, c_int};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr;
use std::rc::{Rc, Weak};
use std::str;

use crate::bindings::*;
use crate::readonly::RoNode;
use crate::tree::node::Node;

pub(crate) type DocumentRef = Rc<RefCell<_Document>>;
pub(crate) type DocumentWeak = Weak<RefCell<_Document>>;

/// Fast, dependency-free hasher for the pointer-keyed `nodes` cache. libxml node
/// pointers are unique and well-distributed, so SipHash's `RandomState` (the std
/// default) is wasted work on a map that is probed on EVERY `Node::wrap` /
/// lookup. This FxHash-style multiply-rotate mixes the address in a couple of
/// instructions. Internal to the node bookkeeping map; not public API.
#[derive(Default, Clone, Copy)]
pub(crate) struct BuildNodePtrHasher;
impl std::hash::BuildHasher for BuildNodePtrHasher {
  type Hasher = NodePtrHasher;
  #[inline]
  fn build_hasher(&self) -> NodePtrHasher {
    NodePtrHasher(0)
  }
}
#[doc(hidden)]
pub(crate) struct NodePtrHasher(u64);
impl NodePtrHasher {
  #[inline]
  fn add(&mut self, i: u64) {
    self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517c_c1b7_2722_0a95);
  }
}
impl std::hash::Hasher for NodePtrHasher {
  #[inline]
  fn finish(&self) -> u64 {
    self.0
  }
  #[inline]
  fn write_usize(&mut self, i: usize) {
    self.add(i as u64);
  }
  #[inline]
  fn write_u64(&mut self, i: u64) {
    self.add(i);
  }
  fn write(&mut self, bytes: &[u8]) {
    for chunk in bytes.chunks(8) {
      let mut buf = [0u8; 8];
      buf[..chunk.len()].copy_from_slice(chunk);
      self.add(u64::from_le_bytes(buf));
    }
  }
}

/// The `_Document::nodes` bookkeeping map: `xmlNodePtr` → wrapped `Node`, hashed
/// with [`BuildNodePtrHasher`] rather than the std SipHash default.
pub(crate) type NodeMap = HashMap<xmlNodePtr, Node, BuildNodePtrHasher>;

#[cfg(test)]
mod node_ptr_hasher_tests {
  use super::BuildNodePtrHasher;
  use std::collections::HashSet;
  use std::hash::{BuildHasher, Hasher};

  fn hash_ptr(addr: usize) -> u64 {
    // Mirrors the key path: a thin raw pointer's `Hash` calls `write_usize`.
    let mut h = BuildNodePtrHasher.build_hasher();
    h.write_usize(addr);
    h.finish()
  }

  #[test]
  fn deterministic() {
    assert_eq!(hash_ptr(0x7f00_1234_5000), hash_ptr(0x7f00_1234_5000));
  }

  #[test]
  fn no_collisions_on_aligned_allocator_run() {
    // libxml node structs are large + aligned, so real addresses come in
    // aligned runs — the case a weak hash would clump. Assert the FxHash
    // mix keeps 10k such keys collision-free (guards against a degenerate
    // edit to the algorithm/constant).
    let base = 0x7f00_0000_0000usize;
    let mut seen = HashSet::new();
    for i in 0..10_000usize {
      assert!(seen.insert(hash_ptr(base + i * 160)), "collision at i={i}");
    }
  }
}

#[derive(Debug, Copy, Clone, Default)]
/// Save Options for Document
pub struct SaveOptions {
  /// format save output
  pub format: bool,
  /// drop the xml declaration
  pub no_declaration: bool,
  /// no empty tags
  pub no_empty_tags: bool,
  /// disable XHTML1 specific rules
  pub no_xhtml: bool,
  /// force XHTML1 specific rules
  pub xhtml: bool,
  /// force XML serialization on HTML doc
  pub as_xml: bool,
  /// force HTML serialization on XML doc
  pub as_html: bool,
  /// format with non-significant whitespace
  pub non_significant_whitespace: bool,
}

#[derive(Debug)]
pub(crate) struct _Document {
  /// pointer to a libxml document
  pub(crate) doc_ptr: xmlDocPtr,
  /// hashed pointer-to-Node bookkeeping table
  nodes: NodeMap,
}

impl _Document {
  /// Internal bookkeeping function, so far only used by `Node::wrap`
  pub(crate) fn insert_node(&mut self, node_ptr: xmlNodePtr, node: Node) {
    self.nodes.insert(node_ptr, node);
  }
  /// Internal bookkeeping function, so far only used by `Node::wrap`
  pub(crate) fn get_node(&self, node_ptr: xmlNodePtr) -> Option<&Node> {
    self.nodes.get(&node_ptr)
  }
  /// Internal bookkeeping function
  pub(crate) fn forget_node(&mut self, node_ptr: xmlNodePtr) {
    self.nodes.remove(&node_ptr);
  }
}

/// A libxml2 Document
#[derive(Clone)]
pub struct Document(pub(crate) DocumentRef);

impl Drop for _Document {
  ///Free document when it goes out of scope
  fn drop(&mut self) {
    unsafe {
      if !self.doc_ptr.is_null() {
        xmlFreeDoc(self.doc_ptr);
      }
    }
  }
}

impl fmt::Display for Document {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string_with_options(SaveOptions::default()))
  }
}

impl Document {
  /// Creates a new empty libxml2 document
  pub fn new() -> Result<Self, ()> {
    crate::init_parser();
    unsafe {
      let c_version = CString::new("1.0").unwrap();
      let c_version_bytes = c_version.as_bytes();
      let doc_ptr = xmlNewDoc(c_version_bytes.as_ptr());
      if doc_ptr.is_null() {
        Err(())
      } else {
        let doc = _Document {
          doc_ptr,
          nodes: NodeMap::default(),
        };
        Ok(Document(Rc::new(RefCell::new(doc))))
      }
    }
  }

  /// Obtain the underlying libxml2 `xmlDocPtr` for this Document
  pub fn doc_ptr(&self) -> xmlDocPtr {
    self.0.borrow().doc_ptr
  }

  /// Creates a new `Document` from an existing libxml2 pointer
  pub fn new_ptr(doc_ptr: xmlDocPtr) -> Self {
    let doc = _Document {
      doc_ptr,
      nodes: NodeMap::default(),
    };
    Document(Rc::new(RefCell::new(doc)))
  }

  pub(crate) fn null_ref() -> DocumentRef {
    Rc::new(RefCell::new(_Document {
      doc_ptr: ptr::null_mut(),
      nodes: NodeMap::default(),
    }))
  }

  /// Write document to `filename`
  pub fn save_file(&self, filename: &str) -> Result<c_int, ()> {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
      let retval = xmlSaveFile(c_filename.as_ptr(), self.doc_ptr());
      if retval < 0 {
        return Err(());
      }
      Ok(retval)
    }
  }

  pub(crate) fn register_node(&self, node_ptr: xmlNodePtr) -> Node {
    Node::wrap(node_ptr, &self.0)
  }

  /// Get the root element of the document
  pub fn get_root_element(&self) -> Option<Node> {
    unsafe {
      let node_ptr = xmlDocGetRootElement(self.doc_ptr());
      if node_ptr.is_null() {
        None
      } else {
        Some(self.register_node(node_ptr))
      }
    }
  }

  /// Get the root element of the document (read-only)
  pub fn get_root_readonly(&self) -> Option<RoNode> {
    unsafe {
      let node_ptr = xmlDocGetRootElement(self.doc_ptr());
      if node_ptr.is_null() {
        None
      } else {
        Some(RoNode(node_ptr))
      }
    }
  }

  /// Sets the root element of the document
  pub fn set_root_element(&mut self, root: &Node) {
    unsafe {
      xmlDocSetRootElement(self.doc_ptr(), root.node_ptr());
    }
    root.set_linked();
  }

  /// Remove the internal DTD subset (the `<!DOCTYPE …>` declaration)
  /// from this document, if any. Mirrors XML::LibXML's
  /// `Document::removeInternalSubset` (Perl) and the effect of
  /// libxml2's `xmlSetIntSubset(doc, NULL)` — subsequent
  /// serialisation no longer emits the DOCTYPE preamble.
  ///
  /// Safe to call on a document with no internal subset (no-op).
  /// Unlinks the DTD node from the document and frees it via
  /// `xmlFreeDtd`.
  pub fn remove_internal_subset(&mut self) {
    unsafe {
      let dtd = xmlGetIntSubset(self.doc_ptr());
      if !dtd.is_null() {
        xmlUnlinkNode(dtd as xmlNodePtr);
        xmlFreeDtd(dtd);
      }
    }
  }

  fn ptr_as_result(&mut self, node_ptr: xmlNodePtr) -> Result<Node, ()> {
    if node_ptr.is_null() {
      Err(())
    } else {
      let node = self.register_node(node_ptr);
      Ok(node)
    }
  }

  /// Import a `Node` from another `Document`
  ///
  /// The source `node` must be `Unlinked` (detached from its origin
  /// document tree). Calling on a `Linked` source returns `Err(())`,
  /// matching the long-standing contract of this method. Calling on a
  /// `RustOwned` source also returns `Err(())`: the source has been
  /// claimed by the Rust wrapper and re-linking it would set up a
  /// double-free. Drop the source's wrapper first if you genuinely
  /// want to discard it.
  pub fn import_node(&mut self, node: &mut Node) -> Result<Node, ()> {
    if !node.is_unlinked() || node.is_rust_owned() {
      return Err(());
    }
    // Also remove this node from the prior document hash
    node
      .get_docref()
      .upgrade()
      .unwrap()
      .borrow_mut()
      .forget_node(node.node_ptr());

    let node_ptr = unsafe { xmlDocCopyNode(node.node_ptr(), self.doc_ptr(), 1) };
    node.set_linked();
    self.ptr_as_result(node_ptr)
  }

  /// Build a fresh `Document` whose root is a deep copy of `node`'s subtree.
  ///
  /// Unlike [`Document::import_node`], this does not require the source
  /// node to be unlinked and does not mutate the source node's wrapper
  /// state. It is suitable for code that repeatedly extracts subtrees
  /// from a single source document and needs each extracted subtree as
  /// its own independently-owned `Document` — a pattern that the older
  /// `import_node` route handles poorly:
  ///
  /// * `import_node` gates on `Node::is_unlinked()`, a wrapper-side flag
  ///   with no public setter; the gate flips to `false` as a side
  ///   effect of a previous successful import (`set_linked()` mutates
  ///   the borrowed wrapper Rc), so every subsequent extract errors.
  /// * A bare `xmlDocCopyNode(src, dst, 1)` returns NULL on the second
  ///   sibling in the same source document, because the recursive
  ///   descent relies on dictionary state that the first copy has
  ///   marked dirty.
  ///
  /// This method works as follows:
  /// 1. `xmlNewDoc` — fresh empty target document.
  /// 2. `xmlDocCopyNode(node, target, 1)` — recursive copy of the
  ///    source subtree into the target, with libxml2 handling
  ///    namespace inheritance (`xmlNewReconciliedNs`) during the copy.
  /// 3. `xmlDocSetRootElement` + `xmlSetTreeDoc` — plant the copy as
  ///    the new root and retarget every doc pointer in the subtree.
  /// 4. `xmlReconciliateNs` — final pass that lifts any remaining
  ///    namespace declarations into the new document so it owns 100%
  ///    of its ns nodes.
  ///
  /// The returned `Document` shares no C-side state with the source —
  /// dropping either is independent of the other.
  ///
  /// Returns `Err(())` if any of the underlying libxml2 calls returns
  /// NULL (typically OOM, or `node` is itself NULL).
  pub fn dup_node_into_new_doc(node: &Node) -> Result<Document, ()> {
    let copy_ptr = unsafe { xmlCopyNode(node.node_ptr(), 1) };
    if copy_ptr.is_null() {
      return Err(());
    }
    let doc_ptr = unsafe {
      let c_version = CString::new("1.0").unwrap();
      xmlNewDoc(c_version.as_bytes().as_ptr())
    };
    if doc_ptr.is_null() {
      unsafe { xmlFreeNode(copy_ptr) };
      return Err(());
    }
    unsafe {
      xmlDocSetRootElement(doc_ptr, copy_ptr);
      // DEBUG: omit xmlSetTreeDoc + xmlReconciliateNs.
    }
    // The source node's wrapper state is left untouched. The new
    // `_Node::drop` rules already prevent a UAF on the source: a
    // detached subtree whose `node->doc` still points at the source
    // document is treated as doc-owned and not freed by the wrapper.
    // If a caller wants the source node's C allocation reclaimed
    // (because the source doc tree-walk won't reach an unlinked
    // subtree), they should call `Node::set_rust_owned` on the
    // source after this returns.
    Ok(Document::new_ptr(doc_ptr))
  }
  /// Serializes the `Document` with options
  pub fn to_string_with_options(&self, options: SaveOptions) -> String {
    unsafe {
      // allocate a buffer to dump into
      let buf = xmlBufferCreate();
      let c_utf8 = CString::new("UTF-8").unwrap();
      let mut xml_options = 0;

      if options.format {
        xml_options += xmlSaveOption_XML_SAVE_FORMAT;
      }
      if options.no_declaration {
        xml_options += xmlSaveOption_XML_SAVE_NO_DECL;
      }
      if options.no_empty_tags {
        xml_options += xmlSaveOption_XML_SAVE_NO_EMPTY;
      }
      if options.no_xhtml {
        xml_options += xmlSaveOption_XML_SAVE_NO_XHTML;
      }
      if options.xhtml {
        xml_options += xmlSaveOption_XML_SAVE_XHTML;
      }
      if options.as_xml {
        xml_options += xmlSaveOption_XML_SAVE_AS_XML;
      }
      if options.as_html {
        xml_options += xmlSaveOption_XML_SAVE_AS_HTML;
      }
      if options.non_significant_whitespace {
        xml_options += xmlSaveOption_XML_SAVE_WSNONSIG;
      }

      let save_ctx = xmlSaveToBuffer(buf, c_utf8.as_ptr(), xml_options as i32);
      let _size = xmlSaveDoc(save_ctx, self.doc_ptr());
      let _size = xmlSaveClose(save_ctx);

      let result = xmlBufferContent(buf);
      // `xmlBufferContent` returns NULL when the buffer never allocated its
      // backing store — e.g. `xmlBufferCreate` returned NULL under memory
      // pressure, a real event on a many-worker fleet. `CStr::from_ptr(NULL)`
      // would then `strlen(NULL)` → SIGSEGV inside libc; degrade to an empty
      // serialization instead (the buffer handle is still safe to free).
      if result.is_null() {
        xmlBufferFree(buf);
        return String::new();
      }
      let c_string = CStr::from_ptr(result as *const c_char);
      let node_string = c_string.to_string_lossy().into_owned();
      xmlBufferFree(buf);

      node_string
    }
  }

  /// Serializes a `Node` owned by this `Document`
  pub fn node_to_string(&self, node: &Node) -> String {
    unsafe {
      // allocate a buffer to dump into
      let buf = xmlBufferCreate();

      // dump the node
      xmlNodeDump(
        buf,
        self.doc_ptr(),
        node.node_ptr(),
        1, // level of indentation
        0, /* disable formatting */
      );
      let result = xmlBufferContent(buf);
      // `xmlBufferContent` returns NULL when the buffer never allocated its
      // backing store — e.g. `xmlBufferCreate` returned NULL under memory
      // pressure, a real event on a many-worker fleet. `CStr::from_ptr(NULL)`
      // would then `strlen(NULL)` → SIGSEGV inside libc; degrade to an empty
      // serialization instead (the buffer handle is still safe to free).
      if result.is_null() {
        xmlBufferFree(buf);
        return String::new();
      }
      let c_string = CStr::from_ptr(result as *const c_char);
      let node_string = c_string.to_string_lossy().into_owned();
      xmlBufferFree(buf);

      node_string
    }
  }
  /// Serializes a `RoNode` owned by this `Document`
  pub fn ronode_to_string(&self, node: &RoNode) -> String {
    unsafe {
      // allocate a buffer to dump into
      let buf = xmlBufferCreate();

      // dump the node
      xmlNodeDump(
        buf,
        self.doc_ptr(),
        node.node_ptr(),
        1, // level of indentation
        0, /* disable formatting */
      );
      let result = xmlBufferContent(buf);
      // `xmlBufferContent` returns NULL when the buffer never allocated its
      // backing store — e.g. `xmlBufferCreate` returned NULL under memory
      // pressure, a real event on a many-worker fleet. `CStr::from_ptr(NULL)`
      // would then `strlen(NULL)` → SIGSEGV inside libc; degrade to an empty
      // serialization instead (the buffer handle is still safe to free).
      if result.is_null() {
        xmlBufferFree(buf);
        return String::new();
      }
      let c_string = CStr::from_ptr(result as *const c_char);
      let node_string = c_string.to_string_lossy().into_owned();
      xmlBufferFree(buf);

      node_string
    }
  }

  /// Creates a node for an XML processing instruction
  pub fn create_processing_instruction(&mut self, name: &str, content: &str) -> Result<Node, ()> {
    unsafe {
      let c_name = CString::new(name).unwrap();
      let c_name_bytes = c_name.as_bytes();
      let c_content = CString::new(content).unwrap();
      let c_content_bytes = c_content.as_bytes();

      let node_ptr: xmlNodePtr = xmlNewDocPI(
        self.doc_ptr(),
        c_name_bytes.as_ptr(),
        c_content_bytes.as_ptr(),
      );
      if node_ptr.is_null() {
        Err(())
      } else {
        Ok(self.register_node(node_ptr))
      }
    }
  }

  /// Cast the document as a libxml Node
  pub fn as_node(&self) -> Node {
    // Note: this method is important to keep, as it enables certain low-level libxml2 idioms
    // In particular, method dispatch based on NodeType is only possible when the document can be cast as a Node
    //
    // Memory management is not an issue, as a document node can not be unbound/removed, and does not require
    // any additional deallocation than the Drop of a Document object.
    self.register_node(self.doc_ptr() as xmlNodePtr)
  }

  /// Duplicates the libxml2 Document into a new instance
  pub fn dup(&self) -> Result<Self, ()> {
    let doc_ptr = unsafe { xmlCopyDoc(self.doc_ptr(), 1) };
    if doc_ptr.is_null() {
      Err(())
    } else {
      let doc = _Document {
        doc_ptr,
        nodes: NodeMap::default(),
      };
      Ok(Document(Rc::new(RefCell::new(doc))))
    }
  }

  /// Duplicates a source libxml2 Document into the empty Document self
  pub fn dup_from(&mut self, source: &Self) -> Result<(), ()> {
    if !self.doc_ptr().is_null() {
      return Err(());
    }

    let doc_ptr = unsafe { xmlCopyDoc(source.doc_ptr(), 1) };
    if doc_ptr.is_null() {
      return Err(());
    }
    self.0.borrow_mut().doc_ptr = doc_ptr;
    Ok(())
  }
}

mod c14n;
