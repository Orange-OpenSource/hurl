# Frequently Asked Questions

- [General](#general)
    - [Why "Hurl"?](#why-hurl)
    - [Yet Another Tool, I already use X](#yet-another-tool-i-already-use-x)
    - [Hurl is build on top of libcurl, but what is added?](#hurl-is-build-on-top-of-libcurl-but-what-is-added)
    - [Why shouldn't I use Hurl?](#why-shouldnt-i-use-hurl)
    - [I have a large numbers of tests, how to run just specific tests?](#i-have-a-large-numbers-of-tests-how-to-run-just-specific-tests)
    - [How can I use my Hurl files outside Hurl?](#how-can-i-use-my-hurl-files-outside-hurl)
    - [Can I do calculation within a Hurl file?](#can-i-do-calculation-within-a-hurl-file)
- [macOS](#macos)
    - [How can I use a custom libcurl (from Homebrew by instance)?](#how-can-i-use-a-custom-libcurl-from-homebrew-by-instance)
    - [Hurl error: SSL certificate has expired](#hurl-error-ssl-certificate-has-expired)
    - [Hurl warning on Big Sur: Closing connection 0](#hurl-warning-on-big-sur-closing-connection-0)

## General

### Why "Hurl"?

The name Hurl is a tribute to the awesome [curl], with a focus on the HTTP protocol.
While it may have an informal meaning not particularly elegant, [other eminent tools] have set a precedent in naming.

### Yet Another Tool, I already use X

We think that Hurl has some advantages compared to similar tools.

Hurl is foremost a command line tool and should be easy to use on a local computer, or in a CI/CD pipeline. Some
tools in the same space as Hurl ([Postman] for instance), are GUI oriented, and we find it
less attractive than CLI. As a command line tool, Hurl can be used to get HTTP datas (like [curl]),
but also as a test tool for HTTP sessions, or even as documentation.

Having a text based [file format] is another advantage. The Hurl format is simple,
focused on the HTTP domain, can serve as documentation and can be read or written by non-technical people.

For instance posting JSON data with Hurl can be done with this simple file:

``` 
POST http://localhost:3000/api/login
{
  "username": "xyz",
  "password": "xyz"
}
```

With [curl]:

```
curl --header "Content-Type: application/json" \
     --request POST \
     --data '{"username": "xyz","password": "xyz"}' \
     http://localhost:3000/api/login
``` 


[Karate], a tool combining API test automation, mocking, performance-testing, has
similar features but offers also much more at a cost of an increased complexity.

Comparing Karate file format:

```
Scenario: create and retrieve a cat

Given url 'http://myhost.com/v1/cats'
And request { name: 'Billie' }
When method post
Then status 201
And match response == { id: '#notnull', name: 'Billie }

Given path response.id
When method get
Then status 200
``` 

And Hurl:

```
# Scenario: create and retrieve a cat

POST http://myhost.com/v1/cats
{ "name": "Billie" }
HTTP/* 201
[Captures]
cat_id: jsonpath "$.id"
[Asserts]
jsonpath "$.name" == "Billie"

GET http://myshost.com/v1/cats/{{cat_id}}
HTTP/* 200
```

A key point of Hurl is to work on the HTTP domain. In particular, there is no Javascript runtime, Hurl works on the
raw HTTP requests/responses, and not on a DOM managed by a HTML engine. For security, this can be seen as a feature:
let's say you want to test backend validation, you want to be able to bypass the browser or javascript validations and
directly test a backend endpoint.

Finally, with no headless browser and working on the raw HTTP data, Hurl is also
really reliable with a very small probability of false positives. Integration tests with tools like
[Selenium] can, in this regard, be challenging to maintain.

Just use what is convenient for you. In our case, it's Hurl!

### Hurl is build on top of libcurl, but what is added?

Hurl has two main functionalities on top of [curl]:

1. Chain several requests:

   With its [captures], it enables to inject data received from a response into
   following requests. [CSRF tokens]
   are typical examples in a standard web session.

2. Test HTTP responses:

   With its [asserts], responses can be easily tested.

### Why shouldn't I use Hurl?

If you need a GUI. Currently, Hurl does not offer a GUI version (like [Postman]). While we
think that it can be useful, we prefer to focus for the time-being on the core, keeping something simple and fast.
Contributions to build a GUI are welcome.


### I have a large numbers of tests, how to run just specific tests?

By convention, you can organize Hurl files into different folders or prefix them.

For example, you can split your tests into two folders critical and additional.

```
critical/test1.hurl
critical/test2.hurl
additional/test1.hurl
additional/test2.hurl
```

You can simply run your critical tests with

```
hurl critical/*.hurl
```

### How can I use my Hurl files outside Hurl?

Hurl file can be exported to a json file with `hurlfmt`.
This json file can then be easily parsed for converting a different format, getting ad-hoc information,...

For example, the Hurl file

```hurl
GET https://example.org/api/users/1
User-Agent: Custom

HTTP/1.1 200
[Asserts]
jsonpath "$.name" equals "Bob"

```

will be converted to json with the following command:

```
hurlfmt test.hurl --format json | jq
{
  "entries": [
    {
      "request": {
        "method": "GET",
        "url": "https://example.org/api/users/1",
        "headers": [
          {
            "name": "User-Agent",
            "value": "Custom"
          }
        ]
      },
      "response": {
        "version": "HTTP/1.1",
        "status": 200,
        "asserts": [
          {
            "query": {
              "type": "jsonpath",
              "expr": "$.name"
            },
            "predicate": {
              "type": "equal",
              "value": "Bob"
            }
          }
        ]
      }
    }
  ]
}
```


### Can I do calculation within a Hurl file?

Currently, the templating is very simple, only accessing variables.
Calculations can be done beforehand, before running the Hurl File.

For example, with date calculations, variables `now` and `tomorrow` can be used as param or expected value.

```
TODAY=$(date '+%y%m%d')
TOMORROW=$(date '+%y%m%d' -d"+1days")
hurl --variable "today=$TODAY" --variable "tomorrow=$TOMORROW" test.hurl
```

## macOS

### How can I use a custom libcurl (from Homebrew by instance)?

No matter how you've installed Hurl (using the precompiled binary for macOS or with [Homebrew])
Hurl is linked against the built-in system libcurl. If you want to use another libcurl (for instance,
if you've installed curl with Homebrew and want Hurl to use Homebrew's libcurl), you can patch Hurl with
the following command:

```shell
sudo install_name_tool -change /usr/lib/libcurl.4.dylib PATH_TO_CUSTOM_LIBCURL PATH_TO_HURL_BIN
```

For instance:

```shell
# /usr/local/opt/curl/lib/libcurl.4.dylib is installed by `brew install curl`
sudo install_name_tool -change /usr/lib/libcurl.4.dylib /usr/local/opt/curl/lib/libcurl.4.dylib /usr/local/bin/hurl
```

### Hurl error: SSL certificate has expired

If you have a `SSL certificate has expired` error on valid certificates with Hurl, it can be due to the macOS libcurl certificates
not updated. On Mojave, the built-in curl (`/usr/bin/curl`) relies on the `/etc/ssl/cert.pem` file for root CA verification,
and some certificates has expired. To solve this problem:

1. Edit `/etc/ssl/cert.pem` and remove the expired certificate (for instance, the `DST Root CA X3` has expired)
2. Use a recent curl (installed with Homebrew) and [configure Hurl to use it].

### Hurl warning on Big Sur: Closing connection 0

In Big Sur, the system version of libcurl (7.64.1), has a bug that [erroneously
displays `* Closing connection 0` on `stderr`]. To fix Hurl not to output this
warning, one can link Hurl to a newer version of libcurl.

For instance, to use the latest libcurl with Homebrew:

```shell
$ brew install curl
$ sudo install_name_tool -change /usr/lib/libcurl.4.dylib /usr/local/opt/curl/lib/libcurl.4.dylib /usr/local/bin/hurl
```

[curl]: https://curl.haxx.se
[other eminent tools]: https://git.wiki.kernel.org/index.php/GitFaq#Why_the_.27Git.27_name.3F
[Postman]: https://www.postman.com
[file format]: /docs/hurl-file.md
[Karate]: https://github.com/intuit/karate
[Selenium]: https://www.selenium.dev
[captures]: /docs/capturing-response.md
[CSRF tokens]: https://en.wikipedia.org/wiki/Cross-site_request_forgery
[asserts]: /docs/asserting-response.md
[configure Hurl to use it]: #how-can-i-use-a-custom-libcurl-from-homebrew-by-instance
[Homebrew]: https://brew.sh
[erroneously displays `* Closing connection 0` on `stderr`]: https://github.com/curl/curl/issues/3891
