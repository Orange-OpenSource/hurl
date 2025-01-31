# Filters

## Definition

[Captures] and [asserts] share a common structure: query. A query is used to extract data from an HTTP response; this data 
can come from the HTTP response body, the HTTP response headers or from the HTTP meta-information (like `duration` for instance)...

In this example, the query __`jsonpath "$.books[0].name"`__ is used in a capture to save data and in an assert to test 
the HTTP response body.

__Capture__:

<div class="schema-container schema-container u-font-size-2 u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-1">name<span class="schema-label">variable</span></span>
   <span> : </span>
   <span class="schema-token schema-color-2">jsonpath "$.books[0].name"<span class="schema-label">query</span></span>
 </div>
</div>

__Assert__:

<div class="schema-container schema-container u-font-size-2 u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">jsonpath "$.books[0].name"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-3">== "Dune"<span class="schema-label">predicate</span></span>
 </div>
</div>

In both case, the query is exactly the same: queries are the core structure of asserts and captures. Sometimes, you want
to process data extracted by queries: that's the purpose of __filters__.

Filters are used to transform value extracted by a query and can be used in asserts and captures to refine data. Filters 
__can be chained__, allowing for fine-grained data extraction. 


<div class="schema-container schema-container u-font-size-2 u-font-size-3-md">
 <div class="schema">
    <span class="schema-token schema-color-2">jsonpath "$.name"<span class="schema-label">query</span></span>
    <span class="schema-token schema-color-1">split "," nth 0<span class="schema-label">2 filters</span></span>
    <span class="schema-token schema-color-3">== "Herbert"<span class="schema-label">predicate</span></span>
 </div>
</div>


## Example

```hurl
GET https://example.org/api
HTTP 200
[Captures]
name: jsonpath "$user.id" replace /\d/ "x"
[Asserts]
header "x-servers" split "," count == 2
header "x-servers" split "," nth 0 == "rec1"
header "x-servers" split "," nth 1 == "rec3"
jsonpath "$.books" count == 12
```

## Description

### base64Decode

Decode a base 64 encoded string into bytes.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.token" base64Decode == hex,e4bda0e5a5bde4b896e7958c;
```

### base64Encode

Encode bytes into base 64 encoded string.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
bytes base64Encode == "5L2g5aW95LiW55WM"
```

### count

Counts the number of items in a collection.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.books" count == 12
```

### daysAfterNow

Returns the number of days between now and a date in the future.

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Expire-Date" daysAfterNow > 15
```

### daysBeforeNow

Returns the number of days between now and a date in the past.

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Start-Date" daysBeforeNow < 100
```

### decode

Decode bytes to string using encoding.

```hurl
# The 'Content-Type' HTTP response header does not precise the charset 'gb2312'
# so body must be decoded explicitly by Hurl before processing any text based assert
GET https://example.org/hello_china
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
# Content-Type has no encoding clue, we must decode ourselves the body response.
bytes decode "gb2312" xpath "string(//body)" == "你好世界"
```

### format

Formats a date to a string given [a specification format].

```hurl
GET https://example.org
HTTP 200
[Asserts]
cookie "LSID[Expires]" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
```

### htmlEscape

Converts the characters `&`, `<` and `>` to HTML-safe sequence.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.text" htmlEscape == "a &gt; b"
```

### htmlUnescape

Converts all named and numeric character references (e.g. `&gt;`, `&#62;`, `&#x3e;`) to the corresponding Unicode characters.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.escaped_html[1]" htmlUnescape == "Foo © bar 𝌆"
```

### jsonpath 

Evaluates a [JSONPath] expression.

```hurl
GET https://example.org/api
HTTP 200
[Captures]
books: xpath "string(//body/@data-books)" 
[Asserts]
variable "books" jsonpath "$[0].name" == "Dune"
variable "books" jsonpath "$[0].author" == "Franck Herbert"
```


### nth

Returns the element from a collection at a zero-based index.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.books" nth 2 == "Children of Dune"
```

### regex

Extracts regex capture group. Pattern must have at least one capture group.

```hurl
GET https://example.org/foo
HTTP 200
[Captures]
param1: header "header1"
param2: header "header2" regex "Hello (.*)!"
param3: header "header2" regex /Hello (.*)!/
```

### replace

Replaces all occurrences of old string with new string.

```hurl
GET https://example.org/foo
HTTP 200
[Captures]
url: jsonpath "$.url" replace "http://" "https://"
[Asserts]
jsonpath "$.ips" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"
```

### split

Splits to a list of strings around occurrences of the specified delimiter.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.ips" split ", " count == 3
```

### toDate

Converts a string to a date given [a specification format].

```hurl
GET https:///example.org
HTTP 200
[Asserts]
header "Expires" toDate "%a, %d %b %Y %H:%M:%S GMT" daysBeforeNow > 1000
```


ISO 8601 / RFC 3339 date and time format have shorthand format `%+`:

```hurl
GET https://example.org/api/books
HTTP 200
[Asserts]
jsonpath "$.published" == "2023-01-23T18:25:43.511Z"
jsonpath "$.published" toDate "%Y-%m-%dT%H:%M:%S%.fZ" format "%A" == "Monday"
jsonpath "$.published" toDate "%+" format "%A" == "Monday" # %+ can be used to parse ISO 8601 / RFC 3339
```

### toFloat

Converts to float number.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.pi" toFloat == 3.14
```

### toInt

Converts to integer number.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.id" toInt == 123
```

### urlDecode

Replaces %xx escapes with their single-character equivalent.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.encoded_url" urlDecode == "https://mozilla.org/?x=шеллы"
```

### urlEncode

Percent-encodes all the characters which are not included in unreserved chars (see [RFC3986]) with the exception of forward slash (/).

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.url" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
```

### xpath

Evaluates a [XPath] expression.

```hurl
GET https://example.org/hello_gb2312
HTTP 200
[Asserts]
bytes decode "gb2312" xpath "string(//body)" == "你好世界"
```


[Captures]: /docs/capturing-response.md
[asserts]: /docs/asserting-response.md
[RFC3986]: https://www.rfc-editor.org/rfc/rfc3986
[a specification format]: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
[XPath]: https://en.wikipedia.org/wiki/XPath
[JSONPath]: https://goessner.net/articles/JsonPath/
