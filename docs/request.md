# Request

## Definition

Request describes an HTTP request: a mandatory [method] and [URL], followed by optional [headers].

Then, [options], [query parameters], [form parameters], [multipart form data], [cookies], and [basic authentication]
can be used to configure the HTTP request.

Finally, an optional [body] can be used to configure the HTTP request body.

## Example

```hurl
GET https://example.org/api/dogs?id=4567
User-Agent: My User Agent
Content-Type: application/json
[BasicAuth]
alice: secret
```

## Structure

<div class="hurl-structure-schema">
  <div class="hurl-structure">
    <div class="hurl-structure-col-0">
        <div class="hurl-part-0">
            PUT https://sample.net
        </div>
        <div class="hurl-part-1">
            accept: */*<br>x-powered-by: Express<br>user-agent: Test
        </div>
        <div class="hurl-part-2">
            [Options]<br>...
        </div>
        <div class="hurl-part-2">
            [Query]<br>...
        </div>
        <div class="hurl-part-2">
            [Form]<br>...
        </div>
        <div class="hurl-part-2">
            [BasicAuth]<br>...
        </div>
        <div class="hurl-part-2">
            [Cookies]<br>...
        </div>
        <div class="hurl-part-2">
            ...
        </div>
        <div class="hurl-part-2">
            ...
        </div>
        <div class="hurl-part-3">
            {<br>
            &nbsp;&nbsp;"type": "FOO",<br>
            &nbsp;&nbsp;"value": 356789,<br>
            &nbsp;&nbsp;"ordered": true,<br>
            &nbsp;&nbsp;"index": 10<br>
            }
        </div>
    </div>
    <div class="hurl-structure-col-1">
        <div class="hurl-request-explanation-part-0">
            <a href="#method">Method</a> and <a href="#url">URL</a> (mandatory)
        </div>
        <div class="hurl-request-explanation-part-1">
            <br><a href="#headers">HTTP request headers</a> (optional)
        </div>
        <div class="hurl-request-explanation-part-2">
            <br>
            <br>
            <br>
            <br>
            <br>
        </div>
        <div class="hurl-request-explanation-part-2">
            <a href="#options">Options</a>, <a href="#query-parameters">query strings</a>, <a href="#form-parameters">form params</a>, <a href="#cookies">cookies</a>, <a href="#basic-authentication">authentication</a> ...<br>(optional sections, unordered)
        </div>
        <div class="hurl-request-explanation-part-2">
            <br>
            <br>
            <br>
            <br>
        </div>
        <div class="hurl-request-explanation-part-3">
            <br>
        </div>
        <div class="hurl-request-explanation-part-3">
            <a href="#body">HTTP request body</a> (optional)
        </div>
    </div>
</div>
</div>


[Headers], if present, follow directly after the [method] and [URL]. This allows Hurl format to 'look like' the real HTTP format.
Contrary to HTTP headers, other parameters are defined in sections (`[Cookies]`, `[Query]`, `[Form]` etc...)
These sections are not ordered and can be mixed in any way:

```hurl
GET https://example.org/api/dogs
User-Agent: My User Agent
[Query]
id: 4567
order: newest
[BasicAuth]
alice: secret
```

```hurl
GET https://example.org/api/dogs
User-Agent: My User Agent
[BasicAuth]
alice: secret
[Query]
id: 4567
order: newest
```

The last optional part of a request configuration is the request [body]. Request body must be the last parameter of a request
(after [headers] and request sections). Like headers, body have no explicit marker:

```hurl
POST https://example.org/api/dogs?id=4567
User-Agent: My User Agent
{
 "name": "Ralphy"
}
```

## Description

### Method

Mandatory HTTP request method, usually one of `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`,
`TRACE` and `PATCH`. 

> Other methods can be used like `QUERY` with the constraint of using only uppercase chars.

### URL

Mandatory HTTP request URL.

URL can contain query parameters, even if using a [query parameters section] is preferred.

```hurl
# A request with URL containing query parameters.
GET https://example.org/forum/questions/?search=Install%20Linux&order=newest

# A request with query parameters section, equivalent to the first request.
GET https://example.org/forum/questions/
[Query]
search: Install Linux
order: newest
```

> Query parameters in query parameter section are not URL encoded.

When query parameters are present in the URL and in a query parameters section, the resulting request will
have both parameters.

### Headers

Optional list of HTTP request headers.

A header consists of a name, followed by a `:` and a value.

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
Accept: */*
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
```

> Headers directly follow URL, without any section name, contrary to query parameters, form parameters
> or cookies

Note that a header usually doesn't start with double quotes. If a header value starts with double quotes, double
quotes will be part of the header value:

```hurl
PATCH https://example.org/file.txt
If-Match: "e0023aa4e"
```

`If-Match` request header will be sent will the following value `"e0023aa4e"` (started and ended with double quotes).

Headers must follow directly after the [method] and [URL].

### Options

Options used to execute this request.

Options such as [`--location`], [`--verbose`], [`--insecure`] can be used at the command line and applied to every
request of an Hurl file. An `[Options]` section can be used to apply option to only one request (without passing options
to the command line), while other requests are unaffected.

```hurl
GET https://example.org
# An options section, each option is optional and applied only to this request...
[Options]
aws-sigv4: aws:amz:sts     # generate AWS SigV4 Authorization header
cacert: /etc/cert.pem      # custom certificate file
cert: /etc/client-cert.pem # client authentication certificate
key: /etc/client-cert.key  # client authentication certificate key
compressed: true           # request a compressed response
connect-timeout: 20s       # connect timeout
delay: 3s                  # delay for this request (aka sleep)
http3: true                # use HTTP/3 protocol version
insecure: true             # allow insecure SSL connections and transfers
ipv6: true                 # use IPv6 addresses
limit-rate: 32000          # limit this request to the specidied speed (bytes/s)
location: true             # follow redirection for this request
max-redirs: 10             # maximum number of redirections
max-time: 30s              # maximum time for a request/response
output: out.html           # dump the response to this file
path-as-is: true           # do not handle sequences of /../ or /./ in URL path
retry: 10                  # number of retry if HTTP/asserts errors
retry-interval: 500ms      # interval between retry
skip: false                # skip this request
unix-socket: sock          # use Unix socket for transfer
user: bob:secret           # use basic authentication
proxy: my.proxy:8012       # define proxy (host:port where host can be an IP address)
variable: country=Italy    # define variable country
variable: planet=Earth     # define variable planet
verbose: true              # allow verbose output
very-verbose: true         # allow more verbose output    
```

> Variable defined in an `[Options]` section are defined also for the next entries. This is
> the exception, all other options are defined only for the current request.

### Query parameters

Optional list of query parameters.

A query parameter consists of a field, followed by a `:` and a value. The query parameters section starts with
`[Query]`. Contrary to query parameters in the URL, each value in the query parameters section is not
URL encoded.

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
[Query]
order: newest
search: {{custom-search}}
count: 100
```

If there are any parameters in the URL, the resulted request will have both parameters.

### Form parameters

A form parameters section can be used to send data, like [HTML form].

This section contains an optional list of key values, each key followed by a `:` and a value. Key values will be
encoded in key-value tuple separated by '&', with a '=' between the key and the value, and sent in the body request.
The content type of the request is `application/x-www-form-urlencoded`. The form parameters section starts
with `[Form]`.

```hurl
POST https://example.org/contact
[Form]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

Form parameters section can be seen as syntactic sugar over body section (values in form parameters section
are not URL encoded.). A [oneline string body] could be used instead of a forms parameters section.

~~~hurl
# Run a POST request with form parameters section:
POST https://example.org/test
[Form]
name: John Doe
key1: value1

# Run the same POST request with a body section:
POST https://example.org/test
Content-Type: application/x-www-form-urlencoded
`name=John%20Doe&key1=value1`
~~~

When both [body section] and form parameters section are present, only the body section is taken into account.

### Multipart Form Data

A multipart form data section can be used to send data, with key / value and file content
(see [multipart/form-data on MDN]).

The form parameters section starts with `[Multipart]`.

```hurl
POST https://example.org/upload
[Multipart]
field1: value1
field2: file,example.txt;
# One can specify the file content type:
field3: file,example.zip; application/zip
```

Files are relative to the input Hurl file, and cannot contain implicit parent directory (`..`). You can use  
[`--file-root` option] to specify the root directory of all file nodes.

Content type can be specified or inferred based on the filename extension:

- `.gif`: `image/gif`,
- `.jpg`: `image/jpeg`,
- `.jpeg`: `image/jpeg`,
- `.png`: `image/png`,
- `.svg`: `image/svg+xml`,
- `.txt`: `text/plain`,
- `.htm`: `text/html`,
- `.html`: `text/html`,
- `.pdf`: `application/pdf`,
- `.xml`: `application/xml`

By default, content type is `application/octet-stream`.

As an alternative to a `[Multipart]` section, multipart forms can also be sent with a [multiline string body]:

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

> When using a multiline string body to send a multipart form data, files content must be inlined in the Hurl file.

### Cookies

Optional list of session cookies for this request.

A cookie consists of a name, followed by a `:` and a value. Cookies are sent per request, and are not added to
the cookie storage session, contrary to a cookie set in a header response. (for instance `Set-Cookie: theme=light`). The
cookies section starts with `[Cookies]`.

```hurl
GET https://example.org/index.html
[Cookies]
theme: light
sessionToken: abc123
```

Cookies section can be seen as syntactic sugar over corresponding request header.

```hurl
# Run a GET request with cookies section:
GET https://example.org/index.html
[Cookies]
theme: light
sessionToken: abc123

# Run the same GET request with a header:
GET https://example.org/index.html
Cookie: theme=light; sessionToken=abc123
```

### Basic Authentication

A basic authentication section can be used to perform [basic authentication].

Username is followed by a `:` and a password. The basic authentication section starts with
`[BasicAuth]`. Username and password are _not_ base64 encoded.


```hurl
# Perform basic authentication with login `bob` and password `secret`.
GET https://example.org/protected
[BasicAuth]
bob: secret
```

> Spaces surrounded username and password are trimmed. If you
> really want a space in your password (!!), you could use [Hurl unicode literals \u{20}].

This is equivalent (but simpler) to construct the request with a [Authorization] header:

```hurl
# Authorization header value can be computed with `echo -n 'bob:secret' | base64`
GET https://example.org/protected
Authorization: Basic Ym9iOnNlY3JldA== 
```

Basic authentication allows per request authentication.
If you want to add basic authentication to all the requests of a Hurl file
you can use [`-u/--user` option].

### Body

Optional HTTP body request.

If the body of the request is a [JSON] string or a [XML] string, the value can be
directly inserted without any modification. For a text based body that is neither JSON nor XML,
one can use [multiline string body] that starts with <code>&#96;&#96;&#96;</code> and ends
with <code>&#96;&#96;&#96;</code>. Multiline string body support "language hint" and can be used
to create [GraphQL queries].

For a precise byte control of the request body, [Base64] encoded string, [hexadecimal string]
or [included file] can be used to describe exactly the body byte content.

> You can set a body request even with a `GET` body, even if this is not a common practice.

The body section must be the last section of the request configuration.

#### JSON body

JSON request body is used to set a literal JSON as the request body.

```hurl
# Create a new doggy thing with JSON body:
POST https://example.org/api/dogs
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```

JSON request body can be [templatized with variables]:

```hurl
# Create a new catty thing with JSON body:
POST https://example.org/api/cats
{
    "id": 42,
    "lives": {{ lives_count }},
    "name": "{{ name }}"
}
```

Escapes are not processed (and particularly [Hurl Unicode literals] are not supported): `\n` is two consecutive 
chars (`\` followed by a `n`), not a single newline char:

```hurl
# Create a new catty thing with JSON body:
POST https://example.org/api/cats
{
    "text1": "\n is two chars \ and n",
    "text2": "\u{1D11E} is nine chars"
}
```


When using JSON request body, the content type `application/json` is automatically set.

JSON request body can be seen as syntactic sugar of [multiline string body] with `json` identifier:

~~~hurl
# Create a new doggy thing with JSON body:
POST https://example.org/api/dogs
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

If you don't want templates to be evaluated inside JSON body you can use [multiline string body] with `raw` identifier:

~~~hurl
# {{name}} is not a variable
POST https://example.org/api/cats
Content-Type: application/json
```raw
{
  "id": 42,
  "name": "{{ name }}"
}
```
~~~

#### XML body

XML request body is used to set a literal XML as the request body.

~~~hurl
# Create a new soapy thing XML body:
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
Content-Length: 299
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="http://example.net">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
~~~

Like JSON body, escapes are not processed (`\n` is two consecutive `\` followed by a `n`).

XML request body can be seen as syntactic sugar of [multiline string body] with `xml` identifier:

~~~hurl
# Create a new soapy thing XML body:
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
Content-Length: 299
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
```xml
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="http://example.net">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
```
~~~

> Contrary to JSON body, the succinct syntax of XML body can not use variables. If you need to use variables in your
> XML body, use a simple [multiline string body] with variables.

#### GraphQL query

GraphQL query uses [multiline string body] with `graphql` identifier:


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

GraphQL query body can use [GraphQL variables]:

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

GraphQL query, as every multiline string body, can use Hurl variables.

~~~hurl
POST https://example.org/starwars/graphql
```graphql
{
  human(id: "{{human_id}}") {
    name
    height(unit: FOOT)
  }
}
```
~~~

> Hurl variables and GraphQL variables can be mixed in the same body.

#### Multiline string body

For text based body that are neither JSON nor XML, one can use multiline string, started and ending with
<code>&#96;&#96;&#96;</code>.

~~~hurl
POST https://example.org/models
```
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL! air, moon roof, loaded",4799.00
```
~~~

The standard usage of a multiline string is:

~~~
```
line1
line2
line3
```
~~~

is evaluated as "line1\nline2\nline3\n".

Multiline string body can be [templatized with variables]:

~~~hurl
POST https://example.org/models
[Options]
var1: lemon
var2: yellow
```
Fruit,Color
{{var1}},{{var2}}
```
~~~

Escapes are not processed (i.e. [Hurl Unicode literals] are not supported): `\n` is two consecutive
chars (`\` followed by a `n`), not a single newline char.

Multiline string body can use language identifier, like `json`, `xml`, `graphql` or `raw`. Depending on the language identifier,
an additional 'Content-Type' request header is sent, and the real body (bytes sent over the wire) can be different from the 
raw multiline text.

~~~hurl
POST https://example.org/api/dogs
```json
{
    "id": 0,
    "name": "Frieda"
}
```
~~~

Raw multiline string body don't evaluate templates:

~~~hurl
# {{name}} is not a variable
POST https://example.org/api/cats
Content-Type: application/json
```raw
{
    "id": 42,
    "lives": {{ lives_count }},
    "name": "{{ name }}"
}
```
~~~

#### Oneline string body

For text based body that do not contain newlines, one can use oneline string, started and ending with <code>&#96;</code>.

~~~hurl
POST https://example.org/helloworld
`Hello world!`
~~~

#### Base64 body

Base64 body is used to set binary data as the request body.

Base64 body starts with `base64,` and end with `;`. MIME's Base64 encoding is supported (newlines and white spaces may be
present anywhere but are to be ignored on decoding), and `=` padding characters might be added.

```hurl
POST https://example.org
# Some random comments before body
base64,TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIG
FkaXBpc2NpbmcgZWxpdC4gSW4gbWFsZXN1YWRhLCBuaXNsIHZlbCBkaWN0dW0g
aGVuZHJlcml0LCBlc3QganVzdG8gYmliZW5kdW0gbWV0dXMsIG5lYyBydXRydW
0gdG9ydG9yIG1hc3NhIGlkIG1ldHVzLiA=;
```

#### Hex body

Hex body is used to set binary data as the request body.

Hex body starts with `hex,` and end with `;`.

```hurl
PUT https://example.org
# Send a café, encoded in UTF-8
hex,636166c3a90a;
```

#### File body

To use the binary content of a local file as the body request, file body can be used. File body starts with
`file,` and ends with `;``

```hurl
POST https://example.org
# Some random comments before body
file,data.bin;
```

File are relative to the input Hurl file, and cannot contain implicit parent directory (`..`). You can use  
[`--file-root` option] to specify the root directory of all file nodes.

[method]: #method
[URL]: #url
[headers]: #headers
[Headers]: #headers
[query parameters]: #query-parameters
[form parameters]: #form-parameters
[multipart form data]: #multipart-form-data
[cookies]: #cookies
[basic authentication]: #basic-authentication
[body]: #body
[query parameters section]: #query-parameters
[HTML form]: https://developer.mozilla.org/en-US/docs/Learn/Forms
[multiline string body]: #multiline-string-body
[oneline string body]: #oneline-string-body
[body section]: #body
[multipart/form-data on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST
[`--file-root` option]: /docs/manual.md#file-root
[JSON]: https://www.json.org
[XML]: https://en.wikipedia.org/wiki/XML
[Base64]: https://en.wikipedia.org/wiki/Base64
[hexadecimal string]: #hex-body
[included file]: #file-body
[`--file-root` option]: /docs/manual.md#file-root
[`-u/--user` option]: /docs/manual.md#user
[Hurl unicode literals \u{20}]: /docs/hurl-file.md#special-characters-in-strings
[Authorization]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
[`--location`]: /docs/manual.md#location
[`--verbose`]: /docs/manual.md#verbose
[`--insecure`]: /docs/manual.md#insecure
[templatized with variables]: /docs/templates.md#templating-body
[GraphQL queries]: #graphql-query
[GraphQL variables]: https://graphql.org/learn/queries/#variables
[options]: #options
[Hurl Unicode literals]: /docs/hurl-file.md#special-characters-in-strings
