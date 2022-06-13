<div class="home-logo">
    <img class="light-img" src="/docs/assets/img/logo-light.svg" width="277px" height="72px" alt="Hurl logo"/>
    <img class="dark-img" src="/docs/assets/img/logo-dark.svg" width="277px" height="72px" alt="Hurl logo"/>
</div>



# What's Hurl?

Hurl is a command line tool that runs <b>HTTP requests</b> defined in a simple <b>plain text format</b>.

It can perform requests, capture values and evaluate queries on headers and body response. Hurl is very
versatile: it can be used for both <b>fetching data</b> and <b>testing HTTP</b> sessions.

```hurl
# Get home:
GET https://example.org

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"

# Do login!
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}

HTTP/1.1 302
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

It is well adapted for <b>REST / JSON apis</b>

```hurl
POST https://example.org/api/tests
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

<b>HTML content</b>

```hurl
GET https://example.org

HTTP/1.1 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"
```

and even SOAP apis

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

Hurl can also be used to test HTTP endpoints performances:

```hurl
GET https://example.org/api/v1/pets

HTTP/1.0 200
[Asserts]
duration < 1000  # Duration in ms
```

And responses bytes content

```hurl
GET https://example.org/data.tar.gz

HTTP/1.0 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```


# Why Hurl?

<ul class="showcase-container">
 <li class="showcase-item"><h2 class="showcase-item-title">Text Format</h2>For both devops and developers</li>
 <li class="showcase-item"><h2 class="showcase-item-title">Fast CLI</h2>A command line for local dev and continuous integration</li>
 <li class="showcase-item"><h2 class="showcase-item-title">Single Binary</h2>Easy to install, with no runtime required</li>
</ul>

# Powered by curl

Hurl is a lightweight binary written in [Rust]. Under the hood, Hurl HTTP engine is
powered by [libcurl], one of the most powerful and reliable file transfer library.
With its text file format, Hurl adds syntactic sugar to run and tests HTTP requests,
but it's still the [curl] that we love.

# Feedbacks

[Feedback, suggestion, bugs or improvements] are welcome!

```hurl
POST https://hurl.dev/api/feedback
{
  "name": "John Doe",
  "feedback": "Hurl is awesome !"
}
HTTP/1.1 200
```

# Resources

[License]

[Blog]

[Tutorial]

[Documentation]

[GitHub]

[XPath]: https://en.wikipedia.org/wiki/XPath
[JSONPath]: https://goessner.net/articles/JsonPath/
[Rust]: https://www.rust-lang.org
[curl]: https://curl.se
[the installation section]: /docs/installation.md
[Feedback, suggestion, bugs or improvements]: https://github.com/Orange-OpenSource/hurl/issues
[License]: /docs/license.md
[Tutorial]: /docs/tutorial/your-first-hurl-file.md
[Documentation]: /docs/installation.md
[Blog]: blog.md
[GitHub]: https://github.com/Orange-OpenSource/hurl
[libcurl]: https://curl.se/libcurl/
