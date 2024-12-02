# Chaining Requests

## Adding Another Request

Our basic Hurl file is for the moment:

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

We're only running one HTTP request and have already added lots of tests on the response. Don't hesitate to add
many tests, the more asserts you write, the less fragile your tests suite is.

Now, we want to perform other HTTP requests and keep adding tests. In the same file, we can simply write another
request following our first request. Let's say we want to test that our server returns a [404 page] on a broken link:

1. Modify `basic.hurl` to add a second request on a broken URL:

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


# Check that we have a 404 response for broken links:
GET http://localhost:3000/not-found
HTTP 404
[Asserts]
header "Content-Type" == "text/html; charset=utf-8"
xpath "string(//h2)" == "Error"
xpath "string(//h3)" == "Not Found"
```

Now, we have two entries in our Hurl file: each entry is composed of one request and one expected response
description.

> In a Hurl file, response descriptions are optional. We could also have written
> our file with only requests:
>
> ```hurl
> GET http://localhost:3000
> GET http://localhost:3000/not-found
> ```
> But it would have performed nearly zero test. This type of Hurl file can be useful
> if you use Hurl to get data for instance.

2. Run `basic.hurl`:

```shell
$ hurl --test basic.hurl
[1mbasic.hurl[0m: [1;32mSuccess[0m (2 request(s) in 21 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 2 (90.9/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          22 ms
```

We can see that the test is still OK. Now two requests are being run in sequence, and each response can be
tested independently.

## Test REST API

So far we have tested two HTML endpoints. We're going to see now how to test a REST API.

Our website exposes a health REST resource, available at <http://localhost:3000/api/health>.
Let's use Hurl to check it.

1. In a shell, use Hurl to test the </api/health> endpoint:

```shell
$ echo 'GET http://localhost:3000/api/health' | hurl
{"status":"RUNNING","healthy":true,"operationId":6212054377712155,"reportedDate":"2023-07-21T16:11:24.053Z"}
```

Being a classic CLI application, we can use the standard input with Hurl instead of a file to provide requests
to be executed, and pipe the result to various tools like [`jq`]:

```shell
$ echo 'GET http://localhost:3000/api/health' | hurl | jq
[1m{
  [0m[1;34m"status"[0m[1m: [0m[32m"RUNNING"[0m[1m,
  [0m[1;34m"healthy"[0m[1m: [0m[34mtrue[0m[1m,
  [0m[1;34m"operationId"[0m[1m: [0m[34m8629192252836205[0m[1m,
  [0m[1;34m"reportedDate"[0m[1m: [0m[32m"2023-08-04T11:04:52.516Z"[0m[1m
[1m}[0m
```

We can test our health API it with a [JSONPath assert]. JSONPath asserts have the same structure as XPath asserts: a query
followed by a test. A [JSONPath query] is a simple expression to inspect a JSON object.

2. Modify `basic.hurl` to add a third request that asserts our </api/health> REST API:

```hurl
# Checking our home page:
# ...

# Check that we have a 404 response for broken links:
# ...

# Check our health API:
GET http://localhost:3000/api/health
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$.status" == "RUNNING"
jsonpath "$.healthy" == true
jsonpath "$.operationId" exists
```

Like XPath assert, JSONPath predicate values are typed. Strings, booleans, numbers, dates and
collections are supported. 

Let's practice writing JSONPath asserts by using another API. 

In our Movies Box website, user can search movies using different criteria like actor names, director names or 
released date. The search page is exposed at <http://localhost:3000/search>. Go to the search page and type "1982": you 
will see some movies that have been released in 1982. Our server exposed a REST API at 
<http://localhost:3000/api/search> and the search page use a [XHR] to get the search results. You can see the XHR in 
action by using the Developer Tools of your browser: 

<div class="picture">
    <picture>
        <source srcset="/docs/assets/img/developer-tools.avif" type="image/avif">
        <source srcset="/docs/assets/img/developer-tools.webp" type="image/webp">
        <source srcset="/docs/assets/img/developer-tools.png" type="image/png">
        <img class="u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/developer-tools.png" alt="Firefox Developer Tool"/>
    </picture>
</div>

We can use this REST API to add checks on search results through the API endpoint.

3. Add JSONPath asserts on the </api/search> REST APIs:

```hurl
# Checking our home page:
# ...

# Check that we have a 404 response for broken links:
# ...

# Check our health API:
# ...

# Check search API:
GET http://localhost:3000/api/search?q=1982&sort=name
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$" count == 5
jsonpath "$[0].name" == "Blade Runner"
jsonpath "$[0].director" == "Ridley Scott"
jsonpath "$[0].release_date" == "1982-06-25"
```

> To keep things simple in this tutorial, we have mocked data
> in our "Movies Box" application. That's something you don't want to do when building
> your application, you want to build an app production ready. A better way to
> do this should have been to expose a "debug" or "integration" mode on our app
> defined by environment variables. If our app is launched in "integration" mode,
> mocked data is used and asserts can be tested on known values. Our app could also use
> a mocked database, configured in our tests suits.

Note that the search API use query parameters `q` and `sort` that's why we have written the URL with
query parameters <http://localhost:3000/api/search?q=1982&sort=name>. We can set the query parameters
in the URL, or use a [query parameter section].

4. Use a query parameter section in `basic.hurl`:

```hurl
# Checking our home page:
# ...

# Check that we have a 404 response for broken links:
# ...

# Check our health API:
# ...

# Check search API:
GET http://localhost:3000/api/search
[QueryStringParams]
q: 1982
sort: name
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$" count == 5
jsonpath "$[0].name" == "Blade Runner"
jsonpath "$[0].director" == "Ridley Scott"
jsonpath "$[0].release_date" == "1982-06-25"
```

For the moment, we have just tested that values returned by the server are equals to expected values. You can also
use other type of assertions like [`startsWith`], [`endsWith`], [`contains`], [`matches`] etc... For instance, we could
test that the `release_date` of Blade Runner is 1982:

5. Use `startsWith` to test the release date: 

```hurl
# Checking our home page:
# ...

# Check that we have a 404 response for broken links:
# ...

# Check our health API:
# ...

# Check search API:
GET http://localhost:3000/api/search
[QueryStringParams]
q: 1982
sort: name
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$" count == 5
jsonpath "$[0].name" == "Blade Runner"
jsonpath "$[0].director" == "Ridley Scott"
jsonpath "$[0].release_date" startsWith "1982"
```

We could make our test stricter by validating the format of `release_date`. By using [filters], we can transform query
values. We're already using a filter, [`count`] that returns the number of elements in a collection. Now we are 
going to use a `regex` filter to extract part of a string:

5. Use a `regex` filter to test the release date:

```hurl
# Checking our home page:
# ...

# Check that we have a 404 response for broken links:
# ...

# Check our health API:
# ...

# Check search API:
GET http://localhost:3000/api/search
[QueryStringParams]
q: 1982
sort: name
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$" count == 5
jsonpath "$[0].name" == "Blade Runner"
jsonpath "$[0].director" == "Ridley Scott"
jsonpath "$[0].release_date" regex /(\d{4})-\d{2}-\d{2}/ == "1982"
```

Let's decompose our final assert: 

- __`jsonpath "$[0].release_date"`__    
  this is the JSONPath query that extracts some date from our response
- __`regex /(\d{4})-\d{2}-\d{2}/"`__    
  this is a regex filter with a [regular expression] `/(\d{4})-\d{2}-\d{2}/`. Regular 
expression can be written with `/.../` like in JavaScript for instance. Note that the regular expression has a
capture group `(\d{4})` that will extract the 4 digits year from the previous query
- __`== "1982"`__    
  this is our test value

As you can see, [filters] are very powerful; they can be combined to refine values for better tests. 


Finally, our basic Hurl file with four HTTP requests looks like:

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


# Check that we have a 404 response for broken links:
GET http://localhost:3000/not-found
HTTP 404
[Asserts]
header "Content-Type" == "text/html; charset=utf-8"
xpath "string(//h2)" == "Error"
xpath "string(//h3)" == "Not Found"


# Check our health API:
GET http://localhost:3000/api/health
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$.status" == "RUNNING"
jsonpath "$.healthy" == true
jsonpath "$.operationId" exists


# Check search API:
GET http://localhost:3000/api/search
[QueryStringParams]
q: 1982
sort: name
HTTP 200
[Asserts]
header "Content-Type" == "application/json; charset=utf-8"
jsonpath "$" count == 5
jsonpath "$[0].name" == "Blade Runner"
jsonpath "$[0].director" == "Ridley Scott"
jsonpath "$[0].release_date" regex /(\d{4})-\d{2}-\d{2}/ == "1982"
```

6. Run `basic.hurl` and check that every assert of every request has been successful:

```shell
$ hurl --test basic.hurl
[1mbasic.hurl[0m: [1;32mSuccess[0m (4 request(s) in 21 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 4 (181.8/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          22 ms
```

## Recap

We can simply chain requests with Hurl, adding asserts on every response. As your Hurl file will grow,
don't hesitate to add many comments: your Hurl file will be a valuable and testable documentation
for your applications.

[404 page]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404
[JSONPath assert]: /docs/asserting-response.md#jsonpath-assert
[JSONPath query]: https://goessner.net/articles/JsonPath/
[query parameter section]: /docs/request.md#query-parameters
[`--test`]: /docs/manual.md#test
[XHR]: https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest
[`startsWith`]: /docs/asserting-response.md#predicates
[`endsWith`]: /docs/asserting-response.md#predicates
[`contains`]: /docs/asserting-response.md#predicates
[`matches`]: /docs/asserting-response.md#predicates
[filters]: /docs/filters.md
[`count`]: /docs/filters.md#count
[regular expression]: https://en.wikipedia.org/wiki/Regular_expression
[`jq`]: https://github.com/jqlang/jq
