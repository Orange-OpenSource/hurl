//! The parser functionality

use crate::bindings::*;
use crate::c_helpers::*;
use crate::error::StructuredError;
use crate::tree::*;

use std::convert::AsRef;
use std::error::Error;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs;
use std::io;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;

enum XmlParserOption {
  Recover = 1,
  Nodefdtd = 4,
  Noerror = 32,
  Nowarning = 64,
  Pedantic = 128,
  Noblanks = 256,
  Nonet = 2048,
  Noimplied = 8192,
  Compact = 65_536,
  Huge = 524_288,
  Ignoreenc = 2_097_152,
}

enum HtmlParserOption {
  Recover = 1,
  Nodefdtd = 4,
  Noerror = 32,
  Nowarning = 64,
  Pedantic = 128,
  Noblanks = 256,
  Nonet = 2048,
  Noimplied = 8192,
  Huge = 524_288,
  Compact = 65_536,
  Ignoreenc = 2_097_152,
}

/// Parser Options
pub struct ParserOptions<'a> {
  /// Relaxed parsing
  pub recover: bool,
  /// do not default a doctype if not found
  pub no_def_dtd: bool,
  /// do not default a doctype if not found
  pub no_error: bool,
  /// suppress warning reports
  pub no_warning: bool,
  /// pedantic error reporting
  pub pedantic: bool,
  /// remove blank nodes
  pub no_blanks: bool,
  /// Forbid network access
  pub no_net: bool,
  /// Do not add implied html/body... elements
  pub no_implied: bool,
  /// relax any hardcoded limit from the parser
  pub huge: bool,
  /// compact small text nodes
  pub compact: bool,
  /// ignore internal document encoding hint
  pub ignore_enc: bool,
  /// manually-specified encoding. A name containing a NUL byte is rejected: the
  /// `parse_*` methods return `XmlParseError::GotNullPointer`, as libxml2 does for
  /// a parse it cannot run, and `parse_string_with_diagnostics` returns
  /// `XmlParseFailure::InvalidEncoding`.
  pub encoding: Option<&'a str>,
}

impl ParserOptions<'_> {
  pub(crate) fn to_flags(&self, format: &ParseFormat) -> i32 {
    macro_rules! to_option_flag {
      (
        $condition:expr => $variant:ident
      ) => {
        if $condition {
          match format {
            ParseFormat::HTML => HtmlParserOption::$variant as i32,
            ParseFormat::XML => XmlParserOption::$variant as i32,
          }
        } else {
          0
        }
      };
    }
    // return the combined flags
    to_option_flag!(self.recover => Recover)
      + to_option_flag!(self.no_def_dtd => Nodefdtd)
      + to_option_flag!(self.no_error => Noerror)
      + to_option_flag!(self.no_warning => Nowarning)
      + to_option_flag!(self.pedantic => Pedantic)
      + to_option_flag!(self.no_blanks => Noblanks)
      + to_option_flag!(self.no_net => Nonet)
      + to_option_flag!(self.no_implied => Noimplied)
      + to_option_flag!(self.huge => Huge)
      + to_option_flag!(self.compact => Compact)
      + to_option_flag!(self.ignore_enc => Ignoreenc)
  }
}

impl Default for ParserOptions<'_> {
  fn default() -> Self {
    ParserOptions {
      recover: true,
      no_def_dtd: false,
      no_error: true,
      no_warning: true,
      pedantic: false,
      no_blanks: false,
      no_net: false,
      no_implied: false,
      huge: false,
      compact: false,
      ignore_enc: false,
      encoding: None,
    }
  }
}

///Parser Errors
pub enum XmlParseError {
  ///Parsing returned a null pointer as document pointer
  GotNullPointer,
  ///Could not open file error.
  FileOpenError,
  ///Document too large for libxml2.
  DocumentTooLarge,
}

/// An owned error from setting up or running a parser context.
#[derive(Debug)]
#[non_exhaustive]
pub enum XmlParseFailure {
  /// The input exceeds libxml2's signed 32-bit length limit.
  DocumentTooLarge,
  /// The encoding name contains an interior NUL byte.
  InvalidEncoding(std::ffi::NulError),
  /// libxml2 could not allocate a parser context.
  ContextAllocationFailed,
  /// Parsing returned no document; holds every diagnostic libxml2 reported.
  ParseFailed(Vec<StructuredError>),
}

impl fmt::Display for XmlParseFailure {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DocumentTooLarge => f.write_str("Document too large for i32"),
      Self::InvalidEncoding(error) => write!(f, "Invalid encoding name: {error}"),
      Self::ContextAllocationFailed => f.write_str("Could not allocate parser context"),
      Self::ParseFailed(diagnostics) => f.write_str(
        diagnostics
          .first()
          .and_then(|error| error.message.as_deref())
          .map_or("Parser returned no document", str::trim_end),
      ),
    }
  }
}

impl Error for XmlParseFailure {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::InvalidEncoding(error) => Some(error),
      _ => None,
    }
  }
}

struct ParserContext {
  ptr: ptr::NonNull<xmlParserCtxt>,
  format: ParseFormat,
}

impl ParserContext {
  fn new(format: ParseFormat) -> Result<Self, XmlParseFailure> {
    let context = unsafe {
      match format {
        ParseFormat::XML => xmlNewParserCtxt(),
        ParseFormat::HTML => htmlNewParserCtxt(),
      }
    };
    let ptr = ptr::NonNull::new(context).ok_or(XmlParseFailure::ContextAllocationFailed)?;
    Ok(Self { ptr, format })
  }
}

impl Drop for ParserContext {
  fn drop(&mut self) {
    // These destructors free the context, not the returned document.
    unsafe {
      match self.format {
        ParseFormat::XML => xmlFreeParserCtxt(self.ptr.as_ptr()),
        ParseFormat::HTML => htmlFreeParserCtxt(self.ptr.as_ptr()),
      }
    }
  }
}

/// The most diagnostics kept per parse. libxml2 2.13+ stops reporting at 100 itself; older
/// versions report every error, which hostile input can make one per few bytes
/// (1 MB of `&a;` gives 333,333 errors, ~33 MB).
pub const MAX_DIAGNOSTICS: usize = 100;

/// Appends each reported error to the `Vec<StructuredError>` at `ctx`, up to `MAX_DIAGNOSTICS`.
#[cfg(libxml_older_than_2_12)]
unsafe extern "C" fn collect_diagnostic(ctx: *mut c_void, error: xmlErrorPtr) {
  unsafe { push_diagnostic(ctx, error) }
}
#[cfg(not(libxml_older_than_2_12))]
unsafe extern "C" fn collect_diagnostic(ctx: *mut c_void, error: *const xmlError) {
  unsafe { push_diagnostic(ctx, error) }
}
unsafe fn push_diagnostic(ctx: *mut c_void, error: *const xmlError) {
  unsafe {
    let errors = &mut *(ctx as *mut Vec<StructuredError>);
    if errors.len() < MAX_DIAGNOSTICS && !error.is_null() {
      errors.push(StructuredError::from_raw(error));
    }
  }
}

/// Installs a structured error handler for the calling thread, restoring the previously
/// installed one when dropped (including on unwind). libxml2 keeps this handler per thread,
/// so parses on other threads are unaffected. A per-context handler would be preferable,
/// but before libxml2 2.13 the HTML parser ignores `sax->serror`, and from 2.13
/// `XML_PARSE_NOERROR` silences it.
struct ThreadErrorHandler {
  saved_handler: xmlStructuredErrorFunc,
  saved_data: *mut c_void,
}

impl ThreadErrorHandler {
  /// # Safety
  /// `data` must remain valid for `handler` until the returned guard is dropped.
  unsafe fn install(handler: xmlStructuredErrorFunc, data: *mut c_void) -> Self {
    unsafe {
      let saved = ThreadErrorHandler {
        saved_handler: *__xmlStructuredError(),
        saved_data: *__xmlStructuredErrorContext(),
      };
      xmlSetStructuredErrorFunc(data, handler);
      saved
    }
  }
}

impl Drop for ThreadErrorHandler {
  fn drop(&mut self) {
    unsafe { xmlSetStructuredErrorFunc(self.saved_data, self.saved_handler) }
  }
}

/// Collects the errors libxml2 reports on the current thread while it is alive.
struct ErrorCollector {
  // Declared first so it is dropped first: the handler is restored before `errors` goes.
  _handler: ThreadErrorHandler,
  // Boxed so the address handed to libxml2 stays put when the collector moves.
  #[allow(clippy::box_collection)]
  errors: Box<Vec<StructuredError>>,
}

impl ErrorCollector {
  fn install() -> Self {
    let mut errors: Box<Vec<StructuredError>> = Box::default();
    let data = &mut *errors as *mut Vec<StructuredError> as *mut c_void;
    let handler = unsafe { ThreadErrorHandler::install(Some(collect_diagnostic), data) };
    ErrorCollector {
      _handler: handler,
      errors,
    }
  }

  fn take(&mut self) -> Vec<StructuredError> {
    std::mem::take(&mut self.errors)
  }
}

/// What libxml2 reported during an `is_well_formed_html` check.
#[derive(Default)]
struct WellFormedness {
  /// An unknown-tag error, which the check does not count (see `is_well_formed_html`).
  unknown_tag: std::cell::Cell<bool>,
  /// Any other error: the document is ill-formed, as libxml2 itself judged it.
  other_error: std::cell::Cell<bool>,
}

/// Records the error in the `WellFormedness` at `ctx`.
#[cfg(libxml_older_than_2_12)]
unsafe extern "C" fn note_well_formedness(ctx: *mut c_void, error: xmlErrorPtr) {
  unsafe { record_well_formedness(ctx, error) }
}
#[cfg(not(libxml_older_than_2_12))]
unsafe extern "C" fn note_well_formedness(ctx: *mut c_void, error: *const xmlError) {
  unsafe { record_well_formedness(ctx, error) }
}
unsafe fn record_well_formedness(ctx: *mut c_void, error: *const xmlError) {
  unsafe {
    if error.is_null() || (*error).level < xmlErrorLevel_XML_ERR_ERROR {
      return; // warnings do not affect well-formedness
    }
    let state = &*(ctx as *const WellFormedness);
    if (*error).code as xmlParserErrors == xmlParserErrors_XML_HTML_UNKNOWN_TAG {
      state.unknown_tag.set(true);
    } else {
      state.other_error.set(true);
    }
  }
}

impl Error for XmlParseError {}

impl fmt::Debug for XmlParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{self}")
  }
}

impl fmt::Display for XmlParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        XmlParseError::GotNullPointer => "Got a Null pointer",
        XmlParseError::FileOpenError => "Unable to open path to file.",
        XmlParseError::DocumentTooLarge => "Document too large for i32.",
      }
    )
  }
}

/// Default encoding when not provided.
const DEFAULT_ENCODING: *const c_char = ptr::null();

/// Default URL when not provided.
const DEFAULT_URL: *const c_char = ptr::null();

/// Open file function.
fn xml_open(filename: &str) -> io::Result<*mut c_void> {
  let ptr = Box::into_raw(Box::new(fs::File::open(filename)?));
  Ok(ptr as *mut c_void)
}

/// Read callback for an FS file.
unsafe extern "C" fn xml_read(context: *mut c_void, buffer: *mut c_char, len: c_int) -> c_int {
  unsafe {
    // Len is always positive, typically 40-4000 bytes.
    let file = context as *mut fs::File;
    let buf = slice::from_raw_parts_mut(buffer as *mut u8, len as usize);
    match io::Read::read(&mut *file, buf) {
      Ok(v) => v as c_int,
      Err(_) => -1,
    }
  }
}

type XmlReadCallback = unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int;

/// Close callback for an FS file.
unsafe extern "C" fn xml_close(context: *mut c_void) -> c_int {
  unsafe {
    // Take rust ownership of the context and then drop it.
    let file = context as *mut fs::File;
    let _ = Box::from_raw(file);
    0
  }
}

type XmlCloseCallback = unsafe extern "C" fn(*mut c_void) -> c_int;

///Convert usize to i32 safely.
fn try_usize_to_i32(value: usize) -> Result<i32, XmlParseError> {
  if cfg!(target_pointer_width = "16") || (value < i32::MAX as usize) {
    // Cannot safely use our value comparison, but the conversion if always safe.
    // Or, if the value can be safely represented as a 32-bit signed integer.
    Ok(value as i32)
  } else {
    // Document too large, cannot parse using libxml2.
    Err(XmlParseError::DocumentTooLarge)
  }
}

/// Convert an optional encoding name into a C string, rejecting interior NUL bytes.
/// The caller must keep the returned `CString` alive for as long as its pointer is in use;
/// take the pointer with `as_deref()`, as matching on the `Option` by value drops it (#216).
fn encoding_to_cstring(encoding: Option<&str>) -> Result<Option<CString>, std::ffi::NulError> {
  encoding.map(CString::new).transpose()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Enum for the parse formats supported by libxml2
pub enum ParseFormat {
  /// Strict parsing for XML
  XML,
  /// Relaxed parsing for HTML
  HTML,
}
/// Parsing API wrapper for libxml2
pub struct Parser {
  /// The `ParseFormat` for this parser
  pub format: ParseFormat,
}
impl Default for Parser {
  /// Create a parser for XML documents
  fn default() -> Self {
    // avoid deadlocks from using multiple parsers
    crate::init_parser();
    Parser {
      format: ParseFormat::XML,
    }
  }
}
impl Parser {
  /// Create a parser for HTML documents
  pub fn default_html() -> Self {
    // avoid deadlocks from using multiple parsers
    crate::init_parser();
    Parser {
      format: ParseFormat::HTML,
    }
  }

  /// Parses the XML/HTML file `filename` to generate a new `Document`
  pub fn parse_file(&self, filename: &str) -> Result<Document, XmlParseError> {
    self.parse_file_with_options(filename, ParserOptions::default())
  }

  /// Parses the XML/HTML file `filename` with a manually-specified parser-options
  /// to generate a new `Document`
  pub fn parse_file_with_options(
    &self,
    filename: &str,
    parser_options: ParserOptions,
  ) -> Result<Document, XmlParseError> {
    // Create extern C callbacks for to read and close a Rust file through
    // a void pointer.
    let ioread: Option<XmlReadCallback> = Some(xml_read);
    let ioclose: Option<XmlCloseCallback> = Some(xml_close);
    // Process encoding before opening the file, so an invalid name cannot leak `ioctx`.
    let encoding_cstring =
      encoding_to_cstring(parser_options.encoding).map_err(|_| XmlParseError::GotNullPointer)?;
    let encoding_ptr = encoding_cstring
      .as_deref()
      .map_or(DEFAULT_ENCODING, CStr::as_ptr);

    let ioctx = match xml_open(filename) {
      Ok(v) => v,
      Err(_) => return Err(XmlParseError::FileOpenError),
    };

    // Process url.
    let url_ptr = DEFAULT_URL;

    unsafe {
      xmlKeepBlanksDefault(1);
    }

    let options = parser_options.to_flags(&self.format);

    match self.format {
      ParseFormat::XML => unsafe {
        let doc_ptr = xmlReadIO(ioread, ioclose, ioctx, url_ptr, encoding_ptr, options);
        if doc_ptr.is_null() {
          Err(XmlParseError::GotNullPointer)
        } else {
          Ok(Document::new_ptr(doc_ptr))
        }
      },
      ParseFormat::HTML => unsafe {
        let doc_ptr = htmlReadIO(ioread, ioclose, ioctx, url_ptr, encoding_ptr, options);
        if doc_ptr.is_null() {
          Err(XmlParseError::GotNullPointer)
        } else {
          Ok(Document::new_ptr(doc_ptr))
        }
      },
    }
  }

  ///Parses the XML/HTML bytes `input` to generate a new `Document`
  pub fn parse_string<Bytes: AsRef<[u8]>>(&self, input: Bytes) -> Result<Document, XmlParseError> {
    self.parse_string_with_options(input, ParserOptions::default())
  }

  /// Parse XML or HTML with options, returning the document together with every
  /// error and warning libxml2 reported while parsing it. On failure, the
  /// diagnostics are returned in `XmlParseFailure::ParseFailed`.
  ///
  /// Diagnostics are collected regardless of `no_error` / `no_warning`, which only
  /// control printing, and are capped at [`MAX_DIAGNOSTICS`]. As with the other parser
  /// methods, `recover` must be disabled to reject malformed input; with recovery on,
  /// the errors arrive alongside the recovered document.
  ///
  /// While parsing, this replaces the calling thread's libxml2 structured error
  /// handler (`xmlSetStructuredErrorFunc`) and restores it afterwards, so a handler
  /// installed by the application does not see these errors.
  pub fn parse_string_with_diagnostics<Bytes: AsRef<[u8]>>(
    &self,
    input: Bytes,
    parser_options: ParserOptions,
  ) -> Result<(Document, Vec<StructuredError>), XmlParseFailure> {
    let bytes = input.as_ref();
    let size = try_usize_to_i32(bytes.len()).map_err(|_| XmlParseFailure::DocumentTooLarge)?;
    let encoding_cstring =
      encoding_to_cstring(parser_options.encoding).map_err(XmlParseFailure::InvalidEncoding)?;
    let encoding_ptr = encoding_cstring
      .as_deref()
      .map_or(DEFAULT_ENCODING, CStr::as_ptr);
    let options = parser_options.to_flags(&self.format);
    let context = ParserContext::new(self.format)?;
    let mut collector = ErrorCollector::install();
    unsafe {
      let document = match self.format {
        ParseFormat::XML => xmlCtxtReadMemory(
          context.ptr.as_ptr(),
          bytes.as_ptr().cast(),
          size,
          DEFAULT_URL,
          encoding_ptr,
          options,
        ),
        ParseFormat::HTML => htmlCtxtReadMemory(
          context.ptr.as_ptr(),
          bytes.as_ptr().cast(),
          size,
          DEFAULT_URL,
          encoding_ptr,
          options,
        ),
      };
      let diagnostics = collector.take();
      if document.is_null() {
        Err(XmlParseFailure::ParseFailed(diagnostics))
      } else {
        Ok((Document::new_ptr(document), diagnostics))
      }
    }
  }

  ///Parses the XML/HTML bytes `input` with a manually-specified
  ///parser-options to generate a new `Document`
  pub fn parse_string_with_options<Bytes: AsRef<[u8]>>(
    &self,
    input: Bytes,
    parser_options: ParserOptions,
  ) -> Result<Document, XmlParseError> {
    // Process input bytes.
    let input_bytes = input.as_ref();
    let input_ptr = input_bytes.as_ptr() as *const c_char;
    let input_len = try_usize_to_i32(input_bytes.len())?;

    // Process encoding.
    let encoding_cstring =
      encoding_to_cstring(parser_options.encoding).map_err(|_| XmlParseError::GotNullPointer)?;
    let encoding_ptr = encoding_cstring
      .as_deref()
      .map_or(DEFAULT_ENCODING, CStr::as_ptr);

    // Process url.
    let url_ptr = DEFAULT_URL;

    let options = parser_options.to_flags(&self.format);

    match self.format {
      ParseFormat::XML => unsafe {
        let docptr = xmlReadMemory(input_ptr, input_len, url_ptr, encoding_ptr, options);
        if docptr.is_null() {
          Err(XmlParseError::GotNullPointer)
        } else {
          Ok(Document::new_ptr(docptr))
        }
      },
      ParseFormat::HTML => unsafe {
        let docptr = htmlReadMemory(input_ptr, input_len, url_ptr, encoding_ptr, options);
        if docptr.is_null() {
          Err(XmlParseError::GotNullPointer)
        } else {
          Ok(Document::new_ptr(docptr))
        }
      },
    }
  }

  /// Checks a string for well-formedness.
  pub fn is_well_formed_html<Bytes: AsRef<[u8]>>(&self, input: Bytes) -> bool {
    self.is_well_formed_html_with_encoding(input, None)
  }

  /// Checks a string for well-formedness with manually-specified encoding.
  ///
  /// The verdict is libxml2's, with one allowance: libxml2 before 2.14 has an HTML4-era
  /// parser that reports HTML5, SVG and MathML elements (`<main>`, `<svg>`, `<math>`, ...)
  /// as unknown tags. Those reports are not counted, matching libxml2 2.14+, which no
  /// longer makes them. Any other error still makes the document ill-formed. While
  /// checking, this replaces the calling thread's libxml2 structured error handler and
  /// restores it afterwards.
  pub fn is_well_formed_html_with_encoding<Bytes: AsRef<[u8]>>(
    &self,
    input: Bytes,
    encoding: Option<&str>,
  ) -> bool {
    // Process input string.
    let input_bytes = input.as_ref();
    if input_bytes.is_empty() {
      return false;
    }
    let input_ptr = input_bytes.as_ptr() as *const c_char;
    let input_len = match try_usize_to_i32(input_bytes.len()) {
      Ok(v) => v,
      Err(_) => return false,
    };

    // Process encoding.
    let encoding_cstring = match encoding_to_cstring(encoding) {
      Ok(v) => v,
      Err(_) => return false,
    };
    let encoding_ptr = encoding_cstring
      .as_deref()
      .map_or(DEFAULT_ENCODING, CStr::as_ptr);

    // Process url.
    let url_ptr = DEFAULT_URL;
    // disable generic error lines from libxml2
    match self.format {
      ParseFormat::XML => false, // TODO: Add support for XML at some point
      ParseFormat::HTML => unsafe {
        let ctxt = htmlNewParserCtxt();
        // Declared before the guard, so it outlives the handler that writes to it.
        let reported = WellFormedness::default();
        let _handler = ThreadErrorHandler::install(
          Some(note_well_formedness),
          &reported as *const WellFormedness as *mut c_void,
        );
        let docptr = htmlCtxtReadMemory(ctxt, input_ptr, input_len, url_ptr, encoding_ptr, 10_596); // htmlParserOption = 4+32+64+256+2048+8192
        // libxml2's own verdict, unless only unknown-tag errors cleared it.
        let well_formed = (!ctxt.is_null() && (*ctxt).wellFormed > 0)
          || (reported.unknown_tag.get() && !reported.other_error.get());
        let well_formed_final = if well_formed {
          // Basic well-formedness passes, let's check if we have an <html> element as root too
          // (no early returns here: `ctxt` and `docptr` are freed below)
          if docptr.is_null() {
            false
          } else {
            // xmlNodeGetName is null-safe, so a document without a root element yields null here
            let name_ptr = xmlNodeGetName(xmlDocGetRootElement(docptr));
            !name_ptr.is_null() && CStr::from_ptr(name_ptr).to_bytes() == b"html"
          }
        } else {
          false
        };

        if !ctxt.is_null() {
          htmlFreeParserCtxt(ctxt);
        }
        if !docptr.is_null() {
          xmlFreeDoc(docptr);
        }
        well_formed_final
      },
    }
  }
}
