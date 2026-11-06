//! Custom I/O callbacks for libxml2.
//!
//! libxml2 routes every URL it loads (XML documents, `xsl:import` /
//! `xsl:include` targets, RelaxNG `<include>`, DTD external subsets,
//! etc.) through a chain of registered "input callback" handlers. The
//! default chain handles `file://`, `http://`, `ftp://`, etc.; an
//! application can prepend its own handler for a custom URL scheme via
//! `xmlRegisterInputCallbacks`.
//!
//! This module wraps that C API in a safe, closure-friendly surface.
//! The motivating use case is shipping a single-binary CLI that bundles
//! XSLT stylesheets / RNG schemas via `include_bytes!` and serves them
//! through a synthetic scheme (e.g. `embed:///foo.xsl`), so `xsl:import`
//! chains resolve without ever touching the disk.
//!
//! ```no_run
//! use libxml::io;
//!
//! static MAIN: &[u8] = b"<?xml version=\"1.0\"?>\n<root/>";
//!
//! io::register_input_callback(
//!   |url| url.starts_with("embed:///"),
//!   |url| match url.strip_prefix("embed:///") {
//!     Some("main.xml") => Some(MAIN.to_vec()),
//!     _ => None,
//!   },
//! );
//! ```
//!
//! ## Lifetime, threading, order
//!
//! Closures live for the process lifetime — libxml2 has no per-handler
//! unregister API. They may run on any thread (hence `Send + Sync`) and
//! **must not panic**: unwinding across the `extern "C"` trampoline
//! aborts on Rust 2024+. libxml2 walks callbacks newest-first; the
//! trampolines snapshot the registry and drop the lock before invoking
//! a user closure, so a closure that re-enters libxml2 won't
//! self-deadlock. If `open` returns `None`, libxml2 falls through to
//! the next handler — including its default file/HTTP loaders.

use std::ffi::{CStr, c_char, c_int, c_void};
use std::sync::{Arc, Mutex, OnceLock};

use crate::bindings::xmlRegisterInputCallbacks;

type MatchFn = Box<dyn Fn(&str) -> bool + Send + Sync + 'static>;
type OpenFn = Box<dyn Fn(&str) -> Option<Vec<u8>> + Send + Sync + 'static>;

struct Callback {
  match_url: MatchFn,
  open: OpenFn,
}

fn callbacks() -> &'static Mutex<Vec<Arc<Callback>>> {
  static CALLBACKS: OnceLock<Mutex<Vec<Arc<Callback>>>> = OnceLock::new();
  CALLBACKS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Clone the registry under the lock and return with the guard
/// dropped. Cloning is a cheap refcount bump per entry. The lock is
/// not held across user closures, so a closure that re-enters libxml2
/// can't self-deadlock on the registry mutex.
fn snapshot() -> Vec<Arc<Callback>> {
  callbacks().lock().unwrap().clone()
}

/// Register a custom input callback with libxml2.
///
/// `match_url` is consulted for every URL libxml2 considers loading.
/// Return `true` to claim the URL; the same callback's `open` is then
/// invoked. Either function can defer: `match_url` returning `false`
/// skips the callback; `open` returning `None` falls through to the
/// next registered handler, including libxml2's defaults.
///
/// Closures are `Send + Sync + 'static` because libxml2 may invoke
/// them from any thread. They are appended to a process-static
/// registry; there is no per-handler unregister.
///
/// # Example
///
/// Most commonly used to resolve `xsl:import` / `xsl:include` via
/// libxslt, or RelaxNG `<include>` via `xmlRelaxNGParse`. Note that
/// this crate's own `Parser::parse_file` uses Rust file I/O directly
/// and bypasses libxml2's URL machinery, so it does *not* trigger
/// these callbacks — wire them up via libxslt's `parse_bytes` or
/// libxml2's `xmlReadFile`.
///
/// Register before other threads start parsing: the first call adds the
/// trampolines to libxml2's input-callback table, which libxml2 does not lock.
///
/// ```no_run
/// use libxml::io;
///
/// static HELLO: &[u8] = b"<?xml version=\"1.0\"?>\n<hello>world</hello>";
///
/// io::register_input_callback(
///   |url| url.starts_with("embed:///"),
///   |url| (url == "embed:///hello.xml").then(|| HELLO.to_vec()),
/// );
/// ```
pub fn register_input_callback<M, O>(match_url: M, open: O)
where
  M: Fn(&str) -> bool + Send + Sync + 'static,
  O: Fn(&str) -> Option<Vec<u8>> + Send + Sync + 'static,
{
  callbacks().lock().unwrap().push(Arc::new(Callback {
    match_url: Box::new(match_url),
    open: Box::new(open),
  }));

  // libxml2 records the trampoline pointers in a static table;
  // registering twice would push duplicate entries that delegate to
  // the same Rust registry. The OnceLock avoids that.
  static REGISTERED: OnceLock<()> = OnceLock::new();
  REGISTERED.get_or_init(|| {
    crate::init_parser();
    unsafe {
      xmlRegisterInputCallbacks(
        Some(trampoline_match),
        Some(trampoline_open),
        Some(trampoline_read),
        Some(trampoline_close),
      );
    }
  });
}

/// Per-open state owned by libxml2 via `*mut c_void` until
/// `trampoline_close` reclaims and drops it.
struct OpenState {
  bytes: Vec<u8>,
  position: usize,
}

unsafe extern "C" fn trampoline_match(filename: *const c_char) -> c_int {
  if filename.is_null() {
    return 0;
  }
  // SAFETY: libxml2 guarantees `filename` is a NUL-terminated C string
  // for the call's lifetime. Non-UTF-8 URLs can't match anyway.
  let url = match unsafe { CStr::from_ptr(filename) }.to_str() {
    Ok(s) => s,
    Err(_) => return 0,
  };
  // Newest-first, mirroring `trampoline_open`'s walk.
  for cb in snapshot().iter().rev() {
    if (cb.match_url)(url) {
      return 1;
    }
  }
  0
}

unsafe extern "C" fn trampoline_open(filename: *const c_char) -> *mut c_void {
  if filename.is_null() {
    return std::ptr::null_mut();
  }
  // SAFETY: see `trampoline_match`.
  let url = match unsafe { CStr::from_ptr(filename) }.to_str() {
    Ok(s) => s,
    Err(_) => return std::ptr::null_mut(),
  };
  // Newest-first — the most recent registration wins.
  for cb in snapshot().iter().rev() {
    if !(cb.match_url)(url) {
      continue;
    }
    if let Some(bytes) = (cb.open)(url) {
      return Box::into_raw(Box::new(OpenState { bytes, position: 0 })) as *mut c_void;
    }
  }
  std::ptr::null_mut()
}

unsafe extern "C" fn trampoline_read(
  context: *mut c_void,
  buffer: *mut c_char,
  len: c_int,
) -> c_int {
  if context.is_null() || buffer.is_null() || len <= 0 {
    return -1;
  }
  // SAFETY: `context` came from `Box::into_raw` in `trampoline_open`
  // and is not yet reclaimed; libxml2 holds one reference per handle.
  let state = unsafe { &mut *(context as *mut OpenState) };
  let remaining = state.bytes.len().saturating_sub(state.position);
  let n = remaining.min(len as usize);
  if n == 0 {
    return 0;
  }
  // SAFETY: bounds checked above; src and dst are disjoint allocations.
  unsafe {
    std::ptr::copy_nonoverlapping(
      state.bytes.as_ptr().add(state.position),
      buffer as *mut u8,
      n,
    );
  }
  state.position += n;
  n as c_int
}

unsafe extern "C" fn trampoline_close(context: *mut c_void) -> c_int {
  if context.is_null() {
    return -1;
  }
  // SAFETY: unique reclamation site for the box from `trampoline_open`.
  let _state = unsafe { Box::from_raw(context as *mut OpenState) };
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::bindings::{xmlFreeDoc, xmlReadFile};
  use std::ffi::CString;
  use std::sync::atomic::{AtomicUsize, Ordering};

  static SAMPLE_XML: &[u8] = br#"<?xml version="1.0"?>
<root attr="ok"><child/></root>"#;

  /// `Parser::parse_file` bypasses the input-callback machinery (it
  /// reads via Rust file I/O), so the test must call `xmlReadFile`
  /// directly — the same entry point libxslt uses for `xsl:import`.
  fn read_file_via_libxml2(url: &str) -> bool {
    let c = CString::new(url).unwrap();
    unsafe {
      let doc = xmlReadFile(c.as_ptr(), std::ptr::null(), 0);
      if doc.is_null() {
        return false;
      }
      xmlFreeDoc(doc);
      true
    }
  }

  /// Scenarios share one `#[test]` so they run sequentially. libxml2
  /// < 2.13 has a thread-safety bug in the input-callback path that
  /// deadlocks concurrent `xmlReadFile` calls under cargo's default
  /// parallel test runner.
  #[test]
  fn input_callback_scenarios() {
    register_input_callback(
      |url| url.starts_with("embed:///"),
      |url| (url == "embed:///sample.xml").then(|| SAMPLE_XML.to_vec()),
    );

    // 1. Happy path.
    assert!(read_file_via_libxml2("embed:///sample.xml"));

    // 2. `open` returning `None` declines this match; libxml2 falls
    //    through to the default file loader, which also fails.
    assert!(!read_file_via_libxml2("embed:///unknown.xml"));

    // 3. Unrelated URLs aren't claimed by our match — they reach the
    //    default file handler and fail there.
    assert!(!read_file_via_libxml2(
      "/nonexistent/definitely/missing.xml"
    ));

    // 4. Re-entrancy: an `open` closure that calls into libxml2 must
    //    not self-deadlock on the registry mutex.
    register_input_callback(
      |url| url == "reentrant:///outer",
      |_url| {
        let _ = read_file_via_libxml2("embed:///sample.xml");
        Some(SAMPLE_XML.to_vec())
      },
    );
    assert!(read_file_via_libxml2("reentrant:///outer"));

    // 5. Newest-wins ordering: two callbacks claim the same URL; only
    //    the most recent registration runs and produces the bytes.
    static FIRST_OPENED: AtomicUsize = AtomicUsize::new(0);
    static SECOND_OPENED: AtomicUsize = AtomicUsize::new(0);
    register_input_callback(
      |url| url == "ordered:///x",
      |_| {
        FIRST_OPENED.fetch_add(1, Ordering::SeqCst);
        Some(b"<a>first</a>".to_vec())
      },
    );
    register_input_callback(
      |url| url == "ordered:///x",
      |_| {
        SECOND_OPENED.fetch_add(1, Ordering::SeqCst);
        Some(SAMPLE_XML.to_vec())
      },
    );
    assert!(read_file_via_libxml2("ordered:///x"));
    assert_eq!(
      SECOND_OPENED.load(Ordering::SeqCst),
      1,
      "newest registration should run",
    );
    assert_eq!(
      FIRST_OPENED.load(Ordering::SeqCst),
      0,
      "older registration should not be consulted",
    );
  }
}
