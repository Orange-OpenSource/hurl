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

[Doc](/docs/request.md#method)

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

[Doc](/docs/request.md#headers)

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

[Doc](/docs/request.md#query-parameters)

### Basic Authentification

```hurl
GET https://example.org/protected
[BasicAuth]
bob: secret
```

[Doc](/docs/request.md#basic-authentification)

This is equivalent to construct the request with a [Authorization] header:

```hurl
# Authorization header value can be computed with `echo -n 'bob:secret' | base64`
GET https://example.org/protected
Authorization: Basic Ym9iOnNlY3JldA== 
```

Basic authentification allows per request authentification.
If you want to add basic authentification to all the request of a Hurl file
you could use [`-u/--user` option].

## Sending Data

### Sending HTML Form Datas

```hurl
POST https://example.org/contact
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

[Doc](/docs/request.md#form-parameters)

### Sending Multipart Form Datas

```hurl
POST https://example.org/upload
[MultipartFormData]
field1: value1
field2: file,example.txt;
# On can specify the file content type:
field3: file,example.zip; application/zip
```

[Doc](/docs/request.md#multipart-form-data)

### Posting a JSON Body

With an inline JSON:

```hurl
POST https://example.org/api/tests
{
    "id": "456",
    "evaluate": true
}
```

[Doc](/docs/request.md#json-body)

With a local file:

```hurl
POST https://example.org/api/tests
Content-Type: application/json
file,data.json;
```

[Doc](/docs/request.md#file-body)

### Templating a JSON / XML Body

Using templates with [JSON body] or [XML body] is not currently supported in Hurl.
Besides, you can use templates in [raw string body] with variables to send a JSON or XML body:

~~~hurl
PUT https://example.org/api/hits
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

[Doc](/docs/request.md#raw-string-body)

## Testing Response

### Testing Response Headers

Use implicit response asserts to test header values:

```hurl
GET https://example.org/index.html

HTTP/1.0 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

[Doc](/docs/asserting-response.md#headers)


Or use explicit response asserts with [predicates]:

```hurl
GET https://example.org

HTTP/1.1 302
[Asserts]
header "Location" contains "www.example.net"
```

[Doc](/docs/asserting-response.md#header-assert)


### Testing REST Apis

Asserting JSON body response (node values, collection count etc...) with [JSONPath]:

```hurl
GET https://example.org/order
screencapability: low

HTTP/1.1 200
[Asserts]
jsonpath "$.validated" == true
jsonpath "$.userInfo.firstName" == "Franck"
jsonpath "$.userInfo.lastName" == "Herbert"
jsonpath "$.hasDevice" == false
jsonpath "$.links" count == 12
jsonpath "$.state" != null
jsonpath "$.order" matches "^order-\\d{8}$"
jsonpath "$.order" matches /^order-\d{8}$/     # Alternative syntax with regex litteral
```

[Doc](/docs/asserting-response.md#jsonpath-assert)


Testing status code:

```hurl
GET https://example.org/order/435

HTTP/1.1 200
```

[Doc](/docs/asserting-response.md#version-status)

```hurl
GET https://example.org/order/435

# Testing status code is in a 200-300 range
HTTP/1.1 *
[Asserts]
status >= 200
status < 300
```

[Doc](/docs/asserting-response.md#status-assert)


### Testing HTML Response

```hurl
GET https://example.org

HTTP/1.1 200
Content-Type: text/html; charset=UTF-8

[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" == 2  # Check the number of p
xpath "//p" count == 2  # Similar assert for p
xpath "boolean(count(//h2))" == false  # Check there is no h2  
xpath "//h2" not exists  # Similar assert for h2
xpath "string(//div[1])" matches /Hello.*/
```

[Doc](/docs/asserting-response.md#xpath-assert)

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

[Doc](/docs/asserting-response.md#cookie-assert)

### Testing Bytes Content


Check the SHA-256 response body hash:

```hurl
GET https://example.org/data.tar.gz

HTTP/* *
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

[Doc](/docs/asserting-response.md#sha-256-assert)


## Others

### Testing Endpoint Performance

```hurl
GET https://sample.org/helloworld

HTTP/* *
[Asserts]
duration < 1000   # Check that response time is less than one second
```

[Doc](/docs/asserting-response.md#duration-assert)

### Using SOAP Apis

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

HTTP/1.1 200
```

[Doc](/docs/request.md#xml-body)

### Capturing and Using a CSRF Token

```hurl
GET https://example.org

HTTP/* 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"

POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}

HTTP/* 302
```

[Doc](/docs/capturing-response.md#xpath-capture)

### Checking Byte Order Mark (BOM) in Response Body

```hurl
GET https://example.org/data.bin

HTTP/* 200
[Asserts]
bytes startsWith hex,efbbbf;
```

[Doc](/docs/asserting-response.md#bytes-assert)


[JSON body]: /docs/request.md#json-body
[XML body]: /docs/request.md#xml-body
[raw string body]: /docs/request.md#raw-string-body
[predicates]: /docs/asserting-response.md#predicates
[JSONPath]: https://goessner.net/articles/JsonPath/
[Basic authentication]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#basic_authentication_scheme
[`Authorization` header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[Hurl tests suite]: https://github.com/Orange-OpenSource/hurl/tree/master/integration/tests_ok
[Authorization]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[`-u/--user` option]: /docs/man-page.md#user
[curl]: https://curl.se
[entry]: /docs/entry.md
[`--test` option]: /docs/man-page.md#test