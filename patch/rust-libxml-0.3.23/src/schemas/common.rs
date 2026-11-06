//!
//! Common Utilities
//!
use crate::bindings;

use crate::error::StructuredError;

use std::ffi::c_void;

/// Provides a callback to the C side of things to accumulate xmlErrors to be
/// handled back on the Rust side.
#[cfg(libxml_older_than_2_12)]
pub unsafe extern "C" fn structured_error_handler(ctx: *mut c_void, error: bindings::xmlErrorPtr) {
  assert!(!ctx.is_null());
  let errlog = unsafe { &mut *{ ctx as *mut Vec<StructuredError> } };

  let error = unsafe { StructuredError::from_raw(error) };

  errlog.push(error);
}

#[cfg(not(libxml_older_than_2_12))]
pub unsafe extern "C" fn structured_error_handler(
  ctx: *mut c_void,
  error: *const bindings::xmlError,
) {
  assert!(!ctx.is_null());
  let errlog = unsafe { &mut *{ ctx as *mut Vec<StructuredError> } };

  let error = unsafe { StructuredError::from_raw(error) };

  errlog.push(error);
}
