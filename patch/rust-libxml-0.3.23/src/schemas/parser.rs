//!
//! Wrapping of the Parser Context (xmlSchemaParserCtxt)
//!
use super::common;

use crate::bindings;
use crate::error::StructuredError;
use crate::tree::document::Document;

use std::ffi::CString;
use std::os::raw::c_char;

/// Wrapper on xmlSchemaParserCtxt
pub struct SchemaParserContext {
  inner: *mut bindings::_xmlSchemaParserCtxt,
  errlog: *mut Vec<StructuredError>,
}

impl SchemaParserContext {
  /// Create a schema parsing context from a Document object
  pub fn from_document(doc: &Document) -> Self {
    crate::init_parser();
    let parser = unsafe { bindings::xmlSchemaNewDocParserCtxt(doc.doc_ptr()) };

    if parser.is_null() {
      panic!("Failed to create schema parser context from XmlDocument"); // TODO error handling
    }

    Self::from_raw(parser)
  }

  /// Create a schema parsing context from a buffer in memory
  pub fn from_buffer<Bytes: AsRef<[u8]>>(buff: Bytes) -> Self {
    crate::init_parser();
    let buff_bytes = buff.as_ref();
    let buff_ptr = buff_bytes.as_ptr() as *const c_char;
    let buff_len = buff_bytes.len() as i32;

    let parser = unsafe { bindings::xmlSchemaNewMemParserCtxt(buff_ptr, buff_len) };

    if parser.is_null() {
      panic!("Failed to create schema parser context from buffer"); // TODO error handling
    }

    Self::from_raw(parser)
  }

  /// Create a schema parsing context from an URL
  pub fn from_file(path: &str) -> Self {
    crate::init_parser();
    let path = CString::new(path).unwrap(); // TODO error handling for \0 containing strings
    let path_ptr = path.as_bytes_with_nul().as_ptr() as *const c_char;

    let parser = unsafe { bindings::xmlSchemaNewParserCtxt(path_ptr) };

    if parser.is_null() {
      panic!("Failed to create schema parser context from path"); // TODO error handling
    }

    Self::from_raw(parser)
  }

  /// Drains error log from errors that might have accumulated while parsing schema
  pub fn drain_errors(&mut self) -> Vec<StructuredError> {
    assert!(!self.errlog.is_null());
    let errors = unsafe { &mut *self.errlog };
    std::mem::take(errors)
  }

  /// Return a raw pointer to the underlying xmlSchemaParserCtxt structure
  pub fn as_ptr(&self) -> *mut bindings::_xmlSchemaParserCtxt {
    self.inner
  }
}

/// Private Interface
impl SchemaParserContext {
  fn from_raw(parser: *mut bindings::_xmlSchemaParserCtxt) -> Self {
    let errors: Box<Vec<StructuredError>> = Box::default();

    unsafe {
      let reference: *mut Vec<StructuredError> = std::mem::transmute(errors);
      bindings::xmlSchemaSetParserStructuredErrors(
        parser,
        Some(common::structured_error_handler),
        reference as *mut _,
      );

      Self {
        inner: parser,
        errlog: reference,
      }
    }
  }
}

impl Drop for SchemaParserContext {
  fn drop(&mut self) {
    unsafe {
      bindings::xmlSchemaFreeParserCtxt(self.inner);
      if !self.errlog.is_null() {
        let errors: Box<Vec<StructuredError>> = std::mem::transmute(self.errlog);
        drop(errors)
      }
    }
  }
}
