# Frequently Asked Questions

## General

### Why "Hurl"?

The name Hurl is a tribute to the awesome [curl], with a focus on the HTTP protocol.
While it may have an informal meaning not particularly elegant, [other eminent tools] have set a precedent in naming.

### Yet Another Tool, I already use X

We think that Hurl has some advantages compared to similar tools.

Hurl is foremost a command line tool and should be easy to use on a local computer, or in a CI/CD pipeline. Some
tools in the same space as Hurl ([Postman] for instance), are GUI oriented, and we find it
less attractive than CLI. As a command line tool, Hurl can be used to get HTTP data (like [curl]),
but also as a test tool for HTTP sessions, or even as documentation.

Having a text based [file format] is another advantage. The Hurl format is simple,
focused on the HTTP domain, can serve as documentation and can be read or written by non-technical people.

For instance putting JSON data with Hurl can be done with this simple file:

``` 
PUT http://localhost:3000/api/login
{
  "username": "xyz",
  "password": "xyz"
}
```

With [curl]:

```
curl --header "Content-Type: application/json" \
     --request PUT \
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
HTTP 201
[Captures]
cat_id: jsonpath "$.id"
[Asserts]
jsonpath "$.name" == "Billie"

GET http://myshost.com/v1/cats/{{cat_id}}
HTTP 200
```

A key point of Hurl is to work on the HTTP domain. In particular, there is no JavaScript runtime, Hurl works on the
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

Hurl benefits from the features of the `libcurl` against it is linked. You can check `libcurl` version with `hurl --version`.

For instance on macOS:

```shell
$ hurl --version
hurl 2.0.0 libcurl/7.79.1 (SecureTransport) LibreSSL/3.3.6 zlib/1.2.11 nghttp2/1.45.1
Features (libcurl):  alt-svc AsynchDNS HSTS HTTP2 IPv6 Largefile libz NTLM NTLM_WB SPNEGO SSL UnixSockets
Features (built-in): brotli
```

You can also check which `libcurl` is used.

On macOS:

```shell
$ which hurl
/opt/homebrew/bin/hurl
$ otool -L /opt/homebrew/bin/hurl:
	/usr/lib/libxml2.2.dylib (compatibility version 10.0.0, current version 10.9.0)
	/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation (compatibility version 150.0.0, current version 1858.112.0)
	/usr/lib/libcurl.4.dylib (compatibility version 7.0.0, current version 9.0.0)
	/usr/lib/libiconv.2.dylib (compatibility version 7.0.0, current version 7.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1311.100.3)
```

On Linux:

```shell
$ which hurl
/root/.cargo/bin/hurl
$ ldd /root/.cargo/bin/hurl
ldd /root/.cargo/bin/hurl
	linux-vdso.so.1 (0x0000ffff8656a000)
	libxml2.so.2 => /usr/lib/aarch64-linux-gnu/libxml2.so.2 (0x0000ffff85fe8000)
	libcurl.so.4 => /usr/lib/aarch64-linux-gnu/libcurl.so.4 (0x0000ffff85f45000)
	libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x0000ffff85f21000)
	...
	libkeyutils.so.1 => /lib/aarch64-linux-gnu/libkeyutils.so.1 (0x0000ffff82ed5000)
	libffi.so.7 => /usr/lib/aarch64-linux-gnu/libffi.so.7 (0x0000ffff82ebc000)
```

Note that some Hurl features are dependent on `libcurl` capacities: for instance, if your `libcurl` doesn't support
HTTP/2 Hurl won't be able to send HTTP/2 request. 


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

```shell
$ hurl --test critical/*.hurl
```

### How can I use my Hurl files outside Hurl?

Hurl file can be exported to a JSON file with `hurlfmt`.
This JSON file can then be easily parsed for converting a different format, getting ad-hoc information,...

For example, the Hurl file

```hurl
GET https://example.org/api/users/1
User-Agent: Custom
HTTP 200
[Asserts]
jsonpath "$.name" == "Bob"
```

will be converted to JSON with the following command:

```shell
$ hurlfmt test.hurl --out json | jq
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
        "version": "HTTP",
        "status": 200,
        "asserts": [
          {
            "query": {
              "type": "jsonpath",
              "expr": "$.name"
            },
            "predicate": {
              "type": "==",
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

```shell
$ TODAY=$(date '+%y%m%d')
$ TOMORROW=$(date '+%y%m%d' -d"+1days")
$ hurl --variable "today=$TODAY" --variable "tomorrow=$TOMORROW" test.hurl
```

You can also use environment variables that begins with `HURL_` to inject data in an Hurl file.
For instance, to inject `today` and `tomorrow` variables:

```shell
$ export HURL_today=$(date '+%y%m%d')
$ export HURL_tomorrow=$(date '+%y%m%d' -d"+1days")
$ hurl test.hurl
```

You can also use [filters] to process HTTP responses in asserts and captures.

## macOS

### How can I use a custom libcurl (from Homebrew by instance)?

No matter how you've installed Hurl (using the precompiled binary for macOS or with [Homebrew])
Hurl is linked against the built-in system libcurl. If you want to use another libcurl (for instance,
if you've installed curl with Homebrew and want Hurl to use Homebrew's libcurl), you can patch Hurl with
the following command:

```shell
$ sudo install_name_tool -change /usr/lib/libcurl.4.dylib PATH_TO_CUSTOM_LIBCURL PATH_TO_HURL_BIN
```

For instance:

```shell
# /usr/local/opt/curl/lib/libcurl.4.dylib is installed by `brew install curl`
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
[Homebrew]: https://brew.sh
[filters]: /docs/filters.md
