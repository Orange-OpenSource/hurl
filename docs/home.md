<div class="home-logo">
    <img class="u-theme-light" src="/docs/assets/img/logo-light.svg" width="277px" height="72px" alt="Hurl logo"/>
    <img class="u-theme-dark" src="/docs/assets/img/logo-dark.svg" width="277px" height="72px" alt="Hurl logo"/>
</div>

# What's Hurl?

Hurl is a command line tool that runs <b>HTTP requests</b> defined in a simple <b>plain text format</b>.

It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very
versatile: it can be used for both <b>fetching data</b> and <b>testing HTTP</b> sessions.

Hurl makes it easy to work with <b>HTML</b> content, <b>REST / SOAP / GraphQL</b> APIs, or any other <b>XML / JSON</b> based APIs. 

```hurl
# Get home:
GET https://example.org
HTTP 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"


# Do login!
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
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

<div id="home-demo"></div>

It is well adapted for <b>REST / JSON APIs</b>

```hurl
POST https://example.org/api/tests
{
    "id": "4568",
    "evaluate": true
}
HTTP 200
[Asserts]
header "X-Frame-Options" == "SAMEORIGIN"
jsonpath "$.status" == "RUNNING"    # Check the status code
jsonpath "$.tests" count == 25      # Check the number of items
jsonpath "$.id" matches /\d{4}/     # Check the format of the id
```

<b>HTML content</b>

```hurl
GET https://example.org
HTTP 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"
```

<b>GraphQL</b> 

~~~hurl
POST https://example.org/graphql
```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```
HTTP 200
~~~

and even <b>SOAP APIs</b>

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

Hurl can also be used to test the <b>performance</b> of HTTP endpoints

```hurl
GET https://example.org/api/v1/pets
HTTP 200
[Asserts]
duration < 1000  # Duration in ms
```

And check response bytes

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

Finally, Hurl is easy to <b>integrate in CI/CD</b>, with text, JUnit, TAP and HTML reports

<div class="picture">
    <picture>
        <source srcset="/docs/assets/img/home-waterfall-light.avif" type="image/avif">
        <source srcset="/docs/assets/img/home-waterfall-light.webp" type="image/webp">
        <source srcset="/docs/assets/img/home-waterfall-light.png" type="image/png">
        <img class="u-theme-light u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/home-waterfall-light.png" width="480" alt="HTML report"/>
    </picture>
    <picture>
        <source srcset="/docs/assets/img/home-waterfall-dark.avif" type="image/avif">
        <source srcset="/docs/assets/img/home-waterfall-dark.webp" type="image/webp">
        <source srcset="/docs/assets/img/home-waterfall-dark.png" type="image/png">
        <img class="u-theme-dark u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/home-waterfall-dark.png" width="480" alt="HTML report"/>
    </picture>
</div>

# Why Hurl?

<ul class="showcase-container">
    <li class="showcase-item"><h2 class="showcase-item-title">Text Format</h2>For both devops and developers</li>
    <li class="showcase-item"><h2 class="showcase-item-title">Fast CLI</h2>A command line for local dev and continuous integration</li>
    <li class="showcase-item"><h2 class="showcase-item-title">Single Binary</h2>Easy to install, with no runtime required</li>
</ul>

# Powered by curl

Hurl is a lightweight binary written in [Rust]. Under the hood, Hurl HTTP engine is
powered by [libcurl], one of the most powerful and reliable file transfer libraries.
With its text file format, Hurl adds syntactic sugar to run and test HTTP requests,
but it's still the [curl] that we love: __fast__, __efficient__ and __HTTP/3 ready__.

# Feedbacks

To support its development, [star Hurl on GitHub]!

[Feedback, suggestion, bugs or improvements] are welcome.

```hurl
POST https://hurl.dev/api/feedback
{
  "name": "John Doe",
  "feedback": "Hurl is awesome!"
}
HTTP 200
```

# Resources

[License]

[Blog]

[Tutorial]

[Documentation] (download [HTML], [PDF], [Markdown]) 

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
[star Hurl on GitHub]: https://github.com/Orange-OpenSource/hurl/stargazers
[HTML]: /docs/standalone/hurl-6.0.0.html
[PDF]: /docs/standalone/hurl-6.0.0.pdf
[Markdown]: /docs/standalone/hurl-6.0.0.md
