//! Shared canonicalization logic and types.
//!
use std::ffi::c_int;

use crate::bindings::{
  xmlC14NMode_XML_C14N_1_0, xmlC14NMode_XML_C14N_1_1, xmlC14NMode_XML_C14N_EXCLUSIVE_1_0,
};

/// Options for configuring how to canonicalize XML
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CanonicalizationOptions {
  /// Canonicalization specification to use
  pub mode: CanonicalizationMode,
  /// If true, keep `<!-- ... -->` comments, otherwise remove
  pub with_comments: bool,
  /// Namespaces to keep even if they are unused. By default, in [CanonicalizationMode::ExclusiveCanonical1_0], unused namespaces are removed.
  ///
  /// Doesn't apply to other canonicalization modes.
  pub inclusive_ns_prefixes: Vec<String>,
}

/// Canonicalization specification to use
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum CanonicalizationMode {
  /// Original C14N 1.0 spec
  Canonical1_0,
  /// Exclusive C14N 1.0 spec
  #[default]
  ExclusiveCanonical1_0,
  /// C14N 1.1 spec
  Canonical1_1,
}

impl From<CanonicalizationMode> for c_int {
  fn from(mode: CanonicalizationMode) -> Self {
    let c14n_mode = match mode {
      CanonicalizationMode::Canonical1_0 => xmlC14NMode_XML_C14N_1_0,
      CanonicalizationMode::ExclusiveCanonical1_0 => xmlC14NMode_XML_C14N_EXCLUSIVE_1_0,
      CanonicalizationMode::Canonical1_1 => xmlC14NMode_XML_C14N_1_1,
    };

    c_int::from(c14n_mode as i32)
  }
}
