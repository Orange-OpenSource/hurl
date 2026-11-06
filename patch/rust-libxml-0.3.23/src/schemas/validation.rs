//!
//! Wrapping of the Validation Context (xmlSchemaValidCtxt)
//!
use super::common;

use super::Schema;
use super::SchemaParserContext;

use crate::bindings;

use crate::tree::document::Document;
use crate::tree::node::Node;

use crate::error::StructuredError;

use std::ffi::CString;
use std::os::raw::c_char;

/// Wrapper on xmlSchemaValidCtxt
pub struct SchemaValidationContext {
  ctxt: *mut bindings::_xmlSchemaValidCtxt,
  errlog: *mut Vec<StructuredError>,
  _schema: Schema,
}

impl SchemaValidationContext {
  /// Create a schema validation context from a parser object
  pub fn from_parser(parser: &mut SchemaParserContext) -> Result<Self, Vec<StructuredError>> {
    let schema = Schema::from_parser(parser);

    match schema {
      Ok(s) => {
        let ctx = unsafe { bindings::xmlSchemaNewValidCtxt(s.as_ptr()) };

        if ctx.is_null() {
          panic!("Failed to create validation context from XML schema") // TODO error handling
        }

        Ok(Self::from_raw(ctx, s))
      }
      Err(e) => Err(e),
    }
  }

  /// Validates a given Document, that is to be tested to comply with the loaded XSD schema definition
  pub fn validate_document(&mut self, doc: &Document) -> Result<(), Vec<StructuredError>> {
    let rc = unsafe { bindings::xmlSchemaValidateDoc(self.ctxt, doc.doc_ptr()) };

    self.validation_result(rc)
  }

  /// Validates a given file from path for its compliance with the loaded XSD schema definition
  pub fn validate_file(&mut self, path: &str) -> Result<(), Vec<StructuredError>> {
    let path = CString::new(path).unwrap(); // TODO error handling for \0 containing strings
    let path_ptr = path.as_bytes_with_nul().as_ptr() as *const c_char;

    let rc = unsafe { bindings::xmlSchemaValidateFile(self.ctxt, path_ptr, 0) };

    self.validation_result(rc)
  }

  /// Validates a branch or leaf of a document given as a Node against the loaded XSD schema definition
  pub fn validate_node(&mut self, node: &Node) -> Result<(), Vec<StructuredError>> {
    let rc = unsafe { bindings::xmlSchemaValidateOneElement(self.ctxt, node.node_ptr()) };

    self.validation_result(rc)
  }

  fn validation_result(&mut self, rc: i32) -> Result<(), Vec<StructuredError>> {
    if rc == 0 {
      return Ok(());
    }
    let mut errors = self.drain_errors();
    if errors.is_empty() {
      errors.push(StructuredError {
        message: Some(format!(
          "Schema validation failed with status {rc} without a diagnostic"
        )),
        level: crate::error::XmlErrorLevel::Fatal,
        filename: None,
        line: None,
        col: None,
        domain: bindings::xmlErrorDomain_XML_FROM_SCHEMASV as i32,
        code: bindings::xmlParserErrors_XML_SCHEMAV_INTERNAL as i32,
      });
    }
    Err(errors)
  }

  /// Drains error log from errors that might have accumulated while validating something
  pub fn drain_errors(&mut self) -> Vec<StructuredError> {
    assert!(!self.errlog.is_null());
    let errors = unsafe { &mut *self.errlog };
    std::mem::take(errors)
  }

  /// Return a raw pointer to the underlying xmlSchemaValidCtxt structure
  pub fn as_ptr(&self) -> *mut bindings::_xmlSchemaValidCtxt {
    self.ctxt
  }
}

/// Private Interface
impl SchemaValidationContext {
  fn from_raw(ctx: *mut bindings::_xmlSchemaValidCtxt, schema: Schema) -> Self {
    let errors: Box<Vec<StructuredError>> = Box::default();

    unsafe {
      let reference: *mut Vec<StructuredError> = std::mem::transmute(errors);
      bindings::xmlSchemaSetValidStructuredErrors(
        ctx,
        Some(common::structured_error_handler),
        reference as *mut _,
        // Box::into_raw(Box::new(Rc::downgrade(&errors))) as *mut _,
      );
      Self {
        ctxt: ctx,
        errlog: reference,
        _schema: schema,
      }
    }
  }
}

impl Drop for SchemaValidationContext {
  fn drop(&mut self) {
    unsafe {
      bindings::xmlSchemaFreeValidCtxt(self.ctxt);
      if !self.errlog.is_null() {
        let errors: Box<Vec<StructuredError>> = std::mem::transmute(self.errlog);
        drop(errors)
      }
    }
  }
}
