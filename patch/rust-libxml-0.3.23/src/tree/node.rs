//! Node, and related, feature set
//!
use libc::{c_char, c_void};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};
use std::ptr;
use std::rc::Rc;
use std::str;

use crate::bindings::*;
use crate::c_helpers::*;
use crate::tree::namespace::Namespace;
use crate::tree::nodetype::NodeType;
use crate::tree::{Document, DocumentRef, DocumentWeak};
use crate::xpath::Context;

/// Deprecated, unused. Formerly the `Rc::strong_count` threshold consulted by
/// [`Node::node_ptr_mut`]. That heuristic was replaced by a
/// `RefCell::try_borrow_mut` check (a high clone count is not an aliasing
/// conflict), so this value is no longer read. Retained only for API
/// compatibility.
#[deprecated(note = "no longer used; node_ptr_mut now relies on RefCell::try_borrow_mut")]
pub static mut NODE_RC_MAX_GUARD: usize = 2;

/// Deprecated no-op. The strong-count guard it tuned was removed in favor of a
/// `RefCell::try_borrow_mut` check in [`Node::node_ptr_mut`]; calling this has
/// no effect. Retained so existing callers continue to compile.
#[deprecated(note = "no-op: node_ptr_mut now relies on RefCell::try_borrow_mut")]
pub fn set_node_rc_guard(_value: usize) {}

type NodeRef = Rc<RefCell<_Node>>;

/// Lifecycle state of a wrapped libxml2 node.
///
/// The three variants correspond to disjoint lifetime-ownership regimes
/// that drive `_Node`'s `Drop` behavior:
///
/// * `Linked` — the node is attached to its document tree. The owning
///   document's `xmlFreeDoc` will reclaim it; the wrapper must not free.
/// * `Unlinked` — `xmlUnlinkNode` has detached the node from its
///   parent/siblings, but `node->doc` still points at the source xmlDoc.
///   The source still owns the C allocation. The wrapper must not free
///   unless `node->doc` itself is NULL (a true C-level orphan).
/// * `RustOwned` — the caller has explicitly transferred ownership to
///   the Rust wrapper via `Node::set_rust_owned`. The C allocation will
///   be freed via `xmlFreeNode` when the last `Node` clone drops,
///   regardless of `node->doc`.
///
/// The state-space is intentionally smaller than `(unlinked: bool,
/// rust_owned: bool)`: `(Linked, RustOwned)` is meaningless, so we make
/// it unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Linkage {
  Linked,
  Unlinked,
  RustOwned,
}

#[derive(Debug)]
struct _Node {
  /// libxml's xmlNodePtr
  node_ptr: xmlNodePtr,
  /// Reference to parent `Document`
  document: DocumentWeak,
  /// Lifecycle state — see `Linkage` for the semantics of each variant.
  linkage: Linkage,
}

/// An xml node
#[derive(Clone, Debug)]
pub struct Node(NodeRef);

impl Hash for Node {
  /// Generates a hash value from the `node_ptr` value.
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.node_ptr().hash(state);
  }
}

impl PartialEq for Node {
  /// Two nodes are considered equal, if they point to the same xmlNode.
  fn eq(&self, other: &Node) -> bool {
    std::ptr::eq(self.node_ptr(), other.node_ptr())
  }
}

impl Eq for Node {}

impl Drop for _Node {
  /// Free the C node if and only if it has been detached from any
  /// libxml2 document. The historical heuristic — "free if the wrapper
  /// is `unlinked`" — conflates two distinct states:
  ///   * tree-detached (parent==NULL, but doc still points at the
  ///     source xmlDoc and that xmlDoc still owns the allocation), and
  ///   * Rust-owned (no document references the node; the wrapper
  ///     itself must call xmlFreeNode or memory leaks).
  ///
  /// Detaching a node from its parent (xmlUnlinkNode) does NOT clear
  /// node->doc — the document still owns the underlying memory and its
  /// xmlFreeDoc will reclaim it. When the wrapper drops, firing
  /// xmlFreeNode in that situation is a use-after-free in waiting:
  /// any other Rust wrapper or C-side reference to the same xmlNodePtr
  /// (a sibling's prev/next, the document's idtable, an ID lookup that
  /// hasn't run yet) sees freed memory.
  ///
  /// We now key the free on the C-level node->doc field: if it's NULL
  /// the node is truly orphan and we own it; otherwise the source
  /// document is the lifetime owner. This matches what xmlAddChild /
  /// xmlDocSetRootElement / xmlSetTreeDoc actually do — they install a
  /// doc pointer to claim ownership, and xmlUnlinkNode preserves that
  /// pointer because it only severs sibling/parent links.
  ///
  /// Real-world driver: a host application that detaches sibling
  /// subtrees from a still-live source document, copies them out via
  /// xmlCopyNode, and then drops the wrapper for each detached
  /// subtree. Under the old `unlinked`-based rule, every wrapper drop
  /// fired xmlFreeNode against memory that the source doc still
  /// considered live, corrupting its dictionary and causing the next
  /// xmlCopyNode against a sibling to return NULL.
  fn drop(&mut self) {
    let node_ptr = self.node_ptr;
    if node_ptr.is_null() {
      return;
    }
    match self.linkage {
      // Source document owns the allocation; `xmlFreeDoc` will reclaim
      // it. Firing `xmlFreeNode` here would be a use-after-free in
      // waiting (sibling/parent links, ID table, prev clones).
      Linkage::Linked => {}
      // Detached, but `node->doc` still points at the source. Free only
      // if `node->doc` is NULL — a true C-level orphan no document will
      // reach via tree walks.
      Linkage::Unlinked => {
        let still_doc_owned = unsafe {
          let n: *const crate::bindings::_xmlNode = node_ptr as *const _;
          !(*n).doc.is_null()
        };
        if !still_doc_owned {
          unsafe { xmlFreeNode(node_ptr) };
        }
      }
      // Caller took ownership; the wrapper is the sole lifetime owner.
      Linkage::RustOwned => unsafe { xmlFreeNode(node_ptr) },
    }
  }
}

impl Node {
  /// Create a new node, bound to a given document.
  pub fn new(name: &str, ns: Option<Namespace>, doc: &Document) -> Result<Self, ()> {
    // We will only allow to work with document-bound nodes for now, to avoid the problems of memory management.

    let c_name = CString::new(name).unwrap();
    let ns_ptr = match ns {
      None => ptr::null_mut(),
      Some(ns) => ns.ns_ptr(),
    };
    unsafe {
      let node = xmlNewDocRawNode(
        doc.doc_ptr(),
        ns_ptr,
        c_name.as_bytes().as_ptr(),
        ptr::null(),
      );
      if node.is_null() {
        Err(())
      } else {
        Ok(Node::wrap_new(node, &doc.0))
      }
    }
  }

  /// Immutably borrows the underlying libxml2 `xmlNodePtr` pointer
  pub fn node_ptr(&self) -> xmlNodePtr {
    self.0.borrow().node_ptr
  }

  /// Mutably borrows the underlying libxml2 `xmlNodePtr`.
  ///
  /// Returns `Err` only when this node's `RefCell` is *actively* borrowed — a
  /// genuine re-entrant aliasing conflict on the same wrapped node. A high
  /// `Rc::strong_count` is not a conflict and does not block the borrow: clones
  /// are normal (the owning document caches one wrapper per node, and callers
  /// hold their own), and all clones share a single `RefCell`, so
  /// `try_borrow_mut` is the exact per-node exclusion check. This replaces the
  /// former `Rc::strong_count <= NODE_RC_MAX_GUARD` heuristic, which rejected
  /// benign clones and could panic (via `borrow_mut`) on the very conflict it
  /// was meant to catch.
  ///
  /// The shared-`RefCell` invariant holds across clones and repeated accessor
  /// calls for a node that is **linked** in the tree (both resolve to the cached
  /// wrapper). It does not extend to an unlinked or imported node: `set_unlinked`
  /// / `import_node` drop the pointer from the cache (its C node may be freed and
  /// the address reused), so re-wrapping it afterwards yields an independent
  /// `RefCell`.
  ///
  /// This guards the Rust wrapper, not the C node: mutations through the returned
  /// raw pointer, and the C node's lifetime, remain the caller's responsibility.
  pub fn node_ptr_mut(&mut self) -> Result<xmlNodePtr, String> {
    match self.0.try_borrow_mut() {
      Ok(inner) => Ok(inner.node_ptr),
      // Do not touch `self.0` here (e.g. `get_name`): it is actively borrowed,
      // so any further borrow would panic.
      Err(_) => {
        Err("Can not mutably reference a Node that is already actively borrowed".to_string())
      }
    }
  }

  /// Wrap a libxml node ptr with a Node
  fn _wrap(node_ptr: xmlNodePtr, unlinked: bool, document: &DocumentRef) -> Node {
    // If already seen, return saved Node
    if let Some(node) = document.borrow().get_node(node_ptr) {
      return node.clone();
    }
    // If newly encountered pointer, wrap
    let node = _Node {
      node_ptr,
      document: Rc::downgrade(document),
      linkage: if unlinked {
        Linkage::Unlinked
      } else {
        Linkage::Linked
      },
    };
    let wrapped_node = Node(Rc::new(RefCell::new(node)));
    document
      .borrow_mut()
      .insert_node(node_ptr, wrapped_node.clone());
    wrapped_node
  }
  /// Wrap a node already linked to a `document` tree
  pub(crate) fn wrap(node_ptr: xmlNodePtr, document: &DocumentRef) -> Node {
    Node::_wrap(node_ptr, false, document)
  }
  /// Wrap, a node owned by, but not yet linked to, a `document`
  pub(crate) fn wrap_new(node_ptr: xmlNodePtr, document: &DocumentRef) -> Node {
    Node::_wrap(node_ptr, true, document)
  }

  /// Create a new text node, bound to a given document
  pub fn new_text(content: &str, doc: &Document) -> Result<Self, ()> {
    // We will only allow to work with document-bound nodes for now, to avoid the problems of memory management.
    let c_content = CString::new(content).unwrap();
    unsafe {
      let node = xmlNewDocText(doc.doc_ptr(), c_content.as_bytes().as_ptr());
      if node.is_null() {
        Err(())
      } else {
        Ok(Node::wrap_new(node, &doc.0))
      }
    }
  }

  /// Create a new comment node, bound to a given document. The returned
  /// node is unlinked — attach it via `add_child`, `add_prev_sibling` or
  /// `add_next_sibling`.
  ///
  /// Returns `Err(())` if the C string cannot be constructed (the comment
  /// contains an embedded NUL), or if libxml2 returns NULL.
  pub fn new_comment(content: &str, doc: &Document) -> Result<Self, ()> {
    let c_content = CString::new(content).map_err(|_| ())?;
    unsafe {
      let node = xmlNewDocComment(doc.doc_ptr(), c_content.as_bytes().as_ptr());
      if node.is_null() {
        Err(())
      } else {
        Ok(Node::wrap_new(node, &doc.0))
      }
    }
  }
  /// Create a mock node, used for a placeholder argument
  pub fn mock(doc: &Document) -> Self {
    Node::new("mock", None, doc).unwrap()
  }

  /// Create a mock node, used for a placeholder argument
  pub fn null() -> Self {
    Node(Rc::new(RefCell::new(_Node {
      node_ptr: ptr::null_mut(),
      document: Rc::downgrade(&Document::null_ref()),
      linkage: Linkage::Unlinked,
    })))
  }

  /// `libc::c_void` isn't hashable and cannot be made hashable
  pub fn to_hashable(&self) -> usize {
    self.node_ptr() as usize
  }

  pub(crate) fn get_docref(&self) -> DocumentWeak {
    self.0.borrow().document.clone()
  }

  /// Returns the next sibling if it exists
  pub fn get_next_sibling(&self) -> Option<Node> {
    let ptr = xmlNextSibling(self.node_ptr());
    self.ptr_as_option(ptr)
  }

  /// Returns the previous sibling if it exists
  pub fn get_prev_sibling(&self) -> Option<Node> {
    let ptr = xmlPrevSibling(self.node_ptr());
    self.ptr_as_option(ptr)
  }

  /// Returns the first child if it exists
  pub fn get_first_child(&self) -> Option<Node> {
    let ptr = xmlGetFirstChild(self.node_ptr());
    self.ptr_as_option(ptr)
  }
  /// Returns the last child if it exists
  pub fn get_last_child(&self) -> Option<Node> {
    let ptr = unsafe { xmlGetLastChild(self.node_ptr()) };
    self.ptr_as_option(ptr)
  }

  /// Returns the next element sibling if it exists
  pub fn get_next_element_sibling(&self) -> Option<Node> {
    match self.get_next_sibling() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          match current_node.get_next_sibling() {
            Some(sibling) => {
              current_node = sibling;
            }
            _ => {
              break;
            }
          }
        }
        if current_node.is_element_node() {
          Some(current_node)
        } else {
          None
        }
      }
    }
  }

  /// Returns the previous element sibling if it exists
  pub fn get_prev_element_sibling(&self) -> Option<Node> {
    match self.get_prev_sibling() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          match current_node.get_prev_sibling() {
            Some(sibling) => {
              current_node = sibling;
            }
            _ => {
              break;
            }
          }
        }
        if current_node.is_element_node() {
          Some(current_node)
        } else {
          None
        }
      }
    }
  }

  /// Returns the first element child if it exists
  pub fn get_first_element_child(&self) -> Option<Node> {
    match self.get_first_child() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          match current_node.get_next_sibling() {
            Some(sibling) => {
              current_node = sibling;
            }
            _ => {
              break;
            }
          }
        }
        if current_node.is_element_node() {
          Some(current_node)
        } else {
          None
        }
      }
    }
  }

  /// Returns the last element child if it exists
  pub fn get_last_element_child(&self) -> Option<Node> {
    match self.get_last_child() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          match current_node.get_prev_sibling() {
            Some(sibling) => {
              current_node = sibling;
            }
            _ => {
              break;
            }
          }
        }
        if current_node.is_element_node() {
          Some(current_node)
        } else {
          None
        }
      }
    }
  }

  /// Returns all child nodes of the given node as a vector
  pub fn get_child_nodes(&self) -> Vec<Node> {
    self.wrap_children(|_| true)
  }

  /// Returns all child elements of the given node as a vector
  pub fn get_child_elements(&self) -> Vec<Node> {
    // (only the elements are wrapped: a text child is never registered just to be filtered out)
    self.wrap_children(|child| xmlGetNodeType(child) == xmlElementType_XML_ELEMENT_NODE)
  }

  /// The children `keep` accepts, in document order, each wrapped once, against one upgrade of the document, into a
  /// vector sized by a first walk over the list.
  fn wrap_children(&self, keep: impl Fn(xmlNodePtr) -> bool) -> Vec<Node> {
    let first = xmlGetFirstChild(self.node_ptr());
    if first.is_null() {
      return Vec::new();
    }
    let mut count = 0;
    let mut child = first;
    while !child.is_null() {
      if keep(child) {
        count += 1;
      }
      child = xmlNextSibling(child);
    }
    let mut children = Vec::with_capacity(count);
    if count > 0 {
      let doc_ref = self.0.borrow().document.upgrade().unwrap();
      let mut child = first;
      while !child.is_null() {
        if keep(child) {
          children.push(Node::wrap(child, &doc_ref));
        }
        child = xmlNextSibling(child);
      }
    }
    children
  }

  /// Returns the parent if it exists
  pub fn get_parent(&self) -> Option<Node> {
    let ptr = xmlGetParent(self.node_ptr());
    self.ptr_as_option(ptr)
  }

  /// Get the node type
  pub fn get_type(&self) -> Option<NodeType> {
    NodeType::from_int(xmlGetNodeType(self.node_ptr()))
  }

  /// Add a previous sibling
  pub fn add_prev_sibling(
    &mut self,
    new_sibling: &mut Node,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    new_sibling.set_linked();
    unsafe {
      if xmlAddPrevSibling(self.node_ptr_mut()?, new_sibling.node_ptr_mut()?).is_null() {
        Err(From::from("add_prev_sibling returned NULL"))
      } else {
        Ok(())
      }
    }
  }

  /// Add a next sibling
  pub fn add_next_sibling(
    &mut self,
    new_sibling: &mut Node,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    new_sibling.set_linked();
    unsafe {
      if xmlAddNextSibling(self.node_ptr_mut()?, new_sibling.node_ptr_mut()?).is_null() {
        Err(From::from("add_next_sibling returned NULL"))
      } else {
        Ok(())
      }
    }
  }

  /// Returns true if it is a text node
  pub fn is_text_node(&self) -> bool {
    self.get_type() == Some(NodeType::TextNode)
  }

  /// Checks if the given node is an Element
  pub fn is_element_node(&self) -> bool {
    self.get_type() == Some(NodeType::ElementNode)
  }

  /// Checks if the underlying libxml2 pointer is `NULL`
  pub fn is_null(&self) -> bool {
    self.node_ptr().is_null()
  }

  /// Returns the name of the node (empty string if name pointer is `NULL`)
  pub fn get_name(&self) -> String {
    let name_ptr = xmlNodeGetName(self.node_ptr());
    if name_ptr.is_null() {
      return String::new();
    } //empty string
    c_str_to_string(unsafe { CStr::from_ptr(name_ptr) })
  }

  /// Sets the name of this `Node`
  pub fn set_name(&mut self, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let c_name = CString::new(name).unwrap();
    unsafe { xmlNodeSetName(self.node_ptr_mut()?, c_name.as_bytes().as_ptr()) }
    Ok(())
  }

  /// Returns the content of the node
  /// (assumes UTF-8 XML document)
  pub fn get_content(&self) -> String {
    let content_ptr = unsafe { xmlNodeGetContent(self.node_ptr()) };
    if content_ptr.is_null() {
      //empty string when none
      return String::new();
    }
    let c_string = unsafe { CStr::from_ptr(content_ptr as *const c_char) };
    let rust_utf8 = c_str_to_string(c_string);
    bindgenFree(content_ptr as *mut c_void);
    rust_utf8
  }

  /// Sets the text content of this `Node`
  pub fn set_content(&mut self, content: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let c_content = CString::new(content).unwrap();
    unsafe {
      xmlNodeSetContent(self.node_ptr_mut()?, c_content.as_bytes().as_ptr());
    }
    Ok(())
  }

  /// Returns the value of property `name`
  pub fn get_property(&self, name: &str) -> Option<String> {
    with_c_str(name, |c_name| unsafe {
      let node_ptr = self.node_ptr();
      // (`xmlGetProp` is `xmlHasProp` and a copy of the value)
      property_value(xmlHasProp(node_ptr, c_name), || {
        xmlGetProp(node_ptr, c_name)
      })
    })
  }

  /// Read property `name`, as [`Node::get_property`] does, and pass its value to `f`; unlike `get_property`, the common
  /// case allocates nothing. The name is a C string (`c"id"`), and a value that libxml2 holds as a single text or CDATA
  /// node (the case libxml2's own `xmlGetProp` also shortcuts) of at most 128 bytes is copied into a buffer on the
  /// stack. A longer value, or one of any other shape (an entity reference in the value, a DTD default), is read into a
  /// `String` as `get_property` reads it. Returns `None` when the node has no such property.
  ///
  /// `f` reads its own copy of the value, never libxml2's, so `f` may change the document, this property included.
  pub fn with_property<R>(&self, name: &CStr, f: impl FnOnce(&str) -> R) -> Option<R> {
    let node_ptr = self.node_ptr();
    if node_ptr.is_null() {
      return None;
    }
    let c_name = name.as_ptr() as *const xmlChar;
    let mut buffer = [0u8; 128];
    // The value is copied out of libxml2's memory before `f` runs: a change `f` makes may free that memory.
    let value: Cow<str> = unsafe {
      let attr = xmlHasProp(node_ptr, c_name);
      if attr.is_null() {
        return None;
      }
      match single_text_value(attr) {
        Some(content) => {
          let bytes = content.to_bytes();
          match buffer.get_mut(..bytes.len()) {
            Some(copy) => {
              copy.copy_from_slice(bytes);
              match str::from_utf8(copy) {
                Ok(s) => Cow::Borrowed(s),
                Err(_) => String::from_utf8_lossy(copy),
              }
            }
            None => Cow::Owned(c_str_to_string(content)),
          }
        }
        None => Cow::Owned(take_xml_string(xmlGetProp(node_ptr, c_name))?),
      }
    };
    Some(f(&value))
  }

  /// Returns the value of property `name` in namespace `ns`
  pub fn get_property_ns(&self, name: &str, ns: &str) -> Option<String> {
    with_c_str(name, |c_name| {
      with_c_str(ns, |c_ns| unsafe {
        let node_ptr = self.node_ptr();
        // (`xmlGetNsProp` is `xmlHasNsProp` and a copy of the value)
        property_value(xmlHasNsProp(node_ptr, c_name, c_ns), || {
          xmlGetNsProp(node_ptr, c_name, c_ns)
        })
      })
    })
  }

  /// Returns the value of property `name` with no namespace
  pub fn get_property_no_ns(&self, name: &str) -> Option<String> {
    with_c_str(name, |c_name| unsafe {
      let node_ptr = self.node_ptr();
      // (`xmlGetNoNsProp` is `xmlHasNsProp` with no namespace and a copy of the value)
      property_value(xmlHasNsProp(node_ptr, c_name, ptr::null()), || {
        xmlGetNoNsProp(node_ptr, c_name)
      })
    })
  }

  /// Return an attribute as a `Node` struct of type AttributeNode
  pub fn get_property_node(&self, name: &str) -> Option<Node> {
    let attr_node = with_c_str(name, |c_name| unsafe {
      xmlHasProp(self.node_ptr(), c_name)
    });
    self.ptr_as_option(attr_node as xmlNodePtr)
  }

  /// Return an attribute in a namespace `ns` as a `Node` of type AttributeNode
  pub fn get_property_node_ns(&self, name: &str, ns: &str) -> Option<Node> {
    let attr_node = with_c_str(name, |c_name| {
      with_c_str(ns, |c_ns| unsafe {
        xmlHasNsProp(self.node_ptr(), c_name, c_ns)
      })
    });
    self.ptr_as_option(attr_node as xmlNodePtr)
  }

  /// Return an attribute with no namespace as a `Node` of type AttributeNode
  pub fn get_property_node_no_ns(&self, name: &str) -> Option<Node> {
    let attr_node = with_c_str(name, |c_name| unsafe {
      xmlHasNsProp(self.node_ptr(), c_name, ptr::null())
    });
    self.ptr_as_option(attr_node as xmlNodePtr)
  }

  /// Check if a property has been defined, without allocating its value
  pub fn has_property(&self, name: &str) -> bool {
    let value_ptr = with_c_str(name, |c_name| unsafe {
      xmlHasProp(self.node_ptr(), c_name)
    });
    !value_ptr.is_null()
  }

  /// Check if property `name` in namespace `ns` exists
  pub fn has_property_ns(&self, name: &str, ns: &str) -> bool {
    let value_ptr = with_c_str(name, |c_name| {
      with_c_str(ns, |c_ns| unsafe {
        xmlHasNsProp(self.node_ptr(), c_name, c_ns)
      })
    });
    !value_ptr.is_null()
  }

  /// Check if property `name` with no namespace exists
  pub fn has_property_no_ns(&self, name: &str) -> bool {
    let value_ptr = with_c_str(name, |c_name| unsafe {
      xmlHasNsProp(self.node_ptr(), c_name, ptr::null())
    });
    !value_ptr.is_null()
  }

  /// Alias for has_property
  pub fn has_attribute(&self, name: &str) -> bool {
    self.has_property(name)
  }
  /// Alias for has_property_ns
  pub fn has_attribute_ns(&self, name: &str, ns: &str) -> bool {
    self.has_property_ns(name, ns)
  }

  /// Alias for has_property_no_ns
  pub fn has_attribute_no_ns(&self, name: &str) -> bool {
    self.has_property_no_ns(name)
  }

  /// Sets the value of property `name` to `value`
  pub fn set_property(
    &mut self,
    name: &str,
    value: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    with_c_str(name, |c_name| {
      with_c_str(value, |c_value| {
        unsafe { xmlSetProp(self.node_ptr_mut()?, c_name, c_value) };
        Ok(())
      })
    })
  }
  /// Sets a namespaced attribute
  pub fn set_property_ns(
    &mut self,
    name: &str,
    value: &str,
    ns: &Namespace,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    with_c_str(name, |c_name| {
      with_c_str(value, |c_value| {
        unsafe { xmlSetNsProp(self.node_ptr_mut()?, ns.ns_ptr(), c_name, c_value) };
        Ok(())
      })
    })
  }

  /// Removes the property of given `name`
  pub fn remove_property(&mut self, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let attr_node = with_c_str(name, |c_name| -> Result<_, Box<dyn Error + Send + Sync>> {
      Ok(unsafe { xmlHasProp(self.node_ptr_mut()?, c_name) })
    })?;
    unsafe {
      if !attr_node.is_null() {
        let remove_prop_status = xmlRemoveProp(attr_node);
        if remove_prop_status == 0 {
          Ok(())
        } else {
          // Propagate libxml2 failure to remove
          Err(From::from(format!(
            "libxml2 failed to remove property with status: {remove_prop_status:?}"
          )))
        }
      } else {
        // silently no-op if asked to remove a property which is not present
        Ok(())
      }
    }
  }

  /// Removes the property of given `name` and namespace (`ns`)
  pub fn remove_property_ns(
    &mut self,
    name: &str,
    ns: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    let attr_node = with_c_str(name, |c_name| {
      with_c_str(ns, |c_ns| -> Result<_, Box<dyn Error + Send + Sync>> {
        Ok(unsafe { xmlHasNsProp(self.node_ptr_mut()?, c_name, c_ns) })
      })
    })?;
    unsafe {
      if !attr_node.is_null() {
        let remove_prop_status = xmlRemoveProp(attr_node);
        if remove_prop_status == 0 {
          Ok(())
        } else {
          // Propagate libxml2 failure to remove
          Err(From::from(format!(
            "libxml2 failed to remove property with status: {remove_prop_status:?}"
          )))
        }
      } else {
        // silently no-op if asked to remove a property which is not present
        Ok(())
      }
    }
  }

  /// Removes the property of given `name` with no namespace
  pub fn remove_property_no_ns(&mut self, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let attr_node = with_c_str(name, |c_name| -> Result<_, Box<dyn Error + Send + Sync>> {
      Ok(unsafe { xmlHasNsProp(self.node_ptr_mut()?, c_name, ptr::null()) })
    })?;
    if !attr_node.is_null() {
      let remove_prop_status = unsafe { xmlRemoveProp(attr_node) };
      if remove_prop_status == 0 {
        Ok(())
      } else {
        // Propagate libxml2 failure to remove
        Err(From::from(format!(
          "libxml2 failed to remove property with status: {remove_prop_status:?}"
        )))
      }
    } else {
      // silently no-op if asked to remove a property which is not present
      Ok(())
    }
  }

  /// Alias for get_property
  pub fn get_attribute(&self, name: &str) -> Option<String> {
    self.get_property(name)
  }

  /// Alias for with_property
  pub fn with_attribute<R>(&self, name: &CStr, f: impl FnOnce(&str) -> R) -> Option<R> {
    self.with_property(name, f)
  }

  /// Alias for get_property_ns
  pub fn get_attribute_ns(&self, name: &str, ns: &str) -> Option<String> {
    self.get_property_ns(name, ns)
  }

  /// Alias for get_property_no_ns
  pub fn get_attribute_no_ns(&self, name: &str) -> Option<String> {
    self.get_property_no_ns(name)
  }

  /// Alias for get_property_node
  pub fn get_attribute_node(&self, name: &str) -> Option<Node> {
    self.get_property_node(name)
  }

  /// Alias for get_property_node_ns
  pub fn get_attribute_node_ns(&self, name: &str, ns: &str) -> Option<Node> {
    self.get_property_node_ns(name, ns)
  }

  /// Alias for get_property_node_no_ns
  pub fn get_attribute_node_no_ns(&self, name: &str) -> Option<Node> {
    self.get_property_node_no_ns(name)
  }

  /// Alias for set_property
  pub fn set_attribute(
    &mut self,
    name: &str,
    value: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    self.set_property(name, value)
  }
  /// Alias for set_property_ns
  pub fn set_attribute_ns(
    &mut self,
    name: &str,
    value: &str,
    ns: &Namespace,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    self.set_property_ns(name, value, ns)
  }

  /// Alias for remove_property
  pub fn remove_attribute(&mut self, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    self.remove_property(name)
  }

  /// Alias for remove_property_ns
  pub fn remove_attribute_ns(
    &mut self,
    name: &str,
    ns: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    self.remove_property_ns(name, ns)
  }

  /// Alias for remove_property_no_ns
  pub fn remove_attribute_no_ns(&mut self, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    self.remove_property_no_ns(name)
  }

  /// Get a copy of the attributes of this node
  pub fn get_properties(&self) -> HashMap<String, String> {
    let mut count = 0;
    let mut current_prop = xmlGetFirstProperty(self.node_ptr());
    while !current_prop.is_null() {
      count += 1;
      current_prop = xmlNextPropertySibling(current_prop);
    }
    let mut attributes = HashMap::with_capacity(count);

    let mut current_prop = xmlGetFirstProperty(self.node_ptr());
    while !current_prop.is_null() {
      let name_ptr = xmlAttrName(current_prop);
      if name_ptr.is_null() {
        // A property with a NULL name can't be keyed; skip it rather than
        // `strlen(NULL)` inside `CStr::from_ptr`.
        current_prop = xmlNextPropertySibling(current_prop);
        continue;
      }
      let name = c_str_to_string(unsafe { CStr::from_ptr(name_ptr) });
      // Read the value straight from the attribute node we already hold, rather
      // than re-resolving it by name: `get_property` calls `xmlGetProp`, which
      // re-scans the whole attribute list with `xmlStrEqual` and allocates a
      // fresh `CString` for the name on every call — quadratic over the
      // attribute count, and a hot path during document build / math parsing.
      let value = attr_node_value(current_prop);
      attributes.insert(name, value);
      current_prop = xmlNextPropertySibling(current_prop);
    }

    attributes
  }

  /// Get a copy of this node's attributes and their namespaces
  pub fn get_properties_ns(&self) -> HashMap<(String, Option<Namespace>), String> {
    let mut attributes = HashMap::new();

    let mut current_prop = xmlGetFirstProperty(self.node_ptr());
    while !current_prop.is_null() {
      let name_ptr = xmlAttrName(current_prop);
      if name_ptr.is_null() {
        // A property with a NULL name can't be keyed; skip it rather than
        // `strlen(NULL)` inside `CStr::from_ptr`.
        current_prop = xmlNextPropertySibling(current_prop);
        continue;
      }
      let name = c_str_to_string(unsafe { CStr::from_ptr(name_ptr) });
      // Same direct-read optimization as `get_properties`: the value is read
      // from the attribute node itself, avoiding a by-name `xmlGetNsProp`
      // re-scan (and a per-attribute `CString` allocation).
      let value = attr_node_value(current_prop);
      let ns_ptr = xmlAttrNs(current_prop);
      if ns_ptr.is_null() {
        attributes.insert((name, None), value);
      } else {
        let ns = Namespace { ns_ptr };
        attributes.insert((name, Some(ns)), value);
      }
      current_prop = xmlNextPropertySibling(current_prop);
    }

    attributes
  }

  /// Alias for `get_properties`
  pub fn get_attributes(&self) -> HashMap<String, String> {
    self.get_properties()
  }

  /// Alias for `get_properties_ns`
  pub fn get_attributes_ns(&self) -> HashMap<(String, Option<Namespace>), String> {
    self.get_properties_ns()
  }

  /// Gets the active namespace associated of this node
  pub fn get_namespace(&self) -> Option<Namespace> {
    let ns_ptr = xmlNodeNs(self.node_ptr());
    if ns_ptr.is_null() {
      None
    } else {
      Some(Namespace { ns_ptr })
    }
  }

  /// Gets a list of namespaces associated with this node.
  ///
  /// `xmlGetNsList` mallocs an ARRAY of `xmlNsPtr` that the CALLER must
  /// free — the namespaces it points at belong to the document and must NOT
  /// be freed. The historical TODO here tried `xmlFreeNs(list)` (frees the
  /// first namespace struct — hence the remembered segfault) and gave up,
  /// leaking the array on EVERY call: measured downstream at 2,429 lost
  /// blocks over 30 documents (valgrind), a leading term of a ~150 KB/page
  /// RSS climb across a 115,000-page render whose materialization path calls
  /// this per node. The array is freed with the crate's per-target
  /// allocator shim (`bindgenFree` — NOT the `xmlFree` global, which is not
  /// a linkable symbol on MSVC).
  pub fn get_namespaces(&self, doc: &Document) -> Vec<Namespace> {
    let list_ptr_raw = unsafe { xmlGetNsList(doc.doc_ptr(), self.node_ptr()) };
    if list_ptr_raw.is_null() {
      Vec::new()
    } else {
      let mut namespaces = Vec::new();
      let mut ptr_iter = list_ptr_raw as *mut xmlNsPtr;
      unsafe {
        while !ptr_iter.is_null() && !(*ptr_iter).is_null() {
          namespaces.push(Namespace { ns_ptr: *ptr_iter });
          ptr_iter = ptr_iter.add(1);
        }
      }
      crate::c_helpers::bindgenFree(list_ptr_raw as *mut std::os::raw::c_void);
      namespaces
    }
  }

  /// Get a list of namespaces declared with this node
  pub fn get_namespace_declarations(&self) -> Vec<Namespace> {
    if self.get_type() != Some(NodeType::ElementNode) {
      // only element nodes can have declarations
      return Vec::new();
    }
    let mut namespaces = Vec::new();
    let mut ns_ptr = xmlNodeNsDeclarations(self.node_ptr());
    while !ns_ptr.is_null() {
      if !xmlNsPrefix(ns_ptr).is_null() || !xmlNsHref(ns_ptr).is_null() {
        namespaces.push(Namespace { ns_ptr });
      }
      ns_ptr = xmlNextNsSibling(ns_ptr);
    }
    namespaces
  }

  /// Sets a `Namespace` for the node
  pub fn set_namespace(
    &mut self,
    namespace: &Namespace,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    unsafe {
      xmlSetNs(self.node_ptr_mut()?, namespace.ns_ptr());
    }
    Ok(())
  }

  /// Looks up the prefix of a namespace from its URI, basedo around a given `Node`
  pub fn lookup_namespace_prefix(&self, href: &str) -> Option<String> {
    if href.is_empty() {
      return None;
    }
    let c_href = CString::new(href).unwrap();
    unsafe {
      let ptr_mut = self.node_ptr();
      let ns_ptr = xmlSearchNsByHref(xmlGetDoc(ptr_mut), ptr_mut, c_href.as_bytes().as_ptr());
      if !ns_ptr.is_null() {
        let ns = Namespace { ns_ptr };
        let ns_prefix = ns.get_prefix();
        Some(ns_prefix)
      } else {
        None
      }
    }
  }

  /// Looks up the uri of a namespace from its prefix, basedo around a given `Node`
  pub fn lookup_namespace_uri(&self, prefix: &str) -> Option<String> {
    if prefix.is_empty() {
      return None;
    }
    let c_prefix = CString::new(prefix).unwrap();
    unsafe {
      let ns_ptr = xmlSearchNs(
        xmlGetDoc(self.node_ptr()),
        self.node_ptr(),
        c_prefix.as_bytes().as_ptr(),
      );
      if !ns_ptr.is_null() {
        let ns = Namespace { ns_ptr };
        let ns_prefix = ns.get_href();
        if !ns_prefix.is_empty() {
          Some(ns_prefix)
        } else {
          None
        }
      } else {
        None
      }
    }
  }

  // TODO: Clear a future Document namespaces vec
  /// Removes the namespaces of this `Node` and it's children!
  pub fn recursively_remove_namespaces(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
    xmlNodeRecursivelyRemoveNs(self.node_ptr_mut()?);
    Ok(())
  }

  /// Get a set of class names from this node's attributes
  pub fn get_class_names(&self) -> HashSet<String> {
    let mut set = HashSet::new();
    if let Some(value) = self.get_property("class") {
      for n in value.split(' ') {
        set.insert(n.to_owned());
      }
    }
    set
  }

  /// Creates a new `Node` as child to the self `Node`
  pub fn add_child(&mut self, child: &mut Node) -> Result<(), String> {
    child.set_linked();
    unsafe {
      let new_child_ptr = xmlAddChild(self.node_ptr_mut()?, child.node_ptr_mut()?);
      if new_child_ptr.is_null() {
        Err("add_child encountered NULL pointer".to_string())
      } else {
        Ok(())
      }
    }
  }

  /// Creates a new `Node` as child to the self `Node`
  pub fn new_child(
    &mut self,
    ns: Option<Namespace>,
    name: &str,
  ) -> Result<Node, Box<dyn Error + Send + Sync>> {
    let c_name = CString::new(name).unwrap();
    let ns_ptr = match ns {
      None => ptr::null_mut(),
      Some(mut ns) => ns.ns_ptr_mut(),
    };
    unsafe {
      let new_ptr = xmlNewChild(
        self.node_ptr_mut()?,
        ns_ptr,
        c_name.as_bytes().as_ptr(),
        ptr::null(),
      );
      Ok(Node::wrap(new_ptr, &self.get_docref().upgrade().unwrap()))
    }
  }

  /// Adds a new text child, to this `Node`
  pub fn add_text_child(
    &mut self,
    ns: Option<Namespace>,
    name: &str,
    content: &str,
  ) -> Result<Node, Box<dyn Error + Send + Sync>> {
    let c_name = CString::new(name).unwrap();
    let c_content = CString::new(content).unwrap();
    let ns_ptr = match ns {
      None => ptr::null_mut(),
      Some(mut ns) => ns.ns_ptr_mut(),
    };
    unsafe {
      let new_ptr = xmlNewTextChild(
        self.node_ptr_mut()?,
        ns_ptr,
        c_name.as_bytes().as_ptr(),
        c_content.as_bytes().as_ptr(),
      );
      Ok(Node::wrap(new_ptr, &self.get_docref().upgrade().unwrap()))
    }
  }

  /// Append text to this `Node`
  pub fn append_text(&mut self, content: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let c_len = content.len() as i32;
    if c_len > 0 {
      let c_content = CString::new(content).unwrap();
      unsafe {
        xmlNodeAddContentLen(self.node_ptr_mut()?, c_content.as_bytes().as_ptr(), c_len);
      }
    }
    Ok(())
  }

  /// Unbinds the Node from its siblings and Parent, but not from the Document it belongs to.
  ///
  /// At the libxml2 level, `xmlUnlinkNode` severs `parent`/`prev`/`next`
  /// links but **leaves `node->doc` set** — the source document is
  /// still recorded as the lifetime owner of the underlying allocation.
  /// Consequences for subsequent handling:
  ///
  /// * **Re-attach** the node into a tree (via `add_child`,
  ///   `add_prev_sibling`, `add_next_sibling`, or
  ///   `Document::set_root_element`). Lifetime stays with the source
  ///   document; nothing more to do.
  /// * **Copy it out** into a new document via
  ///   [`Document::dup_node_into_new_doc`] (or a comparable libxml2
  ///   path). The copy is independent; the original is still detached.
  /// * **Discard the original**: call [`Node::set_rust_owned`] on it.
  ///   The wrapper's `Drop` will then `xmlFreeNode` the C subtree when
  ///   the last clone drops. Without this call, the detached subtree
  ///   leaks: the wrapper's `Drop` is a no-op (it must be, to avoid
  ///   freeing memory the source document still considers live), and
  ///   the source document's eventual `xmlFreeDoc` walks the tree
  ///   topology, so it never reaches a parent==NULL subtree.
  ///
  /// In short: an unlinked node that is *neither* re-attached *nor*
  /// marked `set_rust_owned` is a leak until process exit.
  pub fn unlink_node(&mut self) {
    let node_type = self.get_type();
    if node_type != Some(NodeType::DocumentNode)
      && node_type != Some(NodeType::DocumentFragNode)
      && !self.is_unlinked()
    {
      // only unlink nodes that are currently marked as linked
      self.set_unlinked();
      unsafe {
        xmlUnlinkNode(self.node_ptr());
      }
    }
  }
  /// Alias for `unlink_node`
  pub fn unlink(&mut self) {
    self.unlink_node()
  }
  /// Alias for `unlink_node`
  pub fn unbind_node(&mut self) {
    self.unlink_node()
  }
  /// Alias for `unlink_node`
  pub fn unbind(&mut self) {
    self.unlink_node()
  }

  /// Checks if node is marked as unlinked
  ///
  /// Returns `true` for both `Unlinked` and `RustOwned` states — both
  /// are detached from any document tree.
  pub fn is_unlinked(&self) -> bool {
    !matches!(self.0.borrow().linkage, Linkage::Linked)
  }

  /// Take ownership of this detached node's C allocation.
  ///
  /// After this call, the C node will be freed via `xmlFreeNode` when the
  /// last `Node` clone drops, regardless of whether `node->doc` is still
  /// set. The flag is sticky and propagates to every clone (they share
  /// the same `_Node`), so transferring ownership once is sufficient.
  ///
  /// Use this on a subtree that has been:
  ///   1. detached from its parent via `unlink_node`, AND
  ///   2. either copied out into another document (e.g. via
  ///      `Document::dup_node_into_new_doc`) or otherwise will not be
  ///      re-attached to any tree.
  ///
  /// Without this call, detached nodes are *not* freed by the wrapper —
  /// they remain attributed to `node->doc` for safety, and `xmlFreeDoc`
  /// reclaims them only if they're still reachable from the doc root.
  /// A subtree that has been unlinked but not re-attached and not marked
  /// rust-owned will leak until process exit.
  ///
  /// # Safety preconditions
  ///
  /// Calling this on a node that is still `Linked` (part of a document
  /// tree) is **undefined behavior**: the node's later free will leave
  /// dangling pointers in its former parent / siblings / ID table. The
  /// wrapper cannot cheaply verify this, so the caller must guarantee
  /// it. In debug builds we panic if the precondition is obviously
  /// violated.
  pub fn set_rust_owned(&self) {
    let mut inner = self.0.borrow_mut();
    debug_assert!(
      !matches!(inner.linkage, Linkage::Linked),
      "set_rust_owned called on a Linked node — call unlink_node first"
    );
    inner.linkage = Linkage::RustOwned;
  }

  /// Returns whether the C allocation is owned by the Rust wrapper.
  /// See `set_rust_owned`.
  pub fn is_rust_owned(&self) -> bool {
    matches!(self.0.borrow().linkage, Linkage::RustOwned)
  }

  /// Detach this node and free its entire C subtree IMMEDIATELY.
  ///
  /// This is the discard operation `unlink_node` alone does not provide: an
  /// unlinked doc-owned node is freed by nobody (see `unlink_node`), so a
  /// caller discarding a subtree for good must transfer ownership somewhere.
  /// `set_rust_owned` defers the free to the last wrapper drop — but a stray
  /// clone parked in a long-lived collection postpones that free past the
  /// owning document's `xmlFreeDoc`, and the eventual `xmlFreeNode` then
  /// reads the freed document (its dictionary) — a use-after-free.
  ///
  /// `free_subtree` instead frees NOW, after *neutralizing* every registered
  /// Rust wrapper into the subtree: each wrapper's shared inner `node_ptr`
  /// is nulled — clones held anywhere become inert (`Drop` is a no-op, the
  /// null-guarded accessors return their empty defaults) — and the
  /// bookkeeping entries are cleared, so a future allocation reusing one of
  /// the freed addresses cannot collide with a stale wrapper. Attribute
  /// nodes are neutralized along with elements and texts; namespace
  /// declarations and attribute values are owned by their nodes and freed by
  /// `xmlFreeNode` itself.
  ///
  /// Documents (and document-fragment shells) are refused — tearing a whole
  /// document down belongs to `Document`'s `Drop`.
  ///
  /// # Safety contract (mirrors C `xmlFreeNode`)
  ///
  /// The subtree must be garbage to the caller. Wrappers created in an
  /// earlier bookkeeping epoch (re-wrapped after an unlink cleared their
  /// entry) cannot be found and neutralized here; using one after this call
  /// is undefined behavior, exactly as using a freed `xmlNodePtr` in C.
  pub fn free_subtree(mut self) {
    if matches!(
      self.get_type(),
      Some(NodeType::DocumentNode) | Some(NodeType::DocumentFragNode)
    ) {
      return;
    }
    // Sever from parent/siblings (and clear self's bookkeeping entry).
    self.unlink_node();
    let top = self.0.borrow().node_ptr;
    if top.is_null() {
      return; // already freed/neutralized through another handle
    }
    if let Some(doc) = self.get_docref().upgrade() {
      // Walk the subtree iteratively (an explicit stack — recursion would
      // risk the call stack on deep trees), neutralizing wrappers.
      let mut stack: Vec<xmlNodePtr> = vec![top];
      while let Some(ptr) = stack.pop() {
        let mut child = xmlGetFirstChild(ptr);
        while !child.is_null() {
          stack.push(child);
          child = xmlNextSibling(child);
        }
        // Attribute structs share the xmlNode prefix layout through `ns`,
        // but only ELEMENTS have a `properties` field — reading it through
        // an attribute or text pointer would be out of bounds.
        if NodeType::from_int(xmlGetNodeType(ptr)) == Some(NodeType::ElementNode) {
          let mut attr = xmlGetFirstProperty(ptr);
          while !attr.is_null() {
            stack.push(attr as xmlNodePtr);
            attr = xmlNextPropertySibling(attr);
          }
        }
        let mut doc_borrowed = doc.borrow_mut();
        if let Some(wrapper) = doc_borrowed.get_node(ptr) {
          wrapper.0.borrow_mut().node_ptr = ptr::null_mut();
        }
        doc_borrowed.forget_node(ptr);
      }
    }
    // Neutralize this handle itself (its registry entry went away with
    // unlink_node, so the walk above could not reach it), then free.
    self.0.borrow_mut().node_ptr = ptr::null_mut();
    unsafe { xmlFreeNode(top) };
  }

  fn ptr_as_option(&self, node_ptr: xmlNodePtr) -> Option<Node> {
    if node_ptr.is_null() {
      None
    } else {
      // (the document upgraded in place, not through a cloned `Weak`)
      let doc_ref = self.0.borrow().document.upgrade().unwrap();
      let new_node = Node::wrap(node_ptr, &doc_ref);
      Some(new_node)
    }
  }

  /// internal helper to ensure the node is marked as linked/imported/adopted in the main document tree.
  ///
  /// Transitions from `Unlinked` to `Linked`. If the node is already
  /// `RustOwned`, this is a no-op in release builds and a debug-assert
  /// failure in debug builds: the caller has explicitly taken ownership
  /// and re-linking it would set up a guaranteed double-free
  /// (the new parent's `xmlFreeDoc` will free the subtree, then this
  /// wrapper's `Drop` will free it again). There is no public path to
  /// untake ownership; the right fix is for the caller to never re-link
  /// a `RustOwned` node.
  ///
  /// Public since 0.3.20: a consumer holding long-lived `Node` handles into a
  /// document (id caches and the like) needs to declare "the document owns
  /// this memory" before dropping the handles — some of which may have been
  /// `unlink_node`ed by intermediate processing. Without this, the only safe
  /// teardown was to leak each wrapper's allocation (latexml-oxide's
  /// `DocOwnedNode`, ~100+ bytes per id-cache entry per document — measured
  /// as a leading retention term at ~150 KB/page across a 115k-page render).
  pub fn set_linked(&self) {
    let mut inner = self.0.borrow_mut();
    debug_assert!(
      !matches!(inner.linkage, Linkage::RustOwned),
      "set_linked called on a RustOwned node — re-linking would set up a double free; \
       drop the wrapper before re-attaching, or do not call set_rust_owned"
    );
    if matches!(inner.linkage, Linkage::Unlinked) {
      inner.linkage = Linkage::Linked;
    }
  }

  /// internal helper to ensure the node is marked as unlinked/removed from the main document tree.
  ///
  /// Transitions from `Linked` to `Unlinked`. Leaves `RustOwned`
  /// untouched — that variant already implies "detached".
  pub(crate) fn set_unlinked(&self) {
    {
      let mut inner = self.0.borrow_mut();
      if matches!(inner.linkage, Linkage::Linked) {
        inner.linkage = Linkage::Unlinked;
      }
    }
    self
      .get_docref()
      .upgrade()
      .unwrap()
      .borrow_mut()
      .forget_node(self.node_ptr());
  }

  /// find nodes via xpath, at a specified node or the document root
  pub fn findnodes(&self, xpath: &str) -> Result<Vec<Node>, ()> {
    let mut context = Context::from_node(self)?;
    context.findnodes(xpath, Some(self))
  }

  /// Search this node for XPath `path`, and return only the first match.
  pub fn at_xpath(&self, path: &str, ns_binlings: &[(&str, &str)]) -> Result<Option<Node>, ()> {
    let mut context = Context::from_node(self)?;
    for (prefix, href) in ns_binlings {
      context.register_namespace(prefix, href)?;
    }
    let nodes = context.findnodes(path, Some(self))?;

    Ok(nodes.first().cloned())
  }

  /// Get a list of ancestor Node for this Node.
  pub fn ancestors(&self) -> Vec<Node> {
    let node_ptr = self.node_ptr();

    let mut res = Vec::new();

    let ancestor_ptrs = node_ancestors(node_ptr);

    for ptr in ancestor_ptrs {
      if let Some(node) = self.ptr_as_option(ptr) {
        res.push(node)
      }
    }

    res
  }

  /// find String values via xpath, at a specified node or the document root
  pub fn findvalues(&self, xpath: &str) -> Result<Vec<String>, ()> {
    let mut context = Context::from_node(self)?;
    context.findvalues(xpath, Some(self))
  }

  /// replace a `self`'s `old` child node with a `new` node in the same position
  /// borrowed from Perl's XML::LibXML
  pub fn replace_child_node(
    &mut self,
    mut new: Node,
    mut old: Node,
  ) -> Result<Node, Box<dyn Error + Send + Sync>> {
    // if newNode == oldNode or self == newNode then do nothing, just return nNode.
    if new == old || self == &new {
      // nothing to do here, already in place
      Ok(old)
    } else if self.get_type() == Some(NodeType::ElementNode) {
      match old.get_parent() {
        Some(old_parent) => {
          if &old_parent == self {
            // unlink new to be available for insertion
            new.unlink();
            // mid-child case
            old.add_next_sibling(&mut new)?;
            old.unlink();
            Ok(old)
          } else {
            Err(From::from(format!(
              "Old node was not a child of {:?} parent. Registered parent is {:?} instead.",
              self.get_name(),
              old_parent.get_name()
            )))
          }
        }
        _ => Err(From::from(format!(
          "Old node was not a child of {:?} parent. No registered parent exists.",
          self.get_name()
        ))),
      }
    } else {
      Err(From::from(
        "Can only call replace_child_node an a NodeType::Element type parent.",
      ))
    }
  }
}

fn node_ancestors(node_ptr: xmlNodePtr) -> Vec<xmlNodePtr> {
  if node_ptr.is_null() {
    return Vec::new();
  }

  let mut parent_ptr = xmlGetParent(node_ptr);

  if parent_ptr.is_null() {
    Vec::new()
  } else {
    let mut parents = vec![parent_ptr];

    while !xmlGetParent(parent_ptr).is_null() {
      parent_ptr = xmlGetParent(parent_ptr);
      parents.push(parent_ptr);
    }

    parents
  }
}

/// Read an attribute node's value directly via `xmlNodeGetContent`, freeing the
/// libxml2-allocated buffer. Equivalent to resolving the attribute by name with
/// `xmlGetProp`, but without the list re-scan and name `CString` allocation —
/// the caller already holds the attribute pointer.
fn attr_node_value(attr: xmlAttrPtr) -> String {
  if let Some(content) = unsafe { single_text_value(attr) } {
    return c_str_to_string(content);
  }
  let content_ptr = unsafe { xmlNodeGetContent(attr as xmlNodePtr) };
  if content_ptr.is_null() {
    return String::new();
  }
  let c_string = unsafe { CStr::from_ptr(content_ptr as *const c_char) };
  let value = c_str_to_string(c_string);
  bindgenFree(content_ptr as *mut c_void);
  value
}

/// The value of an attribute that libxml2 holds with no children (`""`) or as a single text or CDATA node (its
/// content): the shapes libxml2's own `xmlGetPropNodeValueInternal` (behind `xmlGetProp`, `xmlGetNsProp`,
/// `xmlGetNoNsProp`) reads without walking the children. `None` for any other shape (an entity reference in the value,
/// a DTD default declaration), which the caller reads through libxml2.
///
/// # Safety
/// `attr` is null or points to a live attribute (or attribute declaration); the borrow lasts while it is not changed.
unsafe fn single_text_value<'a>(attr: xmlAttrPtr) -> Option<&'a CStr> {
  if attr.is_null() || unsafe { (*attr).type_ } != xmlElementType_XML_ATTRIBUTE_NODE {
    return None;
  }
  let text = unsafe { (*attr).children };
  if text.is_null() {
    return Some(c"");
  }
  let text = unsafe { &*text };
  if text.next.is_null()
    && (text.type_ == xmlElementType_XML_TEXT_NODE
      || text.type_ == xmlElementType_XML_CDATA_SECTION_NODE)
    && !text.content.is_null()
  {
    Some(unsafe { CStr::from_ptr(text.content as *const c_char) })
  } else {
    None
  }
}

/// The value of `attr`, which libxml2's lookup (`xmlHasProp`, `xmlHasNsProp`) returned: `None` when it is NULL, read in
/// place when it is one of the shapes `single_text_value` reads, and otherwise copied by `getter`, the libxml2 getter
/// that makes the same lookup. libxml2's getters return NULL whenever that lookup does, so an absent property is
/// looked up once, not again by `getter`.
///
/// # Safety
/// As for `single_text_value`; `getter` makes the lookup that returned `attr`.
unsafe fn property_value(
  attr: xmlAttrPtr,
  getter: impl FnOnce() -> *mut xmlChar,
) -> Option<String> {
  if attr.is_null() {
    return None;
  }
  match unsafe { single_text_value(attr) } {
    Some(content) => Some(c_str_to_string(content)),
    None => take_xml_string(getter()),
  }
}

/// The string a libxml2 getter returned, as a `String`, freeing libxml2's copy; `None` when the getter returned NULL.
fn take_xml_string(value_ptr: *mut xmlChar) -> Option<String> {
  if value_ptr.is_null() {
    return None;
  }
  let value = c_str_to_string(unsafe { CStr::from_ptr(value_ptr as *const c_char) });
  bindgenFree(value_ptr as *mut c_void);
  Some(value)
}

/// A copy of `c` as a `String`, as `c.to_string_lossy().into_owned()` gives it; valid UTF-8 (almost every libxml2
/// string: a document parsed with its encoding ignored, or recovered, may hold other bytes) is checked by
/// `str::from_utf8`'s fast path instead of the lossy decoder's.
fn c_str_to_string(c: &CStr) -> String {
  match c.to_str() {
    Ok(s) => s.to_owned(),
    Err(_) => c.to_string_lossy().into_owned(),
  }
}

/// Pass `s` to `f` as a NUL-terminated string for libxml2: copied to the stack when it is shorter than 128 bytes,
/// otherwise through a `CString`. A string holding a NUL byte takes the `CString` path and panics there, as
/// `CString::new(s).unwrap()` always has in these methods.
fn with_c_str<R>(s: &str, f: impl FnOnce(*const xmlChar) -> R) -> R {
  const STACK: usize = 128;
  let bytes = s.as_bytes();
  if bytes.len() < STACK && !bytes.contains(&0) {
    let mut buffer = [0u8; STACK];
    buffer[..bytes.len()].copy_from_slice(bytes);
    f(buffer.as_ptr())
  } else {
    let c_string = CString::new(s).unwrap();
    f(c_string.as_bytes().as_ptr())
  }
}

mod c14n;
