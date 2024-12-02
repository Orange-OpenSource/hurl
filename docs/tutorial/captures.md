# Captures

We have seen how to chain requests in a Hurl file. In some use cases, you want
to use data from one request and inject it in another one. That's what [captures]
are all about.

## Capturing a CSRF Token

In our website, a user can login at <http://localhost:3000/login>.
The HTML page is a [form] where the user can input:

- a required username
- a required password

If we look at the page HTML content, we can see an HTML form:

```html
<form class="login-form" method="post" action="/login">
    <input type="hidden" name="_csrf" value="0fSk7gRA-UTkS25Fbsyal0dgLPBjVy1YIoNg">
    ...
    <input type="text" name="username" id="username" autocomplete="off" minlength="3" maxlength="32" pattern="[a-zA-Z0-9_-]{3,32}" title="Username must use a-z, A-Z and 0-9" required="">
    ...
    <input type="password" name="password" id="password" autocomplete="off" minlength="6" maxlength="32" required="">
    ...
    <input type="submit" value="Login">
    ...
</form>
```

When the user clicks on 'Login' button, a POST request is sent with form values: the username and a password. 
Our server implements a [_Post / Redirect / Get pattern_]: if the POST submission is successful, the user is redirected 
to his favorites movies page.

Let's try to test it!

Form values can be sent using a [Form parameters section], with each key followed by its corresponding value.

1. Create a new file named `login.hurl`:

```hurl
POST http://localhost:3000/login
[FormParams]
username: fab
password: 12345678
HTTP 302
```

> When sending form data with a Form parameters section, you don't need to set the
> `Content-Type` HTTP header: Hurl infers that the content type of the request is `application/x-www-form-urlencoded`.

2. Run `login.hurl`:

```shell
$ hurl --test login.hurl
[1;31merror[0m: [1mAssert status code[0m
  [1;34m-->[0m login.hurl:5:6
[1;34m   |[0m
[1;34m   |[0m [90mPOST http://localhost:3000/login[0m
[1;34m   |[0m[90m ...[0m
[1;34m 5 |[0m HTTP 302
[1;34m   |[0m[1;31m      ^^^ actual value is <403>[0m
[1;34m   |[0m

[1mlogin.hurl[0m: [1;31mFailure[0m (1 request(s) in 9 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 1 (90.9/s)
Succeeded files:   0 (0.0%)
Failed files:      1 (100.0%)
Duration:          11 ms
```

This is unexpected! Our test is failing, we're not redirected to the favorite movies page.

The reason is quite simple, let's look more precisely at our HTML form:

```html
<form class="login-form" method="post" action="/login">
    <input type="hidden" name="_csrf" value="0fSk7gRA-UTkS25Fbsyal0dgLPBjVy1YIoNg">
    ...
</form>
```

The server login page is protected by a [CSRF token]. In a browser, when the user wants to log in by
sending a POST request, a token is sent along the username/password values. This token is generated server-side, 
and embedded in the HTML. When the POST request is made, our server expects that the request includes a valid token,
and will reject the request if the token is missing or invalid.

In our Hurl file, we're not sending any token, so the server is rejecting our request with a [`403 Forbidden`]
HTTP response.

Unfortunately, we can't hard code the value of a token in our `[FormParams]` section because the token is dynamically 
generated on each request, and a certain fixed value would be valid only during a small period of time.

We need to dynamically _capture_ the value of the CSRF token and pass it to our form. To do so, we are going to:

- perform a first GET request to <http://localhost:3000/login> and capture the CSRF token
- chain with a POST request that contains our username/password value, and our captured CSRF token
- check that the POST response is a redirection, i.e. a [`302 Found`] to the favorites page

So, let's go!

### How to capture values

1. Modify `login.hurl`:

```hurl
# First, display the login page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:3000/login
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"
```

Captures are defined in a `[Captures]` section. Captures are composed of a variable name and a query.
We have already seen queries in [Adding asserts tutorial part]. Since we want to capture value from an HTML
document, we can use a [XPath capture].

> Every query can be used in assert or in capture. You can capture value from JSON response with
> a [JSONPath capture], or [capture cookie value] with the same queries that you use in asserts.

In this capture, `csrf_token` is a variable and `xpath "string(//input[@name='_csrf']/@value)"` is the
XPath query.

Now that we have captured the CSRF token value, we can inject it in the POST request.

2. Add a POST request using `csrf_token` variable in `login.hurl`:

```hurl
# First, display the login page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:3000/login
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"


# Log in user, using the captured CSRF token:
POST http://localhost:3000/login
[FormParams]
username: fab
password: 12345678
_csrf: {{csrf_token}}
HTTP 302
```

3. Run `login.hurl` and verify everything is ok:

```shell
$ hurl --test login.hurl
[1mlogin.hurl[0m: [1;32mSuccess[0m (2 request(s) in 14 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 2 (142.9/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          14 ms
```

## Follow Redirections

Like its HTTP engine [curl], Hurl doesn't follow redirection by default: if a response has a [`302
Found`] status code, Hurl doesn't implicitly run requests until a `200 OK` is reached. This can be useful if you want
to validate each redirection step.

After having logged it, we would like to test the page where the user has been redirected.
This is really simple and can be achieved with a [header assert]: on the response to the POST creation request, we
are going to assert the [`Location`] header, which indicates the redirection URL target.

1. Add a new header assert to test the `Location` header:

```hurl
# First, display the login page to capture
# ...

# Log in user, using the captured CSRF token:
POST http://localhost:3000/login
[FormParams]
username: fab
password: 12345678
_csrf: {{csrf_token}}
HTTP 302
[Asserts]
header "Location" == "/my-movies"
```

2. Add a request to get the favorites page that the user has been redirected to:

```hurl
# First, display the login page to capture
# ...

# Log in user, using the captured CSRF token:
# ...

# Follow redirection and open favorites:
GET http://localhost:3000/my-movies
HTTP 200
[Asserts]
xpath "string(//title)" == "My Movies"
```

3. Run `login.hurl` and verify everything is ok:

```shell
$ hurl --test login.hurl
[1mlogin.hurl[0m: [1;32mSuccess[0m (3 request(s) in 28 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 3 (107.1/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          28 ms
```

> You can force Hurl to follow redirection by using [`-L / --location` option] or using an [`[Options]` section][options].
> In this case, asserts and captures will be run against the last redirection step.

A login workflow is surprisingly hard to do well. You can try to add more test on our `login.hurl` test. With Hurl, try 
now to test the following usecase:

- when a user is not authenticated and goes to <http://localhost:3000/my-movies>, he is redirected to the login page,
- what's happen if the user try to log in with a wrong password,
- after a user log out, he can open the login page again.

You can see a more complete `login.hurl` on [the GitHub repo]. 


## Recap

So, our test file `login.hurl` is now:

```hurl
# First, display the login page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:3000/login
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"


# Log in user, using the captured CSRF token:
POST http://localhost:3000/login
[FormParams]
username: fab
password: 12345678
_csrf: {{csrf_token}}
HTTP 302


# Follow redirection and open favorites:
GET http://localhost:3000/my-movies
HTTP 200
[Asserts]
xpath "string(//title)" == "My Movies"
```

We have seen how to [capture response data] in a variable and use it in others request.
Captures and asserts share the same queries, and can be inter-mixed in the same response.
Finally, Hurl doesn't follow redirect by default, but captures can be used to run each step
of a redirection.


[form]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form
[_Post / Redirect / Get pattern_]: https://en.wikipedia.org/wiki/Post/Redirect/Get
[Form parameters section]: /docs/request.md#form-parameters
[CSRF token]: https://en.wikipedia.org/wiki/Cross-site_request_forgery
[`403 Forbidden`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/403
[`302 Found`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/302
[Adding asserts tutorial part]: /docs/tutorial/adding-asserts.md#structure-of-an-assert
[XPath capture]: /docs/capturing-response.md#xpath-capture
[JSONPath capture]: /docs/capturing-response.md#jsonpath-capture
[capture cookie value]: /docs/capturing-response.md#cookie-capture
[curl]: https://curl.se
[header capture]: /docs/capturing-response.md#header-capture
[`Location`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location
[`-L / --location` option]: /docs/manual.md#location
[capture response data]: /docs/capturing-response.md
[options]: /docs/request.md#options
[captures]: /docs/capturing-response.md
[header assert]: /docs/asserting-response.md#header-assert
[the GitHub repo]: https://github.com/jcamiel/hurl-express-tutorial/tree/main/integration
