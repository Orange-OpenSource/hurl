//!
//! Wrapping of the Schema (xmlSchema)
//!
use std::sync::OnceLock;

use super::SchemaParserContext;

use crate::bindings;

use crate::error::StructuredError;

static SCHEMA_TYPES_LOCK: OnceLock<bool> = OnceLock::new();

/// Wrapper on xmlSchema
pub struct Schema(*mut bindings::_xmlSchema);

impl Schema {
  /// Create schema by having a SchemaParserContext do the actual parsing of the schema it was provided
  pub fn from_parser(parser: &mut SchemaParserContext) -> Result<Self, Vec<StructuredError>> {
    // `xmlSchemaParse` calls `xmlSchemaInitTypes`.
    // `xmlSchemaInitTypes` is a lazy function which is only intended to be
    // called once for optimization purposes - but libxml2 doesn't do this
    // in a thread-safe manner.  We wrap the call in a OnceLock so that it
    // only ever needs to be invoked once - and will do it in a thread-safe
    // way.
    let _ = SCHEMA_TYPES_LOCK.get_or_init(|| {
      unsafe { bindings::xmlSchemaInitTypes() };
      true
    });

    let raw = unsafe { bindings::xmlSchemaParse(parser.as_ptr()) };

    if raw.is_null() {
      Err(parser.drain_errors())
    } else {
      Ok(Self(raw))
    }
  }

  /// Return a raw pointer to the underlying xmlSchema structure
  pub fn as_ptr(&self) -> *mut bindings::_xmlSchema {
    self.0
  }
}

impl Drop for Schema {
  fn drop(&mut self) {
    unsafe { bindings::xmlSchemaFree(self.0) }
  }
}
