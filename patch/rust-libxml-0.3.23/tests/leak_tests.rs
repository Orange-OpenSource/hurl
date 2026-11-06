//! Leak checks that need no valgrind. libxml2's allocations are routed through
//! counting wrappers (`xmlMemSetup`) and Rust's through a counting global
//! allocator; each check repeats an operation and asserts that the number of
//! live allocations does not grow.
//!
//! Everything runs in one test: other tests in this binary would allocate
//! concurrently and disturb the counts.
use std::alloc::{GlobalAlloc, Layout, System};
use std::os::raw::{c_char, c_void};
use std::sync::atomic::{AtomicIsize, Ordering};

use libxml::parser::Parser;
use libxml::tree::c14n::{CanonicalizationMode, CanonicalizationOptions};
use libxml::xpath::{Context, Expression, is_well_formed_xpath};

static RUST_LIVE: AtomicIsize = AtomicIsize::new(0);
/// Every Rust allocation made, freed or not: the count a "without allocating" claim is checked against.
static RUST_ALLOCS: AtomicIsize = AtomicIsize::new(0);
static LIBXML_LIVE: AtomicIsize = AtomicIsize::new(0);

struct CountingAlloc;

unsafe impl GlobalAlloc for CountingAlloc {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let ptr = unsafe { System.alloc(layout) };
    if !ptr.is_null() {
      RUST_LIVE.fetch_add(1, Ordering::Relaxed);
      RUST_ALLOCS.fetch_add(1, Ordering::Relaxed);
    }
    ptr
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    RUST_LIVE.fetch_sub(1, Ordering::Relaxed);
    unsafe { System.dealloc(ptr, layout) }
  }
}

#[global_allocator]
static ALLOCATOR: CountingAlloc = CountingAlloc;

unsafe extern "C" fn count_malloc(size: usize) -> *mut c_void {
  let ptr = unsafe { libc::malloc(size) };
  if !ptr.is_null() {
    LIBXML_LIVE.fetch_add(1, Ordering::Relaxed);
  }
  ptr
}

unsafe extern "C" fn count_realloc(old: *mut c_void, size: usize) -> *mut c_void {
  let ptr = unsafe { libc::realloc(old, size) };
  if old.is_null() && !ptr.is_null() {
    LIBXML_LIVE.fetch_add(1, Ordering::Relaxed);
  }
  ptr
}

unsafe extern "C" fn count_free(ptr: *mut c_void) {
  if !ptr.is_null() {
    LIBXML_LIVE.fetch_sub(1, Ordering::Relaxed);
  }
  unsafe { libc::free(ptr) }
}

// Built on `count_malloc` rather than `strdup`, which MSVC spells `_strdup`.
unsafe extern "C" fn count_strdup(s: *const c_char) -> *mut c_char {
  unsafe {
    let len = libc::strlen(s) + 1;
    let ptr = count_malloc(len) as *mut c_char;
    if !ptr.is_null() {
      std::ptr::copy_nonoverlapping(s, ptr, len);
    }
    ptr
  }
}

/// Live allocations added by `rounds` further runs of `op`, after two warm-up
/// runs (lazy initialisation inside libxml2 is not a leak).
/// The Rust allocations `rounds` runs of `op` make, after two warm-up runs.
fn allocations(rounds: usize, mut op: impl FnMut()) -> isize {
  op();
  op();
  let before = RUST_ALLOCS.load(Ordering::SeqCst);
  for _ in 0..rounds {
    op();
  }
  RUST_ALLOCS.load(Ordering::SeqCst) - before
}

fn growth(counter: &AtomicIsize, rounds: usize, mut op: impl FnMut()) -> isize {
  op();
  op();
  let before = counter.load(Ordering::SeqCst);
  for _ in 0..rounds {
    op();
  }
  counter.load(Ordering::SeqCst) - before
}

#[test]
fn repeated_operations_do_not_leak() {
  // Must precede any libxml2 allocation, so before anything initialises it.
  let rc = unsafe {
    libxml::bindings::xmlMemSetup(
      Some(count_free),
      Some(count_malloc),
      Some(count_realloc),
      Some(count_strdup),
    )
  };
  assert_eq!(rc, 0, "xmlMemSetup failed");
  libxml::init_parser();

  // The compiled expression was released with a plain `free`, leaking its steps.
  let xpath = growth(&LIBXML_LIVE, 100, || {
    assert!(is_well_formed_xpath("//a[@b='c']/d | //e"));
  });
  assert_eq!(
    xpath, 0,
    "is_well_formed_xpath leaked {xpath} libxml2 allocations in 100 calls"
  );

  // The inclusive namespace prefixes were handed to libxml2 as leaked `CString`s.
  let doc = Parser::default()
    .parse_string(r#"<r xmlns:a="urn:a" xmlns:b="urn:b"><a:x b:y="1"/></r>"#)
    .unwrap();
  // Guard against a vacuous pass: the live document's nodes must be counted.
  assert!(
    LIBXML_LIVE.load(Ordering::SeqCst) > 0,
    "libxml2 allocations are not being counted"
  );
  let canonicalize = || {
    let options = CanonicalizationOptions {
      mode: CanonicalizationMode::ExclusiveCanonical1_0,
      with_comments: false,
      inclusive_ns_prefixes: vec!["a".to_string(), "b".to_string()],
    };
    assert!(!doc.canonicalize(options, None).unwrap().is_empty());
  };
  let rust = growth(&RUST_LIVE, 50, canonicalize);
  assert_eq!(
    rust, 0,
    "canonicalize leaked {rust} Rust allocations in 50 calls"
  );
  let libxml = growth(&LIBXML_LIVE, 50, canonicalize);
  assert_eq!(
    libxml, 0,
    "canonicalize leaked {libxml} libxml2 allocations in 50 calls"
  );

  // A compiled expression frees its steps when dropped; its evaluations free their objects.
  let compiled = growth(&LIBXML_LIVE, 100, || {
    let expression = Expression::compile("//a:x | //*[@b:y='1']").unwrap();
    let ctx = Context::new(&doc).unwrap();
    ctx.register_namespace("a", "urn:a").unwrap();
    ctx.register_namespace("b", "urn:b").unwrap();
    assert_eq!(ctx.findnodes_compiled(&expression, None).unwrap().len(), 1);
    let root = doc.get_root_element().unwrap();
    assert_eq!(
      ctx
        .findnodes_compiled(&expression, Some(&root))
        .unwrap()
        .len(),
      1
    );
  });
  assert_eq!(
    compiled, 0,
    "compiled XPath evaluation leaked {compiled} libxml2 allocations in 100 calls"
  );

  // A property read through the stack buffer or through the copying fallback (an entity reference, a DTD default)
  // frees its copy.
  let dtd_doc = Parser::default()
    .parse_string(
      r#"<!DOCTYPE r [<!ENTITY e "EE"><!ATTLIST r d CDATA "dflt">]><r a="v" ent="x&e;y"/>"#,
    )
    .unwrap();
  let dtd_root = dtd_doc.get_root_element().unwrap();
  let stack = growth(&LIBXML_LIVE, 100, || {
    for name in [c"a", c"absent"] {
      dtd_root.with_property(name, |value| assert!(!value.contains('\0')));
    }
  });
  assert_eq!(
    stack, 0,
    "with_property (stack buffer) leaked {stack} libxml2 allocations in 100 rounds"
  );
  // The accessors that read a value in place and pass a name through the stack (`get_property`, `has_property`,
  // `set_property`, `remove_property`) leave nothing behind, whether the name or value fits the 128-byte stack copy or
  // not.
  let long_value = "v".repeat(300);
  let in_place = growth(&LIBXML_LIVE, 100, || {
    let mut root = dtd_root.clone();
    assert_eq!(dtd_root.get_property("a").as_deref(), Some("v"));
    assert_eq!(dtd_root.get_property("absent"), None);
    assert!(dtd_root.has_property("a"));
    root.set_property("w", "x").unwrap();
    root.set_property(&"w".repeat(200), &long_value).unwrap();
    assert_eq!(
      dtd_root.get_property(&"w".repeat(200)).as_deref(),
      Some(long_value.as_str())
    );
    root.remove_property(&"w".repeat(200)).unwrap();
    root.remove_property("w").unwrap();
  });
  assert_eq!(
    in_place, 0,
    "the in-place property accessors leaked {in_place} libxml2 allocations in 100 rounds"
  );
  // A short name reaches libxml2 without a Rust allocation: `has_property` makes none, `get_property` only its
  // returned `String`.
  let has = allocations(100, || assert!(dtd_root.has_property("a")));
  assert_eq!(
    has, 0,
    "has_property made {has} Rust allocations in 100 calls"
  );
  let get = allocations(100, || {
    assert_eq!(dtd_root.get_property("a").as_deref(), Some("v"))
  });
  assert_eq!(
    get, 100,
    "get_property made {get} Rust allocations in 100 calls"
  );

  // The paths that free a libxml2 string through `bindgenFree` — `findvalues_compiled` (via `get_nodes_as_str`) and
  // `with_property`'s copying fallback — are counted off MSVC only: there `bindgenFree` frees with the C runtime's
  // `free` (`c_helpers.rs`), which the counting hooks set by `xmlMemSetup` above never see.
  #[cfg(not(all(target_family = "windows", target_env = "msvc")))]
  {
    let values = growth(&LIBXML_LIVE, 100, || {
      let expression = Expression::compile("//*[@b:y='1']").unwrap();
      let ctx = Context::new(&doc).unwrap();
      ctx.register_namespace("b", "urn:b").unwrap();
      let root = doc.get_root_element().unwrap();
      assert_eq!(
        ctx
          .findvalues_compiled(&expression, Some(&root))
          .unwrap()
          .len(),
        1
      );
    });
    assert_eq!(
      values, 0,
      "findvalues_compiled leaked {values} libxml2 allocations in 100 calls"
    );
    let copied = growth(&LIBXML_LIVE, 100, || {
      for name in [c"ent", c"d"] {
        dtd_root.with_property(name, |value| assert!(!value.contains('\0')));
      }
    });
    assert_eq!(
      copied, 0,
      "with_property (copy) leaked {copied} libxml2 allocations in 100 rounds"
    );
    // (`get_property`'s and `get_properties`' copying fallback frees libxml2's copy)
    let copied_get = growth(&LIBXML_LIVE, 100, || {
      assert_eq!(dtd_root.get_property("ent").as_deref(), Some("xEEy"));
      assert_eq!(dtd_root.get_property("d").as_deref(), Some("dflt"));
      assert_eq!(dtd_root.get_properties().len(), 2);
    });
    assert_eq!(
      copied_get, 0,
      "get_property / get_properties (copy) leaked {copied_get} libxml2 allocations in 100 rounds"
    );
  }
}
