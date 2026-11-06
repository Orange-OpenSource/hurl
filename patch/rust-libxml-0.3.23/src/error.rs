//!
//! Wrapper for xmlError
//!
use super::bindings;

use std::ffi::{CStr, c_char, c_int};

/// Rust enum variant of libxml2's xmlErrorLevel
#[derive(Debug)]
pub enum XmlErrorLevel {
  /// No error
  None,
  /// A simple warning
  Warning,
  /// A recoverable error
  Error,
  /// A fatal error
  Fatal,
}

impl XmlErrorLevel {
  /// Convert an xmlErrorLevel provided by libxml2 (as an integer) into a Rust enum
  pub fn from_raw(error_level: bindings::xmlErrorLevel) -> XmlErrorLevel {
    match error_level {
      bindings::xmlErrorLevel_XML_ERR_NONE => XmlErrorLevel::None,
      bindings::xmlErrorLevel_XML_ERR_WARNING => XmlErrorLevel::Warning,
      bindings::xmlErrorLevel_XML_ERR_ERROR => XmlErrorLevel::Error,
      bindings::xmlErrorLevel_XML_ERR_FATAL => XmlErrorLevel::Fatal,
      _ => unreachable!("Should never receive an error level not in the range 0..=3"),
    }
  }
}

/// Wrapper around xmlErrorPtr.
/// Some fields have been omitted for simplicity/safety
#[derive(Debug)]
pub struct StructuredError {
  /// Human-friendly error message, lossily converted into UTF-8 from the underlying
  /// C string. May be `None` if an error message is not provided by libxml2.
  pub message: Option<String>,
  /// The error's level
  pub level: XmlErrorLevel,
  /// The filename, lossily converted into UTF-8 from the underlying C string.
  /// May be `None` if a filename is not provided by libxml2, such as when validating
  /// an XML document stored entirely in memory.
  pub filename: Option<String>,
  /// The linenumber, or None if not applicable.
  pub line: Option<c_int>,
  /// The column where the error is present, or None if not applicable.
  pub col: Option<c_int>,

  /// The module that the error came from. See libxml's xmlErrorDomain enum.
  pub domain: c_int,
  /// The variety of error. See libxml's xmlParserErrors enum.
  pub code: c_int,
}

impl StructuredError {
  /// Copies the error information stored at `error_ptr` into a new `StructuredError`
  ///
  /// # Safety
  /// This function must be given a pointer to a valid `xmlError` struct. Typically, you
  /// will acquire such a pointer by implementing one of a number of callbacks
  /// defined in libXml which are provided an `xmlError` as an argument.
  ///
  /// This function copies data from the memory `error_ptr` but does not deallocate
  /// the error. Depending on the context in which this function is used, you may
  /// need to take additional steps to avoid a memory leak.
  pub unsafe fn from_raw(error_ptr: *const bindings::xmlError) -> Self {
    unsafe {
      let error = *error_ptr;
      let message = StructuredError::ptr_to_string(error.message);
      let level = XmlErrorLevel::from_raw(error.level);
      let filename = StructuredError::ptr_to_string(error.file);

      let line = if error.line == 0 {
        None
      } else {
        Some(error.line)
      };
      let col = if error.int2 == 0 {
        None
      } else {
        Some(error.int2)
      };

      StructuredError {
        message,
        level,
        filename,
        line,
        col,
        domain: error.domain,
        code: error.code,
      }
    }
  }

  /// Human-readable informative error message.
  ///
  /// This function is a hold-over from the original bindings to libxml's error
  /// reporting mechanism. Instead of calling this method, you can access the
  /// StructuredError `message` field directly.
  #[deprecated(
    since = "0.3.3",
    note = "Please use the `message` field directly instead."
  )]
  pub fn message(&self) -> &str {
    self.message.as_deref().unwrap_or("")
  }

  /// Returns the provided c_str as Some(String), or None if the provided pointer is null.
  fn ptr_to_string(c_str: *mut c_char) -> Option<String> {
    if c_str.is_null() {
      return None;
    }

    let raw_str = unsafe { CStr::from_ptr(c_str) };
    Some(String::from_utf8_lossy(raw_str.to_bytes()).to_string())
  }
}
