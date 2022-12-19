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

### Basic Authentication

```hurl
GET https://example.org/protected
[BasicAuth]
bob: secret
```

[Doc](/docs/request.md#basic-authentication)

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

[Doc](/docs/request.md#form-parameters)

### Sending Multipart Form Data

```hurl
POST https://example.org/upload
[MultipartFormData]
field1: value1
field2: file,example.txt;
# One can specify the file content type:
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

[Doc](/docs/templates.md)

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

[Doc](/docs/request.md#multiline-string-body)

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

[Doc](/docs/request.md#graphql-body)

## Testing Response

### Testing Response Headers

Use implicit response asserts to test header values:

```hurl
GET https://example.org/index.html

HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

[Doc](/docs/asserting-response.md#headers)


Or use explicit response asserts with [predicates]:

```hurl
GET https://example.org

HTTP 302
[Asserts]
header "Location" contains "www.example.net"
```

[Doc](/docs/asserting-response.md#header-assert)


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

[Doc](/docs/asserting-response.md#jsonpath-assert)


Testing status code:

```hurl
GET https://example.org/order/435

HTTP 200
```

[Doc](/docs/asserting-response.md#version-status)

```hurl
GET https://example.org/order/435

# Testing status code is in a 200-300 range
HTTP *
[Asserts]
status >= 200
status < 300
```

[Doc](/docs/asserting-response.md#status-assert)


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

[Doc](/docs/asserting-response.md#xpath-assert)

### Testing Set-Cookie Attributes

```hurl
GET http://myserver.com/home

HTTP 200
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

### HTTP Version

Testing HTTP version (1.0, 1.1 or 2):

```hurl
GET https://example.org/order/435
HTTP/2 200
```

[Doc](/docs/asserting-response.md#version-status)

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
retry: true

HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"
```

[Doc](/docs/entry.md#retry)



### Testing Endpoint Performance

```hurl
GET https://sample.org/helloworld

HTTP *
[Asserts]
duration < 1000   # Check that response time is less than one second
```

[Doc](/docs/asserting-response.md#duration-assert)

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

[Doc](/docs/request.md#xml-body)

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

[Doc](/docs/capturing-response.md#xpath-capture)

### Checking Byte Order Mark (BOM) in Response Body

```hurl
GET https://example.org/data.bin

HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
```

[Doc](/docs/asserting-response.md#bytes-assert)


[JSON body]: /docs/request.md#json-body
[XML body]: /docs/request.md#xml-body
[XML multiline string body]: /docs/request.md#multiline-string-body
[predicates]: /docs/asserting-response.md#predicates
[JSONPath]: https://goessner.net/articles/JsonPath/
[Basic authentication]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#basic_authentication_scheme
[`Authorization` header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[Hurl tests suite]: https://github.com/Orange-OpenSource/hurl/tree/master/integration/tests_ok
[Authorization]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[`-u/--user` option]: /docs/manual.md#user
[curl]: https://curl.se
[entry]: /docs/entry.md
[`--test` option]: /docs/manual.md#test
[Hurl templates]: /docs/templates.md
