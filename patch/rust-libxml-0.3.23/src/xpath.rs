//! The `XPath` functionality

use crate::bindings::*;
use crate::c_helpers::*;
use crate::error::StructuredError;
use crate::readonly::RoNode;
use crate::tree::{Document, DocumentRef, DocumentWeak, Node};
use libc::{c_char, c_void, size_t};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::fmt;
use std::rc::Rc;
use std::str;

///Thinly wrapped libxml2 xpath context
pub(crate) type ContextRef = Rc<RefCell<_Context>>;

#[derive(Debug)]
pub(crate) struct _Context(pub(crate) xmlXPathContextPtr);

impl Drop for _Context {
  ///free xpath context when it goes out of scope
  fn drop(&mut self) {
    unsafe {
      xmlXPathFreeContext(self.0);
    }
  }
}

/// An XPath context
#[derive(Clone)]
pub struct Context {
  /// Safe reference to the libxml2 context pointer
  pub(crate) context_ptr: ContextRef,
  ///Document contains pointer, needed for ContextPtr, so we need to borrow Document to prevent it's freeing
  pub(crate) document: DocumentWeak,
}

///Essentially, the result of the evaluation of some xpath expression
#[derive(Debug)]
pub struct Object {
  ///libxml's `ObjectPtr`
  pub ptr: xmlXPathObjectPtr,
  document: DocumentWeak,
}

/// A structured error from an XPath evaluation.
///
/// libxml2 returns a NULL result — which the bare [`Context::evaluate`] family
/// collapses to `Err(())` — for several reasons, most importantly the
/// *"growing nodeset hit limit"* when a `//X[predicate]` query materializes
/// more than `XPATH_MAX_NODESET_LENGTH` (10M) nodes on a huge document. The
/// `*_checked` variants snapshot libxml2's last error here so the cause is
/// recoverable; see [`is_nodeset_limit`](Self::is_nodeset_limit).
#[derive(Debug, Clone)]
pub struct XPathError {
  /// libxml2's message (e.g. `"growing nodeset hit limit"`), if recorded.
  pub message: Option<String>,
  /// libxml2 error code (`xmlParserErrors`). `0` when unknown.
  pub code: i32,
  /// libxml2 error domain (`xmlErrorDomain`). `0` when unknown.
  pub domain: i32,
}

impl XPathError {
  /// Snapshot the thread's last libxml2 error (set when an evaluation returns
  /// NULL); empty message when libxml2 recorded no structured detail.
  fn from_last_error() -> Self {
    let err_ptr = unsafe { xmlGetLastError() };
    if err_ptr.is_null() {
      XPathError {
        message: None,
        code: 0,
        domain: 0,
      }
    } else {
      let se = unsafe { StructuredError::from_raw(err_ptr) };
      XPathError {
        message: se.message,
        code: se.code,
        domain: se.domain,
      }
    }
  }

  /// True when this is libxml2's XPath nodeset-length ceiling — the signal that
  /// a `//`-materializing query grew past the 10M-node internal limit (rather
  /// than a syntax error or a missing namespace). The message text is the
  /// reliable discriminator; the underlying code is a generic memory error.
  pub fn is_nodeset_limit(&self) -> bool {
    self
      .message
      .as_deref()
      .is_some_and(|m| m.contains("nodeset") || m.contains("Memory allocation failed"))
  }
}

impl fmt::Display for XPathError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.message {
      Some(m) => write!(
        f,
        "XPath evaluation error: {} (code {}, domain {})",
        m.trim(),
        self.code,
        self.domain
      ),
      None => write!(f, "XPath evaluation failed (no libxml2 error detail)"),
    }
  }
}

impl std::error::Error for XPathError {}

/// A compiled XPath expression (libxml2's `xmlXPathCompExpr`): parsed once, then evaluated any number of times
/// without the compilation that every string-taking evaluator ([`Context::evaluate`], [`Context::findnodes`], …)
/// repeats on each call. Evaluating it gives the result of evaluating its source string: libxml2's string evaluators
/// compile, optimize and run the expression exactly as [`Expression::compile`] and [`Context::evaluate_compiled`] do.
///
/// An expression holds no document and no context, so one `Expression` may be evaluated against any [`Context`].
/// Two caveats are libxml2's own: a namespace prefix is resolved when the expression runs, from the context it runs
/// in; and the first evaluation of a function call caches the function it found, so an expression calling an
/// extension function should only run in contexts that register that same function. Because evaluation writes that
/// cache into the expression, an `Expression` is neither `Send` nor `Sync`.
#[derive(Debug)]
pub struct Expression {
  ptr: xmlXPathCompExprPtr,
}

impl Expression {
  /// Compile `xpath`, or return libxml2's structured error when it is not a well-formed expression.
  pub fn compile(xpath: &str) -> Result<Expression, XPathError> {
    crate::init_parser();
    let c_xpath = CString::new(xpath).map_err(|_| XPathError {
      message: Some("XPath expression contains a NUL byte".to_string()),
      code: 0,
      domain: 0,
    })?;
    unsafe { xmlResetLastError() };
    let ptr = unsafe { xmlXPathCompile(c_xpath.as_bytes().as_ptr()) };
    if ptr.is_null() {
      Err(XPathError::from_last_error())
    } else {
      Ok(Expression { ptr })
    }
  }

  /// Returns the raw libxml2 compiled-expression pointer behind the struct
  pub fn as_ptr(&self) -> xmlXPathCompExprPtr {
    self.ptr
  }
}

impl Drop for Expression {
  /// free the compiled expression, with its steps, when it goes out of scope
  fn drop(&mut self) {
    unsafe { xmlXPathFreeCompExpr(self.ptr) };
  }
}

impl Context {
  ///create the xpath context for a document
  pub fn new(doc: &Document) -> Result<Context, ()> {
    let ctxtptr = unsafe { xmlXPathNewContext(doc.doc_ptr()) };
    if ctxtptr.is_null() {
      Err(())
    } else {
      Ok(Context {
        context_ptr: Rc::new(RefCell::new(_Context(ctxtptr))),
        document: Rc::downgrade(&doc.0),
      })
    }
  }
  pub(crate) fn new_ptr(docref: &DocumentRef) -> Result<Context, ()> {
    let ctxtptr = unsafe { xmlXPathNewContext(docref.borrow().doc_ptr) };
    if ctxtptr.is_null() {
      Err(())
    } else {
      Ok(Context {
        context_ptr: Rc::new(RefCell::new(_Context(ctxtptr))),
        document: Rc::downgrade(docref),
      })
    }
  }

  /// Returns the raw libxml2 context pointer behind the struct
  pub fn as_ptr(&self) -> xmlXPathContextPtr {
    self.context_ptr.borrow().0
  }

  /// Instantiate a new Context for the Document of a given Node.
  /// Note: the Context is root-level for that document, use `.set_context_node` to limit scope to this node
  pub fn from_node(node: &Node) -> Result<Context, ()> {
    let docref = node.get_docref().upgrade().unwrap();
    Context::new_ptr(&docref)
  }

  /// Register a namespace prefix-href pair on the xpath context
  pub fn register_namespace(&self, prefix: &str, href: &str) -> Result<(), ()> {
    let c_prefix = CString::new(prefix).unwrap();
    let c_href = CString::new(href).unwrap();
    unsafe {
      let result = xmlXPathRegisterNs(
        self.as_ptr(),
        c_prefix.as_bytes().as_ptr(),
        c_href.as_bytes().as_ptr(),
      );
      if result != 0 { Err(()) } else { Ok(()) }
    }
  }

  /// Shared body of the `*_checked` evaluators: reset the thread's last error
  /// (so [`XPathError::from_last_error`] reads THIS call's failure, not a stale
  /// one), run `eval`, and wrap the raw object — surfacing the structured error
  /// when libxml2 returns NULL. `eval` receives the NUL-terminated expression.
  fn eval_checked(
    &self,
    xpath: &str,
    eval: impl FnOnce(*const u8) -> xmlXPathObjectPtr,
  ) -> Result<Object, XPathError> {
    let c_xpath = CString::new(xpath).unwrap();
    unsafe { xmlResetLastError() };
    let ptr = eval(c_xpath.as_bytes().as_ptr());
    if ptr.is_null() {
      Err(XPathError::from_last_error())
    } else {
      Ok(Object {
        ptr,
        document: self.document.clone(),
      })
    }
  }

  ///evaluate an xpath
  pub fn evaluate(&self, xpath: &str) -> Result<Object, ()> {
    self.evaluate_checked(xpath).map_err(|_| ())
  }

  /// Evaluate `xpath`, returning libxml2's structured [`XPathError`] on failure
  /// instead of the bare `()` that [`Context::evaluate`] yields.
  pub fn evaluate_checked(&self, xpath: &str) -> Result<Object, XPathError> {
    self.eval_checked(xpath, |s| unsafe {
      xmlXPathEvalExpression(s, self.as_ptr())
    })
  }

  ///evaluate an xpath on a context Node
  pub fn node_evaluate(&self, xpath: &str, node: &Node) -> Result<Object, ()> {
    self.node_evaluate_checked(xpath, node).map_err(|_| ())
  }

  /// Evaluate `xpath` relative to `node`. See [`Context::evaluate_checked`].
  pub fn node_evaluate_checked(&self, xpath: &str, node: &Node) -> Result<Object, XPathError> {
    self.eval_checked(xpath, |s| unsafe {
      xmlXPathNodeEval(node.node_ptr(), s, self.as_ptr())
    })
  }

  ///evaluate an xpath on a context RoNode
  pub fn node_evaluate_readonly(&self, xpath: &str, node: RoNode) -> Result<Object, ()> {
    self
      .node_evaluate_readonly_checked(xpath, node)
      .map_err(|_| ())
  }

  /// Evaluate `xpath` relative to a read-only `node`. See
  /// [`Context::evaluate_checked`].
  pub fn node_evaluate_readonly_checked(
    &self,
    xpath: &str,
    node: RoNode,
  ) -> Result<Object, XPathError> {
    self.eval_checked(xpath, |s| unsafe {
      xmlXPathNodeEval(node.0, s, self.as_ptr())
    })
  }

  /// localize xpath context to a specific Node
  pub fn set_context_node(&mut self, node: &Node) -> Result<(), ()> {
    unsafe {
      let result = xmlXPathSetContextNode(node.node_ptr(), self.as_ptr());
      if result != 0 {
        return Err(());
      }
    }
    Ok(())
  }

  /// find nodes via xpath, at a specified node or the document root
  pub fn findnodes(&mut self, xpath: &str, node_opt: Option<&Node>) -> Result<Vec<Node>, ()> {
    let evaluated = if let Some(node) = node_opt {
      self.node_evaluate(xpath, node)?
    } else {
      self.evaluate(xpath)?
    };
    Ok(evaluated.get_nodes_as_vec())
  }

  /// find literal values via xpath, at a specified node or the document root
  pub fn findvalues(&mut self, xpath: &str, node_opt: Option<&Node>) -> Result<Vec<String>, ()> {
    let evaluated = if let Some(node) = node_opt {
      self.node_evaluate(xpath, node)?
    } else {
      self.evaluate(xpath)?
    };
    Ok(evaluated.get_nodes_as_str())
  }

  /// find a literal value via xpath, at a specified node or the document root
  pub fn findvalue(&mut self, xpath: &str, node_opt: Option<&Node>) -> Result<String, ()> {
    let evaluated = if let Some(node) = node_opt {
      self.node_evaluate(xpath, node)?
    } else {
      self.evaluate(xpath)?
    };
    Ok(evaluated.to_string())
  }

  /// Shared body of the compiled evaluators, as [`Context::eval_checked`] is the string evaluators': reset the
  /// thread's last error, run the expression, and wrap the raw object or surface the structured error.
  fn compiled_eval_checked(&self, expression: &Expression) -> Result<Object, XPathError> {
    unsafe { xmlResetLastError() };
    let ptr = unsafe { xmlXPathCompiledEval(expression.ptr, self.as_ptr()) };
    if ptr.is_null() {
      Err(XPathError::from_last_error())
    } else {
      Ok(Object {
        ptr,
        document: self.document.clone(),
      })
    }
  }

  /// Evaluate a compiled expression at the context's current node, as [`Context::evaluate_checked`] evaluates a
  /// string.
  pub fn evaluate_compiled(&self, expression: &Expression) -> Result<Object, XPathError> {
    self.compiled_eval_checked(expression)
  }

  /// Evaluate a compiled expression relative to `node`, as [`Context::node_evaluate_checked`] evaluates a string:
  /// libxml2's `xmlXPathNodeEval` sets the context node, then runs the expression. Like it, this leaves `node` as the
  /// context's current node.
  pub fn node_evaluate_compiled(
    &self,
    expression: &Expression,
    node: &Node,
  ) -> Result<Object, XPathError> {
    unsafe { xmlResetLastError() };
    if unsafe { xmlXPathSetContextNode(node.node_ptr(), self.as_ptr()) } < 0 {
      return Err(XPathError::from_last_error());
    }
    self.compiled_eval_checked(expression)
  }

  /// find nodes via a compiled expression, at a specified node or the context's current node (see
  /// [`Context::findnodes`])
  pub fn findnodes_compiled(
    &self,
    expression: &Expression,
    node_opt: Option<&Node>,
  ) -> Result<Vec<Node>, XPathError> {
    let evaluated = match node_opt {
      Some(node) => self.node_evaluate_compiled(expression, node)?,
      None => self.evaluate_compiled(expression)?,
    };
    Ok(evaluated.get_nodes_as_vec())
  }

  /// find literal values via a compiled expression, at a specified node or the context's current node (see
  /// [`Context::findvalues`])
  pub fn findvalues_compiled(
    &self,
    expression: &Expression,
    node_opt: Option<&Node>,
  ) -> Result<Vec<String>, XPathError> {
    let evaluated = match node_opt {
      Some(node) => self.node_evaluate_compiled(expression, node)?,
      None => self.evaluate_compiled(expression)?,
    };
    Ok(evaluated.get_nodes_as_str())
  }

  /// find a literal value via a compiled expression, at a specified node or the context's current node (see
  /// [`Context::findvalue`])
  pub fn findvalue_compiled(
    &self,
    expression: &Expression,
    node_opt: Option<&Node>,
  ) -> Result<String, XPathError> {
    let evaluated = match node_opt {
      Some(node) => self.node_evaluate_compiled(expression, node)?,
      None => self.evaluate_compiled(expression)?,
    };
    Ok(evaluated.to_string())
  }
}

impl Drop for Object {
  /// free the memory allocated
  fn drop(&mut self) {
    unsafe {
      xmlXPathFreeObject(self.ptr);
    }
  }
}

impl Object {
  ///get the number of nodes in the result set
  pub fn get_number_of_nodes(&self) -> usize {
    let v = xmlXPathObjectNumberOfNodes(self.ptr);
    if v == -1 {
      panic!("rust-libxml: xpath: Passed in null pointer!");
    }
    if v == -2 {
      // No nodes found!
      return 0;
    }
    if v < -2 {
      panic!("rust-libxml: xpath: expected non-negative number of result nodes");
    }
    v as usize
  }

  /// returns the result set as a vector of `Node` objects
  pub fn get_nodes_as_vec(&self) -> Vec<Node> {
    let n = self.get_number_of_nodes();
    let mut vec: Vec<Node> = Vec::with_capacity(n);
    let slice = if n > 0 {
      xmlXPathObjectGetNodes(self.ptr, n as size_t)
    } else {
      Vec::new()
    };
    for ptr in slice {
      if ptr.is_null() {
        panic!("rust-libxml: xpath: found null pointer result set");
      }
      let node = Node::wrap(ptr, &self.document.upgrade().unwrap());
      vec.push(node);
    }
    vec
  }

  /// returns the result set as a vector of `RoNode` objects
  pub fn get_readonly_nodes_as_vec(&self) -> Vec<RoNode> {
    let n = self.get_number_of_nodes();
    let mut vec: Vec<RoNode> = Vec::with_capacity(n);
    let slice = if n > 0 {
      xmlXPathObjectGetNodes(self.ptr, n as size_t)
    } else {
      Vec::new()
    };
    for ptr in slice {
      if ptr.is_null() {
        panic!("rust-libxml: xpath: found null pointer result set");
      }
      vec.push(RoNode(ptr));
    }
    vec
  }

  /// returns the result set as a vector of Strings
  pub fn get_nodes_as_str(&self) -> Vec<String> {
    let n = self.get_number_of_nodes();
    let mut vec: Vec<String> = Vec::with_capacity(n);
    let slice = if n > 0 {
      xmlXPathObjectGetNodes(self.ptr, n as size_t)
    } else {
      Vec::new()
    };
    for ptr in slice {
      if ptr.is_null() {
        panic!("rust-libxml: xpath: found null pointer result set");
      }
      let value_ptr = unsafe { xmlXPathCastNodeToString(ptr) };
      if value_ptr.is_null() {
        // OOM in the cast; record an empty string rather than `strlen(NULL)`.
        vec.push(String::new());
        continue;
      }
      let c_value_string = unsafe { CStr::from_ptr(value_ptr as *const c_char) };
      let ready_str = c_value_string.to_string_lossy().into_owned();
      bindgenFree(value_ptr as *mut c_void);
      vec.push(ready_str);
    }
    vec
  }
}

impl fmt::Display for Object {
  /// use if the XPath used was meant to return a string, such as string(//foo/@attr)
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    unsafe {
      let receiver = xmlXPathCastToString(self.ptr);
      if receiver.is_null() {
        // OOM in the cast; write nothing rather than `strlen(NULL)`.
        return Ok(());
      }
      let c_string = CStr::from_ptr(receiver as *const c_char);
      let rust_string = str::from_utf8(c_string.to_bytes()).unwrap().to_owned();
      bindgenFree(receiver as *mut c_void);
      write!(f, "{rust_string}")
    }
  }
}

/// Calls the binding to http://xmlsoft.org/html/libxml-xpath.html#xmlXPathCompile and return true if
/// a non-null pointer is returned. The idea is to use this to validate an xpath independent of context.
/// Tests describing what this validates in tests/xpath_tests.rs
pub fn is_well_formed_xpath(xpath: &str) -> bool {
  crate::init_parser();
  let c_xpath = CString::new(xpath).unwrap();
  let xml_xpath_comp_expr_ptr = unsafe { xmlXPathCompile(c_xpath.as_bytes().as_ptr()) };
  if xml_xpath_comp_expr_ptr.is_null() {
    false
  } else {
    // A compiled expression owns its steps; a plain free would leak them.
    unsafe { xmlXPathFreeCompExpr(xml_xpath_comp_expr_ptr) };
    true
  }
}
