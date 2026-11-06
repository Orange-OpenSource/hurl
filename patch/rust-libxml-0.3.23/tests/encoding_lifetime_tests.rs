//! Red/green tests for the lifetime of the encoding name handed to libxml2.
//!
//! Each parse entry point converts `ParserOptions::encoding` into a C string and
//! passes its pointer to libxml2. If the `CString` is dropped before the call
//! (#216: matched by value), libxml2 reads freed memory, which usually still
//! holds the name, so the bug only shows under valgrind. This binary's
//! allocator overwrites freed memory with a junk name, turning a dangling
//! pointer into an unknown encoding and a failing assertion.
//!
//! The parse tests use XML: on invalid UTF-8 the HTML parser falls back to
//! Latin-1, which would hide the dropped name. The well-formedness check uses
//! UTF-16LE without a BOM instead, which only parses with the name intact.
use std::alloc::{GlobalAlloc, Layout, System};

use libxml::parser::{Parser, ParserOptions};

struct PoisonOnFree;

unsafe impl GlobalAlloc for PoisonOnFree {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    unsafe { System.alloc(layout) }
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    unsafe {
      // A NUL-terminated junk name in place of whatever C string lived here.
      if layout.size() > 0 {
        std::ptr::write_bytes(ptr, b'X', layout.size());
        *ptr.add(layout.size() - 1) = 0;
      }
      System.dealloc(ptr, layout)
    }
  }
}

#[global_allocator]
static ALLOCATOR: PoisonOnFree = PoisonOnFree;

const LATIN1_BYTES: &[u8] = b"<root>caf\xe9</root>";
const LATIN1_FILE: &str = "tests/resources/file01_latin1_nodecl.xml";

fn latin1() -> ParserOptions<'static> {
  ParserOptions {
    encoding: Some("ISO-8859-1"),
    ..Default::default()
  }
}

#[test]
fn parse_string_with_options_keeps_encoding_alive() {
  let doc = Parser::default()
    .parse_string_with_options(LATIN1_BYTES, latin1())
    .unwrap();
  assert_eq!(doc.get_root_element().unwrap().get_content(), "café");
}

#[test]
fn parse_file_with_options_keeps_encoding_alive() {
  let doc = Parser::default()
    .parse_file_with_options(LATIN1_FILE, latin1())
    .unwrap();
  assert_eq!(doc.get_root_element().unwrap().get_content(), "café");
}

#[test]
fn parse_string_with_diagnostics_keeps_encoding_alive() {
  let (doc, _) = Parser::default()
    .parse_string_with_diagnostics(LATIN1_BYTES, latin1())
    .unwrap();
  assert_eq!(doc.get_root_element().unwrap().get_content(), "café");
}

#[test]
fn is_well_formed_html_with_encoding_keeps_encoding_alive() {
  let utf16le: Vec<u8> = "<!DOCTYPE html><html><body>x</body></html>"
    .encode_utf16()
    .flat_map(u16::to_le_bytes)
    .collect();
  assert!(Parser::default_html().is_well_formed_html_with_encoding(utf16le, Some("UTF-16LE")));
}
