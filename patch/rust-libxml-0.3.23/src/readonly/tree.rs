use libc::{c_char, c_void};
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

use crate::bindings::*;
use crate::c_helpers::*;
use crate::tree::Document;
use crate::tree::namespace::Namespace;
use crate::tree::nodetype::NodeType;
use crate::xpath::Context;

/// Lightweight struct for read-only parallel processing
#[derive(Debug, Copy, Clone)]
pub struct RoNode(pub(crate) xmlNodePtr);

// we claim Sync and Send, as we are in read-only mode over the owning document
unsafe impl Sync for RoNode {}
unsafe impl Send for RoNode {}

impl PartialEq for RoNode {
  /// Two nodes are considered equal, if they point to the same xmlNode.
  fn eq(&self, other: &RoNode) -> bool {
    std::ptr::eq(self.0, other.0)
  }
}
impl Eq for RoNode {}

impl RoNode {
  /// Immutably borrows the underlying libxml2 `xmlNodePtr` pointer
  pub fn node_ptr(&self) -> xmlNodePtr {
    self.0
  }

  /// Returns the next sibling if it exists
  pub fn get_next_sibling(self) -> Option<RoNode> {
    let ptr = xmlNextSibling(self.0);
    self.ptr_as_option(ptr)
  }

  /// Returns the previous sibling if it exists
  pub fn get_prev_sibling(self) -> Option<RoNode> {
    let ptr = xmlPrevSibling(self.0);
    self.ptr_as_option(ptr)
  }

  /// Returns the first child if it exists
  pub fn get_first_child(self) -> Option<RoNode> {
    let ptr = xmlGetFirstChild(self.0);
    self.ptr_as_option(ptr)
  }

  /// Returns the last child if it exists
  pub fn get_last_child(self) -> Option<RoNode> {
    let ptr = unsafe { xmlGetLastChild(self.0) };
    self.ptr_as_option(ptr)
  }

  /// Returns the next element sibling if it exists
  pub fn get_next_element_sibling(&self) -> Option<RoNode> {
    match self.get_next_sibling() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          if let Some(sibling) = current_node.get_next_sibling() {
            current_node = sibling;
          } else {
            break;
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
  pub fn get_prev_element_sibling(&self) -> Option<RoNode> {
    match self.get_prev_sibling() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          if let Some(sibling) = current_node.get_prev_sibling() {
            current_node = sibling;
          } else {
            break;
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
  pub fn get_first_element_child(self) -> Option<RoNode> {
    match self.get_first_child() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          if let Some(sibling) = current_node.get_next_sibling() {
            current_node = sibling;
          } else {
            break;
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
  pub fn get_last_element_child(&self) -> Option<RoNode> {
    match self.get_last_child() {
      None => None,
      Some(child) => {
        let mut current_node = child;
        while !current_node.is_element_node() {
          if let Some(sibling) = current_node.get_prev_sibling() {
            current_node = sibling;
          } else {
            break;
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
  pub fn get_child_nodes(self) -> Vec<RoNode> {
    let mut children = Vec::new();
    if let Some(first_child) = self.get_first_child() {
      children.push(first_child);
      while let Some(sibling) = children.last().unwrap().get_next_sibling() {
        children.push(sibling)
      }
    }
    children
  }

  /// Returns all child elements of the given node as a vector
  pub fn get_child_elements(self) -> Vec<RoNode> {
    self
      .get_child_nodes()
      .into_iter()
      .filter(|n| n.get_type() == Some(NodeType::ElementNode))
      .collect::<Vec<RoNode>>()
  }

  /// Returns the parent if it exists
  pub fn get_parent(self) -> Option<RoNode> {
    let ptr = xmlGetParent(self.0);
    self.ptr_as_option(ptr)
  }

  /// Get the node type
  pub fn get_type(self) -> Option<NodeType> {
    NodeType::from_int(xmlGetNodeType(self.0))
  }

  /// Returns true if it is a text node
  pub fn is_text_node(self) -> bool {
    self.get_type() == Some(NodeType::TextNode)
  }

  /// Checks if the given node is an Element
  pub fn is_element_node(self) -> bool {
    self.get_type() == Some(NodeType::ElementNode)
  }

  /// Checks if the underlying libxml2 pointer is `NULL`
  pub fn is_null(self) -> bool {
    self.0.is_null()
  }

  /// Returns the name of the node (empty string if name pointer is `NULL`)
  pub fn get_name(self) -> String {
    let name_ptr = xmlNodeGetName(self.0);
    if name_ptr.is_null() {
      return String::new();
    } //empty string
    let c_string = unsafe { CStr::from_ptr(name_ptr) };
    c_string.to_string_lossy().into_owned()
  }

  /// Returns the content of the node
  /// (assumes UTF-8 XML document)
  pub fn get_content(self) -> String {
    let content_ptr = unsafe { xmlNodeGetContent(self.0) };
    if content_ptr.is_null() {
      //empty string when none
      return String::new();
    }
    let c_string = unsafe { CStr::from_ptr(content_ptr as *const c_char) };
    let rust_utf8 = c_string.to_string_lossy().into_owned();
    bindgenFree(content_ptr as *mut c_void);
    rust_utf8
  }

  /// Returns the value of property `name`
  pub fn get_property(self, name: &str) -> Option<String> {
    let c_name = CString::new(name).unwrap();
    let value_ptr = unsafe { xmlGetProp(self.0, c_name.as_bytes().as_ptr()) };
    if value_ptr.is_null() {
      return None;
    }
    let c_value_string = unsafe { CStr::from_ptr(value_ptr as *const c_char) };
    let prop_str = c_value_string.to_string_lossy().into_owned();
    bindgenFree(value_ptr as *mut c_void);
    Some(prop_str)
  }

  /// Returns the value of property `name` in namespace `ns`
  pub fn get_property_ns(self, name: &str, ns: &str) -> Option<String> {
    let c_name = CString::new(name).unwrap();
    let c_ns = CString::new(ns).unwrap();
    let value_ptr =
      unsafe { xmlGetNsProp(self.0, c_name.as_bytes().as_ptr(), c_ns.as_bytes().as_ptr()) };
    if value_ptr.is_null() {
      return None;
    }
    let c_value_string = unsafe { CStr::from_ptr(value_ptr as *const c_char) };
    let prop_str = c_value_string.to_string_lossy().into_owned();
    bindgenFree(value_ptr as *mut c_void);
    Some(prop_str)
  }

  /// Returns the value of property `name` with no namespace
  pub fn get_property_no_ns(self, name: &str) -> Option<String> {
    let c_name = CString::new(name).unwrap();
    let value_ptr = unsafe { xmlGetNoNsProp(self.0, c_name.as_bytes().as_ptr()) };
    if value_ptr.is_null() {
      return None;
    }
    let c_value_string = unsafe { CStr::from_ptr(value_ptr as *const c_char) };
    let prop_str = c_value_string.to_string_lossy().into_owned();
    bindgenFree(value_ptr as *mut c_void);
    Some(prop_str)
  }

  /// Return an attribute as a `Node` struct of type AttributeNode
  pub fn get_property_node(self, name: &str) -> Option<RoNode> {
    let c_name = CString::new(name).unwrap();
    unsafe {
      let attr_node = xmlHasProp(self.0, c_name.as_bytes().as_ptr());
      self.ptr_as_option(attr_node as xmlNodePtr)
    }
  }

  /// Return an attribute in a namespace `ns` as a `Node` of type AttributeNode
  pub fn get_property_node_ns(self, name: &str, ns: &str) -> Option<RoNode> {
    let c_name = CString::new(name).unwrap();
    let c_ns = CString::new(ns).unwrap();
    let attr_node =
      unsafe { xmlHasNsProp(self.0, c_name.as_bytes().as_ptr(), c_ns.as_bytes().as_ptr()) };
    self.ptr_as_option(attr_node as xmlNodePtr)
  }

  /// Return an attribute with no namespace as a `Node` of type AttributeNode
  pub fn get_property_node_no_ns(self, name: &str) -> Option<RoNode> {
    let c_name = CString::new(name).unwrap();
    let attr_node = unsafe { xmlHasNsProp(self.0, c_name.as_bytes().as_ptr(), ptr::null()) };
    self.ptr_as_option(attr_node as xmlNodePtr)
  }

  /// Alias for get_property
  pub fn get_attribute(self, name: &str) -> Option<String> {
    self.get_property(name)
  }

  /// Alias for get_property_ns
  pub fn get_attribute_ns(self, name: &str, ns: &str) -> Option<String> {
    self.get_property_ns(name, ns)
  }

  /// Alias for get_property_no_ns
  pub fn get_attribute_no_ns(self, name: &str) -> Option<String> {
    self.get_property_no_ns(name)
  }

  /// Alias for get_property_node
  pub fn get_attribute_node(self, name: &str) -> Option<RoNode> {
    self.get_property_node(name)
  }

  /// Alias for get_property_node_ns
  pub fn get_attribute_node_ns(self, name: &str, ns: &str) -> Option<RoNode> {
    self.get_property_node_ns(name, ns)
  }

  /// Alias for get_property_node_no_ns
  pub fn get_attribute_node_no_ns(self, name: &str) -> Option<RoNode> {
    self.get_property_node_no_ns(name)
  }

  /// Get a copy of the attributes of this node
  pub fn get_properties(self) -> HashMap<String, String> {
    let mut attributes = HashMap::new();

    let mut current_prop = xmlGetFirstProperty(self.0);
    while !current_prop.is_null() {
      let name_ptr = xmlAttrName(current_prop);
      if name_ptr.is_null() {
        // A property with a NULL name can't be keyed; skip it rather than
        // `strlen(NULL)` inside `CStr::from_ptr`.
        current_prop = xmlNextPropertySibling(current_prop);
        continue;
      }
      let c_name_string = unsafe { CStr::from_ptr(name_ptr) };
      let name = c_name_string.to_string_lossy().into_owned();
      let value = self.get_property(&name).unwrap_or_default();
      attributes.insert(name, value);
      current_prop = xmlNextPropertySibling(current_prop);
    }

    attributes
  }

  /// Get a copy of this node's attributes and their namespaces
  pub fn get_properties_ns(self) -> HashMap<(String, Option<Namespace>), String> {
    let mut attributes = HashMap::new();

    let mut current_prop = xmlGetFirstProperty(self.0);
    while !current_prop.is_null() {
      let name_ptr = xmlAttrName(current_prop);
      if name_ptr.is_null() {
        // A property with a NULL name can't be keyed; skip it rather than
        // `strlen(NULL)` inside `CStr::from_ptr`.
        current_prop = xmlNextPropertySibling(current_prop);
        continue;
      }
      let c_name_string = unsafe { CStr::from_ptr(name_ptr) };
      let name = c_name_string.to_string_lossy().into_owned();
      let ns_ptr = xmlAttrNs(current_prop);
      if ns_ptr.is_null() {
        let value = self.get_property_no_ns(&name).unwrap_or_default();
        attributes.insert((name, None), value);
      } else {
        let ns = Namespace { ns_ptr };
        let value = self
          .get_property_ns(&name, &ns.get_href())
          .unwrap_or_default();
        attributes.insert((name, Some(ns)), value);
      }
      current_prop = xmlNextPropertySibling(current_prop);
    }

    attributes
  }

  /// Alias for `get_properties`
  pub fn get_attributes(self) -> HashMap<String, String> {
    self.get_properties()
  }

  /// Alias for `get_properties_ns`
  pub fn get_attributes_ns(self) -> HashMap<(String, Option<Namespace>), String> {
    self.get_properties_ns()
  }

  /// Check if a property has been defined, without allocating its value
  pub fn has_property(self, name: &str) -> bool {
    let c_name = CString::new(name).unwrap();
    let value_ptr = unsafe { xmlHasProp(self.0, c_name.as_bytes().as_ptr()) };
    !value_ptr.is_null()
  }

  /// Check if property `name` in namespace `ns` exists
  pub fn has_property_ns(self, name: &str, ns: &str) -> bool {
    let c_name = CString::new(name).unwrap();
    let c_ns = CString::new(ns).unwrap();
    let value_ptr =
      unsafe { xmlHasNsProp(self.0, c_name.as_bytes().as_ptr(), c_ns.as_bytes().as_ptr()) };
    !value_ptr.is_null()
  }

  /// Check if property `name` with no namespace exists
  pub fn has_property_no_ns(self, name: &str) -> bool {
    let c_name = CString::new(name).unwrap();
    let value_ptr = unsafe { xmlHasNsProp(self.0, c_name.as_bytes().as_ptr(), ptr::null()) };
    !value_ptr.is_null()
  }

  /// Alias for has_property
  pub fn has_attribute(self, name: &str) -> bool {
    self.has_property(name)
  }

  /// Alias for has_property_ns
  pub fn has_attribute_ns(self, name: &str, ns: &str) -> bool {
    self.has_property_ns(name, ns)
  }

  /// Alias for has_property_no_ns
  pub fn has_attribute_no_ns(self, name: &str) -> bool {
    self.has_property_no_ns(name)
  }

  /// Gets the active namespace associated of this node
  pub fn get_namespace(self) -> Option<Namespace> {
    let ns_ptr = xmlNodeNs(self.0);
    if ns_ptr.is_null() {
      None
    } else {
      Some(Namespace { ns_ptr })
    }
  }

  /// Gets a list of namespaces associated with this node
  pub fn get_namespaces(self, doc: &Document) -> Vec<Namespace> {
    let list_ptr_raw = unsafe { xmlGetNsList(doc.doc_ptr(), self.0) };
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
      // The array is the caller's; the namespaces it points at belong to the document.
      crate::c_helpers::bindgenFree(list_ptr_raw as *mut std::os::raw::c_void);
      namespaces
    }
  }

  /// Get a list of namespaces declared with this node
  pub fn get_namespace_declarations(self) -> Vec<Namespace> {
    if self.get_type() != Some(NodeType::ElementNode) {
      // only element nodes can have declarations
      return Vec::new();
    }
    let mut namespaces = Vec::new();
    let mut ns_ptr = xmlNodeNsDeclarations(self.0);
    while !ns_ptr.is_null() {
      if !xmlNsPrefix(ns_ptr).is_null() || !xmlNsHref(ns_ptr).is_null() {
        namespaces.push(Namespace { ns_ptr });
      }
      ns_ptr = xmlNextNsSibling(ns_ptr);
    }
    namespaces
  }

  /// Looks up the prefix of a namespace from its URI, basedo around a given `Node`
  pub fn lookup_namespace_prefix(self, href: &str) -> Option<String> {
    if href.is_empty() {
      return None;
    }
    let c_href = CString::new(href).unwrap();
    unsafe {
      let ptr_mut = self.0;
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
  pub fn lookup_namespace_uri(self, prefix: &str) -> Option<String> {
    if prefix.is_empty() {
      return None;
    }
    let c_prefix = CString::new(prefix).unwrap();
    unsafe {
      let ns_ptr = xmlSearchNs(xmlGetDoc(self.0), self.0, c_prefix.as_bytes().as_ptr());
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

  /// Get a set of class names from this node's attributes
  pub fn get_class_names(self) -> HashSet<String> {
    let mut set = HashSet::new();
    if let Some(value) = self.get_property("class") {
      for n in value.split(' ') {
        set.insert(n.to_owned());
      }
    }
    set
  }

  /// find read-only nodes via xpath, at the specified node and a given document
  pub fn findnodes(self, xpath: &str, owner: &Document) -> Result<Vec<RoNode>, ()> {
    let context = Context::new(owner)?;
    let evaluated = context.node_evaluate_readonly(xpath, self)?;
    Ok(evaluated.get_readonly_nodes_as_vec())
  }

  /// Read-only nodes are always linked
  pub fn is_unlinked(self) -> bool {
    false
  }

  /// Read-only nodes are always document-owned.
  ///
  /// Mirrors `Node::is_rust_owned` for API uniformity. `RoNode` is a
  /// `Copy` borrow into a document tree owned by a parent `Document`;
  /// it has no `Drop` and cannot take ownership of the C allocation,
  /// so this is unconditionally `false`.
  pub fn is_rust_owned(self) -> bool {
    false
  }
  /// Read-only nodes only need a null check
  fn ptr_as_option(self, node_ptr: xmlNodePtr) -> Option<RoNode> {
    if node_ptr.is_null() {
      None
    } else {
      Some(RoNode(node_ptr))
    }
  }

  /// `libc::c_void` isn't hashable and cannot be made hashable
  pub fn to_hashable(self) -> usize {
    self.0 as usize
  }
  /// Create a mock node, used for a placeholder argument
  pub fn null() -> Self {
    RoNode(ptr::null_mut())
  }
}
