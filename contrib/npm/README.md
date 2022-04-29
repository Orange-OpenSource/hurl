# Hurl

Hurl is a command line tool written in Rust that runs <b>HTTP requests</b> defined in a simple <b>plain text format</b>.

The `@orangeopensource/hurl` package allows JavaScript developers to use Hurl in npm scripts.

Hurl can perform requests, capture values and evaluate queries on headers and body response. Hurl is very
versatile: it can be used for both <b>fetching data</b> and <b>testing HTTP</b> sessions.


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

Hurl can run HTTP requests but can also be used to <b>test HTTP responses</b>.
Different types of queries and predicates are supported, from [XPath](https://en.wikipedia.org/wiki/XPath) and 
[JSONPath](https://goessner.net/articles/JsonPath/) on body response, to assert on status code and response headers.

It is well adapted for <b>REST / JSON apis</b>

```hurl
POST https://api.example.net/tests
{
    "id": "4568",
    "evaluate": true
}

HTTP/1.1 200
[Asserts]
header "X-Frame-Options" == "SAMEORIGIN"
jsonpath "$.status" == "RUNNING"    # Check the status code
jsonpath "$.tests" count == 25      # Check the number of items
jsonpath "$.id" matches /\d{4}/     # Check the format of the id
```

and <b>HTML content</b>

```hurl
GET https://example.net

HTTP/1.1 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"
```

## Installation

```
npm install --save-dev @orangeopensource/hurl
```

This will download the appropriate Hurl binaries for your platform. `hurlmft` binary is also installed, which 
you can use for [exporting Hurl files to JSON files](https://hurl.dev/docs/frequently-asked-questions.html#how-can-i-use-my-hurl-files-outside-hurl). 


## Usage

In your `package.json` file:

```
{
  "name": "sample-app",
  "scripts": {
    "test": "hurl --test --glob test/*.hurl",
    ...
  },
  ...
```



## Documentation

See <https://hurl.dev>

## Samples

See <https://hurl.dev/docs/samples.html>