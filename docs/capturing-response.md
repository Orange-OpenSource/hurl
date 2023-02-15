# Capturing Response

## Captures

Captures are optional values that are __extracted from the HTTP response__ and stored in a named variable.
These captures may be the response status code, part of or the entire the body, and response headers.

Captured variables can be accessed through a run session; each new value of a given variable overrides the last value.

Captures can be useful for using data from one request in another request, such as when working with [CSRF tokens].
Variables in a Hurl file can be created from captures or [injected into the session].

```hurl
# An example to show how to pass a CSRF token
# from one request to another:

# First GET request to get CSRF token value:
GET https://example.org

HTTP 200
# Capture the CSRF token value from html body.
[Captures]
csrf_token: xpath "normalize-space(//meta[@name='_csrf_token']/@content)"


# Do the login !
POST https://acmecorp.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

Structure of a capture:

<div class="schema-container schema-container u-font-size-2 u-font-size-3-sm">
 <div class="schema">
   <span class="schema-token schema-color-1">my_var<span class="schema-label">variable</span></span>
   <span> : </span>
   <span class="schema-token schema-color-2">xpath "string(//h1)"<span class="schema-label">query</span></span>
 </div>
</div>

A capture consists of a variable name, followed by `:` and a query. Captures
section starts with `[Captures]`.


### Query

Queries are used to extract data from an HTTP response.

A query can be of the following type:

- [`status`](#status-capture)
- [`header`](#header-capture)
- [`url`](#url-capture)
- [`cookie`](#cookie-capture)
- [`body`](#body-capture)
- [`bytes`](#bytes-capture)
- [`xpath`](#xpath-capture)
- [`jsonpath`](#jsonpath-capture)
- [`regex`](#regex-capture)
- [`variable`](#variable-capture)
- [`duration`](#duration-capture)

Extracted data can then be further refined using [filters].

### Status capture

Capture the received HTTP response status code. Status capture consists of a variable name, followed by a `:`, and the
keyword `status`.

```hurl
GET https://example.org

HTTP 200
[Captures]
my_status: status
```

### Header capture

Capture a header from the received HTTP response headers. Header capture consists of a variable name, followed by a `:`,
then the keyword `header` and a header name.

```hurl
POST https://example.org/login
[FormParams]
user: toto
password: 12345678

HTTP 302
[Captures]
next_url: header "Location"
```

### URL capture

Capture the last fetched URL. This is most meaningful if you have told Hurl to follow redirection (see [`[Options]`section][options] or
[`--location` option]). URL capture consists of a variable name, followed by a `:`, and the keyword `url`.

```hurl
GET https://example.org/redirecting
[Options]
location: true

HTTP 200
[Captures]
landing_url: url
```

### Cookie capture

Capture a [`Set-Cookie`] header from the received HTTP response headers. Cookie
capture consists of a variable name, followed by a `:`, then the keyword `cookie`
and a cookie name.

```hurl
GET https://example.org/cookies/set

HTTP 200
[Captures]
session-id: cookie "LSID"
```

Cookie attributes value can also be captured by using the following format:
`<cookie-name>[cookie-attribute]`. The following attributes are supported:
`Value`, `Expires`, `Max-Age`, `Domain`, `Path`, `Secure`, `HttpOnly` and `SameSite`.

```hurl
GET https://example.org/cookies/set

HTTP 200
[Captures]
value1: cookie "LSID"
value2: cookie "LSID[Value]"     # Equivalent to the previous capture
expires: cookie "LSID[Expires]"
max-age: cookie "LSID[Max-Age]"
domain: cookie "LSID[Domain]"
path: cookie "LSID[Path]"
secure: cookie "LSID[Secure]"
http-only: cookie "LSID[HttpOnly]"
same-site: cookie "LSID[SameSite]"
```


### Body capture

Capture the entire body (decoded as text) from the received HTTP response

```hurl
GET https://example.org/home

HTTP 200
[Captures]
my_body: body
```

### Bytes capture

Capture the entire body (as a raw bytestream) from the received HTTP response

```hurl
GET https://example.org/data.bin

HTTP 200
[Captures]
my_data: bytes
```


### XPath capture

Capture a [XPath] query from the received HTTP body decoded as a string.
Currently, only XPath 1.0 expression can be used.

```hurl
GET https://example.org/home

# Capture the identifier from the dom node <div id="pet0">5646eaf23</div
HTTP 200
[Captures]
ped-id: xpath "normalize-space(//div[@id='pet0'])"

# Open the captured page.
GET https://example.org/home/pets/{{pet-id}}

HTTP 200
```

XPath captures are not limited to node values (like string, or boolean); any
valid XPath can be captured and asserted with variable asserts.

```hurl
# Test that the XML endpoint return 200 pets
GET https://example.org/api/pets

HTTP 200
[Captures]
pets: xpath "//pets"
[Asserts]
variable "pets" count == 200
```


### JSONPath capture

Capture a [JSONPath] query from the received HTTP body.

```hurl
POST https://example.org/api/contact
[FormParams]
token: {{token}}
email: toto@rookie.net

HTTP 200
[Captures]
contact-id: jsonpath "$['id']"
```

> Explain that the value selected by the JSONPath is coerced to a string when only one node is selected.

As with [XPath captures], JSONPath captures can be anything from string, number, to object and collections.
For instance, if we have a JSON endpoint that returns the following JSON:

```
{
  "a_null": null,
  "an_object": {
    "id": "123"
  },
  "a_list": [
    1,
    2,
    3
  ],
  "an_integer": 1,
  "a float": 1.1,
  "a_bool": true,
  "a_string": "hello"
}
```

We can capture the following paths:

```hurl
GET https://example.org/captures-json

HTTP 200
[Captures]
an_object:  jsonpath "$['an_object']"
a_list:     jsonpath "$['a_list']"
a_null:     jsonpath "$['a_null']"
an_integer: jsonpath "$['an_integer']"
a_float:    jsonpath "$['a_float']"
a_bool:     jsonpath "$['a_bool']"
a_string:   jsonpath "$['a_string']"
all:        jsonpath "$"
```


### Regex capture

Capture a regex pattern from the HTTP received body, decoded as text.

```hurl
GET https://example.org/helloworld

HTTP 200
[Captures]
id_a: regex "id_a:([0-9]+)"
id_b: regex "id_b:(\\d+)"   # pattern using double quote 
id_c: regex /id_c:(\d+)/    # pattern using forward slash
name: regex "Hello ([a-zA-Z]+)"
```

The regex pattern must have at least one capture group, otherwise the
capture will fail. When the pattern is a double-quoted string, metacharacters beginning with a backslash in the pattern
(like `\d`, `\s`) must be escaped; literal pattern enclosed by `/` can also be used to avoid metacharacters escaping. 


### Variable capture

Capture the value of a variable into another.

```hurl
GET https://example.org/helloworld

HTTP 200
[Captures]
in: body
name: variable "in"
```

### Duration capture

Capture the response time of the request in ms.

```hurl
GET https://example.org/helloworld

HTTP 200
[Captures]
duration_in_ms: duration
```


[CSRF tokens]: https://en.wikipedia.org/wiki/Cross-site_request_forgery
[injected into the session]: /docs/templates.md#injecting-variables
[`Set-Cookie`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
[XPath]: https://en.wikipedia.org/wiki/XPath
[JSONPath]: https://goessner.net/articles/JsonPath/
[XPath captures]: #xpath-capture
[Javascript-like Regular expression syntax]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions
[options]: /docs/request.md#options
[`--location` option]: /docs/manual.md#location
[filters]: /docs/filters.md