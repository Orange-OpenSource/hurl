# Error Rendering Implementation


![UML diagram for errors](errors.svg)

## Implementation note

- `pub fn error_parsing_rich` is only implemented for `ParserError`, and ultimately call `to_string` on error
- `pub fn error_runtime_rich` is only implemented for `RunnerError`, and ultimately call `to_string` on error
- `pub fn debug_error` is only implemented for `RunnerError`, and ultimately call `to_string` on error
- `OutputError` calls directly `to_string`

The only difference between the two errors are a supplemental parameter `entry_src_info` in `RunnerError`.

> [!NOTE]  
> Do we delete `error_parsing_rich`, `error_runtime_rich` and call `to_string` on error? It feels not good
> to have generic method 

In `hurl_core` we have a trait `ParseError` and a struct `ParseError`:

`hurl_core/src/parser/error.rs`:

```rust
pub struct ParseError {
    pub pos: Pos,
    pub recoverable: bool,
    pub kind: ParseErrorKind,
}
```

`hurl_core/src/combinator.rs`::

```rust
pub trait ParseError {
    /// Is this error recoverable or not?
    fn is_recoverable(&self) -> bool;

    /// Transforms this error to a recoverable one.
    fn to_recoverable(self) -> Self;

    /// Transforms this error to a non-recoverable one.
    fn to_non_recoverable(self) -> Self;
}
```


## Example of errors


### ParseError

```
error: Parsing jsonpath expression
  --> tests_error_parser/jsonpath.hurl:4:10
   |
 4 | jsonpath ? equals 1
   |          ^ expecting a jsonpath expression
   |
```

### OutputError

```
error: Decompression error
  --> tests_failed/output_decompress.hurl:5:1
   |
 5 | GET http://localhost:8000/error-output-decompress
   | ^ could not uncompress response with gzip
   |
```

### RunnerError

```
error: Assert body value
  --> tests_failed/assert_base64.hurl:12:8
   |
   | GET http://localhost:8000/error-assert-base64
   | ...
12 | base64,bGluZTEKbGluZTIKbGluZTMK;
   |        ^^^^^^^^^^^^^^^^^^^^^^^^ actual value is <6c696e65310a6c696e65320d0a6c696e65330a>
   |
```

```
error: Assert failure
  --> tests_failed/assert_query_cookie.hurl:7:0
   |
   | GET http://localhost:8000/error-assert-query-cookie
   | ...
 7 | cookie "cookie1[Secure]" == false      # This is not valid, Secure attribute exists or not but does have a value
   |   actual:   none
   |   expected: boolean <false>
   |
```

```
error: Assert body value
   --> tests_failed/diff.hurl:40:1
    |
    | GET http://localhost:8000/diff/change/line2
    | ...
 40 |   "first_name": "John",
    |   -  "first_name": "John",
    |   +  "first_name": "Bob",
    |
```


> [!NOTE]  
> Make implicit assert error identical to explicit errors

Explicit:

```
error: Assert failure
  --> /tmp/test.hurl:4:0
   |
   | GET https://hurl.dev
   | ...
 4 | status == 304
   |   actual:   integer <200>
   |   expected: integer <304>
   |
```

Implicit:

```
error: Assert status code
  --> /tmp/test.hurl:4:6
   |
   | GET https://hurl.dev
 4 | HTTP 304
   |      ^^^ actual value is <200>
   |
```