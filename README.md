<a href="https://hurl.dev"><img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/docs/logo.svg?sanitize=true" align="center" width="264px"/></a>

<br/>

[![deploy status](https://travis-ci.org/Orange-OpenSource/hurl.svg?branch=master)](https://travis-ci.org/Orange-OpenSource/hurl/)
[![documentation](https://img.shields.io/badge/-documentation-informational)](https://hurl.dev)


# What's Hurl?

Hurl is a command line tool and a simple plain text format for describing an HTTP session.

Hurl is used in command lines or scripts to run HTTP sessions. Hurl can performs requests, capture values
and evaluate queries on headers and body response. Hurl is very versatile: it can be used to get HTTP data and
also to test HTTP sessions.


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
GET https://api.example.net/health
GET https://api.example.net/health
GET https://api.example.net/health
```

# Also an HTTP Test Tool

Hurl can run HTTP requests but can also be used to test HTTP responses.
Different type of queries and predicates are supported, from [XPath](https://en.wikipedia.org/wiki/XPath)
and [JSONPath](https://goessner.net/articles/JsonPath/) on body response, to assert on status code and response headers.

```hurl
GET https://example.net

HTTP/1.1 200
[Asserts]
xpath "normalize-space(//head/title)" equals "Hello world!"
```

and is well adapted for REST/json apis

```hurl
POST https://api.example.net/tests
{
    "id": "456",
    "evaluate": true
}

HTTP/1.1 200
[Asserts]
jsonpath "$.status" equals "RUNNING"      # Check the status code
jsonpath "$.tests" countEquals 25         # Check the number of items

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

# Documentation

Visit the [Hurl web site](https://hurl.dev) to find out how to install and use Hurl.

- [Installation](https://hurl.dev/docs/installation.html)
- [Samples](https://hurl.dev/docs/samples.html)
- [File Format](https://hurl.dev/docs/entry.html)


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
