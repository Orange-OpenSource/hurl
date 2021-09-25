<a href="https://hurl.dev"><img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/docs/logo.svg?sanitize=true" align="center" width="264px"/></a>

<br/>

[![deploy status](https://github.com/Orange-OpenSource/hurl/workflows/CI/badge.svg)](https://github.com/Orange-OpenSource/hurl/actions)
[![Crates.io](https://img.shields.io/crates/v/hurl.svg)](https://crates.io/crates/hurl)
[![documentation](https://img.shields.io/badge/-documentation-informational)](https://hurl.dev)

Table of Contents
=================

   * [Presentation](#presentation)
      * [What's Hurl?](#whats-hurl)
      * [Also an HTTP Test Tool](#also-an-http-test-tool)
      * [Powered by curl](#powered-by-curl)
      * [Why Hurl?](#why-hurl)
   * [Documentation](#documentation)
   * [Samples](#samples)
      * [Getting Data](#getting-data)
         * [Query Params](#query-params)
      * [Sending Data](#sending-data)
         * [Sending HTML Form Datas](#sending-html-form-datas)
         * [Sending Multipart Form Datas](#sending-multipart-form-datas)
         * [Posting a JSON Body](#posting-a-json-body)
         * [Templating a JSON/XML Body](#templating-a-jsonxml-body)
      * [Testing Response](#testing-response)
         * [Testing Response Headers](#testing-response-headers)
         * [Testing REST Apis](#testing-rest-apis)
         * [Testing HTML Response](#testing-html-response)
         * [Testing Set-Cookie Attributes](#testing-set-cookie-attributes)
      * [Others](#others)
         * [Testing Endpoint Performance](#testing-endpoint-performance)
         * [Using SOAP Apis](#using-soap-apis)
         * [Capturing and Using a CSRF Token](#capturing-and-using-a-csrf-token)
   * [Usage](#usage)
      * [Options](#options)
      * [Environment](#environment)
      * [Exit codes](#exit-codes)
   * [Building](#building)
      * [Linux, macOS](#linux-macos)
      * [Windows](#windows)
   * [Feedbacks](#feedbacks)


# Presentation

## What's Hurl? 

Hurl is a command line tool that performs HTTP requests defined in a simple plain text format.

It can perform requests, capture values and evaluate queries on headers and body response.
Hurl is very versatile: it can be used for both fetching data and testing HTTP sessions.

```hurl
# Get home:
GET https://example.net

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"

# Do login!
POST https://example.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}

HTTP/1.1 302
```

Chaining multiple requests is easy:

```hurl
GET https://api.example.net/health
GET https://api.example.net/step1
GET https://api.example.net/step2
GET https://api.example.net/step3
```

## Also an HTTP Test Tool

Hurl can run HTTP requests but can also be used to test HTTP responses.
Different type of queries and predicates are supported, from [XPath](https://en.wikipedia.org/wiki/XPath)
and [JSONPath](https://goessner.net/articles/JsonPath/) on body response, to assert on status code and response headers.

```hurl
GET https://example.net

HTTP/1.1 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"
```

It is well adapted for REST/json apis

```hurl
POST https://api.example.net/tests
{
    "id": "456",
    "evaluate": true
}

HTTP/1.1 200
[Asserts]
jsonpath "$.status" == "RUNNING"      # Check the status code
jsonpath "$.tests" count == 25        # Check the number of items

```

and even SOAP apis

```hurl
POST https://example.net/InStock
Content-Type: application/soap+xml; charset=utf-8
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="http://www.example.org">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>

HTTP/1.1 200
```

Hurl can also be used to test HTTP endpoints performances:

```hurl
GET http://api.example.org/v1/pets

HTTP/1.0 200
[Asserts]
duration < 1000  # Duration in ms
```

## Powered by curl

Hurl is a lightweight binary written in [Rust](https://www.rust-lang.org). Under the hood, Hurl HTTP engine is
powered by [libcurl](https://curl.haxx.se/libcurl/), one of the most powerful and reliable file transfer library.
With its text file format, Hurl adds syntactic sugar to run and tests HTTP requests, but it's still the curl that
we love.

## Why Hurl?

- Text format for both devops and developers
- Fast command-line for both local dev and continuous integration
- Single binary, easy to install, with no runtime required

# Documentation

Visit the [Hurl web site](https://hurl.dev) to find out how to install and use Hurl. Precompiled binaries
for Linux and macOS (Windows really soon!) are also available in the 
[GitHub releases section](https://github.com/Orange-OpenSource/hurl/releases).

- [Installation](https://hurl.dev/docs/installation.html)
- [Samples](https://hurl.dev/docs/samples.html)
- [File Format](https://hurl.dev/docs/entry.html)

# Samples

To run a sample, you can edit a file with the sample content, and use Hurl:

```
$ vi sample.hurl

GET https://example.net

$ hurl sample.hurl
```

## Getting Data

A simple GET:

```hurl
GET https://example.net
```

[Doc](https://hurl.dev/docs/request.html#method)

A simple GET with headers:

```hurl
GET https://example.net/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
Accept: */*
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
```

[Doc](https://hurl.dev/docs/request.html#headers)

### Query Params

```hurl
GET https://example.net/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
[QueryStringParams]
order: newest
search: something to search
count: 100
```

Or:

```hurl
GET https://example.net/news?order=newest&search=something%20to%20search&count=100
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
```

[Doc](https://hurl.dev/docs/request.html#query-parameters)

## Sending Data

### Sending HTML Form Datas

```hurl
POST https://example.net/contact
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

[Doc](https://hurl.dev/docs/request.html#form-parameters)

### Sending Multipart Form Datas

```hurl
POST https://example.net/upload
[MultipartFormData]
field1: value1
field2: file,example.txt;
# On can specify the file content type:
field3: file,example.zip; application/zip
```

[Doc](https://hurl.dev/docs/request.html#multipart-form-data)

### Posting a JSON Body

With an inline JSON:

```hurl
POST https://api.example.net/tests
{
    "id": "456",
    "evaluate": true
}
```

[Doc](https://hurl.dev/docs/request.html#json-body)

With a local file:

```hurl
POST https://api.example.net/tests
Content-Type: application/json
file,data.json;
```

[Doc](https://hurl.dev/docs/request.html#file-body)

### Templating a JSON/XML Body

Using templates with [JSON body](https://hurl.dev/docs/request.html#json-body) or [XML body](https://hurl.dev/docs/request.html#xml-body)
 is not currently supported in Hurl. Besides, you can use templates in [raw string body](https://hurl.dev/docs/request.html#raw-string-body)
 with variables to send a JSON or XML body:

~~~hurl
PUT https://api.example.net/hits
Content-Type: application/json
```
{
    "key0": "{{a_string}}",
    "key1": {{a_bool}},
    "key2": {{a_null}},
    "key3": {{a_number}}
}
```
~~~

Variables can be initialized via command line:

```bash
$ hurl --variable key0=apple --variable key1=true --variable key2=null --variable key3=42 test.hurl
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

[Doc](https://hurl.dev/docs/request.html#raw-string-body)

## Testing Response

### Testing Response Headers

Use implicit response asserts to test header values:

```hurl
GET http://www.example.org/index.html

HTTP/1.0 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

[Doc](https://hurl.dev/docs/asserting-response.html#headers)


Or use explicit response asserts with [predicates](https://hurl.dev/docs/asserting-response.html#predicates):

```hurl
GET https://example.net

HTTP/1.1 302
[Asserts]
header "Location" contains "www.example.net"
```

[Doc](https://hurl.dev/docs/asserting-response.html##header-assert)

### Testing REST Apis

Asserting JSON body response with [JSONPath](https://goessner.net/articles/JsonPath/):

```hurl
GET https//example.org/order
screencapability: low

HTTP/1.1 200
[Asserts]
jsonpath "$.validated" == true
jsonpath "$.userInfo.firstName" == "Franck"
jsonpath "$.userInfo.lastName" == "Herbert"
jsonpath "$.hasDevice" == false
jsonpath "$.links" count == 12
jsonpath "$.state" != null
```

[Doc](https://hurl.dev/docs/asserting-response.html#jsonpath-assert)

Testing status code:

```hurl
GET https//example.org/order/435

HTTP/1.1 200
```

[Doc](https://hurl.dev/docs/asserting-response.html#version-status)

```hurl
GET https//example.org/order/435

# Testing status code is in a 200-300 range
HTTP/1.1 *
[Asserts]
status >= 200
status < 300
```

[Doc](https://hurl.dev/docs/asserting-response.html#status-assert)


### Testing HTML Response

```hurl
GET https://example.com

HTTP/1.1 200
Content-Type: text/html; charset=UTF-8

[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" == 2                             # Check the number of p
xpath "//p" count == 2                              # Similar assert for p
xpath "boolean(count(//h2))" == false               # Check there is no h2
xpath "//h2" not exists                             # Similar assert for h2
```

[Doc](https://hurl.dev/docs/asserting-response.html#xpath-assert)

### Testing Set-Cookie Attributes

```hurl
GET http://myserver.com/home

HTTP/1.0 200
[Asserts]
cookie "JSESSIONID" == "8400BAFE2F66443613DC38AE3D9D6239"
cookie "JSESSIONID[Value]" == "8400BAFE2F66443613DC38AE3D9D6239"
cookie "JSESSIONID[Expires]" contains "Wed, 13 Jan 2021"
cookie "JSESSIONID[Secure]" exists
cookie "JSESSIONID[HttpOnly]" exists
cookie "JSESSIONID[SameSite]" == "Lax"
```

[Doc](https://hurl.dev/docs/asserting-response.html#cookie-assert)

## Others

### Testing Endpoint Performance

```hurl
GET https://sample.org/helloworld

HTTP/* *
[Asserts]
duration < 1000   # Check that response time is less than one second
```

[Doc](https://hurl.dev/docs/asserting-response.html#duration-assert)

### Using SOAP Apis

```hurl
POST https://example.net/InStock
Content-Type: application/soap+xml; charset=utf-8
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="http://www.example.org">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>

HTTP/1.1 200
```

[Doc](https://hurl.dev/docs/request.html#xml-body)

### Capturing and Using a CSRF Token

```hurl
GET https://example.net

HTTP/* 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"

POST https://example.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}

HTTP/* 302
```

[Doc](https://hurl.dev/docs/capturing-response.html#xpath-capture)

# Usage

## Options

Options that exist in curl have exactly the same semantic.

Option | Description
 --- | --- 
`--color` | Colorize Output
`-b, --cookie <file>` | Read cookies from file (using the Netscape cookie file format). Combined with `-c, --cookie-jar`, you can simulate a cookie storage between successive Hurl runs.
`--compressed` | Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.
`--connect-timeout <seconds>` | Maximum time in seconds that you allow Hurl's connection to take. See also `-m, --max-time` option.
`-c, --cookie-jar <file>` | Write cookies to FILE after running the session (only for one session). The file will be written using the Netscape cookie file format. Combined with `-b, --cookie`,you can simulate a cookie storage between successive Hurl runs.
`--fail-at-end` | Continue executing requests to the end of the Hurl file even when an assert error occurs. By default, Hurl exits after an assert error in the HTTP response. Note that this option does not affect the behavior with multiple input Hurl files. All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.
`--file-root <dir>` | Set root filesystem to import files in Hurl. This is used for both files in multipart form data and request body. When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.
`-h, --help` | Usage help. This lists all current command line options with a short description.
`--html <dir>` | Generate html report in dir. If the html report already exists, it will be updated with the new test results.
`-i, --include` | Include the HTTP headers in the output (last entry).
`--interactive` | Stop between requests. This is similar to a break point, You can then continue (Press C) or quit (Press Q).
`--json <file>`| Write full session(s) to a json file. The format is very closed to HAR format. If the json file already exists, the file will be updated with the new test results.

`--k, --insecure` | This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.
`-L, --location` | Follow redirect. You can limit the amount of redirects to follow by using the `--max-redirs` option.
`-m, --max-time <seconds>` | Maximum time in seconds that you allow a request/response to take. This is the standard timeout. See also `--connect-timeout` option.
`--max-redirs <num>` | Set maximum number of redirection-followings allowed. By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.
`--no-color` | Do not colorize Output
`--noproxy <no-proxy-list>` | Comma-separated list of hosts which do not use a proxy. Override value from Environment variable no_proxy.
`--to-entry <entry-number` | Execute Hurl file to ENTRY_NUMBER (starting at 1). Ignore the remaining of the file. It is useful for debugging a session.
`-o, --output <file>` | Write output to <file> instead of stdout.
`-x, --proxy [protocol://]host[:port]` | Use the specified proxy.
`-u, --user <user:password>` | Add basic Authentication header to each request.
`--variable <name=value>` | Define variable (name/value) to be used in Hurl templates. Only string values can be defined.
`--variables-file <file>` | Set properties file in which your define your variables. Each variable is defined as name=value exactly as with `--variable` option. Note that defining a variable twice produces an error.
`-v, --verbose` | Turn on verbose output on standard error stream. Useful for debugging. A line starting with '>' means data sent by Hurl. A line staring with '&lt;' means data received by Hurl. A line starting with '*' means additional info provided by Hurl. If you only want HTTP headers in the output, -i, \-\-include might be the option you're looking for.
`-V, --version`| Prints version information

## Environment

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using
the [-x, \-\-proxy](#proxy) option.

Variable | Description
--- | ---
`http_proxy [protocol://]<host>[:port]` | Sets the proxy server to use for HTTP.
`https_proxy [protocol://]<host>[:port]` | Sets the proxy server to use for HTTPS.
`all_proxy [protocol://]<host>[:port]` | Sets the proxy server to use if no protocol-specific proxy is set.
`no_proxy <comma-separated list of hosts>` | list of host names that shouldn't go through any proxy.


## Exit codes

Value | Description
--- | ---
`1` | Failed to parse command-line options.
`2` | Input File Parsing Error.
`3` | Runtime error (such as failure to connect to host).
`4` | Assert Error.

# Building

## Linux, macOS

Hurl depends on libssl, libcurl and libxml2 native libraries. You will need their development files in your platform.

```shell
# debian based distributions
apt install -y pkg-config libssl-dev libcurl4-openssl-dev libxml2-dev

# redhat based distributions
yum install -y pkg-config gcc openssl-devel libxml2-devel

# arch based distributions
pacman -Sy --noconfirm pkgconf gcc openssl libxml2

# osx
brew install pkg-config gcc openssl libxml2
```

Hurl is written in [Rust](https://www.rust-lang.org/). You should [install](https://www.rust-lang.org/tools/install) the latest stable release.

```shell
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustc --version
cargo --version
```

Build

```shell
git clone https://github.com/Orange-OpenSource/hurl
cd hurl
cargo build --release
./target/release/hurl --version
```

Install Binary

```shell
cargo install --path packages/hurl
```

## Windows

please follow the [contrib/windows section](contrib/windows/README.md)

# Feedbacks

Hurl is still in beta, any feedback, suggestion, bugs or improvements are welcome.

```hurl
POST https://hurl.dev/api/feedback
{
    "name": "John Doe",
    "feedback": "Hurl is awesome !"
}
HTTP/1.1 200
```
