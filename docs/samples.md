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

A particular response can be saved with [`[Options] section`](/docs/request.md#options):

```hurl
GET https://example.ord/cats/123
[Options]
output: cat123.txt    # use - to output to stdout
HTTP 200

GET https://example.ord/dogs/567
HTTP 200
```

Finally, Hurl can take files as input, or directories. In the latter case, Hurl will search files with `.hurl` extension recursively.

```shell
$ hurl --test integration/*.hurl
$ hurl --test .
```

You can check [Hurl tests suite] for more samples.

## Getting Data

A simple GET:

```hurl
GET https://example.org
```

Requests can be chained:

```hurl
GET https://example.org/a
GET https://example.org/b
HEAD https://example.org/c
GET https://example.org/c
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

> With `[QueryStringParams]` section, params don't need to be URL escaped.

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

Basic authentication section allows per request authentication. If you want to add basic authentication to all the
requests of a Hurl file you could use [`-u/--user` option]:

```shell
$ hurl --user bob:secret login.hurl
```

[`--user`] option can also be set per request:

```hurl
GET https://example.org/login
[Options]
user: bob:secret
HTTP 200

GET https://example.org/login
[Options]
user: alice:secret
HTTP 200
```

### Passing Data between Requests 

[Captures] can be used to pass data from one request to another:

```hurl
POST https://sample.org/orders
HTTP 201
[Captures]
order_id: jsonpath "$.order.id"

GET https://sample.org/orders/{{order_id}}
HTTP 200
```

[Doc](/docs/capturing-response.md)

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

[Doc](/docs/request.md#multiline-string-body)



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

### Using Dynamic Datas

[Functions] like `newUuid` and `newDate` can be used in templates to create dynamic datas:


A file that creates a dynamic email (i.e `0531f78f-7f87-44be-a7f2-969a1c4e6d97@test.com`):

```hurl
POST https://example.org/api/foo
{
  "name": "foo",
  "email": "{{newUuid}}@test.com"
}
```

A file that creates a dynamic query parameter (i.e `2024-12-02T10:35:44.461731Z`):

```hurl
GET https://example.org/api/foo
[QueryStringParams]
date: {{newDate}}
HTTP 200
```

[Doc](/docs/templates.md#functions)

## Testing Response

Responses are optional, everything after `HTTP` is part of the response asserts.

```hurl
# A request with (almost) no check:
GET https://foo.com

# A status code check:
GET https://foo.com
HTTP 200

# A test on response body
GET https://foo.com
HTTP 200
[Asserts]
jsonpath "$.state" == "running"
```

### Testing Status Code

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

Implicit and explicit asserts can be combined:

```hurl
GET https://example.org/index.html
HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
[Asserts]
header "Location" contains "www.example.net"
```

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
jsonpath "$.created" isIsoDate
```

[Doc](/docs/asserting-response.md#jsonpath-assert)


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

[Doc](/docs/asserting-response.md#cookie-assert)

### Testing Bytes Content

Check the SHA-256 response body hash:

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

[Doc](/docs/asserting-response.md#sha-256-assert)

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

[Doc](/docs/asserting-response.md#ssl-certificate-assert)

### Checking Full Body

Use implicit body to test an exact JSON body match:

```hurl
GET https://example.org/api/cats/123
HTTP 200
{
  "name" : "Purrsloud",
  "species" : "Cat",
  "favFoods" : ["wet food", "dry food", "<strong>any</strong> food"],
  "birthYear" : 2016,
  "photo" : "https://learnwebcode.github.io/json-example/images/cat-2.jpg"
}
```

[Doc](/docs/asserting-response.md#json-body)

Or an explicit assert file:

```hurl
GET https://example.org/index.html
HTTP 200
[Asserts]
body == file,cat.json;
```

[Doc](/docs/asserting-response.md#body-assert)

Implicit asserts supports XML body:

```hurl
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
```

[Doc](/docs/asserting-response.md#xml-body)

Plain text:

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

[Doc](/docs/asserting-response.md#multiline-string-body)


One line:

```hurl
POST https://example.org/helloworld
HTTP 200
`Hello world!`
```

[Doc](/docs/asserting-response.md#oneline-string-body)

File:

```hurl
GET https://example.org
HTTP 200
file,data.bin;
```

[Doc](/docs/asserting-response.md#file-body)

## Reports

### HTML Report

```shell
$ hurl --test --report-html build/report/ *.hurl
```

[Doc](/docs/running-tests.md#generating-report)

### JSON Report

```shell
$ hurl --test --report-json build/report/ *.hurl
```

[Doc](/docs/running-tests.md#generating-report)


### JUnit Report

```shell
$ hurl --test --report-junit build/report.xml *.hurl
```

[Doc](/docs/running-tests.md#generating-report)

### TAP Report

```shell
$ hurl --test --report-tap build/report.txt *.hurl
```

[Doc](/docs/running-tests.md#generating-report)

### JSON Output

A structured output of running Hurl files can be obtained with [`--json` option]. Each file will produce a JSON export of the run.


```shell
$ hurl --json *.hurl
```

## Others

### HTTP Version

Testing HTTP version (HTTP/1.0, HTTP/1.1, HTTP/2 or HTTP/3):

```hurl
GET https://foo.com
HTTP/3 200

GET https://bar.com
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
retry: 10   # maximum number of retry, -1 for unlimited
retry-interval: 500ms
HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"
```

[Doc](/docs/entry.md#retry)

### Delaying Requests

Add delay for every request, or a particular request:

```hurl
# Delaying this request by 5 seconds (aka sleep)
GET https://example.org/turtle
[Options]
delay: 5s
HTTP 200

# No delay!
GET https://example.org/turtle
HTTP 200
```

[Doc](/docs/manual.md#delay)

### Skipping Requests

```hurl
# a, c, d are run, b is skipped
GET https://example.org/a

GET https://example.org/b
[Options]
skip: true

GET https://example.org/c

GET https://example.org/d
```

[Doc](/docs/manual.md#skip)

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

### Redacting Secrets

Using command-line for known values:

```shell
$ hurl --secret token=1234 file.hurl
```

```hurl
POST https://example.org
X-Token: {{token}}
{
  "name": "Alice",
  "value: 100
}
HTTP 200
```

[Doc](/docs/templates.md#secrets)

Using `redact` for dynamic values:

```hurl
# Get an authorization token:
GET https://example.org/token
HTTP 200
[Captures]
token: header "X-Token" redact

# Send an authorized request:
POST https://example.org
X-Token: {{token}}
{
  "name": "Alice",
  "value: 100
}
HTTP 200
```

[Doc](/docs/capturing-response.md#redacting-secrets)

### Checking Byte Order Mark (BOM) in Response Body

```hurl
GET https://example.org/data.bin
HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
```

[Doc](/docs/asserting-response.md#bytes-assert)

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

The Access Key is given per [`--user`], either with command line option or within the [`[Options]`](/docs/request.md#options) section:

```hurl
POST https://sts.eu-central-1.amazonaws.com/
[Options]
aws-sigv4: aws:amz:eu-central-1:sts
user: bob=secret
[FormParams]
Action: GetCallerIdentity
Version: 2011-06-15
```

[Doc](/docs/manual.md#aws-sigv4)

### Using curl Options

curl options (for instance [`--resolve`] or [`--connect-to`]) can be used as CLI argument. In this case, they're applicable
to each request of an Hurl file.

```shell
$ hurl --resolve foo.com:8000:127.0.0.1 foo.hurl
```

Use  [`[Options]` section](/docs/request.md#options) to configure a specific request:

```hurl
GET http://bar.com
HTTP 200


GET http://foo.com:8000/resolve
[Options]
resolve: foo.com:8000:127.0.0.1
HTTP 200
`Hello World!`
```

[Doc](/docs/request.md#options)


[JSON body]: /docs/request.md#json-body
[XML body]: /docs/request.md#xml-body
[XML multiline string body]: /docs/request.md#multiline-string-body
[multiline string body]: /docs/request.md#multiline-string-body
[predicates]: /docs/asserting-response.md#predicates
[JSONPath]: https://goessner.net/articles/JsonPath/
[Basic authentication]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#basic_authentication_scheme
[`Authorization` header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[Hurl tests suite]: https://github.com/Orange-OpenSource/hurl/tree/master/integration/hurl/tests_ok
[Authorization]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[`-u/--user` option]: /docs/manual.md#user
[curl]: https://curl.se
[entry]: /docs/entry.md
[`--test` option]: /docs/manual.md#test
[`--user`]: /docs/manual.md#user
[Hurl templates]: /docs/templates.md
[AWS Signature Version 4]: https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html
[Captures]: /docs/capturing-response.md
[`--json` option]: /docs/manual.md#json
[`--resolve`]: /docs/manual.md#resolve
[`--connect-to`]: /docs/manual.md#connect-to
[Functions]: /docs/templates.md#functions
