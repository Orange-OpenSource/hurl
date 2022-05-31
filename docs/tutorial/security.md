# Security

In the previous part, we have tested the basic creation of a quiz, through the <http://localhost:8080/new-quiz>
endpoint. Our test file `create-quiz.hurl` is now:

```hurl
# First, get the quiz creation page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:8080/new-quiz

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"

# Create a new quiz, using the captured CSRF token.
POST http://localhost:8080/new-quiz
[FormParams]
name: Simpson
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c
_csrf: {{csrf_token}}

HTTP/1.1 302
[Captures]
detail_url: header "Location"
[Asserts]
header "Location" matches "/quiz/detail/[a-f0-9]{8}"

# Open the newly created quiz detail page:
GET {{detail_url}}
HTTP/1.1 200
```

So far, we have tested a "simple" form creation: every value of the form is valid and sanitized, but what if the user
put an invalid email?

## Server Side Validation

In the browser, there is client-side validation helping users enter data and avoid unnecessary server load.

Our HTML form is:

```html
<form action="/new-quiz" method="POST">
    ...
    <input id="name" type="text" name="name" minlength="4" maxlength="32" value="" required>...
    <input id="email" type="email" name="email" value="">...
    ...
</form>
```

The first input, name, has validation HTML attributes: `minlenght="4"`, `maxlenght="32"` and `required`.
In a browser, these attributes will prevent user to fill invalid data like a missing value or a name too long. If your
tests rely on a "headless" browser, this type of validation can block you to test your server-side
validation. Client-side validation can also use JavaScript, and it can be a challenge to send invalid data to your server.

But server-side validation is critical to secure your app. You must always validate and sanitize data on your backend,
and try to test it.

As Hurl is not a browser, but merely an HTTP runner on top of [curl], sending and testing invalid data is easy.

1. Add a POST request to create a new quiz in `create-quiz.hurl`, with an invalid name. We check that the status code is 200 (user is
   not redirected to the quiz detail page), and that the label for "name" field has an `invalid` class:

```hurl
# First, get the quiz creation page to capture
# ...

# Create a new quiz, using the captured CSRF token.
# ...

# Open the newly created quiz detail page:
# ...

# Test various server-side validations: 

# Invalid form name value: too short
POST http://localhost:8080/new-quiz
[FormParams]
name: x
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c
_csrf: {{csrf_token}}

HTTP/1.1 200
[Asserts]
xpath "//label[@for='name'][@class='invalid']" exists
```

2. Add a POST request to create a new quiz with an email name. We check that the status
   code is 200 (user is not redirected to the quiz detail page), and that the label for "email" field has an
   `invalid` class:

```hurl
# First, get the quiz creation page to capture
# ...

# Create a new quiz, using the captured CSRF token.
# ...

# Open the newly created quiz detail page:
# ...

# Test various server-side validations: 

# Invalid form name value: too short
# ...
# Invalid email parameter
POST http://localhost:8080/new-quiz
[FormParams]
name: Barth
email: barthsimpson
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c
_csrf: {{csrf_token}}

HTTP/1.1 200
[Asserts]
xpath "//label[@for='email'][@class='invalid']" exists
```

3. Finally, add a POST request with no CSRF token, to test that our endpoint has CRSF protection:

```hurl
# First, get the quiz creation page to capture
# ...

# Create a new quiz, using the captured CSRF token.
# ...

# Open the newly created quiz detail page:
# ...

# Test various server-side validations: 

# Invalid form name value: too short
# ...
# Invalid email parameter
# ...
# No CSRF token:
POST http://localhost:8080/new-quiz
[FormParams]
name: Barth
email: barth.simpson@provider.net
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c

HTTP/1.1 403
```

> We're using [the exist predicate] to check labels in the DOM

4. Run `create-quiz.hurl` and verify everything is ok:

```shell
$ hurl --test create-quiz.hurl
create-quiz.hurl: RUNNING [1/1]
create-quiz.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  33ms
```

## Comments

So Hurl, being close to the HTTP layer, has no "browser protection" / client-side validation: it facilitates
the testing of your app's security with no preconception.

Another use case is checking if there are no comment in your served HTML. These leaks can reveal sensitive information
and [is it recommended] to trim HTML comments in your production files.

Popular front-end construction technologies use client-side JavaScript like [ReactJS] or [Vue.js].
If you use one of this framework, and you inspect the DOM with the browser developer tools, you won't see any comment
because the framework is managing the DOM and removing them.

But, if you look at the HTML page sent on the network, i.e. is the real HTML document sent by the
server (and not _the document dynamically created by the framework_), you can still see those HTML comments.

With Hurl, you will be able to check the content of the _real_ network data.

1. In the first entry of `create-quiz.hurl`, add a [XPath assert] when getting the quiz creation page:

```hurl
# First, get the quiz creation page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:8080/new-quiz

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"
[Asserts]
xpath "//comment" count == 0     # Check that we don't leak comments

# ...
```


2. Run `create-quiz.hurl` and verify everything is ok:

```shell
$ hurl --test create-quiz.hurl
create-quiz.hurl: RUNNING [1/1]
create-quiz.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  33ms
```

## Recap

So, our test file `create-quiz.hurl` is now:

```hurl
# First, get the quiz creation page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:8080/new-quiz

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"
[Asserts]
xpath "//comment" count == 0     # Check that we don't leak comments

# Create a new quiz, using the captured CSRF token.
POST http://localhost:8080/new-quiz
[FormParams]
name: Simpson
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c
_csrf: {{csrf_token}}

HTTP/1.1 302
[Captures]
detail_url: header "Location"
[Asserts]
header "Location" matches "/quiz/detail/[a-f0-9]{8}"

# Open the newly created quiz detail page:
GET {{detail_url}}
HTTP/1.1 200

# Test various server-side validations:

# Invalid form name value: too short
POST http://localhost:8080/new-quiz
[FormParams]
name: x
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c
_csrf: {{csrf_token}}

HTTP/1.1 200
[Asserts]
xpath "//label[@for='name'][@class='invalid']" exists

# Invalid email parameter:
POST http://localhost:8080/new-quiz
[FormParams]
name: Barth
email: barthsimpson
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c
_csrf: {{csrf_token}}

HTTP/1.1 200
[Asserts]
xpath "//label[@for='email'][@class='invalid']" exists

# No CSRF token:
POST http://localhost:8080/new-quiz
[FormParams]
name: Barth
email: barth.simpson@provider.net
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c

HTTP/1.1 403
```

We have seen that Hurl can be used as a security tool, to check you server-side validation.
Until now, we have done all our tests locally, and in the next session we are going to see how simple
it is to integrate Hurl in a CI/CD pipeline like [GitHub Action] or [GitLab CI/CD].


[curl]: https://curl.se
[the exist predicate]: /docs/asserting-response.md#predicates
[is it recommended]: https://owasp.org/www-project-web-security-testing-guide/latest/4-Web_Application_Security_Testing/01-Information_Gathering/05-Review_Webpage_Content_for_Information_Leakage
[DOM]: https://en.wikipedia.org/wiki/Document_Object_Model
[ReactJS]: https://reactjs.org
[Vue.js]: https://vuejs.org
[XPath assert]: /docs/asserting-response.md#xpath-assert
[GitHub Action]: https://github.com/features/actions
[GitLab CI/CD]: https://docs.gitlab.com/ee/ci/
