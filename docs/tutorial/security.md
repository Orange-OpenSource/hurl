# Security

In the [previous part], we have tested our login workflow. So far, we have tested a "simple" form creation: each value of 
the form is valid and sanitized, but what if the user put invalid data? We're going to test a user account creation and see
how we can check that our signup workflow is secure.


## Server Side Validation

In the browser, client-side validation is helping users to enter data and avoid unnecessary server load.

On the signup page, <http://localhost:3000/signup>, we have an HTML form:


```html
<form class="signup-form" method="post" action="/signup">
   ...
   <input type="text" name="username" id="username" autocomplete="off" minlength="3" maxlength="32" pattern="[a-zA-Z0-9_-]{3,32}" title="Username must use a-z, A-Z, 0-9 or _ -" required="">
   ...
   <input type="text" name="name" id="name" autocomplete="off" minlength="3" maxlength="32" pattern="[a-zA-Z\d\s-]{3,32}" required="">
   ...
   <input type="email" name="email" id="email" autocomplete="off" minlength="4" maxlength="32" required="">
    ...
   <input type="password" name="password" id="password" autocomplete="off" minlength="6" maxlength="32" required="">
   ...
   <input type="password" name="password-confirm" id="password-confirm" autocomplete="off" minlength="6" maxlength="32" required="">
</form>
```

The first input, username, has [validation HTML attributes]: `minlength="3"`, `maxlength="32"`, a pattern and `required`.
In a browser, these attributes will prevent the user from entering invalid data like a missing value or a name that is 
too long. If your tests rely on a "headless" browser, it can stop you from testing your server-side validation. 
Client-side validation can also use JavaScript, and it can be a challenge to send invalid data to your server.

But server-side validation is critical to secure your app. You must always validate and sanitize data on your backend,
and try to test it.

As Hurl is not a browser, but merely an HTTP runner on top of [curl], sending and testing invalid data is easy.
To do so, we're going to test the _nominal_ user account creation case, then we'll see how to test with invalid data. 

### Valid user creation

1. Create a new file named `signup.hurl`. We're going to use a new REST API to give us an available username:

```hurl
# First we obtain an available username:
GET http://localhost:3000/api/usernames/available
HTTP 200
[Captures]
username: jsonpath "$.username"
```

Now, we can create a new user. As we have seen in the [previous part], first we have to get a
CSRF token from the signup part, then POST the form to create a user and finally

2. Go to the signup page, and create a new user:

```hurl
# First we obtain an available username:
# ...

# Create a new valid user: get the CSRF token the signup:
GET http://localhost:3000/signup
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"


POST http://localhost:3000/signup
[FormParams]
_csrf: {{csrf_token}}
username: {{username}}
name: Bob
email: {{username}}@example.net
password: 12345678
HTTP 302
[Asserts]
header "Location" == "/my-movies"


# Go to my movies
GET http://localhost:3000/my-movies
HTTP 200
```

Writing each step of a redirection can be a little tedious so we can ask Hurl to automatically follow redirection 
after the POST login. An [`[Options]` section][options] can be used to modify how a request is played:

3. Use an `[Options]` section to follow redirection on the user account creation:

```hurl
# First we obtain an available username:
# ...

# Create a new valid user: get the CSRF token the signup:
GET http://localhost:3000/signup
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"


POST http://localhost:3000/signup
[Options]
location: true
[FormParams]
_csrf: {{csrf_token}}
username: {{username}}
name: Bob
email: {{username}}@example.net
password: 12345678
HTTP 200
[Asserts]
url endsWith "/my-movies"
```

Note that, when following redirection, asserts are run against the final HTTP response. That's why we must have a `200 OK`
instead of a `302 Found`. We can also use an [`url` assert] to check what's the final redirected URL.

4. Run `signup.hurl` and verify that everything is ok:

```shell
$ hurl --test signup.hurl
[1msignup.hurl[0m: [1;32mSuccess[0m (4 request(s) in 26 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 4 (148.1/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          27 ms
```

### Invalid user creation

Now that we have tested a user creation, let's try to create a user with an invalid username. We can try to create a 
two letters long username for instance. In that case, we should be redirected to the signup page, with an error message
displayed.

5. Add a POST user signup with `bo` as username:

```hurl
# First we obtain an available username:
# ...

# Create a new valid user: get the CSRF token the signup:
# ...

# Try an invalid username: too short. We should stay on signup
GET http://localhost:3000/signup
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"

POST http://localhost:3000/signup
[Options]
location: true
[FormParams]
_csrf: {{csrf_token}}
username: bo
name: Bob
email: bob78@example.net
password: 12345678
HTTP 200
[Asserts]
url endsWith "/signup"
xpath "string(//div[@class='form-errors'])" contains "Username must be 3 to 32 chars long"
```

6. Finally, add a POST request with no CSRF token to test that our endpoint has CSRF protection:

```hurl
# First we obtain an available username:
# ...

# Create a new valid user: get the CSRF token the signup:
# ...

# Try an invalid username: too short. We should stay on signup
# ...

# Test CSRF token is mandatory:
POST http://localhost:3000/signup
[FormParams]
username: bob
name: Bob
email: bob78@example.net
password: 12345678
HTTP 403
```

This final test is also interesting because if you're testing your page with a headless browser, the CRSF token is always
created and sent and you don't test that your backend has CSRF protection.



7.Run `signup.hurl` and verify that everything is ok:

```shell
$ hurl --test signup.hurl
[1msignup.hurl[0m: [1;32mSuccess[0m (8 request(s) in 38 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 8 (205.1/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          39 ms
```

## Comments

Hurl being close to the HTTP layer has no "browser protection" / client-side validation: it facilitates
the testing of your app's security with no preconception.

Another security use case is checking that your served HTML isn't leaking comments. Comments can reveal sensitive information
and [is it recommended] to trim HTML comments in your production files.

Popular front-end frameworks like [ReactJS] or [Vue.js] use client-side JavaScript rendering.
If you use one of these frameworks, and you inspect the DOM with the browser developer tools, you won't see any comments
because the framework managing the DOM is removing them.

But, if you look at the HTML page sent on the network, i.e. the real HTML document sent by the
server (and not _the document dynamically created by the framework_), you can still see those HTML comments.

With Hurl, you will be able to check the content of the _real_ network data.

1. In the second entry of `signup.hurl`, add a [XPath assert] when getting the quiz creation page:

```hurl
# First we obtain an available username:
# ...

# Create a new valid user: get the CSRF token the signup:
GET http://localhost:3000/signup
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"
[Asserts]
xpath "//comment" count == 0     # Check that we don't leak comments
# ...
```


2. Run `signup.hurl` and verify that everything is ok:

```shell
$ hurl --test signup.hurl
[1msignup.hurl[0m: [1;32mSuccess[0m (8 request(s) in 47 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 8 (166.7/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          48 ms
```

## Recap

So, our test file `signup.hurl` is now:

```hurl
# First we obtain an available username:
GET http://localhost:3000/api/usernames/available
HTTP 200
[Captures]
username: jsonpath "$.username"


# Create a new valid user: get the CSRF token the signup:
GET http://localhost:3000/signup
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"
[Asserts]
xpath "//comment" count == 0     # Check that we don't leak comments


POST http://localhost:3000/signup
[Options]
location: true
[FormParams]
_csrf: {{csrf_token}}
username: {{username}}
name: Bob
email: {{username}}@example.net
password: 12345678
HTTP 200
[Asserts]
url endsWith "/my-movies"


# Play some checks on signup form: username too short
# email already taken, invalid pattern for username
GET http://localhost:3000/signup
HTTP 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"


# Create a new user, username too short
POST http://localhost:3000/signup
[Options]
location: true
[FormParams]
_csrf: {{csrf_token}}
username: bo
name: Bob
email: bob78@example.net
password: 12345678
HTTP 200
[Asserts]
url endsWith "/signup"
xpath "string(//div[@class='form-errors'])" contains "Username must be 3 to 32 chars long"


# Test CSRF is mandatory:
POST http://localhost:3000/signup
[FormParams]
username: bob
name: Bob
email: bob78@example.net
password: 12345678
HTTP 403
```

We have seen that Hurl can be used as a security tool to check your server-side validation.
Until now, we have done all our tests locally, and in the next session we are going to see how simple
it is to integrate Hurl in a CI/CD pipeline like [GitHub Action] or [GitLab CI/CD].


[curl]: https://curl.se
[the exist predicate]: /docs/asserting-response.md#predicates
[is it recommended]: https://owasp.org/www-project-web-security-testing-guide/v41/4-Web_Application_Security_Testing/01-Information_Gathering/05-Review_Webpage_Comments_and_Metadata_for_Information_Leakage
[DOM]: https://en.wikipedia.org/wiki/Document_Object_Model
[ReactJS]: https://reactjs.org
[Vue.js]: https://vuejs.org
[XPath assert]: /docs/asserting-response.md#xpath-assert
[GitHub Action]: https://github.com/features/actions
[GitLab CI/CD]: https://docs.gitlab.com/ee/ci/
[previous part]: /docs/tutorial/captures.md
[options]: /docs/request.md#options
[`url` assert]: /docs/asserting-response.md#url-assert
[validation HTML attributes]: https://developer.mozilla.org/en-US/docs/Learn/Forms/Form_validation