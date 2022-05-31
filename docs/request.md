# Request

## Definition

Request describes an HTTP request: a mandatory [method] and [url], followed by optional [headers].

Then, [query parameters], [form parameters], [multipart form datas], [cookies] and
[basic authentication] can be used to configure the HTTP request.

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
            [QueryStringParams]<br>...
        </div>
        <div class="hurl-part-2">
            [FormParams]<br>...
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
            <a href="#query-parameters">Query strings</a>, <a href="#form-parameters">form params</a>, <a href="#cookies">cookies</a>, <a href="#basic-authentification">authentification</a> ...<br>(optional sections, unordered)
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


[Headers], if present, follow directly after the [method] and [url]. This allows Hurl format to 'look like' the real HTTP format.
Contrary to HTTP headers, other parameters are defined in sections (`[Cookies]`, `[QueryStringParams]`, `[FormParams]` etc...)
These sections are not ordered and can be mixed in any way:

```hurl
GET https://example.org/api/dogs
User-Agent: My User Agent
[QueryStringParams]
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
[QueryStringParams]
id: 4567
order: newest
```

The last optional part of a request configuration is the request [body]. Request body must be the last paremeter of a request
(after [headers] and request sections). Like headers, [body] have no explicit marker:

```hurl
POST https://example.org/api/dogs?id=4567
User-Agent: My User Agent
{
 "name": "Ralphy"
}
```

## Description

### Method

Mandatory HTTP request method, one of `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`,
`TRACE`, `PATCH`.

### URL

Mandatory HTTP request url.

Url can contain query parameters, even if using a [query parameters section] is preferred.

```hurl
# A request with url containing query parameters.
GET https://example.org/forum/questions/?search=Install%20Linux&order=newest

# A request with query parameters section, equivalent to the first request.
GET https://example.org/forum/questions/
[QueryStringParams]
search: Install Linux
order: newest
```

> Query parameters in query parameter section are not url encoded.

When query parameters are present in the url and in a query parameters section, the resulting request will
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

> Headers directly follow url, without any section name, contrary to query parameters, form parameters
> or cookies

Note that header usually don't start with double quotes. If the header value starts with double quotes, the double
quotes will be part of the header value:

```hurl
PATCH https://example.org/file.txt
If-Match: "e0023aa4e"
```

`If-Match` request header will be sent will the following value `"e0023aa4e"` (started and ended with double quotes).

Headers must follow directly after the [method] and [url].

### Query parameters

Optional list of query parameters.

A query parameter consists of a field, followed by a `:` and a value. The query parameters section starts with
`[QueryStringParams]`. Contrary to query parameters in the url, each value in the query parameters section is not
url encoded.

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
[QueryStringParams]
order: newest
search: {{custom-search}}
count: 100
```

If there are any parameters in the url, the resulted request will have both parameters.

### Form parameters

A form parameters section can be used to send data, like [HTML form].

This section contains an optional list of key values, each key followed by a `:` and a value. Key values will be
encoded in key-value tuple separated by '&', with a '=' between the key and the value, and sent in the body request.
The content type of the request is `application/x-www-form-urlencoded`. The form parameters section starts
with `[FormParams]`.

```hurl
POST https://example.org/contact
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

Form parameters section can be seen as syntactic sugar over body section (values in form parameters section
are not url encoded.). A [multiline string body] could be used instead of a forms parameters section.

~~~hurl
# Run a POST request with form parameters section:
POST https://example.org/test
[FormParams]
name: John Doe
key1: value1

# Run the same POST request with a body section:
POST https://example.org/test
Content-Type: application/x-www-form-urlencoded
```
name=John%20Doe&key1=value1
```
~~~

When both [body section] and form parameters section are present, only the body section is taken into account.

### Multipart Form Data

A multipart form data section can be used to send data, with key / value and file content
(see [multipart/form-data on MDN]).

The form parameters section starts with `[MultipartFormData]`.

```hurl
POST https://example.org/upload
[MultipartFormData]
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
# Perform basic authentification with login `bob` and password `secret`.
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
If you want to add basic authentication to all the request of a Hurl file
you can use [`-u/--user` option].

### Body

Optional HTTP body request.

If the body of the request is a [JSON] string or a [XML] string, the value can be
directly inserted without any modification. For a text based body that is not JSON nor XML,
one can use multiline string that starts with <code>&#96;&#96;&#96;</code> and ends
with <code>&#96;&#96;&#96;</code>.

For a precise byte control of the request body, [Base64] encoded string, [hexadecimal string]
or [included file] can be used to describe exactly the body byte content.

> You can set a body request even with a `GET` body, even if this is not a common practice.

The body section must be the last section of the request configuration.

#### JSON body

JSON body is used to set a literal JSON as the request body.

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

When using JSON body, the content type `application/json` is automatically set.

#### XML body

XML body is used to set a literal XML as the request body.

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

#### Raw string body

For text based body that are not JSON nor XML, one can used multiline string, started and ending with
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

The standard usage of a raw string is:

~~~
```
line1
line2
line3
```
~~~

is evaluated as "line1\nline2\nline3\n".


To construct an empty string:

~~~
```
```
~~~

or

~~~
``````
~~~


Finaly, raw string can be used without any newline:

~~~
```line``` 
~~~

is evaluated as "line".


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
[url]: #url
[headers]: #headers
[Headers]: #headers
[query parameters]: #query-parameters
[form parameters]: #form-parameters
[multipart form datas]: #multipart-form-data
[cookies]: #cookies
[basic authentication]: #basic-authentication
[body]: #body
[query parameters section]: #query-parameters
[HTML form]: https://developer.mozilla.org/en-US/docs/Learn/Forms
[multiline string body]: #multiline-string-body
[body section]: #body
[multipart/form-data on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST
[`--file-root` option]: /docs/man-page.md#file-root
[JSON]: https://www.json.org
[XML]: https://en.wikipedia.org/wiki/XML
[Base64]: https://en.wikipedia.org/wiki/Base64
[hexadecimal string]: #hex-body
[included file]: #file-body
[`--file-root` option]: /docs/man-page.md#file-root
[`-u/--user` option]: /docs/man-page.md#user
[Hurl unicode literals \u{20}]: /docs/hurl-file.md#special-character-in-strings
[Authorization]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization
