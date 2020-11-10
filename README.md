<a href="https://hurl.dev"><img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/docs/logo.svg?sanitize=true" align="center" width="264px"/></a>

<br/>

[![deploy status](https://github.com/Orange-OpenSource/hurl/workflows/CI/badge.svg)](https://github.com/Orange-OpenSource/hurl/actions)
[![documentation](https://img.shields.io/badge/-documentation-informational)](https://hurl.dev)


# What's Hurl?

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

It is well adapted for REST/json apis

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

# Samples

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

### Testing REST Apis

```hurl
GET https//example.org/order
screencapability: low

HTTP/1.1 200
[Asserts]
jsonpath "$.validated" equals true
jsonpath "$.userInfo.firstName" equals "Franck"
jsonpath "$.userInfo.lastName" equals "Herbert"
jsonpath "$.hasDevice" equals false
jsonpath "$.links" countEquals 12
```

[Doc](https://hurl.dev/docs/asserting-response.html#jsonpath-assert)


### Testing HTML Response

```hurl
GET https://example.com

HTTP/1.1 200
Content-Type: text/html; charset=UTF-8

[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" equals 2                         # Check the number of p
xpath "//p" countEquals 2                           # Similar assert for p
xpath "boolean(count(//h2))" equals false           # Check there is no h2
xpath "//h2" not exists                             # Similar assert for h2
```

[Doc](https://hurl.dev/docs/asserting-response.html#xpath-assert)

## Others

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
