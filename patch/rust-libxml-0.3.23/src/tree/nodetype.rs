//! Types of libxml2 Nodes
//!

use crate::bindings::xmlElementType;

/// Types of xml nodes
#[derive(Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum NodeType {
  ElementNode,
  AttributeNode,
  TextNode,
  CDataSectionNode,
  EntityRefNode,
  EntityNode,
  PiNode,
  CommentNode,
  DocumentNode,
  DocumentTypeNode,
  DocumentFragNode,
  NotationNode,
  HtmlDocumentNode,
  DTDNode,
  ElementDecl,
  AttributeDecl,
  EntityDecl,
  NamespaceDecl,
  XIncludeStart,
  XIncludeEnd,
  DOCBDocumentNode,
}

impl NodeType {
  /// converts an integer from libxml's `enum NodeType`
  /// to an instance of our `NodeType`
  pub fn from_int(i: xmlElementType) -> Option<NodeType> {
    match i {
      1 => Some(NodeType::ElementNode),
      2 => Some(NodeType::AttributeNode),
      3 => Some(NodeType::TextNode),
      4 => Some(NodeType::CDataSectionNode),
      5 => Some(NodeType::EntityRefNode),
      6 => Some(NodeType::EntityNode),
      7 => Some(NodeType::PiNode),
      8 => Some(NodeType::CommentNode),
      9 => Some(NodeType::DocumentNode),
      10 => Some(NodeType::DocumentTypeNode),
      11 => Some(NodeType::DocumentFragNode),
      12 => Some(NodeType::NotationNode),
      13 => Some(NodeType::HtmlDocumentNode),
      14 => Some(NodeType::DTDNode),
      15 => Some(NodeType::ElementDecl),
      16 => Some(NodeType::AttributeDecl),
      17 => Some(NodeType::EntityDecl),
      18 => Some(NodeType::NamespaceDecl),
      19 => Some(NodeType::XIncludeStart),
      20 => Some(NodeType::XIncludeEnd),
      21 => Some(NodeType::DOCBDocumentNode),
      _ => None,
    }
  }
}
