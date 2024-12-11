# Entry

## Definition

A Hurl file is a list of entries, each entry being a mandatory [request], optionally followed by a [response].

Responses are not mandatory, a Hurl file consisting only of requests is perfectly valid. To sum up, responses can be used
to [capture values] to perform subsequent requests, or [add asserts to HTTP responses].

## Example

```hurl
# First, test home title.
GET https://acmecorp.net
HTTP 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"

# Get some news, response description is optional
GET https://acmecorp.net/news

# Do a POST request without CSRF token and check
# that status code is Forbidden 403
POST https://acmecorp.net/contact
[FormParams]
default: false
email: john.doe@rookie.org
number: 33611223344
HTTP 403
```

## Description

### Options

[Options] specified on the command line apply to every entry in an Hurl file. For instance, with [`--location` option],
every entry of a given file will follow redirection:

```shell
$ hurl --location foo.hurl
```

You can use an [`[Options]` section][options] to set option only for a specified request. For instance, in this Hurl file,
the second entry will follow location (so we can test the status code to be 200 instead of 301).

```hurl
GET https://google.fr
HTTP 301

GET https://google.fr
[Options]
location: true
HTTP 200

GET https://google.fr
HTTP 301
```

You can use the `[Options]` section to log a specific entry:

```hurl
# ... previous entries

GET https://api.example.org
[Options]
very-verbose: true
HTTP 200

# ... next entries
```

### Cookie storage

Requests in the same Hurl file share the cookie storage, enabling, for example, session based scenario.

### Redirects

By default, Hurl doesn't follow redirection. To effectively run a redirection, entries should describe each step
of the redirection, allowing insertion of asserts in each response.

```hurl
# First entry, test the redirection (status code and 'Location' header)
GET https://google.fr
HTTP 301
Location: https://www.google.fr/

# Second entry, the 200 OK response
GET https://www.google.fr
HTTP 200
```

Alternatively, one can use [`--location`] / [`--location-trusted`] options to force redirection
to be followed. In this case, asserts are executed on the last received response. Optionally, the number of
redirections can be limited with [`--max-redirs`].

```hurl
# Running hurl --location google.hurl
GET https://google.fr
HTTP 200
```

Finally, you can force redirection on a particular request with an [`[Options]` section][options] and the[`--location`] 
/ [`--location-trusted`] options:

```hurl
GET https://google.fr
[Options]
location-trusted: true
HTTP 200
```

### Retry

Every entry can be retried upon asserts, captures or runtime errors. Retries allow polling scenarios and effective runs 
under flaky conditions. Asserts can be explicit (with an [`[Asserts]` section][asserts]), or implicit (like [headers] or [status code]).

Retries can be set globally for every request (see [`--retry`] and [`--retry-interval`]), 
or activated on a particular request with an [`[Options]` section][options].

For example, in this Hurl file, first we create a new job then we poll the new job until it's completed:

```hurl
# Create a new job
POST http://api.example.org/jobs
HTTP 201
[Captures]
job_id: jsonpath "$.id"
[Asserts]
jsonpath "$.state" == "RUNNING"


# Pull job status until it is completed
GET http://api.example.org/jobs/{{job_id}}
[Options]
retry: 10   # maximum number of retry, -1 for unlimited
retry-interval: 300ms
HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"
```

### Control flow

In `[Options]` section, `skip` and `repeat` can be used to control flow of execution:

- `skip: true/false` skip this request and execute the next one unconditionally,
- `repeat: N` loop the request N times. If there are assert or runtime errors, the requests execution is stopped.

```hurl
# This request will be played exactly 3 times
GET https://example.org/foo
[Options]
repeat: 3
HTTP 200

# This request is skipped
GET https://example.org/foo
[Options]
skip: true
HTTP 200
```

Additionally, a `delay` can be inserted between requests, to add a delay before execution of a request (aka sleep).

```hurl
# A 5 seconds delayed request 
GET https://example.org/foo
[Options]
delay: 5s
HTTP 200
```

[`delay`] and [`repeat`] can also be used globally as command line options:

```shell
$ hurl --delay 500ms --repeat 3 foo.hurl
```



For complete reference, below is a diagram for the executed entries.

<div class="picture">
    <img class="u-theme-light u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/run-cycle-light.svg" alt="Run cycle explanation"/>
    <img class="u-theme-dark u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/run-cycle-dark.svg" alt="Run cycle explanation"/>
</div>


[request]: /docs/request.md
[response]: /docs/response.md
[capture values]: /docs/capturing-response.md
[add asserts to HTTP responses]: /docs/asserting-response.md
[`--location`]: /docs/manual.md#location
[`--location` option]: /docs/manual.md#location
[`--location-trusted`]: /docs/manual.md#location-trusted
[`--max-redirs`]: /docs/manual.md#max-redirs
[Options]: /docs/manual.md#options
[options]: /docs/request.md#options
[headers]: /docs/response.md#headers
[status code]: /docs/response.md#version-status
[asserts]: /docs/response.md#asserts
[Asserts]: /docs/response.md#asserts
[`--retry`]: /docs/manual.md#retry
[`--retry-interval`]: /docs/manual.md#retry-interval
[`delay`]: /docs/manual.md#retry 
[`repeat`]: /docs/manual.md#repeat

