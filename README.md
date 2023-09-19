<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-full-dark.svg?sanitize=true" > 
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-full-light.svg?sanitize=true" > 
    <img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-full-light.svg?sanitize=true" alt="Hurl Logo" width="264px">
</picture>
<br/>

[![deploy status](https://github.com/Orange-OpenSource/hurl/workflows/test/badge.svg)](https://github.com/Orange-OpenSource/hurl/actions)
[![coverage](https://Orange-OpenSource.github.io/hurl/coverage/badges/flat.svg)](https://Orange-OpenSource.github.io/hurl/coverage)
[![Crates.io](https://img.shields.io/crates/v/hurl.svg)](https://crates.io/crates/hurl)
[![documentation](https://img.shields.io/badge/-documentation-ff0288)](https://hurl.dev)

# What's Hurl?

Hurl is a command line tool that runs <b>HTTP requests</b> defined in a simple <b>plain text format</b>.

It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very
versatile: it can be used for both <b>fetching data</b> and <b>testing HTTP</b> sessions.

Hurl makes it easy to work with <b>HTML</b> content, <b>REST / SOAP / GraphQL</b> APIs, or any other <b>XML / JSON</b> based APIs. 

```hurl
# Get home:
GET https://example.org

HTTP 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"


# Do login!
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

Chaining multiple requests is easy:

```hurl
GET https://example.org/api/health
GET https://example.org/api/step1
GET https://example.org/api/step2
GET https://example.org/api/step3
```

# Also an HTTP Test Tool

Hurl can run HTTP requests but can also be used to <b>test HTTP responses</b>.
Different types of queries and predicates are supported, from [XPath] and [JSONPath] on body response,
to assert on status code and response headers.

<a href="https://hurl.dev/player.html?id=hurl&speed=3"><img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/docs/assets/img/poster-hurl.png" width="100%" alt="Hurl Demo"/></a>

It is well adapted for <b>REST / JSON APIs</b>

```hurl
POST https://example.org/api/tests
{
    "id": "4568",
    "evaluate": true
}

HTTP 200
[Asserts]
header "X-Frame-Options" == "SAMEORIGIN"
jsonpath "$.status" == "RUNNING"    # Check the status code
jsonpath "$.tests" count == 25      # Check the number of items
jsonpath "$.id" matches /\d{4}/     # Check the format of the id
```

<b>HTML content</b>

```hurl
GET https://example.org

HTTP 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"
```

<b>GraphQL</b> 

~~~hurl
POST https://example.org/graphql
```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```
HTTP 200
~~~

and even <b>SOAP APIs</b>

```hurl
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="https://example.org">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
HTTP 200
```

Hurl can also be used to test the <b>performance</b> of HTTP endpoints

```hurl
GET https://example.org/api/v1/pets

HTTP 200
[Asserts]
duration < 1000  # Duration in ms
```

And check response bytes

```hurl
GET https://example.org/data.tar.gz

HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

Finally, Hurl is easy to <b>integrate in CI/CD</b>, with text, JUnit and HTML reports

<div class="picture">
    <picture>
        <source media="(prefers-color-scheme: light)" srcset="/docs/assets/img/home-waterfall-light.avif" type="image/avif">
        <source media="(prefers-color-scheme: light)" srcset="/docs/assets/img/home-waterfall-light.webp" type="image/webp">
        <source media="(prefers-color-scheme: light)" srcset="/docs/assets/img/home-waterfall-light.png" type="image/png">
        <source media="(prefers-color-scheme: dark)" srcset="/docs/assets/img/home-waterfall-dark.avif" type="image/avif">
        <source media="(prefers-color-scheme: dark)" srcset="/docs/assets/img/home-waterfall-dark.webp" type="image/webp">
        <source media="(prefers-color-scheme: dark)" srcset="/docs/assets/img/home-waterfall-dark.png" type="image/png">
        <img class="light-img u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/home-waterfall-light.png" width="480" alt="HTML report"/>
    </picture>
</div>

# Why Hurl?

<ul class="showcase-container">
    <li><b>Text Format:</b> for both devops and developers</li>
    <li><b>Fast CLI:</b> a command line for local dev and continuous integration</li>
    <li><b>Single Binary:</b> easy to install, with no runtime required</li>
</ul>

# Powered by curl

Hurl is a lightweight binary written in [Rust]. Under the hood, Hurl HTTP engine is
powered by [libcurl], one of the most powerful and reliable file transfer libraries.
With its text file format, Hurl adds syntactic sugar to run and test HTTP requests,
but it's still the [curl] that we love.

# Feedbacks

To support its development, [star Hurl on GitHub]!

[Feedback, suggestion, bugs or improvements] are welcome.

```hurl
POST https://hurl.dev/api/feedback
{
  "name": "John Doe",
  "feedback": "Hurl is awesome!"
}
HTTP 200
```

# Resources

[License]

[Blog]

[Tutorial]

[Documentation]

[GitHub]

Table of Contents
=================
   * [Samples](#samples)
      * [Getting Data](#getting-data)
         * [HTTP Headers](#http-headers)
         * [Query Params](#query-params)
         * [Basic Authentication](#basic-authentication)
      * [Sending Data](#sending-data)
         * [Sending HTML Form Data](#sending-html-form-data)
         * [Sending Multipart Form Data](#sending-multipart-form-data)
         * [Posting a JSON Body](#posting-a-json-body)
         * [Templating a JSON Body](#templating-a-json-body)
         * [Templating a XML Body](#templating-a-xml-body)
         * [Using GraphQL Query](#using-graphql-query)
      * [Testing Response](#testing-response)
         * [Testing Response Headers](#testing-response-headers)
         * [Testing REST APIs](#testing-rest-apis)
         * [Testing HTML Response](#testing-html-response)
         * [Testing Set-Cookie Attributes](#testing-set-cookie-attributes)
         * [Testing Bytes Content](#testing-bytes-content)
         * [SSL Certificate](#ssl-certificate)
      * [Others](#others)
         * [HTTP Version](#http-version)
         * [Polling and Retry](#polling-and-retry)
         * [Testing Endpoint Performance](#testing-endpoint-performance)
         * [Using SOAP APIs](#using-soap-apis)
         * [Capturing and Using a CSRF Token](#capturing-and-using-a-csrf-token)
         * [Checking Byte Order Mark (BOM) in Response Body](#checking-byte-order-mark-bom-in-response-body)
         * [AWS Signature Version 4 Requests](#aws-signature-version-4-requests)
   * [Manual](#manual)
      * [Name](#name)
      * [Synopsis](#synopsis)
      * [Description](#description)
      * [Hurl File Format](#hurl-file-format)
         * [Capturing values](#capturing-values)
         * [Asserts](#asserts)
      * [Options](#options)
      * [Environment](#environment)
      * [Exit Codes](#exit-codes)
      * [WWW](#www)
      * [See Also](#see-also)
   * [Installation](#installation)
      * [Binaries Installation](#binaries-installation)
         * [Linux](#linux)
            * [Debian / Ubuntu](#debian--ubuntu)
            * [Alpine](#alpine)
            * [Arch Linux / Manjaro](#arch-linux--manjaro)
            * [NixOS / Nix](#nixos--nix)
         * [macOS](#macos)
            * [Homebrew](#homebrew)
            * [MacPorts](#macports)
         * [FreeBSD](#freebsd)
         * [Windows](#windows)
            * [Zip File](#zip-file)
            * [Installer](#installer)
            * [Chocolatey](#chocolatey)
            * [Scoop](#scoop)
            * [Windows Package Manager](#windows-package-manager)
         * [Cargo](#cargo)
         * [Docker](#docker)
         * [npm](#npm)
      * [Building From Sources](#building-from-sources)
         * [Build on Linux](#build-on-linux)
            * [Debian based distributions](#debian-based-distributions)
            * [Red Hat based distributions](#red-hat-based-distributions)
            * [Arch based distributions](#arch-based-distributions)
         * [Build on macOS](#build-on-macos)
         * [Build on Windows](#build-on-windows)
# Samples

To run a sample, edit a file with the sample content, and run Hurl:

```shell
$ vi sample.hurl

GET https://example.org

$ hurl sample.hurl
```

By default, Hurl behaves like [curl] and outputs the last HTTP response's [entry]. To have a test
oriented output, you can use [`--test` option]:

```shell
$ hurl --test sample.hurl
```


You can check [Hurl tests suite] for more samples.

## Getting Data

A simple GET:

```hurl
GET https://example.org
```

[Doc](https://hurl.dev/docs/request.html#method)

### HTTP Headers

A simple GET with headers:

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 
Accept: */*
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
```

[Doc](https://hurl.dev/docs/request.html#headers)

### Query Params

```hurl
GET https://example.org/news
[QueryStringParams]
order: newest
search: something to search
count: 100
```

Or:

```hurl
GET https://example.org/news?order=newest&search=something%20to%20search&count=100
```

[Doc](https://hurl.dev/docs/request.html#query-parameters)

### Basic Authentication

```hurl
GET https://example.org/protected
[BasicAuth]
bob: secret
```

[Doc](https://hurl.dev/docs/request.html#basic-authentication)

This is equivalent to construct the request with a [Authorization] header:

```hurl
# Authorization header value can be computed with `echo -n 'bob:secret' | base64`
GET https://example.org/protected
Authorization: Basic Ym9iOnNlY3JldA== 
```

Basic authentication allows per request authentication.
If you want to add basic authentication to all the requests of a Hurl file
you could use [`-u/--user` option].

## Sending Data

### Sending HTML Form Data

```hurl
POST https://example.org/contact
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

[Doc](https://hurl.dev/docs/request.html#form-parameters)

### Sending Multipart Form Data

```hurl
POST https://example.org/upload
[MultipartFormData]
field1: value1
field2: file,example.txt;
# One can specify the file content type:
field3: file,example.zip; application/zip
```

[Doc](https://hurl.dev/docs/request.html#multipart-form-data)

Multipart forms can also be sent with a [multiline string body]:

~~~hurl
POST https://example.org/upload
Content-Type: multipart/form-data; boundary="boundary"
```
--boundary
Content-Disposition: form-data; name="key1"

value1
--boundary
Content-Disposition: form-data; name="upload1"; filename="data.txt"
Content-Type: text/plain

Hello World!
--boundary
Content-Disposition: form-data; name="upload2"; filename="data.html"
Content-Type: text/html

<div>Hello <b>World</b>!</div>
--boundary--
```
~~~

In that case, files have to be inlined in the Hurl file.

[Doc](https://hurl.dev/docs/request.html#multiline-string-body)



### Posting a JSON Body

With an inline JSON:

```hurl
POST https://example.org/api/tests
{
    "id": "456",
    "evaluate": true
}
```

[Doc](https://hurl.dev/docs/request.html#json-body)

With a local file:

```hurl
POST https://example.org/api/tests
Content-Type: application/json
file,data.json;
```

[Doc](https://hurl.dev/docs/request.html#file-body)

### Templating a JSON Body

```hurl
PUT https://example.org/api/hits
Content-Type: application/json
{
    "key0": "{{a_string}}",
    "key1": {{a_bool}},
    "key2": {{a_null}},
    "key3": {{a_number}}
}
```

Variables can be initialized via command line:

```shell
$ hurl --variable a_string=apple \
       --variable a_bool=true \
       --variable a_null=null \
       --variable a_number=42 \
       test.hurl
```

Resulting in a PUT request with the following JSON body:

```
{
    "key0": "apple",
    "key1": true,
    "key2": null,
    "key3": 42
}
```

[Doc](https://hurl.dev/docs/templates.html)

### Templating a XML Body

Using templates with [XML body] is not currently supported in Hurl. You can use templates in
[XML multiline string body] with variables to send a variable XML body:

~~~hurl
POST https://example.org/echo/post/xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<Request>
    <Login>{{login}}</Login>
    <Password>{{password}}</Password>
</Request>
```
~~~

[Doc](https://hurl.dev/docs/request.html#multiline-string-body)

### Using GraphQL Query

A simple GraphQL query:

~~~hurl
POST https://example.org/starwars/graphql
```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```
~~~

A GraphQL query with variables:

~~~hurl
POST https://example.org/starwars/graphql
```graphql
query Hero($episode: Episode, $withFriends: Boolean!) {
  hero(episode: $episode) {
    name
    friends @include(if: $withFriends) {
      name
    }
  }
}

variables {
  "episode": "JEDI",
  "withFriends": false
}
```
~~~

GraphQL queries can also use [Hurl templates].

[Doc](https://hurl.dev/docs/request.html#graphql-body)

## Testing Response

### Testing Response Headers

Use implicit response asserts to test header values:

```hurl
GET https://example.org/index.html

HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

[Doc](https://hurl.dev/docs/asserting-response.html#headers)


Or use explicit response asserts with [predicates]:

```hurl
GET https://example.org

HTTP 302
[Asserts]
header "Location" contains "www.example.net"
```

[Doc](https://hurl.dev/docs/asserting-response.html#header-assert)


### Testing REST APIs

Asserting JSON body response (node values, collection count etc...) with [JSONPath]:

```hurl
GET https://example.org/order
screencapability: low

HTTP 200
[Asserts]
jsonpath "$.validated" == true
jsonpath "$.userInfo.firstName" == "Franck"
jsonpath "$.userInfo.lastName" == "Herbert"
jsonpath "$.hasDevice" == false
jsonpath "$.links" count == 12
jsonpath "$.state" != null
jsonpath "$.order" matches "^order-\\d{8}$"
jsonpath "$.order" matches /^order-\d{8}$/     # Alternative syntax with regex literal
```

[Doc](https://hurl.dev/docs/asserting-response.html#jsonpath-assert)


Testing status code:

```hurl
GET https://example.org/order/435

HTTP 200
```

[Doc](https://hurl.dev/docs/asserting-response.html#version-status)

```hurl
GET https://example.org/order/435

# Testing status code is in a 200-300 range
HTTP *
[Asserts]
status >= 200
status < 300
```

[Doc](https://hurl.dev/docs/asserting-response.html#status-assert)


### Testing HTML Response

```hurl
GET https://example.org

HTTP 200
Content-Type: text/html; charset=UTF-8

[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" == 2  # Check the number of p
xpath "//p" count == 2  # Similar assert for p
xpath "boolean(count(//h2))" == false  # Check there is no h2  
xpath "//h2" not exists  # Similar assert for h2
xpath "string(//div[1])" matches /Hello.*/
```

[Doc](https://hurl.dev/docs/asserting-response.html#xpath-assert)

### Testing Set-Cookie Attributes

```hurl
GET https://example.org/home

HTTP 200
[Asserts]
cookie "JSESSIONID" == "8400BAFE2F66443613DC38AE3D9D6239"
cookie "JSESSIONID[Value]" == "8400BAFE2F66443613DC38AE3D9D6239"
cookie "JSESSIONID[Expires]" contains "Wed, 13 Jan 2021"
cookie "JSESSIONID[Secure]" exists
cookie "JSESSIONID[HttpOnly]" exists
cookie "JSESSIONID[SameSite]" == "Lax"
```

[Doc](https://hurl.dev/docs/asserting-response.html#cookie-assert)

### Testing Bytes Content

Check the SHA-256 response body hash:

```hurl
GET https://example.org/data.tar.gz

HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

[Doc](https://hurl.dev/docs/asserting-response.html#sha-256-assert)

### SSL Certificate

Check the properties of a SSL certificate:

```hurl
GET https://example.org

HTTP 200
[Asserts]
certificate "Subject" == "CN=example.org"
certificate "Issuer" == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches /[\da-f]+/
```

[Doc](https://hurl.dev/docs/asserting-response.html#ssl-certificate-assert)


## Others

### HTTP Version

Testing HTTP version (1.0, 1.1 or 2):

```hurl
GET https://example.org/order/435
HTTP/2 200
```

[Doc](https://hurl.dev/docs/asserting-response.html#version-status)

### Polling and Retry

Retry request on any errors (asserts, captures, status code, runtime etc...):

```hurl
# Create a new job
POST https://api.example.org/jobs

HTTP 201
[Captures]
job_id: jsonpath "$.id"
[Asserts]
jsonpath "$.state" == "RUNNING"


# Pull job status until it is completed
GET https://api.example.org/jobs/{{job_id}}
[Options]
retry: 10   # maximum number of retry, -1 for unlimited

HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"
```

[Doc](https://hurl.dev/docs/entry.html#retry)



### Testing Endpoint Performance

```hurl
GET https://sample.org/helloworld

HTTP *
[Asserts]
duration < 1000   # Check that response time is less than one second
```

[Doc](https://hurl.dev/docs/asserting-response.html#duration-assert)

### Using SOAP APIs

```hurl
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="https://example.org">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>

HTTP 200
```

[Doc](https://hurl.dev/docs/request.html#xml-body)

### Capturing and Using a CSRF Token

```hurl
GET https://example.org

HTTP 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"


POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

[Doc](https://hurl.dev/docs/capturing-response.html#xpath-capture)

### Checking Byte Order Mark (BOM) in Response Body

```hurl
GET https://example.org/data.bin

HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
```

[Doc](https://hurl.dev/docs/asserting-response.html#bytes-assert)

### AWS Signature Version 4 Requests

Generate signed API requests with [AWS Signature Version 4], as used by several cloud providers.

```hurl
POST https://sts.eu-central-1.amazonaws.com/
[Options]
aws-sigv4: aws:amz:eu-central-1:sts
[FormParams]
Action: GetCallerIdentity
Version: 2011-06-15
```

The Access Key is given per [`--user`]. 

[Doc](https://hurl.dev/docs/manual.html#aws-sigv4)


# Manual

## Name

hurl - run and test HTTP requests.


## Synopsis

**hurl** [options] [FILE...]


## Description

**Hurl** is a command line tool that runs HTTP requests defined in a simple plain text format.

It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very versatile, it can be used for fetching data and testing HTTP sessions: HTML content, REST / SOAP / GraphQL APIs, or any other XML / JSON based APIs.

```shell
$ hurl session.hurl
```

If no input files are specified, input is read from stdin.

```shell
$ echo GET http://httpbin.org/get | hurl
    {
      "args": {},
      "headers": {
        "Accept": "*/*",
        "Accept-Encoding": "gzip",
        "Content-Length": "0",
        "Host": "httpbin.org",
        "User-Agent": "hurl/0.99.10",
        "X-Amzn-Trace-Id": "Root=1-5eedf4c7-520814d64e2f9249ea44e0"
      },
      "origin": "1.2.3.4",
      "url": "http://httpbin.org/get"
    }
```


Output goes to stdout by default. To have output go to a file, use the [`-o, --output`](#output) option:

```shell
$ hurl -o output input.hurl
```

By default, Hurl executes all HTTP requests and outputs the response body of the last HTTP call.

To have a test oriented output, you can use [`--test`](#test) option:

```shell
$ hurl --test *.hurl
```


## Hurl File Format

The Hurl file format is fully documented in [https://hurl.dev/docs/hurl-file.html](https://hurl.dev/docs/hurl-file.html)

It consists of one or several HTTP requests

```hurl
GET http://example.org/endpoint1
GET http://example.org/endpoint2
```


### Capturing values

A value from an HTTP response can be-reused for successive HTTP requests.

A typical example occurs with CSRF tokens.

```hurl
GET https://example.org
HTTP 200
# Capture the CSRF token value from html body.
[Captures]
csrf_token: xpath "normalize-space(//meta[@name='_csrf_token']/@content)"

# Do the login !
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
```

More information on captures can be found here [https://hurl.dev/docs/capturing-response.html](https://hurl.dev/docs/capturing-response.html)

### Asserts

The HTTP response defined in the Hurl file are used to make asserts. Responses are optional.

At the minimum, response includes assert on the HTTP status code.

```hurl
GET http://example.org
HTTP 301
```

It can also include asserts on the response headers

```hurl
GET http://example.org
HTTP 301
Location: http://www.example.org
```

Explicit asserts can be included by combining a query and a predicate

```hurl
GET http://example.org
HTTP 301
[Asserts]
xpath "string(//title)" == "301 Moved"
```

With the addition of asserts, Hurl can be used as a testing tool to run scenarios.

More information on asserts can be found here [https://hurl.dev/docs/asserting-response.html](https://hurl.dev/docs/asserting-response.html)

## Options

Options that exist in curl have exactly the same semantics.

Options specified on the command line are defined for every Hurl file's entry.

For instance:

```shell
$ hurl --location foo.hurl
```

will follow redirection for each entry in `foo.hurl`. You can also define an option only for a particular entry with an `[Options]` section. For instance, this Hurl file:

```hurl
GET https://example.org
HTTP 301

GET https://example.org
[Options]
location: true
HTTP 200
```

will follow a redirection only for the second entry.

| Option                                                                                                            | Description                                                                                                                                                                                                                                                                                                                                                                                        |
|-------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <a href="#aws-sigv4" id="aws-sigv4"><code>--aws-sigv4 &lt;PROVIDER1[:PROVIDER2[:REGION[:SERVICE]]]&gt;</code></a> | Generate an `Authorization` header with an AWS SigV4 signature.<br><br>Use [`-u, --user`](#user) to specify Access Key Id (username) and Secret Key (password).<br><br>To use temporary session credentials (e.g. for an AWS IAM Role), add the `X-Amz-Security-Token` header containing the session token.<br>                                                                                    |
| <a href="#cacert" id="cacert"><code>--cacert &lt;FILE&gt;</code></a>                                              | Specifies the certificate file for peer verification. The file may contain multiple CA certificates and must be in PEM format.<br>Normally Hurl is built to use a default file for this, so this option is typically used to alter that default file.<br>                                                                                                                                          |
| <a href="#cert" id="cert"><code>-E, --cert &lt;CERTIFICATE[:PASSWORD]&gt;</code></a>                              | Client certificate file and password.<br><br>See also [`--key`](#key).<br>                                                                                                                                                                                                                                                                                                                         |
| <a href="#color" id="color"><code>--color</code></a>                                                              | Colorize debug output (the HTTP response output is not colorized). <br>                                                                                                                                                                                                                                                                                                                            |
| <a href="#compressed" id="compressed"><code>--compressed</code></a>                                               | Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.<br>                                                                                                                                                                                                                                                                          |
| <a href="#connect-timeout" id="connect-timeout"><code>--connect-timeout &lt;SECONDS&gt;</code></a>                | Maximum time in seconds that you allow Hurl's connection to take.<br><br>See also [`-m, --max-time`](#max-time).<br>                                                                                                                                                                                                                                                                               |
| <a href="#connect-to" id="connect-to"><code>--connect-to &lt;HOST1:PORT1:HOST2:PORT2&gt;</code></a>               | For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead. This option can be used several times in a command line.<br><br>See also [`--resolve`](#resolve).<br>                                                                                                                                                                                                                 |
| <a href="#continue-on-error" id="continue-on-error"><code>--continue-on-error</code></a>                          | Continue executing requests to the end of the Hurl file even when an assert error occurs.<br>By default, Hurl exits after an assert error in the HTTP response.<br><br>Note that this option does not affect the behavior with multiple input Hurl files.<br><br>All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.<br> |
| <a href="#cookie" id="cookie"><code>-b, --cookie &lt;FILE&gt;</code></a>                                          | Read cookies from FILE (using the Netscape cookie file format).<br><br>Combined with [`-c, --cookie-jar`](#cookie-jar), you can simulate a cookie storage between successive Hurl runs.<br>                                                                                                                                                                                                        |
| <a href="#cookie-jar" id="cookie-jar"><code>-c, --cookie-jar &lt;FILE&gt;</code></a>                              | Write cookies to FILE after running the session (only for one session).<br>The file will be written using the Netscape cookie file format.<br><br>Combined with [`-b, --cookie`](#cookie), you can simulate a cookie storage between successive Hurl runs.<br>                                                                                                                                     |
| <a href="#delay" id="delay"><code>--delay &lt;MILLISECONDS&gt;</code></a>                                         | Sets delay before each request.<br>                                                                                                                                                                                                                                                                                                                                                                |
| <a href="#error-format" id="error-format"><code>--error-format &lt;FORMAT&gt;</code></a>                          | Control the format of error message (short by default or long)<br>                                                                                                                                                                                                                                                                                                                                 |
| <a href="#file-root" id="file-root"><code>--file-root &lt;DIR&gt;</code></a>                                      | Set root file system to import files in Hurl. This is used for both files in multipart form data and request body.<br>When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.<br>                                                                                                                                                           |
| <a href="#location" id="location"><code>-L, --location</code></a>                                                 | Follow redirect. To limit the amount of redirects to follow use the [`--max-redirs`](#max-redirs) option<br>                                                                                                                                                                                                                                                                                       |
| <a href="#glob" id="glob"><code>--glob &lt;GLOB&gt;</code></a>                                                    | Specify input files that match the given glob pattern.<br><br>Multiple glob flags may be used. This flag supports common Unix glob patterns like *, ? and []. <br>However, to avoid your shell accidentally expanding glob patterns before Hurl handles them, you must use single quotes or double quotes around each pattern.<br>                                                                 |
| <a href="#include" id="include"><code>-i, --include</code></a>                                                    | Include the HTTP headers in the output (last entry).<br>                                                                                                                                                                                                                                                                                                                                           |
| <a href="#ignore-asserts" id="ignore-asserts"><code>--ignore-asserts</code></a>                                   | Ignore all asserts defined in the Hurl file.<br>                                                                                                                                                                                                                                                                                                                                                   |
| <a href="#insecure" id="insecure"><code>-k, --insecure</code></a>                                                 | This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.<br>                                                                                                                                                                                                                                                                                                        |
| <a href="#interactive" id="interactive"><code>--interactive</code></a>                                            | Stop between requests.<br>This is similar to a break point, You can then continue (Press C) or quit (Press Q).<br>                                                                                                                                                                                                                                                                                 |
| <a href="#json" id="json"><code>--json</code></a>                                                                 | Output each hurl file result to JSON. The format is very closed to HAR format. <br>                                                                                                                                                                                                                                                                                                                |
| <a href="#key" id="key"><code>--key &lt;KEY&gt;</code></a>                                                        | Private key file name.<br>                                                                                                                                                                                                                                                                                                                                                                         |
| <a href="#max-redirs" id="max-redirs"><code>--max-redirs &lt;NUM&gt;</code></a>                                   | Set maximum number of redirection-followings allowed<br>By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.<br>                                                                                                                                                                                                                                           |
| <a href="#max-time" id="max-time"><code>-m, --max-time &lt;SECONDS&gt;</code></a>                                 | Maximum time in seconds that you allow a request/response to take. This is the standard timeout.<br><br>See also [`--connect-timeout`](#connect-timeout).<br>                                                                                                                                                                                                                                      |
| <a href="#no-color" id="no-color"><code>--no-color</code></a>                                                     | Do not colorize output.<br>                                                                                                                                                                                                                                                                                                                                                                        |
| <a href="#no-output" id="no-output"><code>--no-output</code></a>                                                  | Suppress output. By default, Hurl outputs the body of the last response.<br>                                                                                                                                                                                                                                                                                                                       |
| <a href="#noproxy" id="noproxy"><code>--noproxy &lt;HOST(S)&gt;</code></a>                                        | Comma-separated list of hosts which do not use a proxy.<br>Override value from Environment variable no_proxy.<br>                                                                                                                                                                                                                                                                                  |
| <a href="#output" id="output"><code>-o, --output &lt;FILE&gt;</code></a>                                          | Write output to FILE instead of stdout.<br>                                                                                                                                                                                                                                                                                                                                                        |
| <a href="#path-as-is" id="path-as-is"><code>--path-as-is</code></a>                                               | Tell Hurl to not handle sequences of /../ or /./ in the given URL path. Normally Hurl will squash or merge them according to standards but with this option set you tell it not to do that.<br>                                                                                                                                                                                                    |
| <a href="#proxy" id="proxy"><code>-x, --proxy &lt;[PROTOCOL://]HOST[:PORT]&gt;</code></a>                         | Use the specified proxy.<br>                                                                                                                                                                                                                                                                                                                                                                       |
| <a href="#report-junit" id="report-junit"><code>--report-junit &lt;FILE&gt;</code></a>                            | Generate JUnit File.<br><br>If the FILE report already exists, it will be updated with the new test results.<br>                                                                                                                                                                                                                                                                                   |
| <a href="#report-html" id="report-html"><code>--report-html &lt;DIR&gt;</code></a>                                | Generate HTML report in DIR.<br><br>If the HTML report already exists, it will be updated with the new test results.<br>                                                                                                                                                                                                                                                                           |
| <a href="#report-tap" id="report-tap"><code>--report-tap &lt;FILE&gt;</code></a>                                  | Generate TAP report.<br><br>If the FILE report already exists, it will be updated with the new test results.<br>                                                                                                                                                                                                                                                                                   |
| <a href="#resolve" id="resolve"><code>--resolve &lt;HOST:PORT:ADDR&gt;</code></a>                                 | Provide a custom address for a specific host and port pair. Using this, you can make the Hurl requests(s) use a specified address and prevent the otherwise normally resolved address to be used. Consider it a sort of /etc/hosts alternative provided on the command line.<br>                                                                                                                   |
| <a href="#retry" id="retry"><code>--retry  &lt;NUM&gt;</code></a>                                                 | Maximum number of retries, 0 for no retries, -1 for unlimited retries. Retry happens if any error occurs (asserts, captures, runtimes etc...).<br>                                                                                                                                                                                                                                                 |
| <a href="#retry-interval" id="retry-interval"><code>--retry-interval &lt;MILLISECONDS&gt;</code></a>              | Duration in milliseconds between each retry. Default is 1000 ms.<br>                                                                                                                                                                                                                                                                                                                               |
| <a href="#ssl-no-revoke" id="ssl-no-revoke"><code>--ssl-no-revoke</code></a>                                      | (Windows) This option tells Hurl to disable certificate revocation checks. WARNING: this option loosens the SSL security, and by using this flag you ask for exactly that.<br>                                                                                                                                                                                                                     |
| <a href="#test" id="test"><code>--test</code></a>                                                                 | Activate test mode: with this, the HTTP response is not outputted anymore, progress is reported for each Hurl file tested, and a text summary is displayed when all files have been run.<br>                                                                                                                                                                                                       |
| <a href="#to-entry" id="to-entry"><code>--to-entry &lt;ENTRY_NUMBER&gt;</code></a>                                | Execute Hurl file to ENTRY_NUMBER (starting at 1).<br>Ignore the remaining of the file. It is useful for debugging a session.<br>                                                                                                                                                                                                                                                                  |
| <a href="#user" id="user"><code>-u, --user &lt;USER:PASSWORD&gt;</code></a>                                       | Add basic Authentication header to each request.<br>                                                                                                                                                                                                                                                                                                                                               |
| <a href="#user-agent" id="user-agent"><code>-A, --user-agent &lt;NAME&gt;</code></a>                              | Specify the User-Agent string to send to the HTTP server.<br>                                                                                                                                                                                                                                                                                                                                      |
| <a href="#variable" id="variable"><code>--variable &lt;NAME=VALUE&gt;</code></a>                                  | Define variable (name/value) to be used in Hurl templates.<br>                                                                                                                                                                                                                                                                                                                                     |
| <a href="#variables-file" id="variables-file"><code>--variables-file &lt;FILE&gt;</code></a>                      | Set properties file in which your define your variables.<br><br>Each variable is defined as name=value exactly as with [`--variable`](#variable) option.<br><br>Note that defining a variable twice produces an error.<br>                                                                                                                                                                         |
| <a href="#verbose" id="verbose"><code>-v, --verbose</code></a>                                                    | Turn on verbose output on standard error stream.<br>Useful for debugging.<br><br>A line starting with '>' means data sent by Hurl.<br>A line staring with '<' means data received by Hurl.<br>A line starting with '*' means additional info provided by Hurl.<br><br>If you only want HTTP headers in the output, [`-i, --include`](#include) might be the option you're looking for.<br>         |
| <a href="#very-verbose" id="very-verbose"><code>--very-verbose</code></a>                                         | Turn on more verbose output on standard error stream.<br><br>In contrast to  [`--verbose`](#verbose) option, this option outputs the full HTTP body request and response on standard error. In addition, lines starting with '**' are libcurl debug logs.<br>                                                                                                                                      |
| <a href="#help" id="help"><code>-h, --help</code></a>                                                             | Usage help. This lists all current command line options with a short description.<br>                                                                                                                                                                                                                                                                                                              |
| <a href="#version" id="version"><code>-V, --version</code></a>                                                    | Prints version information<br>                                                                                                                                                                                                                                                                                                                                                                     |

## Environment

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using the [`-x, --proxy`](#proxy) option.

| Variable                                   | Description                                                                                                                                                      |
|--------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `http_proxy [PROTOCOL://]<HOST>[:PORT]`    | Sets the proxy server to use for HTTP.<br>                                                                                                                       |
| `https_proxy [PROTOCOL://]<HOST>[:PORT]`   | Sets the proxy server to use for HTTPS.<br>                                                                                                                      |
| `all_proxy [PROTOCOL://]<HOST>[:PORT]`     | Sets the proxy server to use if no protocol-specific proxy is set.<br>                                                                                           |
| `no_proxy <comma-separated list of hosts>` | List of host names that shouldn't go through any proxy.<br>                                                                                                      |
| `HURL_name value`                          | Define variable (name/value) to be used in Hurl templates. This is similar than [`--variable`](#variable) and [`--variables-file`](#variables-file) options.<br> |
| `NO_COLOR`                                 | When set to a non-empty string, do not colorize output (see [`--no-color`](#no-color) option).<br>                                                               |

## Exit Codes

| Value | Description                                             |
|-------|---------------------------------------------------------|
| `0`   | Success.<br>                                            |
| `1`   | Failed to parse command-line options.<br>               |
| `2`   | Input File Parsing Error.<br>                           |
| `3`   | Runtime error (such as failure to connect to host).<br> |
| `4`   | Assert Error.<br>                                       |

## WWW

[https://hurl.dev](https://hurl.dev)


## See Also

curl(1)  hurlfmt(1)

# Installation

## Binaries Installation

### Linux

Precompiled binary is available at [hurl-4.0.0-x86_64-linux.tar.gz]:

```shell
$ INSTALL_DIR=/tmp
$ curl -silent --location https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl-4.0.0-x86_64-linux.tar.gz | tar xvz -C $INSTALL_DIR
$ export PATH=$INSTALL_DIR/hurl-4.0.0:$PATH
```

#### Debian / Ubuntu

For Debian / Ubuntu, Hurl can be installed using a binary .deb file provided in each Hurl release.

```shell
$ curl --location --remote-name https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl_4.0.0_amd64.deb
$ sudo apt update && sudo apt install ./hurl_4.0.0_amd64.deb
```

#### Alpine

Hurl is available on `testing` channel.

```shell
$ apk add --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing hurl
```

#### Arch Linux / Manjaro

[`hurl-bin` package] for Arch Linux and derived distros is available via [AUR].

#### NixOS / Nix

[NixOS / Nix package] is available on stable channel.

### macOS

Precompiled binary is available at [hurl-4.0.0-x86_64-macos.tar.gz] for x86 CPUs and [hurl-4.0.0-arm64-macos.tar.gz] for ARM CPUS.

#### Homebrew

```shell
$ brew install hurl
```

#### MacPorts

```shell
$ sudo port install hurl
```

### FreeBSD

```shell
$ sudo pkg install hurl
```

### Windows

#### Zip File

Hurl can be installed from a standalone zip file [hurl-4.0.0-win64.zip]. You will need to update your `PATH` variable.

#### Installer

An installer [hurl-4.0.0-win64-installer.exe] is also available.

#### Chocolatey

```shell
$ choco install hurl
```

#### Scoop

```shell
$ scoop install hurl
```

#### Windows Package Manager

```shell
$ winget install hurl
```

### Cargo

If you're a Rust programmer, Hurl can be installed with cargo.

```shell
$ cargo install hurl
```

### Docker

```shell
$ docker pull ghcr.io/orange-opensource/hurl:latest
```

### npm

```shell
$ npm install --save-dev @orangeopensource/hurl
```

## Building From Sources

Hurl sources are available in [GitHub].

### Build on Linux

Hurl depends on libssl, libcurl and libxml2 native libraries. You will need their development files in your platform.


#### Debian based distributions

```shell
$ apt install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev libxml2-dev
```

#### Red Hat based distributions

```shell
$ yum install -y pkg-config gcc openssl-devel libxml2-devel
```

#### Arch based distributions

```shell
$ pacman -Sy --noconfirm pkgconf gcc glibc openssl libxml2
```

### Build on macOS

```shell
$ xcode-select --install
$ brew install pkg-config
```

Hurl is written in [Rust]. You should [install] the latest stable release.

```shell
$ curl https://sh.rustup.rs -sSf | sh -s -- -y
$ source $HOME/.cargo/env
$ rustc --version
$ cargo --version
```

Then build hurl:

```shell
$ git clone https://github.com/Orange-OpenSource/hurl
$ cd hurl
$ cargo build --release
$ ./target/release/hurl --version
```

### Build on Windows

Please follow the [contrib on Windows section].


[XPath]: https://en.wikipedia.org/wiki/XPath
[JSONPath]: https://goessner.net/articles/JsonPath/
[Rust]: https://www.rust-lang.org
[curl]: https://curl.se
[the installation section]: https://hurl.dev/docs/installation.html
[Feedback, suggestion, bugs or improvements]: https://github.com/Orange-OpenSource/hurl/issues
[License]: https://hurl.dev/docs/license.html
[Tutorial]: https://hurl.dev/docs/tutorial/your-first-hurl-file.html
[Documentation]: https://hurl.dev/docs/installation.html
[Blog]: https://hurl.dev/blog/
[GitHub]: https://github.com/Orange-OpenSource/hurl
[libcurl]: https://curl.se/libcurl/
[star Hurl on GitHub]: https://github.com/Orange-OpenSource/hurl/stargazers
[JSON body]: https://hurl.dev/docs/request.html#json-body
[XML body]: https://hurl.dev/docs/request.html#xml-body
[XML multiline string body]: https://hurl.dev/docs/request.html#multiline-string-body
[multiline string body]: https://hurl.dev/docs/request.html#multiline-string-body
[predicates]: https://hurl.dev/docs/asserting-response.html#predicates
[JSONPath]: https://goessner.net/articles/JsonPath/
[Basic authentication]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#basic_authentication_scheme
[`Authorization` header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[Hurl tests suite]: https://github.com/Orange-OpenSource/hurl/tree/master/integration/tests_ok
[Authorization]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[`-u/--user` option]: https://hurl.dev/docs/manual.html#user
[curl]: https://curl.se
[entry]: https://hurl.dev/docs/entry.html
[`--test` option]: https://hurl.dev/docs/manual.html#test
[`--user`]: https://hurl.dev/docs/manual.html#user
[Hurl templates]: https://hurl.dev/docs/templates.html
[AWS Signature Version 4]: https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html
[GitHub]: https://github.com/Orange-OpenSource/hurl
[hurl-4.0.0-win64.zip]: https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl-4.0.0-win64.zip
[hurl-4.0.0-win64-installer.exe]: https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl-4.0.0-win64-installer.exe
[hurl-4.0.0-x86_64-macos.tar.gz]: https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl-4.0.0-x86_64-macos.tar.gz
[hurl-4.0.0-arm64-macos.tar.gz]: https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl-4.0.0-arm64-macos.tar.gz
[hurl-4.0.0-x86_64-linux.tar.gz]: https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl-4.0.0-x86_64-linux.tar.gz
[AUR]: https://wiki.archlinux.org/index.php/Arch_User_Repository
[`hurl-bin` package]: https://aur.archlinux.org/packages/hurl-bin/
[install]: https://www.rust-lang.org/tools/install
[Rust]: https://www.rust-lang.org
[contrib on Windows section]: https://github.com/Orange-OpenSource/hurl/blob/master/contrib/windows/README.md
[NixOS / Nix package]: https://search.nixos.org/packages?from=0&size=1&sort=relevance&type=packages&query=hurl

