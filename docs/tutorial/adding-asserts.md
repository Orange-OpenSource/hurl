# Adding Asserts

Our basic Hurl file is now:

```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:8080

HTTP/1.1 200
```

Currently, we're just checking that our home page is responding with a `200 OK` HTTP status code.
But we also want to check the _content_ of our home page, to ensure that everything is ok. To check the response
of an HTTP request with Hurl, we have to _describe_ tests that the response content must pass.

> We're already implicitly asserting the response with the line\
> `HTTP/1.1 200`\
> On one hand, we are checking that the HTTP protocol version is 1.1; on the other hand, we are checking
> that the HTTP status response code is 200.

To do so, we're going to use [asserts].

As our endpoint <http://localhost:8080> is serving HTML content, it makes sense to use [XPath asserts].
If we want to test a REST api or any sort of api that serves JSON content,
we could use [JSONPath asserts] instead. There are other type of asserts but every one shares
the same structure. So, let's look how to write a [XPath asserts].

## HTML Body Test

### Structure of an assert

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">xpath "string(//h1)"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">contains<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">"Hello"<span class="schema-label">predicate value</span></span>
 </div>
</div>

An assert consists of a query and a predicate. As we want to test the value of the HTML title tag, we're
going to use the [XPath expression] `string(//head/title)`.

1. Asserts are written in an Asserts section, so modify `basic.hurl` file:

```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
```

2. Run `basic.hurl`:

```shell
$ hurl --test basic.hurl
basic.hurl: RUNNING [1/1]
basic.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  14ms
```

There is no error so everything is good!

3. Modify the predicate value to "Welcome to Quaz!"

```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quaz!"
```

4. Run `basic.hurl`:

```shell
$ hurl --test basic.hurl
error: Assert Failure
  --> integration/basic.hurl:6:0
   |
 6 | xpath "string(//head/title)" == "Welcome to Quaz!"
   |   actual:   string <Welcome to Quiz!>
   |   expected: string <Welcome to Quaz!>
   |
```

Hurl has failed now and provides informations on which assert is not valid.

### Typed predicate

If we decompose our assert, `xpath "string(//head/title)"` is the XPath query and `== "Welcome to Quiz!"` is our
predicate to test the query against. You can note that predicates values are typed:

- `xpath "string(//head/title)" == "true"`    
  tests that the XPath expression is returning a string, and
- `xpath "boolean(//head/title)" == true`     
  tests that the XPath expression is returning a boolean

Some queries can also return collections. For instance, the XPath expression `//button` is returning all the button
elements present in the [DOM]. We can use it to ensure that we have exactly two buttons on our home page,
with `count`:

1. Add a new assert in `basic.hurl` to check the number of buttons:

```hurl
# Checking our home page:
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
xpath "//button" count == 2
```

2. We can also check each button's title:

```hurl
# Checking our home page:
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
xpath "//button" count == 2
xpath "string((//button)[1])" contains "Play"
xpath "string((//button)[2])" contains "Create"
```

> XPath queries can sometimes be a little tricky to write but modern browsers can help writing these expressions.
> Try open the Javascript console of your browser (Firefox, Safari or Chrome) and type `$x("string(//head/title)")`
> then press Return. You should see the result of your XPath query.

3. Run `basic.hurl` and check that every assert has been successful:

```shell
$ hurl --test basic.hurl
basic.hurl: RUNNING [1/1]
basic.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  14ms
```


## HTTP Headers Test

We are also going to add tests on the HTTP response headers with explicit [`header` asserts].
As our endpoint is serving UTF-8 encoded HTML content, we can check the value of the [`Content-Type` response header].

1. Add a new assert at the end of `basic.hurl` to test the value of the `Content-Type` HTTP header:

```hurl
# Checking our home page:
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
xpath "//button" count == 2
xpath "string((//button)[1])" contains "Play"
xpath "string((//button)[2])" contains "Create"
# Testing HTTP response headers:
header "Content-Type" == "text/html;charset=UTF-8"
```

> Our HTTP response has only one `Content-Type` header, so we're testing this header value as string.
> The same header could be present multiple times in an HTTP response, with different values.
> In this case, the `header` query will return collections and could be tested with
> `countEqual` or `include` predicates.

For HTTP headers, we can also use an [implicit header assert]. You can use indifferently implicit or
explicit header assert: the implicit one allows you to only check the exact value of the header,
while the explicit one allows you to use other [predicates] (like `contains`, `startsWith`, `matches` etc...).

2. Replace the explicit assert with [implicit header assert]:

```hurl
# Checking our home page:
GET http://localhost:8080

HTTP/1.1 200
# Implicitely testing response headers:
Content-Type: text/html;charset=UTF-8
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
xpath "//button" count == 2
xpath "string((//button)[1])" contains "Play"
xpath "string((//button)[2])" contains "Create"
```

The line `Content-Type: text/html;charset=UTF-8` is testing that the header `Content-Type` is present in the response,
and its value must be exactly `text/html;charset=UTF-8`.

> In the implicit assert, quotes in the header value are part of the value itself.

Finally, we want to check that our server is creating a new session.

When creating a new session, our Spring Boot application should return a [`Set-Cookie` HTTP response header].
So to test it, we can modify our Hurl file with another header assert.

3. Add a header assert on `Set-Cookie` header:

```hurl
# Checking our home page:
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
xpath "//button" count == 2
xpath "string((//button)[1])" contains "Play"
xpath "string((//button)[2])" contains "Create"
# Testing HTTP response headers:
header "Content-Type" == "text/html;charset=UTF-8"
header "Set-Cookie" startsWith "JSESSIONID="
```

For `Set-Cookie` header, we can use the specialized [Cookie assert].
Not only we'll be able to easily tests [cookie attributes] (like `HttpOnly`, or `SameSite`), but also
it simplifies tests on cookies, particularly when there are multiple `Set-Cookie` header in the HTTP response.

> Hurl is not a browser, one can see it as syntactic sugar over [curl]. Hurl
> has no Javascript runtime and stays close to the HTTP layer. With others tools relying on headless browser, it can be
> difficult to access some HTTP requests attributes, like `Set-Cookie` header.

So to test that our server is responding a `HttpOnly` session cookie, we can modify our file and add cookie asserts.

4. Add two cookie asserts on the cookie `JESSIONID`:

```hurl
# Checking our home page:
GET http://localhost:8080

HTTP/1.1 200
[Asserts]
xpath "string(//head/title)" == "Welcome to Quiz!"
xpath "//button" count == 2
xpath "string((//button)[1])" contains "Play"
xpath "string((//button)[2])" contains "Create"
# Testing content type:
header "Content-Type" == "text/html;charset=UTF-8"
# Testing session cookie:
cookie "JSESSIONID" exists
cookie "JSESSIONID[HttpOnly]" exists
```

5. Run `basic.hurl` and check that every assert has been successful:

```shell
$ hurl --test basic.hurl
basic.hurl: RUNNING [1/1]
basic.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  16ms
```

## Performance Test

> TODO: add duration assert

## Recap

Our Hurl file is now around 10 lines long, but we're already testing a lot on our home page:

- we are testing that our home page is responding with a `200 OK`
- we are checking the basic structure of our page: a title, 2 buttons
- we are checking that the content type is UTF-8 HTML
- we are checking that our server has created a session, and that the cookie session has the `HttpOnly` attribute

You can see now that launching and running requests with Hurl is fast, _really_ fast.

In the next session, we're going to see how we chain request tests, and how we add basic check on a REST api.

[asserts]: /docs/asserting-response.md
[XPath asserts]: /docs/asserting-response.md#xpath-assert
[JSONPath asserts]: /docs/asserting-response.md#jsonpath-assert
[XPath expression]: https://en.wikipedia.org/wiki/XPath
[DOM]: https://en.wikipedia.org/wiki/Document_Object_Model
[`header` asserts]: /docs/asserting-response.md#header-assert
[`Content-Type` response header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type
[implicit header assert]: /docs/asserting-response.md#headers
[predicates]: /docs/asserting-response.md#predicates
[`Set-Cookie` HTTP response header]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#creating_cookies
[Cookie assert]: /docs/asserting-response.md#cookie-assert
[cookie attributes]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#attributes
[curl]: https://curl.se

