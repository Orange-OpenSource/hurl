# Entry

## Definition

A Hurl file is a list of entry, each entry being a mandatory [request], optionally followed by a [response].

Responses are not mandatory, a Hurl file consisting only of requests is perfectly valid. To sum up, responses can be used
to [capture values] to perform subsequent requests, or [add asserts to HTTP responses].

## Example

```hurl
# First, test home title.
GET https://acmecorp.net

HTTP/1.1 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"

# Get some news, response description is optional
GET https://acmecorp.net/news

# Do a POST request without csrf token and check
# that status code is Forbidden 403
POST https://acmecorp.net/contact
[FormParams]
default: false
email: john.doe@rookie.org
number: 33611223344

HTTP/1.1 403
```

## Description

### Cookie storage

Requests in the same Hurl file share the cookie storage, enabling, for example, session based scenario.

### Redirects

By default, Hurl doesn't follow redirection. To effectively run a redirection, entries should describe each step
of the redirection, allowing insertion of asserts in each response.

```hurl
# First entry, test the redirection (status code and
# Location header)
GET http://google.fr

HTTP/1.1 301
Location: http://www.google.fr/

# Second entry, the 200 OK response
GET http://www.google.fr

HTTP/1.1 200
```

Alternatively, one can use [`--location`] option to force redirection
to be followed. In this case, asserts are executed on the last received response. Optionally, the number of
redirections can be limited with [`--max-redirs`].

```hurl
# Running hurl --location google.hurl
GET http://google.fr
HTTP/1.1 200
```

[request]: /docs/request.md
[response]: /docs/response.md
[capture values]: /docs/capturing-response.md
[add asserts to HTTP responses]: /docs/asserting-response.md
[`--location`]: /docs/man-page.md#location
[`--max-redirs`]: /docs/man-page.md#max-redirs
