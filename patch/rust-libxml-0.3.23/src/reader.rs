//! Streaming pull-parser (`xmlTextReader`).
//!
//! Building the whole DOM for a very large document is prohibitively
//! memory-hungry (a 600 MB file becomes a ~7 GB tree). [`TextReader`] instead
//! streams the input node-by-node and lets callers materialize only the
//! subtrees they care about, so peak memory is one subtree at a time rather
//! than the entire tree.
//!
//! Two ways to materialize the current subtree:
//! * [`TextReader::expand`] — a **borrowed** [`RoNode`], zero-copy, valid only
//!   until the next [`read`](TextReader::read)/[`read_next`](TextReader::read_next).
//!   Ideal for read-only scanning.
//! * [`TextReader::expand_to_document`] — an **owned** [`Document`] copy
//!   (namespaces reconciled), safe to hold, mutate, transform and free after
//!   the reader has advanced. This is the unit the rest of the pipeline
//!   (XSLT, serialization) consumes.
//!
//! ## Streaming a pattern
//!
//! libxml2's XPath engine is not streamable (it needs a fully-built tree). The
//! streamable subset is "downward" name/descendant matching, which at the
//! reader level is simply a per-element name/namespace test — see
//! [`TextReader::read_to_next`]. Arbitrary predicates are then applied on the
//! small owned subtree, where XPath is limit-safe.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use crate::bindings::*;
use crate::readonly::RoNode;
use crate::tree::{Document, NodeType};

/// A safe wrapper over libxml2's `xmlTextReader` pull parser.
///
/// Owns the underlying reader (and the file handle it opened); dropping the
/// `TextReader` frees both.
pub struct TextReader {
  ptr: xmlTextReaderPtr,
}

/// The reader's own event vocabulary (`xmlReaderTypes`), exposed losslessly.
///
/// [`TextReader::node_type`] maps events `1..=12` onto [`NodeType`] and
/// everything else to `None` — which conflates *end-element* with the two
/// *whitespace* events (13/14). A streaming caller that reconstructs document
/// structure needs all three distinguished; [`TextReader::event`] returns this
/// enum instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReaderEvent {
  /// No node (before the first read / after the last).
  None,
  /// An element start tag (`<x>` or `<x/>` — see [`TextReader::is_empty_element`]).
  Element,
  /// An attribute node (only when navigating attributes explicitly).
  Attribute,
  /// A text node with non-whitespace content.
  Text,
  /// A CDATA section.
  CData,
  /// An entity reference (unresolved).
  EntityReference,
  /// An entity declaration.
  Entity,
  /// A processing instruction.
  ProcessingInstruction,
  /// A comment.
  Comment,
  /// The document node.
  Document,
  /// A DOCTYPE declaration.
  DocumentType,
  /// A document fragment.
  DocumentFragment,
  /// A notation declaration.
  Notation,
  /// Ignorable inter-element whitespace.
  Whitespace,
  /// Whitespace in mixed content (significant per the reader).
  SignificantWhitespace,
  /// An element end tag (`</x>`).
  EndElement,
  /// The end of an expanded entity.
  EndEntity,
  /// The `<?xml …?>` declaration.
  XmlDeclaration,
}

impl ReaderEvent {
  fn from_int(t: i32) -> Self {
    match t {
      1 => ReaderEvent::Element,
      2 => ReaderEvent::Attribute,
      3 => ReaderEvent::Text,
      4 => ReaderEvent::CData,
      5 => ReaderEvent::EntityReference,
      6 => ReaderEvent::Entity,
      7 => ReaderEvent::ProcessingInstruction,
      8 => ReaderEvent::Comment,
      9 => ReaderEvent::Document,
      10 => ReaderEvent::DocumentType,
      11 => ReaderEvent::DocumentFragment,
      12 => ReaderEvent::Notation,
      13 => ReaderEvent::Whitespace,
      14 => ReaderEvent::SignificantWhitespace,
      15 => ReaderEvent::EndElement,
      16 => ReaderEvent::EndEntity,
      17 => ReaderEvent::XmlDeclaration,
      _ => ReaderEvent::None,
    }
  }
}

impl Drop for TextReader {
  fn drop(&mut self) {
    unsafe { xmlFreeTextReader(self.ptr) };
  }
}

/// Borrow a reader-owned `const xmlChar*` (never freed by the caller) as an
/// owned `String`. Returns `None` for a NULL pointer.
fn const_xmlchar_to_string(ptr: *const xmlChar) -> Option<String> {
  if ptr.is_null() {
    return None;
  }
  Some(
    unsafe { CStr::from_ptr(ptr as *const c_char) }
      .to_string_lossy()
      .into_owned(),
  )
}

/// Map libxml2's reader-advance status (`1` = positioned on a node, `0` = end
/// of input, negative = parse error) to a `Result`.
fn read_status(rc: i32) -> Result<bool, ()> {
  match rc {
    1 => Ok(true),
    0 => Ok(false),
    _ => Err(()),
  }
}

impl TextReader {
  /// Open `path` for streaming. `options` is the libxml2 parser-option bitmask
  /// (`0` for defaults). Fails if the reader could not be created (e.g. the
  /// file does not exist).
  pub fn from_file(path: &str, options: i32) -> Result<Self, ()> {
    crate::init_parser();
    let c_path = CString::new(path).map_err(|_| ())?;
    let ptr = unsafe { xmlReaderForFile(c_path.as_ptr(), ptr::null(), options) };
    if ptr.is_null() {
      Err(())
    } else {
      Ok(TextReader { ptr })
    }
  }

  /// Advance to the next node in document order (descending into children).
  ///
  /// `Ok(true)` = positioned on a node, `Ok(false)` = end of input,
  /// `Err(())` = a parse error occurred.
  pub fn read(&mut self) -> Result<bool, ()> {
    read_status(unsafe { xmlTextReaderRead(self.ptr) })
  }

  /// Advance to the next node that is **not** a descendant of the current node
  /// (i.e. skip the current subtree). Use after materializing a subtree to move
  /// past it without walking its children. Same `Ok(true/false)`/`Err`
  /// semantics as [`read`](Self::read).
  pub fn read_next(&mut self) -> Result<bool, ()> {
    read_status(unsafe { xmlTextReaderNext(self.ptr) })
  }

  /// The current node's type. Returns `None` for reader events that have no
  /// [`NodeType`] equivalent — most usefully the *end-of-element* event, which
  /// lets a caller distinguish an opening `<x>` (`Some(ElementNode)`) from a
  /// closing `</x>` (`None`).
  pub fn node_type(&self) -> Option<NodeType> {
    // `xmlTextReaderNodeType` returns an `xmlReaderTypes` value, which coincides
    // numerically with `xmlElementType` ONLY for `1..=12` (element, attribute,
    // text, cdata, entity-ref, entity, PI, comment, document, doctype,
    // fragment, notation). The reader-only events collide with UNRELATED
    // element types — end-element `15 == XML_ELEMENT_DECL`, whitespace
    // `13 == XML_HTML_DOCUMENT_NODE`, significant-whitespace `14 == XML_DTD_NODE`,
    // end-entity `16`, xml-declaration `17` — so passing them through
    // `NodeType::from_int` would mislabel them (e.g. a closing `</x>` as an
    // `ElementDecl`). Those have no `NodeType` equivalent, hence `None`.
    let t = unsafe { xmlTextReaderNodeType(self.ptr) };
    if (1..=12).contains(&t) {
      NodeType::from_int(t as xmlElementType)
    } else {
      None
    }
  }

  /// True when positioned on an element *start* tag.
  pub fn is_element(&self) -> bool {
    self.node_type() == Some(NodeType::ElementNode)
  }

  /// The current reader event, losslessly (see [`ReaderEvent`]). Unlike
  /// [`node_type`](Self::node_type), this distinguishes a closing `</x>`
  /// (`EndElement`) from inter-element whitespace (`Whitespace` /
  /// `SignificantWhitespace`).
  pub fn event(&self) -> ReaderEvent {
    ReaderEvent::from_int(unsafe { xmlTextReaderNodeType(self.ptr) })
  }

  /// The current node's depth in the tree (root element = 0).
  pub fn depth(&self) -> i32 {
    unsafe { xmlTextReaderDepth(self.ptr) }
  }

  /// The current node's local name (no namespace prefix), if any.
  pub fn local_name(&self) -> Option<String> {
    const_xmlchar_to_string(unsafe { xmlTextReaderConstLocalName(self.ptr) })
  }

  /// The current node's namespace URI, if any.
  pub fn namespace_uri(&self) -> Option<String> {
    const_xmlchar_to_string(unsafe { xmlTextReaderConstNamespaceUri(self.ptr) })
  }

  /// Fully build the current node's subtree and borrow it read-only.
  ///
  /// Zero-copy. **The returned [`RoNode`] is owned by the reader and is
  /// invalidated by the next [`read`](Self::read)/[`read_next`](Self::read_next)** — do
  /// not retain it across an advance. For a subtree you can keep, use
  /// [`expand_to_document`](Self::expand_to_document). Returns `None` at end of
  /// input or on error.
  pub fn expand(&self) -> Option<RoNode> {
    self.current_subtree().map(RoNode)
  }

  /// The current node's fully-built subtree as a raw pointer, or `None` at end
  /// of input / on error. Borrowed from the reader — invalidated by the next
  /// advance; callers must copy (see [`expand_to_document`](Self::expand_to_document))
  /// to outlive it.
  fn current_subtree(&self) -> Option<xmlNodePtr> {
    let node = unsafe { xmlTextReaderExpand(self.ptr) };
    (!node.is_null()).then_some(node)
  }

  /// Copy the current node's subtree into a fresh, independently-owned
  /// [`Document`] whose root element is the copy.
  ///
  /// Namespaces declared on un-copied ancestors (e.g. the default `xmlns` on
  /// the real document root) are reconciled onto the copy via
  /// `xmlDOMWrapCloneNode`, so the result is self-contained — safe to hold,
  /// mutate, transform and serialize after the reader has advanced and freed
  /// its own copy of the subtree. Returns `None` at end of input or on error.
  pub fn expand_to_document(&self) -> Option<Document> {
    let node = self.current_subtree()?;
    unsafe {
      let newdoc = xmlNewDoc(c"1.0".as_ptr() as *const xmlChar);
      if newdoc.is_null() {
        return None;
      }
      // xmlDOMWrapCloneNode (unlike xmlDocCopyNode) reconciles the source
      // ancestors' in-scope namespaces onto the clone, so it doesn't dangle
      // into the source once the reader frees it. The wrap context is
      // required: with a NULL context the clone keeps an ns *pointer* but the
      // `xmlns=` decl is never materialized, so serialization silently drops it.
      let ctxt = xmlDOMWrapNewCtxt();
      let mut cloned: xmlNodePtr = ptr::null_mut();
      let src_doc = (*node).doc;
      let rc = xmlDOMWrapCloneNode(
        ctxt,
        src_doc,
        node,
        &mut cloned,
        newdoc,
        ptr::null_mut(), // no destination parent — it becomes the root
        1,               // deep
        0,               // options
      );
      xmlDOMWrapFreeCtxt(ctxt);
      if rc != 0 || cloned.is_null() {
        xmlFreeDoc(newdoc);
        return None;
      }
      xmlDocSetRootElement(newdoc, cloned);
      // Belt-and-suspenders: ensure every namespace used in the detached tree
      // is declared within it (self-contained serialization, no dangling ns).
      xmlReconciliateNs(newdoc, cloned);
      // …but undo `xmlNewReconciledNs`'s prefix minting: a *default* (NULL
      // prefix) namespace declared on an un-copied ancestor comes back as
      // `xmlns:default="…"` (then `default1`, …), so every element serializes
      // as `<default:x>` — the classic "annoying default prefix" trap, and a
      // real corruption for callers that re-serialize subtrees (a fragment
      // using `default:` never re-parses into the right namespace unless that
      // fabricated declaration travels with it). Restore each minted
      // declaration's prefix to the SOURCE element's prefix for the same href
      // (usually NULL), unless that prefix is already taken on the clone.
      let mut decl = (*cloned).nsDef;
      while !decl.is_null() {
        let prefix = (*decl).prefix;
        if !prefix.is_null()
          && xmlStrncmp(prefix, c"default".as_ptr() as *const xmlChar, 7) == 0
        {
          let src_ns = xmlSearchNsByHref(src_doc, node, (*decl).href);
          if !src_ns.is_null() {
            let want = (*src_ns).prefix;
            let mut clash = false;
            let mut other = (*cloned).nsDef;
            while !other.is_null() {
              if other != decl && xmlStrEqual((*other).prefix, want) == 1 {
                clash = true;
                break;
              }
              other = (*other).next;
            }
            if !clash && xmlStrEqual(prefix, want) == 0 {
              let old = (*decl).prefix as *mut ::std::os::raw::c_void;
              (*decl).prefix = if want.is_null() {
                ptr::null()
              } else {
                xmlStrdup(want)
              };
              // Portable free: on MSVC `xmlFree` is not a linkable data
              // symbol (LNK2019 in 0.3.18); `bindgenFree` carries the
              // per-target arm the crate already uses elsewhere.
              crate::c_helpers::bindgenFree(old);
            }
          }
        }
        decl = (*decl).next;
      }
      Some(Document::new_ptr(newdoc))
    }
  }

  /// The current element's attributes as `(qualified-name, value)` pairs in
  /// document order, **including namespace declarations** (`xmlns`,
  /// `xmlns:pfx`), without expanding the subtree.
  ///
  /// This is the streaming way to inspect an element *before* deciding whether
  /// to materialize it — [`expand`](Self::expand) would build the whole
  /// subtree, which for a large container element defeats the point of
  /// streaming. Returns an empty vec on non-element nodes.
  ///
  /// Values are fully entity/charref-decoded (libxml2 reader semantics); a
  /// caller re-serializing them must re-escape.
  pub fn attributes_qname(&mut self) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if unsafe { xmlTextReaderMoveToFirstAttribute(self.ptr) } != 1 {
      return out;
    }
    loop {
      let name = const_xmlchar_to_string(unsafe { xmlTextReaderConstName(self.ptr) });
      let value = const_xmlchar_to_string(unsafe { xmlTextReaderConstValue(self.ptr) });
      if let (Some(n), Some(v)) = (name, value) {
        out.push((n, v));
      }
      if unsafe { xmlTextReaderMoveToNextAttribute(self.ptr) } != 1 {
        break;
      }
    }
    // Restore the reader to the element node so subsequent
    // `local_name`/`expand`/`read` calls see the element, not its last
    // attribute.
    unsafe { xmlTextReaderMoveToElement(self.ptr) };
    out
  }

  /// The current node's text value (text/CDATA content, comment text, or
  /// processing-instruction body). `None` for valueless nodes (e.g. an
  /// element start).
  pub fn value(&self) -> Option<String> {
    const_xmlchar_to_string(unsafe { xmlTextReaderConstValue(self.ptr) })
  }

  /// True when positioned on an empty element tag (`<x/>`), which the reader
  /// reports as a *start* event with no matching end-element event.
  pub fn is_empty_element(&self) -> bool {
    (unsafe { xmlTextReaderIsEmptyElement(self.ptr) }) == 1
  }

  /// Serialize the current node's subtree exactly as it appears in the input
  /// (no XML declaration, no added namespace declarations, attribute order
  /// preserved). Position is unchanged; call [`read_next`](Self::read_next) to
  /// move past the subtree. Returns `None` at end of input or on error.
  ///
  /// Deliberately NOT `xmlTextReaderReadOuterXml`: that API deep-copies the
  /// expanded node *parentless* first, and for content in a *default*
  /// namespace declared on an un-copied ancestor the copy's namespace fixup
  /// (`xmlNewReconciledNs`) then **mints a `default:` prefix** onto every
  /// element — `<para>` serializes as `<default:para
  /// xmlns:default="…">`. Dumping the reader-owned node directly keeps its
  /// ancestors (and their namespace declarations) reachable, so elements
  /// serialize with their original prefixes and no fabricated declarations —
  /// the fragment re-parses correctly inside any wrapper that re-declares the
  /// same namespaces.
  pub fn outer_xml(&self) -> Option<String> {
    let node = self.current_subtree()?;
    unsafe {
      let buf = xmlBufferCreate();
      if buf.is_null() {
        return None;
      }
      let rc = xmlNodeDump(buf, (*node).doc, node, 0, 0);
      let content = xmlBufferContent(buf);
      let result = if rc < 0 || content.is_null() {
        None
      } else {
        Some(
          CStr::from_ptr(content as *const c_char)
            .to_string_lossy()
            .into_owned(),
        )
      };
      xmlBufferFree(buf);
      result
    }
  }

  /// Advance until positioned on the next element whose `(namespace, localname)`
  /// satisfies `want`, or the end of input.
  ///
  /// This is the streaming analogue of a downward `//name` XPath step: the only
  /// XPath subset that is actually streamable. Returns `Ok(true)` when
  /// positioned on a match (then call [`expand`](Self::expand) /
  /// [`expand_to_document`](Self::expand_to_document), and
  /// [`read_next`](Self::read_next) to skip past it), `Ok(false)` at end of input.
  ///
  /// `want` receives the namespace URI (`None` if the element is in no
  /// namespace) and the local name.
  pub fn read_to_next<F>(&mut self, want: F) -> Result<bool, ()>
  where
    F: Fn(Option<&str>, &str) -> bool,
  {
    while self.read()? {
      if self.is_element()
        && let Some(name) = self.local_name()
        && want(self.namespace_uri().as_deref(), &name)
      {
        return Ok(true);
      }
    }
    Ok(false)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const NS: &str = "http://example.org/ns";

  fn write_temp(name: &str, xml: &str) -> String {
    let path = std::env::temp_dir().join(format!(
      "rust-libxml-reader-{}-{name}.xml",
      std::process::id()
    ));
    std::fs::write(&path, xml).unwrap();
    path.to_string_lossy().into_owned()
  }

  /// Stream a multi-section document, collect each `<section>` as an owned
  /// Document, and verify — crucially, *after the reader is dropped* — that the
  /// copies are self-contained: namespaces inherited from the (un-copied) root
  /// are reconciled onto each copy, and content survives.
  #[test]
  fn stream_sections_owned_and_namespace_reconciled() {
    let xml = r#"<?xml version="1.0"?>
<doc xmlns="http://example.org/ns" xmlns:x="http://example.org/x">
  <meta>skip me</meta>
  <section id="a"><title>Alpha</title><p>one</p></section>
  <section id="b"><title>Beta</title><x:note>hi</x:note></section>
</doc>"#;
    let path = write_temp("sections", xml);

    let mut sections = Vec::new();
    {
      let mut reader = TextReader::from_file(&path, 0).unwrap();
      while reader
        .read_to_next(|ns, name| ns == Some(NS) && name == "section")
        .unwrap()
      {
        sections.push(reader.expand_to_document().unwrap());
      }
      // reader dropped here — its copies of the subtrees are freed.
    }

    assert_eq!(sections.len(), 2, "should stream exactly two <section>s");

    // Root of each owned doc is a <section> in the reconciled default ns.
    let root0 = sections[0].get_root_element().unwrap();
    assert_eq!(root0.get_name(), "section");
    assert_eq!(root0.get_attribute("id").as_deref(), Some("a"));
    assert_eq!(
      root0.get_namespace().map(|n| n.get_href()),
      Some(NS.to_string()),
      "default namespace must be reconciled onto the detached copy"
    );

    // Serialization is intact and namespace-declared (no dangling ns → no UAF).
    let s0 = sections[0].to_string();
    assert!(
      s0.contains("http://example.org/ns"),
      "ns decl missing: {s0}"
    );
    // The reconciliation must NOT have minted a `default:` prefix for the
    // inherited default namespace: the copy serializes with `xmlns=`, exactly
    // as a standalone parse of the same subtree would.
    assert!(
      !s0.contains("default:"),
      "default-namespace content must keep a NULL prefix, not a minted default: — {s0}"
    );
    assert!(
      s0.contains(r#"<section xmlns="http://example.org/ns""#),
      "the default declaration must materialize on the copy root: {s0}"
    );
    assert!(
      s0.contains("Alpha") && s0.contains("one"),
      "content lost: {s0}"
    );

    // The second section keeps its prefixed namespace too.
    let s1 = sections[1].to_string();
    assert!(s1.contains("Beta"), "content lost: {s1}");
    assert!(
      s1.contains("http://example.org/x"),
      "prefixed ns lost: {s1}"
    );

    std::fs::remove_file(&path).ok();
  }

  /// `read`/`next`/`is_element`/`local_name` walk the tree and `read_next` skips a
  /// subtree (does not descend).
  #[test]
  fn read_and_next_skip_subtree() {
    let xml = r#"<r><a><deep/></a><b/></r>"#;
    let path = write_temp("skip", xml);
    let mut reader = TextReader::from_file(&path, 0).unwrap();

    assert!(reader.read().unwrap()); // <r>
    assert!(reader.is_element());
    assert_eq!(reader.local_name().as_deref(), Some("r"));

    assert!(reader.read().unwrap()); // <a>
    assert_eq!(reader.local_name().as_deref(), Some("a"));

    // read_next() skips <a>'s subtree (the <deep/>) → lands on <b>.
    assert!(reader.read_next().unwrap());
    assert_eq!(reader.local_name().as_deref(), Some("b"));

    std::fs::remove_file(&path).ok();
  }

  /// Opening a reader on a path that does not exist fails at construction
  /// (`xmlReaderForFile` returns NULL), rather than deferring to the first read.
  #[test]
  fn from_file_on_missing_path_is_err() {
    assert!(TextReader::from_file("/no/such/rust-libxml-reader-missing.xml", 0).is_err());
  }

  /// A well-formedness violation surfaces as `Err(())` from `read`, not a silent
  /// early `Ok(false)` — so a caller streaming a truncated/corrupt file can tell
  /// "document ended" apart from "document is broken".
  #[test]
  fn read_surfaces_parse_error_on_malformed_xml() {
    // </a> closes before the still-open <b> — not well-formed.
    let path = write_temp("malformed", "<a><b></a>");
    let mut reader = TextReader::from_file(&path, 0).unwrap();
    let mut saw_err = false;
    loop {
      match reader.read() {
        Ok(true) => continue,
        Ok(false) => break,
        Err(()) => {
          saw_err = true;
          break;
        }
      }
    }
    assert!(
      saw_err,
      "malformed XML must surface a read error, not Ok(false)"
    );
    std::fs::remove_file(&path).ok();
  }

  /// `read_to_next` that never matches consumes the whole document and returns
  /// `Ok(false)` at end of input (the streaming analogue of an empty node-set).
  #[test]
  fn read_to_next_returns_false_when_pattern_absent() {
    let path = write_temp("nomatch", r#"<doc><a/><b/></doc>"#);
    let mut reader = TextReader::from_file(&path, 0).unwrap();
    let found = reader.read_to_next(|_ns, name| name == "zzz").unwrap();
    assert!(
      !found,
      "no <zzz> exists → read_to_next must reach EOF and return false"
    );
    std::fs::remove_file(&path).ok();
  }

  /// `attributes_qname` reports qualified names + namespace declarations in
  /// document order, without expanding, and leaves the reader positioned on
  /// the element.
  #[test]
  fn attributes_qname_in_order_without_expand() {
    let xml = r#"<doc xmlns="http://example.org/ns" xmlns:x="http://example.org/x">
  <section xml:id="s1" class="c" x:extra="e"><p>body</p></section>
</doc>"#;
    let path = write_temp("attrs", xml);
    let mut reader = TextReader::from_file(&path, 0).unwrap();

    assert!(reader.read().unwrap()); // <doc>
    let root_attrs = reader.attributes_qname();
    assert_eq!(
      root_attrs,
      vec![
        ("xmlns".to_string(), "http://example.org/ns".to_string()),
        ("xmlns:x".to_string(), "http://example.org/x".to_string()),
      ],
      "namespace declarations must be reported as ordinary attributes"
    );
    // Reader restored to the element: name still <doc>, and streaming resumes.
    assert_eq!(reader.local_name().as_deref(), Some("doc"));

    assert!(
      reader
        .read_to_next(|_, name| name == "section")
        .unwrap()
    );
    assert_eq!(
      reader.attributes_qname(),
      vec![
        ("xml:id".to_string(), "s1".to_string()),
        ("class".to_string(), "c".to_string()),
        ("x:extra".to_string(), "e".to_string()),
      ],
      "attribute order must be document order, names fully qualified"
    );
    std::fs::remove_file(&path).ok();
  }

  /// `outer_xml` on default-namespace content must NOT invent a `default:`
  /// prefix (the `xmlTextReaderReadOuterXml` + parentless-copy trap) and must
  /// not add namespace declarations the input element does not carry.
  #[test]
  fn outer_xml_preserves_default_namespace_content() {
    let xml = r#"<doc xmlns="http://example.org/ns" xmlns:x="http://example.org/x">
  <section a="1" b="&lt;2&gt;"><p>t&amp;t</p><x:note>hi</x:note></section>
</doc>"#;
    let path = write_temp("outerxml", xml);
    let mut reader = TextReader::from_file(&path, 0).unwrap();
    assert!(
      reader
        .read_to_next(|_, name| name == "section")
        .unwrap()
    );
    let outer = reader.outer_xml().unwrap();
    assert_eq!(
      outer,
      r#"<section a="1" b="&lt;2&gt;"><p>t&amp;t</p><x:note>hi</x:note></section>"#,
      "no default: prefix, no added xmlns decls, escaping and attr order intact"
    );
    // Position unchanged: the same subtree can still be skipped as a unit.
    assert_eq!(reader.local_name().as_deref(), Some("section"));
    assert!(reader.read_next().unwrap()); // past </section> → </doc> close
    std::fs::remove_file(&path).ok();
  }

  /// `value` returns text/comment/PI content; `is_empty_element` distinguishes
  /// `<x/>` from `<x></x>`.
  #[test]
  fn value_and_is_empty_element() {
    let xml = r#"<r><?pi data?><!--note--><a/><b></b>text</r>"#;
    let path = write_temp("value", xml);
    let mut reader = TextReader::from_file(&path, 0).unwrap();

    assert!(reader.read().unwrap()); // <r>
    assert!(!reader.is_empty_element());

    assert!(reader.read().unwrap()); // <?pi data?>
    assert_eq!(reader.node_type(), Some(NodeType::PiNode));
    assert_eq!(reader.local_name().as_deref(), Some("pi"));
    assert_eq!(reader.value().as_deref(), Some("data"));

    assert!(reader.read().unwrap()); // <!--note-->
    assert_eq!(reader.node_type(), Some(NodeType::CommentNode));
    assert_eq!(reader.value().as_deref(), Some("note"));

    assert!(reader.read().unwrap()); // <a/>
    assert!(reader.is_empty_element());

    assert!(reader.read().unwrap()); // <b>
    assert!(!reader.is_empty_element());
    assert!(reader.read().unwrap()); // </b> close

    assert!(reader.read().unwrap()); // text
    assert_eq!(reader.node_type(), Some(NodeType::TextNode));
    assert_eq!(reader.value().as_deref(), Some("text"));

    std::fs::remove_file(&path).ok();
  }

  /// The documented contract: an opening `<x>` is `Some(ElementNode)` but a
  /// closing `</x>` is `None` — NOT a bogus `ElementDecl`. `xmlReaderTypes`
  /// END_ELEMENT (15) collides numerically with `XML_ELEMENT_DECL`, so this
  /// pins the `node_type` guard that keeps the two apart.
  #[test]
  fn node_type_distinguishes_open_from_close_tag() {
    let path = write_temp("openclose", r#"<r><a>x</a></r>"#);
    let mut reader = TextReader::from_file(&path, 0).unwrap();

    assert!(reader.read().unwrap()); // <r> open
    assert_eq!(reader.node_type(), Some(NodeType::ElementNode));
    assert!(reader.is_element());

    assert!(reader.read().unwrap()); // <a> open
    assert_eq!(reader.node_type(), Some(NodeType::ElementNode));

    assert!(reader.read().unwrap()); // text "x"
    assert_eq!(reader.node_type(), Some(NodeType::TextNode));
    assert!(!reader.is_element());

    assert!(reader.read().unwrap()); // </a> close
    assert_eq!(reader.local_name().as_deref(), Some("a"));
    assert_eq!(
      reader.node_type(),
      None,
      "a closing tag has no NodeType equivalent — must be None, not ElementDecl"
    );
    assert!(!reader.is_element());

    std::fs::remove_file(&path).ok();
  }
}
