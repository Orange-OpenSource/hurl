# Captures

We have seen how to chain requests in a Hurl file. In some use cases, you want
to use data from one request and inject it in another one. That what captures
are all about.

## Capturing a CSRF Token

In our quiz application, user can create a quiz at <http://localhost:8080/new-quiz>.
The HTML page is a [form] where the user can input:

- a required name
- an optional email
- the 5 questions that will form the new quiz

If we look at the page HTML content, we can see an HTML form:

```html
<form action="/new-quiz" method="POST">
    ...
    <input id="name" type="text" name="name" minlength="4" maxlength="32" value="" required>...
    <input id="email" type="email" name="email" value="">...
    <select name="question0" id="question0" required="">...
        <option value="">--Please choose a question--</option>
        <option value="0fec576c">Which astronaut did NOT walk on the moon?</option>
        <option value="dd894cca">If you multiply the width of a rectangle by the height, what do you get?</option>
        <option value="16f897ab">How far does the Moon move away from Earth each year?</option>
        ...
    </select>
    <select name="question1" id="question1" required="">...
    </select>
    ...
</form>
```

When the user clicks on 'Create' button, a POST request is sent with form values for the newly
created quiz: the author's name, an optional email and the list of 5 question ids. Our server implements a
[_Post / Redirect / Get pattern_]: if the POST submission is successful, the user is redirected to a detail
page of the new quiz, indicating creation success.

Let's try to test it!

Form values can be sent using a [Form parameters section], with each key followed by it
corresponding value.

1. Create a new file named `create-quiz.hurl`:

```hurl
POST http://localhost:8080/new-quiz
[FormParams]
name: Simpson
question0: 16f897ab
question1: dd894cca
question2: 4edc1fdb
question3: 37b9eff3
question4: 0fec576c

HTTP/1.1 302
```

> When sending form datas with a Form parameters section, you don't need to set the
> `Content-Type` HTTP header: Hurl enfers that the content type of the request is `application/x-www-form-urlencoded`.

2. Run `create-quiz.hurl`:

```shell
$ hurl --test create-quiz.hurl
create-quiz.hurl: RUNNING [1/1]
error: Assert Status
  --> integration/create-quiz.hurl:9:10
   |
 9 | HTTP/1.1 302
   |          ^^^ actual value is <403>
   |
create-quiz.hurl: FAILURE
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 0 (0.0%)
Failed:    1 (100.0%)
Duration:  13ms
```

This is unexpected! Our test is failing, we're not redirected to the new quiz detail page.

The reason is quite simple, let's look more precisely at our HTML form:

```html
<form action="/new-quiz" method="POST">
    ...
    <button type="submit">Create</button>
    <input type="hidden" name="_csrf" value="7d4da7d7-2970-442a-adc3-55e5e6ba038a">
</form>
```

The server quiz creation endpoint is protected by a [CSRF token]. In a browser, when the user is creating a new quiz by
sending a POST request, a token is sent along the new quiz values. This token is generated server-side, and embedded
in the HTML. When the POST request is made, our quiz application expects that the request includes a valid token,
and will reject the request if the token is missing or invalid.

In our Hurl file, we're not sending any token, so the server is rejecting our request with a [`403 Forbidden`]
HTTP response.

Unfortunately, we can't hard code the value of a token in our
Form parameters section because the token is dynamically generated on each request, and a certain fixed value
would be valid only during a small period of time.

We need to dynamically _capture_ the value of the CSRF token and pass it to our form. To do so, we are going to:

- perform a first GET request to <http://localhost:8080/new-quiz> and capture the CSRF token
- chain with a POST request that contains our quiz value, and our captured CSRF token
- check that the POST response is a redirection, i.e. a [`302 Found`] to the quiz detail page

So, let's go!

### How to capture values

1. Modify `create-quiz.hurl`:

```hurl
# First, get the quiz creation page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery)
GET http://localhost:8080/new-quiz

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"
```

Captures are defined in a Captures section. Captures are composed of a variable name and a query.
We have already seen queries in [Adding asserts tutorial part]. Since we want to capture value from an HTML
document, we can use a [XPath capture].

> Every query can be used in assert or in capture. You can capture value from JSON response with
> a [JSONPath capture], or [capture cookie value] with the same queries that you use in asserts.

In this capture, `csrf_token` is a variable and `xpath "string(//input[@name='_csrf']/@value)"` is the
XPath query.

Now that we have captured the CSRF token value, we can inject it in the POST request.

2. Add a POST request using `csrf_token` variable in `create-quiz.hurl`:

```hurl
# First, get the quiz creation page to capture
# the CSRF token (see https://en.wikipedia.org/wiki/Cross-site_request_forgery):
GET http://localhost:8080/new-quiz

HTTP/1.1 200
[Captures]
csrf_token: xpath "string(//input[@name='_csrf']/@value)"

# Create a new quiz, using the captured CSRF token:
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
```

3. Run `create-quiz.hurl` and verify everything is ok:

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

## Follow Redirections

Like its HTTP engine [curl], Hurl doesn't follow redirection by default: if a response has a [`302
Found`] status code, Hurl doesn't implicitly run requests until a `200 OK` is reached. This can be useful if you want
to validate each redirection step.

What if we want to follow redirections? We can simply use captures!

After having created a new quiz, we would like to test the page where the user has been redirected.
This is really simple and can be achieved with a [header capture]: on the response to the POST creation request, we
are going to capture the [`Location`] header, which indicates the redirection url target, and use it to
go to the next page.

1. Add a new header capture to capture the `Location` header in a variable named `detail_url`:

```hurl
# First, get the quiz creation page to capture
# ...

# Create a new quiz, using the captured CSRF token:
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
```

Captures and asserts can be mixed in the same response spec. For example, we can check that the redirection after
the quiz creation matches a certain url, and add a header assert with a matches predicate.

2. Add a header assert on the POST response to check the redirection url:

```hurl
# First, get the quiz creation page to capture
# ...

# Create a new quiz, using the captured CSRF token:
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
```

3. Add a request to get the detail page where the user has been redirected:

```hurl
# First, get the quiz creation page to capture
# ...

# Create a new quiz, using the captured CSRF token:
# ...

# Open the newly created quiz detail page:
GET {{detail_url}}
HTTP/1.1 200
```

4. Run `create-quiz.hurl` and verify everything is ok:

```shell
$ hurl --test create-quiz.hurl
create-quiz.hurl: RUNNING [1/1]
create-quiz.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  41ms
```


> You can force Hurl to follow redirection by using [`-L / --location` option].
> In this case, asserts and captures will be run against the last redirection step.


## Recap

So, our test file `create-quiz.hurl` is now:

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

We have seen how to [capture response data] in a variable and use it in others request.
Captures and asserts share the sames queries, and can be inter-mixed in the same response.
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
[`-L / --location` option]: /docs/man-page.md#location
[capture response data]:  /docs/capturing-response.md