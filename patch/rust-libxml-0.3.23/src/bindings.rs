// Issues coming from bindgen
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(missing_docs)]

/// Formerly the well-formedness check's process-global flag, which raced between
/// threads. Unused since 0.3.22; kept so existing references still compile.
#[deprecated(
  since = "0.3.22",
  note = "unused: `Parser::is_well_formed_html` keeps its state per call"
)]
pub static mut HACKY_WELL_FORMED: bool = false;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
