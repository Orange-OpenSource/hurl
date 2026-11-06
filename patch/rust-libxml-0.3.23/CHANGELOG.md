# Change Log

## [0.3.23] (2026-10-31)

### Added

* `xpath::Expression`: an XPath expression compiled once (`Expression::compile`, libxml2's `xmlXPathCompile`) and
  evaluated any number of times, against any `Context`, by `Context::evaluate_compiled`, `node_evaluate_compiled`,
  `findnodes_compiled`, `findvalues_compiled` and `findvalue_compiled`. The string evaluators compile their expression
  on every call; an application repeating a few expressions (latexml-oxide: 35–63 distinct expressions in up to 134k
  calls per document) saves up to 1.7 % of all its instructions, about 1 % on average. The result is the string
  evaluators': libxml2 compiles, optimizes and runs the expression the same way, and a node evaluation leaves its node
  as the context's current node as `node_evaluate` does. Errors are the structured `XPathError`, at compile time for a
  malformed expression. (#223)
* `Node::with_property` (alias `with_attribute`): read a property and pass it to a callback, in the common case
  without allocating. The name is a `&CStr` (`c"id"`). A value that libxml2 holds as a single text node of at most
  128 bytes is copied into a stack buffer; a longer value, or one of another shape (an entity reference in the value,
  a DTD default), is read into a `String` as `get_property` reads it. The result is `get_property`'s. The callback
  holds its own copy, so it may change the document. latexml-oxide reads one private attribute on every text chunk;
  reading it this way saves up to 2.5 % of all its instructions, 0.7–1.2 % on average. (#223)

### Changed

* Fewer copies between Rust and libxml2, with unchanged results (#224, #226):
  * The property accessors (`get_`, `has_`, `set_`, `remove_property` and their `_ns`/`_no_ns`, `_node` and
    `_attribute` forms) pass a name, a namespace and `set_property`'s value to libxml2 without allocating when it is
    shorter than 128 bytes, copied on the stack instead of into a `CString`. A longer string, or one holding a NUL
    byte, takes the `CString` path as before; a NUL byte still panics there.
  * `get_property`, `get_property_ns`, `get_property_no_ns`, `get_properties` and `get_properties_ns` read a value
    that libxml2 holds as a single text node, or as none, in place, instead of through a copy made by libxml2 and freed
    again. Every other shape (an entity reference in the value, a DTD default) is still read through libxml2's own
    getter, and an absent property is looked up once. `xmlGetProp`, `xmlGetNsProp` and `xmlGetNoNsProp` are the same
    lookup followed by the same copy, returning NULL whenever the lookup does, and a test checks the accessors against
    them, and `get_properties` against `xmlNodeGetContent`, for every shape a parser makes and for those only the tree
    API makes (no children, a text child with no content, a CDATA child).
  * A string copied out of libxml2 (`get_name`, `get_content`, property names and values) is checked by
    `str::from_utf8` and decoded lossily only when that check fails, as `to_string_lossy` decoded it before.
  * `get_child_nodes` and `get_child_elements` size their vector by a first walk and upgrade the document once.
    `get_child_elements` wraps only the element children.
  * `get_properties` sizes its map before filling it.

  Measured with `perf stat -e instructions:u` on a release build of latexml-oxide converting 18 arXiv papers: these
  changes save 2.63 % of the instructions summed over the 18 (1.58–3.57 % per paper) against `main` before them
  (7000d0d9), with byte-identical XML output. Measured the same way, latexml-oxide on this release, using the two
  additions above, runs 4.55 % fewer instructions than on 0.3.22.

## [0.3.22] (2026-10-07)

### Fixed

* Use-after-free of the encoding name in `Parser::parse_file_with_options`,
  `Parser::parse_string_with_options` and
  `Parser::is_well_formed_html_with_encoding`, reachable from safe code since
  `ParserOptions::encoding` was introduced in 0.2.12 (2019): the `CString` was
  moved into a `match` arm and dropped before libxml2 read the pointer. When
  the freed memory was reused, the requested encoding was silently ignored or
  the parse failed. (#216, #217)
* Schema validation no longer panics on an internal libxml2 error:
  `SchemaValidationContext::validate_document`, `validate_file` and
  `validate_node` return the errors instead, with a fallback diagnostic when
  libxml2 provides none. Any document containing an entity reference triggered
  the panic, so a service validating untrusted XML could be crashed by a
  request. (#219)
* An encoding name containing a NUL byte panicked. The `parse_*` methods now
  return `XmlParseError::GotNullPointer` and
  `is_well_formed_html_with_encoding` returns `false`; the error enum is
  unchanged. In `parse_file_with_options` the panic also leaked the opened
  file. (#217)
* `Parser::is_well_formed_html_with_encoding` leaked its parser context and
  document (313 bytes per call) when the input had no root element, e.g.
  comment-only HTML. (#217)
* Deadlock on libxml2 before 2.12 when a `SchemaParserContext` was created
  while other threads first used libxml2: its constructors never called
  `init_parser()`, so libxml2's lazily created mutexes could be initialised
  twice (threads blocked in `xmlRMutexLock` from `xmlDictCreate`). The
  `SchemaParserContext` constructors, `Document::new`, `TextReader::from_file`
  and `xpath::is_well_formed_xpath` now initialise libxml2 first, as `Parser`
  already did.
* `Parser::is_well_formed_html` is now thread-safe. It tolerated unknown tags
  (such as `<math>`) through a process-global flag set from libxml2's error
  callback, so one thread's unknown tag could make another thread's malformed
  input pass, and it left that callback installed on the thread for good,
  swallowing later errors and replacing any handler the application had set.
  The state is now per call, and the thread's previous handler is restored.
  `bindings::HACKY_WELL_FORMED` is deprecated and no longer used.
* `Parser::is_well_formed_html` no longer lets one unknown tag hide other
  errors: a `<math>` anywhere made the whole document pass on libxml2 before
  2.14. The verdict is now libxml2's own, except that unknown-tag reports from
  its pre-2.14 HTML4-era parser are not counted, matching 2.14+, which no
  longer makes them. HTML5, SVG and MathML documents are well-formed on every
  supported version.
* `RoNode::get_namespaces` leaked the `xmlGetNsList` array (88 bytes per
  call), as `Node::get_namespaces` did until 0.3.21.
* `xpath::is_well_formed_xpath` leaked the compiled expression's steps (8
  allocations per call): it was released with a plain `free` instead of
  `xmlXPathFreeCompExpr`.
* `Document::canonicalize` leaked every `inclusive_ns_prefixes` string on
  each call: they were handed to libxml2 through `CString::into_raw` and never
  reclaimed.

### Added

* `Parser::parse_string_with_diagnostics`: parses XML or HTML and returns the
  document together with every error and warning libxml2 reported, as
  `Vec<StructuredError>`, or `XmlParseFailure::ParseFailed` holding them when
  no document is produced. Works with or without `recover` / `no_error`, on
  libxml2 2.9 through 2.15; diagnostics are collected per thread and capped at
  `parser::MAX_DIAGNOSTICS` (100) per parse, since libxml2 before 2.13 reports
  one error per few bytes of hostile input (1 MB gave ~33 MB of diagnostics).
  New error type `XmlParseFailure` (`#[non_exhaustive]`). (#218, #220)
* `ParseFormat` derives `Clone` and `Copy`. (#220)
* Tests for `ParserOptions::encoding`, which had none, including
  `tests/encoding_lifetime_tests.rs`: an allocator that poisons freed memory
  makes a dropped encoding name fail without valgrind, in any of the four
  entry points. (#217)
* CI runs the test suite under valgrind. Memory errors fail the job; leaks do
  not yet, as two existing tests still leak. (#217)

### Changed

* Build dependency `bindgen` requirement raised from 0.72.1 to 0.73.1.

## [0.3.21] (2026-08-02)

### Fixed

* `Node::get_namespaces` leaked the `xmlGetNsList` result array on every
  call, since the crate's beginning: the list is a caller-freed malloc'd
  array of `xmlNsPtr` (the namespaces belong to the document); the
  historical attempt freed it with `xmlFreeNs` — the first namespace struct
  — segfaulted, and was commented out. Measured downstream: 2,429 lost
  blocks over 30 documents, a leading term of a ~150 KB/page RSS climb
  across a 115,000-page render. Freed with the per-target allocator shim
  (MSVC-safe).

## [0.3.20] (2026-08-02)

### Changed

* `Node::set_linked` is now public. A consumer holding long-lived `Node`
  handles into a document (id caches) needs to declare "the document owns
  this memory" before dropping handles that intermediate processing may have
  `unlink_node`ed — without it, the only safe teardown was to leak each
  wrapper's allocation (~100+ bytes per handle per document; a leading
  retention term across a 115,000-page render downstream).

## [0.3.19] (2026-08-01)

### Fixed

* Windows/MSVC link failure in 0.3.18 (`LNK2019: unresolved external symbol
  xmlFree`): the `expand_to_document` prefix-restoration freed the minted
  prefix through the `xmlFree` global, which is not a linkable data symbol on
  MSVC builds. It now goes through `c_helpers::bindgenFree`, the crate's
  existing per-target free shim. Any consumer LINKING 0.3.18 on
  `windows-msvc` fails; `cargo check` passes (no link step), which is how it
  escaped. 0.3.18 is yanked.

## [0.3.18] (2026-07-31)

### Fixed

* `TextReader::expand_to_document` no longer serializes default-namespace
  content with a minted `default:` prefix. `xmlReconciliateNs` turns a
  NULL-prefix namespace inherited from an un-copied ancestor into
  `xmlns:default="…"`, so every element re-serialized as `<default:x>` — and a
  dumped *subtree* of the copy never re-parsed into the right namespace. The
  minted declarations' prefixes are restored to the source element's prefixes
  (clash-checked), so the copy serializes exactly as a standalone parse of the
  same subtree would.

### Added

* `TextReader` streaming-split support — the four calls a caller needs to
  partition a huge document into subdocuments without ever materializing a
  large subtree:
  * `attributes_qname()`: the current element's attributes as
    `(qualified-name, value)` pairs in document order, **including namespace
    declarations**, without expanding the subtree (reader restored to the
    element afterwards).
  * `value()`: text/CDATA/comment/PI content of the current node.
  * `is_empty_element()`: distinguishes `<x/>` from `<x></x>`.
  * `event()`: the lossless `xmlReaderTypes` event (`ReaderEvent`), distinguishing
    end-element from whitespace events (both `None` under `node_type()`).
  * `outer_xml()`: serialize the current subtree exactly as input (attribute
    order preserved, no added namespace declarations). Deliberately NOT
    `xmlTextReaderReadOuterXml`, whose parentless deep copy makes
    `xmlNewReconciledNs` mint a `default:` prefix onto default-namespace
    content; dumping the reader-owned node directly keeps ancestor namespace
    declarations reachable so nothing is fabricated.

## [0.3.17] (2026-07-29)

### Added

* `Node::free_subtree`: detach and free an entire C subtree IMMEDIATELY,
  neutralizing every registered Rust wrapper into it (shared `node_ptr`
  nulled — stray clones become inert no-ops; bookkeeping entries cleared so
  address reuse cannot collide with a stale wrapper). This is the discard
  operation `unlink_node` + `set_rust_owned` cannot provide safely when
  clones survive in long-lived collections: their deferred drop can fire
  `xmlFreeNode` after the owning document's `xmlFreeDoc` (use-after-free in
  `xmlDictOwns`). Driver: consumers that discard thousands of replaced
  subtrees per document (latexml-oxide's math parser) leaked them all.

## [0.3.16] (2026-07-12)

### Fixed

* Windows/MSVC vcpkg builds: emit `cargo:rustc-link-lib=bcrypt` and `ws2_32`
  alongside the vcpkg-resolved libraries. vcpkg emits link directives for the
  port's libraries and port dependencies but not for Windows SDK system
  libraries (`Libs.private` in `libxml-2.0.pc`); libxml2 >= 2.12 calls
  `BCryptGenRandom` from `xmlInitRandom`, so linking a static `libxml2.lib`
  into any real target (proc-macro dylib, bin, test) failed with LNK2019.
  Both import libraries ship with every MSVC/Windows SDK toolchain.

## [0.3.15] (2026-07-06)

### Added

* `reader::TextReader` — a safe wrapper over libxml2's `xmlTextReader` pull
  parser, for processing very large documents one subtree at a time instead of
  building the whole DOM (a 600 MB file becomes a ~7 GB tree). Methods:
  `from_file`, `read` / `read_next` (advance / skip-subtree),
  `node_type` / `is_element` / `depth` / `local_name` / `namespace_uri`,
  `read_to_next(pred)` (the streamable downward-name XPath subset), and two
  subtree accessors — `expand()` (borrowed `RoNode`, zero-copy, valid until the
  next advance) and `expand_to_document()` (an owned, self-contained `Document`
  copy with namespaces reconciled onto the detached root via
  `xmlDOMWrapCloneNode` + `xmlReconciliateNs`, so serialization keeps the
  `xmlns=` declarations and nothing dangles into the source once the reader
  advances).
* `xpath::XPathError` and the checked evaluators `Context::evaluate_checked`,
  `node_evaluate_checked`, `node_evaluate_readonly_checked`, returning
  `Result<Object, XPathError>`. libxml2 aborts a `//X[predicate]` evaluation
  (returning NULL) when it materializes more than `XPATH_MAX_NODESET_LENGTH`
  (10M) intermediate nodes and hits the internal *"growing nodeset hit limit"*
  on very large documents; the existing thin wrappers collapsed that into a bare
  `Err(())`. The checked variants snapshot libxml2's structured error (message /
  code / domain, with `is_nodeset_limit()`) so callers can log or branch on the
  real cause. The existing `evaluate` / `node_evaluate` / `node_evaluate_readonly`
  are unchanged (reimplemented as `*_checked(..).map_err(|_| ())`).

### Changed

* **Minimum supported Rust version is now 1.88** (declared via `rust-version`).
  The streaming `reader` uses a let-chain (stabilized in 1.88); the edition is
  already 2024. Consumers on 1.85–1.87 that built 0.3.14 will need to update.
* `Node::_wrap`'s per-document `xmlNodePtr -> Node` cache now hashes pointer keys
  with a small FxHash-style hasher instead of the default SipHash `RandomState`.
  The cache is probed on every `Node` wrap (child/sibling walks, XPath results,
  attribute-node access), and its keys are non-adversarial allocator pointers, so
  SipHash's DoS resistance buys nothing here. Dependency-free (std `Hasher` only),
  the map is never iterated so ordering is irrelevant, and behavior is identical.
  Measured ~28-30% wall reduction on node-heavy XML workloads (large math
  documents).

### Fixed

* `Node::get_properties` / `get_properties_ns` (and their `get_attributes`
  aliases) now read each attribute's value directly from the attribute node
  already in hand (`xmlNodeGetContent`), instead of re-resolving it by name with
  `xmlGetProp` / `xmlGetNsProp`. The old path re-scanned the whole attribute list
  (`xmlStrEqual` per entry) and allocated a fresh `CString` for the name on every
  attribute — quadratic in the attribute count. Same returned map; no behavior
  change.

* Null-guarded the raw-pointer field accessors in `c_helpers` (e.g.
  `xmlNodeType` reading `.type_` at offset 8): a NULL `xmlNodePtr` reaching an
  accessor produced a `segfault at 8` that bypassed `catch_unwind`. The
  accessors now return a benign default for a NULL pointer.
* Hardened the `CStr::from_ptr` surface in `tree::document` (`to_string` /
  node / properties / xpath serialization) against `strlen(NULL)`: when
  libxml2 returns a NULL buffer (e.g. OOM during XSLT serialization of a huge
  document), the wrappers now yield an empty string instead of dereferencing
  NULL (`segfault at 0 in libc`).

## [0.3.14] (2026-06-21)

### Changed

* `Node::node_ptr_mut` now guards mutable access with
  `RefCell::try_borrow_mut` instead of the
  `Rc::strong_count <= NODE_RC_MAX_GUARD` heuristic. It returns `Err` **only**
  when the node's cell is *actively* borrowed (a genuine re-entrant aliasing
  conflict on the same wrapped node); a high `Rc::strong_count` from benign
  `Node` clones — the owning document's node cache plus caller-held handles —
  no longer blocks mutation. All clones share one `RefCell`, so
  `try_borrow_mut` is exactly the per-node exclusion check; the old proxy both
  false-positived on benign clones and could panic via `borrow_mut` on the very
  conflict it was meant to catch.

### Fixed

* Spurious `Can not mutably reference a shared Node` errors when mutating a node
  that merely has several live clones (e.g. large or heavily-shared trees, or
  any node also held in a caller-side cache). Such mutations now succeed; the
  error is reserved for real active-borrow re-entrancy.

### Deprecated

* `NODE_RC_MAX_GUARD` and `set_node_rc_guard` are now no-ops, retained only for
  API compatibility. The strong-count threshold they tuned is no longer
  consulted by `node_ptr_mut`.

## [0.3.13] (2026-06-11)

### Added

* Opt-in **static linking** of libxml2. Set the `LIBXML2_STATIC` environment
  variable and `build.rs` probes via pkg-config with `.statik(true)`,
  emitting `cargo:rustc-link-lib=static=xml2` (plus the transitive `-lm`).
  Point `PKG_CONFIG_PATH` at a non-system prefix holding a PIC `libxml2.a`
  to get a binary with no runtime `libxml2.so` dependency — independent of
  the host's libxml2 SONAME, which bumped `.so.2` → `.so.16` at libxml2 2.14.
  Unset (the default) is unchanged: a normal dynamic link against the host
  libxml2.
* New cargo features **`runtime`** (default) and **`static`** that select how
  bindgen locates libclang to generate the FFI bindings at build time.
  `cargo build --no-default-features --features static` routes bindgen to
  `clang-sys/static`, enabling fully static / musl / Alpine builds where a
  runtime `dlopen` of libclang is unavailable. Together with `LIBXML2_STATIC`,
  this resolves [#110](https://github.com/KWARC/rust-libxml/issues/110).

## [0.3.12] (2026-05-23)

### Added

* New `io` module: `io::register_input_callback(match_url, open)` is
  a safe wrapper around `xmlRegisterInputCallbacks`. Accepts two
  closures (`&str -> bool`, `&str -> Option<Vec<u8>>`) and installs
  them as a custom URL-scheme handler. The intended use is bundling
  XSLT stylesheets / RNG schemas via `include_bytes!` and serving
  them through a synthetic scheme (e.g. `embed:///foo.xsl`), so
  libxslt's `xsl:import` resolution can reach the embedded bytes
  without touching the disk. Closures may run on any thread libxml2
  calls them from (`Send + Sync + 'static`).

## [0.3.11] (2026-05-18)

### Added

* `Document::remove_internal_subset`, a safe wrapper around
  `xmlGetIntSubset` + `xmlUnlinkNode` + `xmlFreeDtd`. Mirrors
  XML::LibXML's `Document::removeInternalSubset` (Perl): drops the
  internal DTD node so subsequent serialisation no longer emits the
  `<!DOCTYPE …>` preamble. Idempotent, safe on doctype-less
  documents.

### Changes

* Whole-crate clippy pass: `cargo clippy --fix --all-targets` plus
  manual touch-ups so the workspace builds warning-free on the
  current stable.
* Added an arm64 build/test job to `.github/workflows/CI.yml`
  (thanks @wetneb).

### Fixed

* Test suite portability on ARM64 — switched to portable types in
  `tests/xml_copy_invariant_tests.rs` so the cross-platform CI run
  passes alongside x86_64 (thanks @wetneb).

## [0.3.10] (2026-05-09)

### Added

* `Node::set_rust_owned` / `Node::is_rust_owned` (and `RoNode::is_rust_owned`)
  for explicit ownership transfer of detached subtrees. Lets callers reclaim
  unlinked nodes that would otherwise leak.
* `Document::dup_node_into_new_doc` — deep-copy a subtree into a fresh
  independent document. Works around `xmlDocCopyNode` returning NULL on
  repeated extraction within one source document.

### Fixed

* `_Node::Drop` no longer fires `xmlFreeNode` against memory the source
  document still owns. Internally backed by a 3-variant `Linkage` enum
  (`Linked` / `Unlinked` / `RustOwned`) replacing the prior `unlinked: bool`.
* `Document::import_node` now rejects `RustOwned` source nodes.

## [0.3.9] (2026-04-22)

### Added

* `Node::new_comment` constructor, wrapping `xmlNewDocComment`. Returns an
  unlinked comment node bound to the given document; propagates an error
  rather than panicking on embedded NULs.
* `libxml::init_parser()`, a top-level safe wrapper around
  `xmlInitParser()` guarded by `std::sync::Once`. Useful for application
  code that touches libxml2 directly without going through `parser::Parser`.

### Changes

* `Parser::default()` and `Parser::default_html()` now route through
  `libxml::init_parser()` instead of their own private `Once`, giving a
  single source of truth for libxml2 parser initialisation.

## [0.3.8] (2025-25-09)

### Changes

* Corrected the vcpkg include directories used during build  

## [0.3.7] (2025-18-08)

### Changes

* Corrected the bindgen configuration for c14n, thanks to @Qubitza

## [0.3.6] (2025-13-07)

### Added

* c14n, thanks to @saks and @tstenner
    * add `canonicalize` method for `Node` and `Document`
    * see `tests/c14n.rs` for examples
* Utility node methods, thanks to @saks (nokogiri inspired)
    * add `Node::ancestors` method
    * add `Node::at_xpath` method. Similar to `findnodes` but nokogiri inspired.

This work still hasn't been extended to `RoNode`, contributions welcome.

### Changes

* Added more low-level headers from bindgen for wrapper uses interested in unsafe low-level libxml2 calls.
  The bindgen coverage has been in flux since the v0.3.4 release, as we stabilize the new build approach.

### Removed

## [0.3.5] (2025-28-04)

This release stabilizes the new "bindgen during build" approach of v0.3.4 to more platforms.

Support for newer libxml2 versions has been improved.
 - CI support for v2.12.9, v2.13.8, v2.14.1

Thanks go to @wetneb and @charmitro for contributing.

### Added

* cargo build: expose libxml2 version to main build script;
* creating a new `Parser` now initializes via `bindings::xmlInitParser`, called at most once.

### Changes

* cargo build: mark `max_align_t` as opaque to fix i386 build failure
* cfg: adapt mutability of error pointer depending on libxml version
* change the return type of `xmlGetNodeType` from `u32` to the more portable `bindings::xmlElementType`
* change the argument type of `NodeType::from_int` from `u32` to the more portable `bindings::xmlElementType`
* protect `Schema` initialization to be closer to thread-safe (note that this wrapper is NOT thread-safe in general)

### Removed
 * The use of `SchemaParserContext` and `SchemaValidationContext` is currently NOT thread safe.
   Hence, The `schema_test` has been weakened to run in a single thread (future improvements welcome).

## [0.3.4] (2025-16-04)

Thanks go to @wetneb, @anwaralameddin, @rudolphfroger, @jcamiel, @imcsk8 for contributions to this release.

### Added

* Node methods: `get_property_no_ns` (alias: `get_attribute_no_ns`), `get_properties_ns` (alias: `get_attributes_ns`), `has_property_no_ns` (alias: `has_attribute_no_ns`), `remove_property_no_ns` (alias: `remove_attribute_no_ns`), `get_property_node_ns` (alias: `get_attribute_node_ns`), `get_property_node_no_ns` (alias: `get_attribute_node_no_ns`)
* Added implementations of `Hash`, `PartialEq` and `Eq` traits for `Namespace`

### Changed

* Call bindgen at build time on Unix platforms (thanks @wetneb)

## [0.3.3] 2023-17-07

### Changed

* Update the implementation of `StructuredError` so that all validation errors are returned from the validation methods present on `SchemaValidationContext`. Previously, all returned validation errors were identical due to libxml reusing a global memory address for all reported errors. Thanks @JDSeiler !
* The `message` method of `StructuredError` has been deprecated.

## [0.3.2] 2023-07-05

### Added

* XPath: `Context::findvalues`, with optional node-bound evaluation, obtaining `String` values.

* `Node::findvalues` method for direct XPath search obtaining `String` values, without first explicitly instantiating a `Context`. Reusing a `Context` remains more efficient.

## [0.3.1] 2022-26-03

* Added: Thanks to @lepapareil, @hurlenko and @ballsteve for contributing installation docs for Windows and MacOS.
* Added: `Node` and `RoNode` now have `has_property` (alias `has_attribute`) and `has_property_ns` (alias `has_attribute_ns`) to check attribute presence without allocating the value.
* Added: `xpath::is_well_formed_xpath`, thanks @bcpeinhardt !

## [0.3.0] 2021-27-05

* Change `Parser::parse_file/string_with_encoding` to `Parser::parse_file/string_with_options`.

* Introduce `ParserOptions` which encapsulates the forced encoding setting together with libxml2s HTML and XML parser options.

* For systems without a pkg-config capability, we now use the `LIBXML2` environment variable to detect an installed libxml2 toolchain. (thanks @przygienda !)

## [0.2.16] 2021-31-01

### Added

* More element-oriented methods: `get_next_element_sibling`, `get_prev_element_sibling`, `get_last_element_child`, added to both `Node` and `RoNode`.
* `Document::ronode_to_string` for printing read-only nodes
* `RoNode::node_ptr` for getting the internal libxml2 raw pointer of a read-only node

## [0.2.15] 2020-28-09

Thanks to @JoshuaNitschke for contributing OS-based package detection for the native libxml2! Also thanks to @coding-yogi, @ignatenkobrain and @simoin for investigating platform-specific issues with this crate, some of which expect resolution in upcoming versions.

### Added

* Support for x64-Windows use via the vcpkg package manager (with new CI monitoring via appveyor).

### Changed

* Added back an `Error` trait implementation for `XmlParseError`

### Removed

* Dropped a large number of memory layout tests that were auto-generated by bindgen, until we have a more sophisticated test-generation setup that can enforce multiple architectures. Ideally this has no day-to-day impact and just makes portability easier in the short-term.

## [0.2.14] 2020-27-03

### Changed

More consistently use `c_char` to successfully compile on ARM targets

## [0.2.13] 2020-16-01

Thanks to @jangernert for the upgrades to `Document` serialization.
Thanks to @lweberk for contributing the `Schema` featureset and to @cbarber for refining the FFI interop.

### Added

* `Document::to_string_with_options` allowing to customize document serialization
* `Document::SaveOptions` containing the currently supported serialization options, as provided internally by libxml
* `Schema` holding and managing `xmlSchemaPtr` as created while parsing by `SchemaParserContext`
* `SchemaParserContext` holding source of XSD and parsing into a `Schema` while gathering and –in case returning– errors that arise from the XSD parser across the FFI to libxml
* `SchemaValidationContext` holding the `Schema` from resulting `SchemaParserContext` parse and offering validation methods for `Document`, `Node` or file path to XML, while gathering and –in case returning– validation errors from the XML validator across the FFI to libxml

### Changed

* the `Document::to_string()` serialization method is now implemented through `fmt::Display` and no longer takes an optional boolean flag. The default behavior is now unformatted serialization - previously `to_string(false)`, while `to_string(true)` can be realized via

 ```
   .to_string_with_options(SaveOptions { format: true, ..SaveOptions::default()})`
  ```

## [0.2.12] 2019-16-06

Thanks to @Alexhuszagh for contributing all enhancements for the `0.2.12` release!

### Added

* BOM-aware Unicode support
* New `Parser` methods allowing to specify an explicit encoding: `parse_file_with_encoding`, `parse_string_with_encoding`, `is_well_formed_html_with_encoding`

### Changed

* Default encodings in `Parser` are now left for libxml to guess internally, rather than defaulted to `utf-8`.

## [0.2.11] 2019-15-04

### Added

* `RoNode::to_hashable` and `RoNode::null` for parity with existing `Node`-leveraging applications

## [0.2.10] 2019-14-04

### Added

* `RoNode` primitive for simple and efficient **read-only** parallel processing
* Benchmarking a 120 MB XML document shows a twenty five fold speedup, when comparing `Node` to parallel rayon processing over `RoNode` with a 32 logical core desktop
* While `RoNode` is added as an experiment for high performance read-only scans, any mutability requires using `Node` and incurring a bookkeeping cost of safety at runtime.
* Introduced benchmarking via `criterion`, only installed during development.
* `benches/parsing_benchmarks` contains examples of parallel scanning via `rayon` iterators.
* added `Document::get_root_readonly` method for obtaining a `RoNode` root.
* added `Context::node_evaluate_readonly` method for searching over a `RoNode`
* added `Context::get_readonly_nodes_as_vec` method for collecting xpath results as `RoNode`

## [0.2.9] 2019-28-03

* Squash memory leak in creating new `Node`s from the Rust API
* Safely unlink `Node`s obtained via XPath searches

## [0.2.8] 2019-25-03

### Changed

Minor internal changes to make the crate compile more reliably under MacOS, and other platforms which enable the `LIBXML_THREAD_ENABLED` compile-time flag. Thank you @caldwell !

## [0.2.7] 2019-09-03

### Added

* implement and test `replace_child_node` for element nodes

## [0.2.6] 2018-07-12

* Internal update to Rust 2018 Edition
* fix deallocation bugs with `.import_node()` and `.get_namespaces()`

## [0.2.5] 2018-26-09

### Added

* `Node::null` placeholder that avoids the tricky memory management of `Node::mock` that can lead to memory leaks. Really a poor substitute for the better `Option<Node>` type with a `None` value, which is **recommended** instead.

## [0.2.4] 2018-24-09

### Added

* `Context::from_node` method for convenient XPath context initialization via a Node object. Possible as nodes keep a reference to their owner `Document` object.

### Changed

* Ensured memory safety of cloning xpath `Context` objects
* Switched to using `Weak` references to the owner document, in `Node`, `Context` and `Object`, to prevent memory leaks in mutli-document pipelines.
* Speedup to XPath node retrieval

## [0.2.3] 2018-19-09

### Added

* `Node::findnodes` method for direct XPath search, without first explicitly instantiating a `Context`. Reusing a `Context` remains more efficient.

## [0.2.2] 2018-23-07

* Expose the underlying `libxml2` data structures in the public crate interface, to enable a first [libxslt](https://crates.io/crates/libxslt) crate proof of concept.

## [0.2.1] 2018-23-07

### Added

* `Node::set_node_rc_guard` which allows customizing the reference-count mutability threshold for Nodes.
* serialization tests for `Document`
* (crate internal) full set of libxml2 bindings as produced via `bindgen` (see #39)
* (crate internal) using libxml2's type language in the wrapper Rust modules
* (crate internal) setup bindings for reuse in higher-level crates, such as libxslt

### Changed

* `NodeType::from_c_int` renamed to `NodeType::from_int`, now accepting a `u32` argument

### Removed

* Removed dependence on custom C code; also removed gcc from build dependencies

## [0.2.0] 2018-19-07

This release adds fundamental breaking changes to the API. The API continues to be considered unstable until the `1.0.0` release.

### Added

* `dup` and `dup_from` methods for deeply duplicating a libxml2 document
* `is_unlinked` for quick check if a `Node` has been unlinked from a parent

### Changed

* safe API for `Node`s and `Document`s, with automatic pointer bookkeeping and memory deallocation, by @triptec
  * `Node`s are now bookkept by their owning document
  * libxml2 low-level memory deallocation is postponed until the `Document` is dropped, with the exception of unlinked nodes, who are deallocated on drop.
  * `Document::get_root_element` now has an option type, and returns `None` for an empty Document
  * `Node::mock` now takes owner `Document` as argument
  * proofed tests with `valgrind` and removed all obvious memory leaks
* All node operations that modify a `Node` now both require a `&mut Node` argument and return a `Result` type.
  * Full list of changed signatures in Node: `remove_attribute`, `remove_property`, `set_name`, `set_content`, `set_property`, `set_property_ns`, `set_attribute`, `set_attribute_ns`, `remove_attribute`, `set_namespace`, `recursively_remove_namespaces`, `append_text`
* Tree transforming operations that use operate on `&mut self`, and no longer return a Node if the return value is identical to the argument.
  * Changed signatures: `add_child`, `add_prev_sibling`, `add_next_sibling`
* `Result` types should always be checked for errors, as mutability conflicts are reported during runtime.

### Removed

* `global` module, which attempted to manage global libxml state for threaded workflows. May be readed after the API stabilizes

## [0.1.2] 2018-12-01

* We welcome Andreas (@triptec) to the core developer team!

### Added

* Workaround `.free` method for freeing nodes, until the `Rc<RefCell<_Node>>` free-on-drop solution by Andreas is introduced in 0.2

## [0.1.1] 2017-18-12

### Added

* `get_first_element_child` - similar to `get_first_child` but only returns XML Elements
* `is_element_node` - check if a given `Node` is an XML Element

### Changed

* Requiring owned `Node` function arguments only when consumed - `add_*` methods largely take `&Node` now.

## [0.1.0] 2017-09-11

Pushing up release to a 0.1, as contributor interest is starting to pick up, and the 0. version were getting a bit silly/wrong.

### Added

* Node methods: `unbind_node`,  `recursively_remove_namespaces`, `set_name`,
* Document methods: `import_node`

### Changed

* Updated gcc build to newer incantation, upped dependency version.

## [0.0.75] 2017-04-06

### Added

* Node methods: `get_namespace_declarations`, `get_property_ns` (alias: `get_attribute_ns`), `remove_property` (alias: `remove_attribute`), `get_attribute_node`, `get_namespace`, `lookup_namespace_prefix`, `lookup_namespace_uri`

* XPath methods: `findvalue` and `findnodes`, with optional node-bound evaluation.

### Changed

* The Node setter for a namespaced attribute is now `set_property_ns` (alias: `set_attribute_ns`)
* Node set_* methods are now consistently defined on `&mut self`
* Refactored wrongly used `url` to `href` for namespace-related Node ops.
* Fixed bug with Node's `get_content` method always returning empty
* More stable `append_text` for node, added tests

## [0.0.74] 2016-25-12

### Changed

* Namespace::new only requires a borrowed &Node now
* Fixed bug with wrongly discarded namespace prefixes on Namespace::new

### Added

* Namespace methods: `get_prefix`, `get_url`

## [0.0.73] 2016-25-12

### Added

* Document method: `as_node`

## [0.0.72] 2016-25-12

### Added

* Node methods: `get_last_child`, `get_child_nodes`, `get_child_elements`, `get_properties`, `get_attributes`

## [0.0.71] 2016-29-11

### Changed

* Namespace::new takes Node argument last

### Added

* Node namespace accessors - `set_namespace`, `get_namespaces`, `set_ns_attribute`, `set_ns_property`
* Namespace registration for XPath

## [0.0.7] 2016-27-11

### Changed

* stricter dependency spec in Cargo.toml
* cargo clippy compliant
* Document's `get_root_element` returns the document pointer as a Node for empty documents, type change from `Option<Node>` to simple `<Node>`

### Added

* Node accessors: `set_attribute`, `get_attribute`, `set_property` (the `attribute` callers are simple aliases for `property`)
* Node `to_hashable` for simple hashing of nodes
* Node `mock` for simple mock nodes in testing

## [0.0.5] 2016-07-01

Thanks to @grray for most of these improvements!

### Changed

* Switched to using the more permissive MIT license, consistent with libxml2 licensing
* Fixed segfault issues with xpath contexts

### Added

* Can now evaluate ```string(/foo//@bar)``` type XPath expressions, and use their result via ```.to_string()```

## [0.0.4] 2016-04-25

### Changed

* The ```Node.add_child``` method now adds a Node, while the old behavior of creating a new node with a given namespace and name is now ```Node.new_child```

### Added

* Can add following siblings via ```Node.add_next_sibling```
* Can now add text nodes via ```Node.new_text```
