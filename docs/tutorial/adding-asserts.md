# Adding Asserts

Our basic Hurl file is now:

```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:3000
HTTP 200
```

Currently, we're just checking that our home page is responding with a `200 OK` HTTP status code.
But we also want to check the _content_ of our home page, to ensure that everything is ok. To check the response
of an HTTP request with Hurl, we have to _describe_ tests that the response content must pass.

To do so, we're going to use [asserts].

As our endpoint <http://localhost:3000> is serving HTML content, it makes sense to use [XPath asserts].
If we want to test a REST API or any sort of API that serves JSON content,
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

1. Asserts are written in an `[Asserts]` section, so modify `basic.hurl` file:

```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Box"
```

2. Run `basic.hurl`:

```shell
$ hurl --test basic.hurl
[1mbasic.hurl[0m: [1;32mSuccess[0m (1 request(s) in 10 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 1 (100.0/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          10 ms

```

There is no error so everything is good!

3. Modify the test value to "Movies Bax"

```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Bax"
```

4. Run `basic.hurl`:

```shell
$ hurl --test basic.hurl
[1;31merror[0m: [1mAssert failure[0m
  [1;34m-->[0m basic.hurl:6:0
[1;34m   |[0m
[1;34m   |[0m [90mGET http://localhost:3000[0m
[1;34m   |[0m[90m ...[0m
[1;34m 6 |[0m xpath "string(//head/title)" == "Movies Bax"
[1;34m   |[0m[1;31m   actual:   string <Movies Box>[0m
[1;34m   |[0m[1;31m   expected: string <Movies Bax>[0m
[1;34m   |[0m

[1mbasic.hurl[0m: [1;31mFailure[0m (1 request(s) in 7 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 1 (125.0/s)
Succeeded files:   0 (0.0%)
Failed files:      1 (100.0%)
Duration:          8 ms
```

Hurl has failed now and provides information on which assert is not valid.

### Typed predicate

Decompose our assert:

- __`xpath "string(//head/title)"`__    
  is the XPath query 
- __`== "Movies Box"`__    
  is our predicate to test the query against

You can note that tested values are typed:

- `xpath "string(//head/title)" ==` __`"true"`__    
  tests that the XPath expression is returning a string, and this string is equal to the string `true`
- `xpath "boolean(//head/title)" ==` __`true`__    
  tests that the XPath expression is returning a boolean, and the boolean is `true`

Some queries can also return collections. For instance, the XPath expression `//button` is returning all the button
elements present in the [DOM]. We can use it to ensure that we have exactly two `<h3>` tag on our home page,
with [`count`]:

1. Add a new assert in `basic.hurl` to check the number of `<h3>` tags:

```hurl
# Checking our home page:
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Box"
xpath "//h3" count == 2
```

2. We can also check each `<h3>`'s content:

```hurl
# Checking our home page:
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Box"
xpath "//h3" count == 2
xpath "string((//h3)[1])" contains "Popular"
xpath "string((//h3)[2])" contains "Featured Today"
```

> XPath queries can sometimes be a little tricky to write but modern browsers can help writing these expressions.
> Try open the JavaScript console of your browser (Firefox, Safari or Chrome) and type `$x("string(//head/title)")`
> then press Return. You should see the result of your XPath query.

3. Run `basic.hurl` and check that every assert has been successful:

```shell
$ hurl --test basic.hurl
[1mbasic.hurl[0m: [1;32mSuccess[0m (1 request(s) in 10 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 1 (100.0/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          10 ms
```


## HTTP Headers Test

We are also going to add tests on the HTTP response headers with explicit [`header` asserts].
As our endpoint is serving UTF-8 encoded HTML content, we can check the value of the [`Content-Type` response header].

1. Add a new assert at the end of `basic.hurl` to test the value of the `Content-Type` HTTP header:

```hurl
# Checking our home page:
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Box"
xpath "//h3" count == 2
xpath "string((//h3)[1])" contains "Popular"
xpath "string((//h3)[2])" contains "Featured Today"
# Testing HTTP response headers:
header "Content-Type" == "text/html; charset=utf-8"
```

> Our HTTP response has only one `Content-Type` header, so we're testing this header value as string.
> The same header could be present multiple times in an HTTP response, with different values.
> In this case, the `header` query will return collections and could be tested with
> [`count`] or [`includes`].

For HTTP headers, we can also use an [implicit header assert]. You can use either implicit or
explicit header assert: the implicit one allows you to only check the exact value of the header,
while the explicit one allows you to use other [predicates] (like `contains`, `startsWith`, `matches` etc...).

2. Replace the explicit assert with [implicit header assert]:

```hurl
# Checking our home page:
GET http://localhost:3000
HTTP 200
# Implicitly testing response headers:
Content-Type: text/html; charset=utf-8
[Asserts]
xpath "string(//head/title)" == "Movies Box"
xpath "//h3" count == 2
xpath "string((//h3)[1])" contains "Popular"
xpath "string((//h3)[2])" contains "Featured Today"
```

The line 

`Content-Type: text/html; charset=utf-8` 

is testing that the header `Content-Type` is present in the response,
and its value must be exactly `text/html; charset=utf-8`.

> In the implicit assert, quotes in the header value are part of the value itself.

Finally, we want to check that our server is creating a new session.

When creating a new session, our Express application returns a [`Set-Cookie` HTTP response header].
So to test it, we can modify our Hurl file with another header assert.

3. Add a header assert on `Set-Cookie` header:

```hurl
# Checking our home page:
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Box"
xpath "//h3" count == 2
xpath "string((//h3)[1])" contains "Popular"
xpath "string((//h3)[2])" contains "Featured Today"
# Testing HTTP response headers:
header "Content-Type" == "text/html; charset=utf-8"
header "Set-Cookie" startsWith "x-session-id="
```

For `Set-Cookie` header, we can use the specialized [Cookie assert].
Not only we'll be able to easily tests [cookie attributes] (like `HttpOnly`, or `SameSite`), but also
it simplifies tests on cookies, particularly when there are multiple `Set-Cookie` header in the HTTP response.

> Hurl is not a browser, one can see it as syntactic sugar over [curl]. Hurl
> has no JavaScript runtime and stays close to the HTTP layer. With others tools relying on headless browser, it can be
> difficult to access some HTTP requests attributes, like `Set-Cookie` header.

So to test that our server is responding with a `HttpOnly` session cookie, we can modify our file and add cookie asserts.

4. Add two cookie asserts on the cookie `x-session-id`:

```hurl
# Checking our home page:
GET http://localhost:3000
HTTP 200
[Asserts]
xpath "string(//head/title)" == "Movies Box"
xpath "//h3" count == 2
xpath "string((//h3)[1])" contains "Popular"
xpath "string((//h3)[2])" contains "Featured Today"
# Testing HTTP response headers:
header "Content-Type" == "text/html; charset=utf-8"
cookie "x-session-id" exists
cookie "x-session-id[HttpOnly]" exists
```

5. Run `basic.hurl` and check that every assert has been successful:

```shell
$ hurl --test basic.hurl
[1mbasic.hurl[0m: [1;32mSuccess[0m (1 request(s) in 7 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 1 (125.0/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          8 ms
```

## Performance Test

> TODO: add duration assert

## Recap

Our Hurl file is now around 10 lines long, but we're already testing a lot on our home page:

- we are testing that our home page is responding with a `200 OK`
- we are checking the basic structure of our page: a title, 2 `<h3>` tags
- we are checking that the content type is UTF-8 HTML
- we are checking that our server has created a session, and that the cookie session has the `HttpOnly` attribute

You can see now that launching and running requests with Hurl is fast, _really_ fast.

In the next session, we're going to see how we chain request tests, and how we add basic check on a REST API.

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
[`count`]: /docs/filters.md#count
[`includes`]: /docs/asserting-response.md#predicates

