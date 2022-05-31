# Debug Tips

Now that we have many requests in our test file, let's review some tips to debug the executed HTTP exchanges.

## Verbose Mode

We can run our test with [`-v/--verbose` option]. In this mode, each entry is displayed, with debugging
information like request HTTP headers, response HTTP headers, cookie storage, duration etc...

```shell
$ hurl -v basic.hurl > /dev/null
* fail fast: true
* insecure: false
* follow redirect: false
* max redirect: 50
* ------------------------------------------------------------------------------
* executing entry 1
* 
* Cookie store:
* 
* Request
* GET http://localhost:8080
* 
> GET / HTTP/1.1
> Host: localhost:8080
> Accept: */*
> User-Agent: hurl/1.2.0
> 
< HTTP/1.1 200 
< Set-Cookie: JSESSIONID=02A8B2F4F604BAE9F016034C13C31282; Path=/; HttpOnly
< X-Content-Type-Options: nosniff
< X-XSS-Protection: 1; mode=block
< Cache-Control: no-cache, no-store, max-age=0, must-revalidate
< Pragma: no-cache
< Expires: 0
< X-Frame-Options: DENY
< Content-Type: text/html;charset=UTF-8
< Content-Language: en-US
< Transfer-Encoding: chunked
< Date: Fri, 04 Jun 2021 12:24:15 GMT
< 
* Response Time: 16ms
* 
* ------------------------------------------------------------------------------
* executing entry 2
* 
* Cookie store:
* #HttpOnly_localhost	FALSE	/	FALSE	0	JSESSIONID	02A8B2F4F604BAE9F016034C13C31282
* 
* Request
* GET http://localhost:8080/not-found
* 
> GET /not-found HTTP/1.1
> Host: localhost:8080
> Accept: */*
> Cookie: JSESSIONID=02A8B2F4F604BAE9F016034C13C31282
> User-Agent: hurl/1.2.0
> 
< HTTP/1.1 404 
< Vary: Origin
< Vary: Access-Control-Request-Method
< Vary: Access-Control-Request-Headers
< X-Content-Type-Options: nosniff
< X-XSS-Protection: 1; mode=block
< Cache-Control: no-cache, no-store, max-age=0, must-revalidate
< Pragma: no-cache
< Expires: 0
< X-Frame-Options: DENY
< Content-Type: text/html;charset=UTF-8
< Content-Language: en-US
< Transfer-Encoding: chunked
< Date: Fri, 04 Jun 2021 12:24:15 GMT
< 
* Response Time: 8ms
* 
...
```

Line beginning by `*` are debug info, lines that begin by `>` are HTTP request headers and lines that begin with
`<` are HTTP response headers.

## Interactive Mode

We can run the whole Hurl file request by request, with the [`--interactive` option]:

```shell
$ hurl --interactive basic.hurl
* fail fast: true
* insecure: false
* follow redirect: false
* max redirect: 50

interactive mode:
Press Q (Quit) or C (Continue)

* ------------------------------------------------------------------------------
* executing entry 1
* 
* Cookie store:
* 
* Request
* GET http://localhost:8080
* 
> GET / HTTP/1.1
> Host: localhost:8080
> Accept: */*
> User-Agent: hurl/1.2.0
> 
< HTTP/1.1 200 
< Set-Cookie: JSESSIONID=829EF66D8B441D9B57B2498CF9989E54; Path=/; HttpOnly
< X-Content-Type-Options: nosniff
< X-XSS-Protection: 1; mode=block
< Cache-Control: no-cache, no-store, max-age=0, must-revalidate
< Pragma: no-cache
< Expires: 0
< X-Frame-Options: DENY
< Content-Type: text/html;charset=UTF-8
< Content-Language: en-US
< Transfer-Encoding: chunked
< Date: Fri, 04 Jun 2021 12:35:04 GMT
< 
* Response Time: 11ms
* 

interactive mode:
Press Q (Quit) or C (Continue)
```

## Include Headers Like curl

We can also run our file to only output HTTP headers, with [`-i/--include` option].
In this mode, headers of the last entry are displayed:

```shell
$ hurl -i basic.hurl
HTTP/1.1 200
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Cache-Control: no-cache, no-store, max-age=0, must-revalidate
Pragma: no-cache
Expires: 0
X-Frame-Options: DENY
Content-Type: application/json
Transfer-Encoding: chunked
Date: Sun, 06 Jun 2021 15:11:31 GMT

[{"id":"c0d80047-4fbe-4d45-a005-91b5c7018b34","created":"1995-12-17T03:24:00Z"....
```

If you want to inspect any entry other than the last entry, you can run your test to a
given entry with the [`--to-entry` option], starting at index 1:

```shell
$ hurl -i --to-entry 2 basic.hurl
HTTP/1.1 404
Vary: Origin
Vary: Access-Control-Request-Method
Vary: Access-Control-Request-Headers
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Cache-Control: no-cache, no-store, max-age=0, must-revalidate
Pragma: no-cache
Expires: 0
X-Frame-Options: DENY
Content-Type: text/html;charset=UTF-8
Content-Language: en-US
Transfer-Encoding: chunked
Date: Sun, 06 Jun 2021 15:14:20 GMT

<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title></title>
    <link rel="stylesheet" href="/style.css">
    <!--<script src="script.js"></script>-->
</head>
<body>
<div>
    <img class="logo" src="/quiz.svg" alt="Quiz logo">
</div>
<h1>Error 404, Page not Found!</h1>

<a href="/">Quiz Home</a>


</body>
</html>
```

## Using a Proxy

Finally, you can use a proxy between Hurl and your server to inspect requests and responses.

For instance, with [mitmproxy]:

1. First, launch mitmproxy, it will listen to connections on 8888 port

    ```shell
$ mitmweb -p 8888 --web-port 8889 --web-open-browser
Web server listening at http://127.0.0.1:8889/
Proxy server listening at http://*:8888
    ```

2. Then, run Hurl with [`-x/--proxy` option]

    ```shell
$ hurl --proxy localhost:8888 basic.hurl
    ```

The web interface of mitmproxy allows you to inspect, intercept any requests run by Hurl, and see
the returned response to Hurl.


[`-v/--verbose` option]: /docs/man-page.md#verbose
[`--interactive` option]: /docs/man-page.md#interactive
[`-i/--include` option]: /docs/man-page.md#include
[`--to-entry` option]: /docs/man-page.md#to-entry
[mitmproxy]: https://mitmproxy.org
[`-x/--proxy` option]: /docs/man-page.md#proxy
