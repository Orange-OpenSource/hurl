# Asserting Response

## Asserts

Asserts are used to test various properties of an HTTP response. Asserts can be implicits (such as version, status, 
headers) or explicit within an `[Asserts]` section. The delimiter of the request / response is `HTTP <STATUS-CODE>`: 
after this delimiter, you'll find the implicit asserts, then an `[Asserts]` section with all the explicit checks.


```hurl
GET https://example.org/api/cats
HTTP 200
# Implicit assert on `Content-Type` Header
Content-Type: application/json; charset=utf-8 
[Asserts]
# Explicit asserts section 
bytes count == 120
header "Content-Type" contains "utf-8"
jsonpath "$.cats" count == 49
jsonpath "$.cats[0].name" == "Felix"
jsonpath "$.cats[0].lives" == 9
```

Body responses can be encoded by server (see [`Content-Encoding` HTTP header]) but asserts in Hurl files are not 
affected by this content compression. All body asserts (`body`, `bytes`, `sha256` etc...) work _after_ content decoding.

Finally, body text asserts (`body`, `jsonpath`, `xpath` etc...) are also decoded to strings based on [`Content-Type` header] 
so these asserts can be written with usual strings. 

## Implicit asserts

### Version - Status

Expected protocol version and status code of the HTTP response.

Protocol version is one of `HTTP/1.0`, `HTTP/1.1`, `HTTP/2`, `HTTP/3` or
`HTTP`; `HTTP` describes any version. Note that there are no status text following the status code.

```hurl
GET https://example.org/404.html
HTTP 404
```

Wildcard keywords `HTTP` and `*` can be used to disable tests on protocol version and status:

```hurl
GET https://example.org/api/pets
HTTP *
# Check that response status code is > 400 and <= 500
[Asserts]
status > 400
status <= 500
```

While `HTTP/1.0`, `HTTP/1.1`, `HTTP/2` and `HTTP/3` explicitly check HTTP version:

```hurl
# Check that our server responds with HTTP/2
GET https://example.org/api/pets
HTTP/2 200 
```


### Headers

Optional list of the expected HTTP response headers that must be in the received response.

A header consists of a name, followed by a `:` and a value.

For each expected header, the received response headers are checked. If the received header is not equal to the 
expected, or not present, an error is raised. The comparison is case-insensitive for the name: expecting a 
`Content-Type` header is equivalent to a `content-type` one. Note that the expected headers list is not fully 
descriptive: headers present in the response and not in the expected list doesn't raise error.

```hurl
# Check that user toto is redirected to home after login.
POST https://example.org/login
[FormParams]
user: toto
password: 12345678
HTTP 302
Location: https://example.org/home
```

> Quotes in the header value are part of the value itself.
>
> This is used by the [ETag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) Header
> ```
> ETag: W/"<etag_value>"
> ETag: "<etag_value>"
> ```


Testing duplicated headers is also possible.

For example with the `Set-Cookie` header:

```
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

You can either test the two header values:

```hurl
GET https://example.org/index.html
Host: example.net
HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

Or only one:

```hurl
GET https://example.org/index.html 
Host: example.net
HTTP 200
Set-Cookie: theme=light
```

If you want to test specifically the number of headers returned for a given header name, or
if you want to test header value with [predicates] (like `startsWith`, `contains`, `exists`)
you can use the explicit [header assert].


## Explicit asserts

Optional list of assertions on the HTTP response within an `[Asserts]` section. Assertions can describe checks
on status code, on the received body (or part of it) and on response headers.

Structure of an assert:

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">jsonpath "$.book"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">contains<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">"Dune"<span class="schema-label">predicate value</span></span>
 </div>
</div>

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">body<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">matches<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">/\d{4}-\d{2}-\d{2}/<span class="schema-label">predicate value</span></span>
 </div>
</div>


An assert consists of a query followed by a predicate. The format of the query
is shared with [captures], and can be one of :

- [`status`](#status-assert)
- [`version`](#version-assert)
- [`header`](#header-assert)
- [`url`](#url-assert)
- [`cookie`](#cookie-assert)
- [`body`](#body-assert)
- [`bytes`](#bytes-assert)
- [`xpath`](#xpath-assert)
- [`jsonpath`](#jsonpath-assert)
- [`regex`](#regex-assert)
- [`sha256`](#sha-256-assert)
- [`md5`](#md5-assert)
- [`variable`](#variable-assert)
- [`duration`](#duration-assert)
- [`certificate`](#ssl-certificate-assert)

Queries are used to extract data from the HTTP response. Queries, in asserts and in captures, can be refined with [filters], like 
[`count`][count] to add tests on collections sizes.


### Predicates

Predicates consist of a predicate function and a predicate value. Predicate functions are:

| Predicate          | Description                                                                         | Example                                                                               | 
|--------------------|-------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------|
| __`==`__           | Query and predicate value are equal                                                 | `jsonpath "$.book" == "Dune"`                                                         |
| __`!=`__           | Query and predicate value are different                                             | `jsonpath "$.color" != "red"`                                                         |
| __`>`__            | Query number is greater than predicate value                                        | `jsonpath "$.year" > 1978`                                                            |
| __`>=`__           | Query number is greater than or equal to the predicate value                        | `jsonpath "$.year" >= 1978`                                                           |
| __`<`__            | Query number is less than that predicate value                                      | `jsonpath "$.year" < 1978`                                                            |
| __`<=`__           | Query number is less than or equal to the predicate value                           | `jsonpath "$.year" <= 1978`                                                           |
| __`startsWith`__   | Query starts with the predicate value<br>Value is string or a binary content        | `jsonpath "$.movie" startsWith "The"`<br><br>`bytes startsWith hex,efbbbf;`           |
| __`endsWith`__     | Query ends with the predicate value<br>Value is string or a binary content          | `jsonpath "$.movie" endsWith "Back"`<br><br>`bytes endsWith hex,ab23456;`             |
| __`contains`__     | Query contains the predicate value<br>Value is string or a binary content           | `jsonpath "$.movie" contains "Empire"`<br><br>`bytes contains hex,beef;`              |
| __`includes`__     | Query collections includes the predicate value                                      | `jsonpath "$.nooks" includes "Dune"`                                                  |
| __`matches`__      | Part of the query string matches the regex pattern described by the predicate value | `jsonpath "$.release" matches "\\d{4}"`<br><br>`jsonpath "$.release" matches /\d{4}/` |
| __`exists`__       | Query returns a value                                                               | `jsonpath "$.book" exists`                                                            |
| __`isBoolean`__    | Query returns a boolean                                                             | `jsonpath "$.succeeded" isBoolean`                                                    |
| __`isCollection`__ | Query returns a collection                                                          | `jsonpath "$.books" isCollection`                                                     |
| __`isEmpty`__      | Query returns an empty collection                                                   | `jsonpath "$.movies" isEmpty`                                                         |
| __`isFloat`__      | Query returns a float                                                               | `jsonpath "$.height" isFloat`                                                         |
| __`isInteger`__    | Query returns an integer                                                            | `jsonpath "$.count" isInteger`                                                        |
| __`isIsoDate`__    | Query string returns a [RFC 3339] date (`YYYY-MM-DDTHH:mm:ss.sssZ`)                 | `jsonpath "$.publication_date" isIsoDate`                                             |
| __`isNumber`__     | Query returns an integer or a float                                                 | `jsonpath "$.count" isNumber`                                                         |
| __`isString`__     | Query returns a string                                                              | `jsonpath "$.name" isString`                                                          |


Each predicate can be negated by prefixing it with `not` (for instance, `not contains` or `not exists`)

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">jsonpath "$.book"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">not contains<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">"Dune"<span class="schema-label">predicate value</span></span>
 </div>
</div>


A predicate value is typed, and can be a string, a boolean, a number, a bytestream, `null` or a collection. Note that
`"true"` is a string, whereas `true` is a boolean.

For instance, to test the presence of a h1 node in an HTML response, the following assert can be used:

```hurl
GET https://example.org/home
HTTP 200
[Asserts]
xpath "boolean(count(//h1))" == true
xpath "//h1" exists # Equivalent but simpler
```

As the XPath query `boolean(count(//h1))` returns a boolean, the predicate value in the assert must be either
`true` or `false` without double quotes. On the other side, say you have an article node and you want to check the value of some
[data attributes]:

```xml
<article
  id="electric-cars"
  data-visible="true"
...
</article>
```

The following assert will check the value of the `data-visible` attribute:

```hurl
GET https://example.org/home
HTTP 200
[Asserts]
xpath "string(//article/@data-visible)" == "true"
```

In this case, the XPath query `string(//article/@data-visible)` returns a string, so the predicate value must be a
string.

The predicate function `==` can be used with string, numbers or booleans; `startWith` and `contains` can only
be used with strings and bytes, while `matches` only works on string. If a query returns a number, using a `matches` 
predicate will cause a runner error.

```hurl
# A really well tested web page...
GET https://example.org/home
HTTP 200
[Asserts]
header "Content-Type" contains "text/html"
header "Last-Modified" == "Wed, 21 Oct 2015 07:28:00 GMT"
xpath "//h1" exists  # Check we've at least one h1
xpath "normalize-space(//h1)" contains "Welcome"
xpath "//h2" count == 13
xpath "string(//article/@data-id)" startsWith "electric"
```

### Status assert

Check the received HTTP response status code. Status assert consists of the keyword `status` followed by a predicate
function and value.

```hurl
GET https://example.org
HTTP *
[Asserts]
status < 300
```

### Version assert

Check the received HTTP version. Version assert consists of the keyword `version` followed by a predicate
function and value.

```hurl
GET https://example.org
HTTP *
[Asserts]
version == "2"
```

### Header assert

Check the value of a received HTTP response header. Header assert consists of the keyword `header` followed by the value
of the header, a predicate function and a predicate value. Like [headers implicit asserts], the check is 
case-insensitive for the name: comparing a `Content-Type` header is equivalent to a `content-type` one.

```hurl
GET https://example.org
HTTP 302
[Asserts]
header "Location" contains "www.example.net"
header "Last-Modified" matches /\d{2} [a-z-A-Z]{3} \d{4}/
```

If there are multiple headers with the same name, the header assert returns a collection, so `count`, `includes` can be
used in this case to test the header list.

Let's say we have this request and response:

```
> GET /hello HTTP/1.1
> Host: example.org
> Accept: */*
> User-Agent: hurl/2.0.0-SNAPSHOT
>
* Response: (received 12 bytes in 11 ms)
*
< HTTP/1.0 200 OK
< Vary: Content-Type
< Vary: User-Agent
< Content-Type: text/html; charset=utf-8
< Content-Length: 12
< Server: Flask Server
< Date: Fri, 07 Oct 2022 20:53:35 GMT
```

One can use explicit header asserts:

```hurl
GET https://example.org/hello
HTTP 200
[Asserts]
header "Vary" count == 2
header "Vary" includes "User-Agent"
header "Vary" includes "Content-Type"
```

Or implicit header asserts:

```hurl
GET https://example.org/hello
HTTP 200
Vary: User-Agent
Vary: Content-Type
```

### URL assert

Check the last fetched URL. This is most meaningful if you have told Hurl to follow redirection (see [`[Options]`section][options] or
[`--location` option]). URL assert consists of the keyword `url` followed by a predicate function and value.

```hurl
GET https://example.org/redirecting
[Options]
location: true
HTTP 200
[Asserts]
url == "https://example.org/redirected"
```


### Cookie assert

Check value or attributes of a [`Set-Cookie`] response header. Cookie assert
consists of the keyword `cookie`, followed by the cookie name (and optionally a
cookie attribute), a predicate function and value.

Cookie attributes value can be checked by using the following format:
`<cookie-name>[cookie-attribute]`. The following attributes are supported: `Value`,
`Expires`, `Max-Age`, `Domain`, `Path`, `Secure`, `HttpOnly` and `SameSite`.

```hurl
GET http://localhost:8000/cookies/set
HTTP 200

# Explicit check of Set-Cookie header value. If the attributes are
# not in this exact order, this assert will fail. 
Set-Cookie: LSID=DQAAAKEaem_vYg; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/accounts; SameSite=Lax;
Set-Cookie: HSID=AYQEVnDKrdst; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly; Path=/
Set-Cookie: SSID=Ap4PGTEq; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/

# Using cookie assert, one can check cookie value and various attributes.
[Asserts]
cookie "LSID" == "DQAAAKEaem_vYg"
cookie "LSID[Value]" == "DQAAAKEaem_vYg"
cookie "LSID[Expires]" exists
cookie "LSID[Expires]" contains "Wed, 13 Jan 2021"
cookie "LSID[Max-Age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[Secure]" exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[SameSite]" == "Lax"
```

> `Secure` and `HttpOnly` attributes can only be tested with `exists` or `not exists` predicates
> to reflect the [Set-Cookie header] semantics (in other words, queries `<cookie-name>[HttpOnly]`
> and `<cookie-name>[Secure]` don't return boolean).

### Body assert

Check the value of the received HTTP response body when decoded as a string. Body assert consists of the keyword `body` 
followed by a predicate function and value.

```hurl
GET https://example.org
HTTP 200
[Asserts]
body contains "<h1>Welcome!</h1>"
```

The encoding used to decode the response body bytes to a string is based on the `charset` value in the `Content-Type` 
header response.

```hurl
# Our HTML response is encoded with GB 2312 (see https://en.wikipedia.org/wiki/GB_2312)
GET https://example.org/cn
HTTP 200
[Asserts]
header "Content-Type" == "text/html; charset=gb2312"
# bytes of the response, without any text decoding:
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB 2312
# text of the response, decoded with GB 2312:
body contains "你好世界"
```

If the `Content-Type` response header doesn't include any encoding hint, a [`decode` filter] can be used to explicitly 
decode the response body bytes.

```hurl
# Our HTML response is encoded using GB 2312.
# But, the 'Content-Type' HTTP response header doesn't precise any charset,
# so we decode explicitly the bytes.
GET https://example.org/cn
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB2312
bytes decode "gb2312" contains "你好世界"
```

Body asserts are automatically decompressed based on the value of `Content-Encoding` response header. So,
whatever is the response compression (`gzip`, `brotli`) etc... asserts values don't depend on the content encoding.

```hurl
# Request a gzipped reponse, the `body` asserts works with ungzipped response
GET https://example.org
Accept-Encoding: gzip
HTTP 200
[Asserts]
header "Content-Encoding" == "gzip"
body contains "<h1>Welcome!</h1>"

# Without content encoding, asserts remains identical
GET https://example.org
HTTP 200
[Asserts]
header "Content-Encoding" not exists
body contains "<h1>Welcome!</h1>"
```


### Bytes assert

Check the value of the received HTTP response body as a bytestream. Body assert consists of the keyword `bytes` 
followed by a predicate function and value.

```hurl
GET https://example.org/data.bin
HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
bytes count == 12424
header "Content-Length" == "12424"
```

Like `body` assert, `bytes` assert works _after_ content encoding decompression (so the predicates values are not
affected by `Content-Encoding` response header value).


### XPath assert

Check the value of a [XPath] query on the received HTTP body decoded as a string (using the `charset` value in the
`Content-Type` header response). Currently, only XPath 1.0 expression can be used. Body assert consists of the
keyword `xpath` followed by a predicate function and value. Values can be string,
boolean or number depending on the XPath query.

Let's say we want to check this HTML response:

```plain
$ curl -v https://example.org

< HTTP/1.1 200 OK
< Content-Type: text/html; charset=UTF-8
...
<!doctype html>
<html>
  <head>
    <title>Example Domain</title>
    ...
  </head>

  <body>
    <div>
      <h1>Example</h1>
      <p>This domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.</p>
      <p><a href="https://www.iana.org/domains/example">More information...</a></p>
    </div>
  </body>
</html>
```

With Hurl, we can write multiple XPath asserts describing the DOM content:

```hurl
GET https://example.org
HTTP 200
Content-Type: text/html; charset=UTF-8
[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" == 2                             # Check the number of <p>
xpath "//p" count == 2                              # Similar assert for <p>
xpath "boolean(count(//h2))" == false               # Check there is no <h2>  
xpath "//h2" not exists                             # Similar assert for <h2> 
```

XML Namespaces are also supported. Let's say you want to check this XML response:

```xml
<?xml version="1.0"?>
<!-- both namespace prefixes are available throughout -->
<bk:book xmlns:bk='urn:loc.gov:books'
         xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <bk:title>Cheaper by the Dozen</bk:title>
    <isbn:number>1568491379</isbn:number>
</bk:book>
```

This XML response can be tested with the following Hurl file:

```hurl
GET http://localhost:8000/assert-xpath
HTTP 200
[Asserts]

xpath "string(//bk:book/bk:title)" == "Cheaper by the Dozen"
xpath "string(//*[name()='bk:book']/*[name()='bk:title'])" == "Cheaper by the Dozen"
xpath "string(//*[local-name()='book']/*[local-name()='title'])" == "Cheaper by the Dozen"

xpath "string(//bk:book/isbn:number)" == "1568491379"
xpath "string(//*[name()='bk:book']/*[name()='isbn:number'])" == "1568491379"
xpath "string(//*[local-name()='book']/*[local-name()='number'])" == "1568491379"
```

The XPath expressions `string(//bk:book/bk:title)` and `string(//bk:book/isbn:number)` are written with `bk` and `isbn`
namespaces.

> For convenience, the first default namespace can be used with `_`


### JSONPath assert

Check the value of a [JSONPath] query on the received HTTP body decoded as a JSON
document. JSONPath assert consists of the keyword `jsonpath` followed by a predicate
function and value.

Let's say we want to check this JSON response:

```plain
curl -v http://httpbin.org/json

< HTTP/1.1 200 OK
< Content-Type: application/json
...

{
  "slideshow": {
    "author": "Yours Truly",
    "date": "date of publication",
    "slides": [
      {
        "title": "Wake up to WonderWidgets!",
        "type": "all"
      },
       ...
    ],
    "title": "Sample Slide Show"
  }
}
```

With Hurl, we can write multiple JSONPath asserts describing the DOM content:


```hurl
GET http://httpbin.org/json
HTTP 200
[Asserts]
jsonpath "$.slideshow.author" == "Yours Truly"
jsonpath "$.slideshow.slides[0].title" contains "Wonder"
jsonpath "$.slideshow.slides" count == 2
jsonpath "$.slideshow.date" != null
jsonpath "$.slideshow.slides[*].title" includes "Mind Blowing!"
```

> Explain that the value selected by the JSONPath is coerced to a string when only
> one node is selected.

In `matches` predicates, metacharacters beginning with a backslash (like `\d`, `\s`) must be escaped.
Alternatively, `matches` predicate support [JavaScript-like Regular expression syntax] to enhance
the readability:

```hurl
GET https://example.org/hello
HTTP 200
[Asserts]

# Predicate value with matches predicate:
jsonpath "$.date" matches "^\\d{4}-\\d{2}-\\d{2}$"
jsonpath "$.name" matches "Hello [a-zA-Z]+!"

# Equivalent syntax:
jsonpath "$.date" matches /^\d{4}-\d{2}-\d{2}$/
jsonpath "$.name" matches /Hello [a-zA-Z]+!/
```

### Regex assert

Check that the HTTP received body, decoded as text, matches a regex pattern.

```hurl
GET https://example.org/hello
HTTP 200
[Asserts]
regex "^(\\d{4}-\\d{2}-\\d{2})$" == "2018-12-31"
# Same assert as previous using regex literals
regex /^(\d{4}-\d{2}-\d{2})$/ == "2018-12-31"
```

The regex pattern must have at least one capture group, otherwise the
assert will fail. The assertion is done on the captured group value. When the regex pattern is a double-quoted string, 
metacharacters beginning with a backslash in the pattern (like `\d`, `\s`) must be escaped; literal pattern enclosed by
`/` can also be used to avoid metacharacters escaping.


### SHA-256 assert

Check response body [SHA-256] hash.

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

Like `body` assert, `sha256` assert works _after_ content encoding decompression (so the predicates values are not
affected by `Content-Encoding` response header). For instance, if we have a resource `a.txt` on a server with a 
given hash `abcdef`, `sha256` value is not affected by `Content-Encoding`:

```hurl
# Without content encoding compression:
GET https://example.org/a.txt
HTTP 200
[Asserts]
sha256 == hex,abcdef;

# With content encoding compression:
GET https://example.org/a.txt
Accept-Encoding: brotli
HTTP 200
[Asserts]
header "Content-Encoding" == "brotli"
sha256 == hex,abcdef;
```

### MD5 assert

Check response body [MD5] hash.

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```

Like `sha256` asserts, `md5` assert works _after_ content encoding decompression (so the predicates values are not
affected by `Content-Encoding` response header)

### Variable assert

```hurl
# Test that the XML endpoint return 200 pets 
GET https://example.org/api/pets
HTTP 200
[Captures]
pets: xpath "//pets"
[Asserts]
variable "pets" count == 200
```

### Duration assert

Check the total duration (sending plus receiving time) of the HTTP transaction.

```hurl
GET https://example.org/helloworld
HTTP 200
[Asserts]
duration < 1000   # Check that response time is less than one second
```

### SSL certificate assert

Check the SSL certificate properties. Certificate assert consists of the keyword `certificate`, followed by the certificate attribute value.

The following attributes are supported: `Subject`, `Issuer`, `Start-Date`, `Expire-Date` and `Serial-Number`.

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Subject" == "CN=example.org"
certificate "Issuer" == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches "[0-9af]+"
```

## Body

Optional assertion on the received HTTP response body. Body section can be seen
as syntactic sugar over [body asserts] (with `==` predicate). If the
body of the response is a [JSON] string or a [XML] string, the body assertion can
be directly inserted without any modification. For a text based body that is neither JSON nor XML,
one can use multiline string that starts with <code>&#96;&#96;&#96;</code> and ends
with <code>&#96;&#96;&#96;</code>. For a precise byte control of the response body,
a [Base64] encoded string or an input file can be used to describe exactly
the body byte content to check.

Like explicit [`body` assert], the body section is automatically decompressed based on the value of `Content-Encoding` 
response header. So, whatever is the response compression (`gzip`, `brotli`, etc...) body section doesn't depend on
the content encoding. For textual body sections (JSON, XML, multiline, etc...), content is also decoded to string, based
on the value of `Content-Type` response header.

### JSON body

```hurl
# Get a doggy thing:
GET https://example.org/api/dogs/{{dog-id}}
HTTP 200
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```

JSON response body can be seen as syntactic sugar of [multiline string body] with `json` identifier:

~~~hurl
# Get a doggy thing:
GET https://example.org/api/dogs/{{dog-id}}
HTTP 200
```json
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```
~~~


### XML body

~~~hurl
GET https://example.org/api/catalog
HTTP 200
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <book id="bk101">
      <author>Gambardella, Matthew</author>
      <title>XML Developer's Guide</title>
      <genre>Computer</genre>
      <price>44.95</price>
      <publish_date>2000-10-01</publish_date>
      <description>An in-depth look at creating applications with XML.</description>
   </book>
</catalog>
~~~

XML response body can be seen as syntactic sugar of [multiline string body] with `xml` identifier:

~~~hurl
GET https://example.org/api/catalog
HTTP 200
```xml
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <book id="bk101">
      <author>Gambardella, Matthew</author>
      <title>XML Developer's Guide</title>
      <genre>Computer</genre>
      <price>44.95</price>
      <publish_date>2000-10-01</publish_date>
      <description>An in-depth look at creating applications with XML.</description>
   </book>
</catalog>
```
~~~


### Multiline string body

~~~hurl
GET https://example.org/models
HTTP 200
```
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL! air, moon roof, loaded",4799.00
```
~~~

The standard usage of a multiline string is :

~~~
```
line1
line2
line3
```
~~~

#### Oneline string body

For text based response body that do not contain newlines, one can use oneline string, started and ending with <code>&#96;</code>.

~~~hurl
POST https://example.org/helloworld
HTTP 200
`Hello world!`
~~~


### Base64 body

Base64 response body assert starts with `base64,` and end with `;`. MIME's Base64 encoding
is supported (newlines and white spaces may be present anywhere but are to be
ignored on decoding), and `=` padding characters might be added.

```hurl
GET https://example.org
HTTP 200
base64,TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIG
FkaXBpc2NpbmcgZWxpdC4gSW4gbWFsZXN1YWRhLCBuaXNsIHZlbCBkaWN0dW0g
aGVuZHJlcml0LCBlc3QganVzdG8gYmliZW5kdW0gbWV0dXMsIG5lYyBydXRydW
0gdG9ydG9yIG1hc3NhIGlkIG1ldHVzLiA=;
```

### File body

To use the binary content of a local file as the body response assert, file body
can be used. File body starts with `file,` and ends with `;``

```hurl
GET https://example.org
HTTP 200
file,data.bin;
```

File are relative to the input Hurl file, and cannot contain implicit parent
directory (`..`). You can use [`--file-root` option] to specify the root directory
of all file nodes.


[predicates]: #predicates
[header assert]: #header-assert
[captures]: /docs/capturing-response.md#query
[data attributes]: https://developer.mozilla.org/en-US/docs/Learn/HTML/Howto/Use_data_attributes
[`Set-Cookie`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
[Set-Cookie header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
[XPath]: https://en.wikipedia.org/wiki/XPath
[JSONPath]: https://goessner.net/articles/JsonPath/
[body asserts]: #body-assert
[JSON]: https://www.json.org
[XML]: https://en.wikipedia.org/wiki/XML
[Base64]: https://en.wikipedia.org/wiki/Base64
[`--file-root` option]: /docs/manual.md#file-root
[JavaScript-like Regular expression syntax]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions
[MD5]: https://en.wikipedia.org/wiki/MD5
[SHA-256]: https://en.wikipedia.org/wiki/SHA-2
[options]: /docs/request.md#options
[`--location` option]: /docs/manual.md#location
[multiline string body]: #multiline-string-body
[filters]: /docs/filters.md
[count]: /docs/filters.md#count
[`decode` filter]: /docs/filters.md#decode
[headers implicit asserts]: #headers
[RFC 3339]: https://www.rfc-editor.org/rfc/rfc3339
[`Content-Encoding` HTTP header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding
[`Content-Type` header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type
[`body` assert]: #body-assert