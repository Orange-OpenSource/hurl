# Debug Tips

Now that we have many requests in our test file, let's review some tips on how to debug the executed HTTP exchanges.

## Verbose Mode

### Using --verbose and --very-verbose for all entries

We can run our test with [`-v/--verbose` option]. In this mode, each entry is displayed with debugging
information like request HTTP headers, response HTTP headers, cookie storage, duration etc...

```shell
$ hurl --verbose --no-output basic.hurl
[1;34m*[0m [1mOptions:[0m
[1;34m*[0m     fail fast: true
[1;34m*[0m     insecure: false
[1;34m*[0m     follow redirect: false
[1;34m*[0m     max redirect: 50
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 1[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:8080
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:8080'
[1;34m*[0m
> [1;35mGET / HTTP/1.1[0m
> [1;36mHost[0m: localhost:8080
> [1;36mAccept[0m: */*
> [1;36mUser-Agent[0m: hurl/1.7.0-snapshot
>
[1;34m*[0m [1mResponse:[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200[0m
< [1;36mSet-Cookie[0m: JSESSIONID=361948EF00AA04CB6659954A8D3EBC9D; Path=/; HttpOnly
< [1;36mX-Content-Type-Options[0m: nosniff
< [1;36mX-XSS-Protection[0m: 1; mode=block
< [1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
< [1;36mPragma[0m: no-cache
< [1;36mExpires[0m: 0
< [1;36mX-Frame-Options[0m: DENY
< [1;36mContent-Type[0m: text/html;charset=UTF-8
< [1;36mContent-Language[0m: en-FR
< [1;36mTransfer-Encoding[0m: chunked
< [1;36mDate[0m: Wed, 17 Aug 2022 07:30:15 GMT
<
[1;34m*[0m
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 2[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m #HttpOnly_localhost	FALSE	/	FALSE	0	JSESSIONID	361948EF00AA04CB6659954A8D3EBC9D
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:8080/not-found
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:8080/not-found' --cookie 'JSESSIONID=361948EF00AA04CB6659954A8D3EBC9D'
[1;34m*[0m
> [1;35mGET /not-found HTTP/1.1[0m
> [1;36mHost[0m: localhost:8080
> [1;36mAccept[0m: */*
> [1;36mCookie[0m: JSESSIONID=361948EF00AA04CB6659954A8D3EBC9D
> [1;36mUser-Agent[0m: hurl/1.7.0-snapshot
>
[1;34m*[0m [1mResponse:[0m
[1;34m*[0m
< [1;32mHTTP/1.1 404[0m
< [1;36mVary[0m: Origin
< [1;36mVary[0m: Access-Control-Request-Method
< [1;36mVary[0m: Access-Control-Request-Headers
< [1;36mX-Content-Type-Options[0m: nosniff
< [1;36mX-XSS-Protection[0m: 1; mode=block
< [1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
< [1;36mPragma[0m: no-cache
< [1;36mExpires[0m: 0
< [1;36mX-Frame-Options[0m: DENY
< [1;36mContent-Type[0m: text/html;charset=UTF-8
< [1;36mContent-Language[0m: en-FR
< [1;36mTransfer-Encoding[0m: chunked
< [1;36mDate[0m: Wed, 17 Aug 2022 07:30:15 GMT
<
[1;34m*[0m
...
```

Lines beginning with `*` are debug info, lines that begin with `>` are HTTP request headers and lines that begin with
`<` are HTTP response headers.

In verbose mode, HTTP request and response bodies are not displayed in the debug logs. If you need to inspect the 
request or response body, you can display more logs with [`--very-verbose`] option:

```shell
$ hurl --very-verbose --no-output basic.hurl
[1;34m*[0m [1mOptions:[0m
[1;34m*[0m     fail fast: true
[1;34m*[0m     insecure: false
[1;34m*[0m     follow redirect: false
[1;34m*[0m     max redirect: 50
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 1[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:8080
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:8080'
[1;34m*[0m
> [1;35mGET / HTTP/1.1[0m
> [1;36mHost[0m: localhost:8080
> [1;36mAccept[0m: */*
> [1;36mUser-Agent[0m: hurl/1.7.0-snapshot
>
[1;34m*[0m [1mRequest body:[0m
[1;34m*[0m
[1;34m*[0m [1mResponse:[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200[0m
< [1;36mSet-Cookie[0m: JSESSIONID=0B417BD5890C001B5B25A9B321FE4800; Path=/; HttpOnly
< [1;36mX-Content-Type-Options[0m: nosniff
< [1;36mX-XSS-Protection[0m: 1; mode=block
< [1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
< [1;36mPragma[0m: no-cache
< [1;36mExpires[0m: 0
< [1;36mX-Frame-Options[0m: DENY
< [1;36mContent-Type[0m: text/html;charset=UTF-8
< [1;36mContent-Language[0m: en-FR
< [1;36mTransfer-Encoding[0m: chunked
< [1;36mDate[0m: Wed, 17 Aug 2022 07:42:46 GMT
<
[1;34m*[0m [1mResponse body:[0m
[1;34m*[0m <!doctype html>
[1;34m*[0m <html lang="en">
[1;34m*[0m <head>
[1;34m*[0m     <meta charset="UTF-8" />
[1;34m*[0m     <meta name="viewport" content="width=device-width, initial-scale=1.0" />
[1;34m*[0m     <title>Welcome to Quiz!</title>
[1;34m*[0m     <link rel="stylesheet" href="/style.css">
[1;34m*[0m     <!--<script src="script.js"></script>-->
[1;34m*[0m </head>
[1;34m*[0m <body>
[1;34m*[0m <div>
...
[1;34m*[0m </body>
[1;34m*[0m </html>
[1;34m*[0m
[1;34m*[0m
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 2[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m #HttpOnly_localhost	FALSE	/	FALSE	0	JSESSIONID	0B417BD5890C001B5B25A9B321FE4800
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:8080/not-found
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:8080/not-found' --cookie 'JSESSIONID=0B417BD5890C001B5B25A9B321FE4800'
[1;34m*[0m
> [1;35mGET /not-found HTTP/1.1[0m
> [1;36mHost[0m: localhost:8080
> [1;36mAccept[0m: */*
> [1;36mCookie[0m: JSESSIONID=0B417BD5890C001B5B25A9B321FE4800
> [1;36mUser-Agent[0m: hurl/1.7.0-snapshot
>
[1;34m*[0m [1mRequest body:[0m
[1;34m*[0m
[1;34m*[0m [1mResponse:[0m
[1;34m*[0m
< [1;32mHTTP/1.1 404[0m
< [1;36mVary[0m: Origin
< [1;36mVary[0m: Access-Control-Request-Method
< [1;36mVary[0m: Access-Control-Request-Headers
< [1;36mX-Content-Type-Options[0m: nosniff
< [1;36mX-XSS-Protection[0m: 1; mode=block
< [1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
< [1;36mPragma[0m: no-cache
< [1;36mExpires[0m: 0
< [1;36mX-Frame-Options[0m: DENY
< [1;36mContent-Type[0m: text/html;charset=UTF-8
< [1;36mContent-Language[0m: en-FR
< [1;36mTransfer-Encoding[0m: chunked
< [1;36mDate[0m: Wed, 17 Aug 2022 07:42:46 GMT
<
[1;34m*[0m [1mResponse body:[0m
[1;34m*[0m <!doctype html>
[1;34m*[0m <html lang="en">
[1;34m*[0m <head>
[1;34m*[0m     <meta charset="UTF-8" />
[1;34m*[0m     <meta name="viewport" content="width=device-width, initial-scale=1.0" />
[1;34m*[0m     <title>Error 404 - Quiz</title>
[1;34m*[0m     <link rel="stylesheet" href="/style.css">
[1;34m*[0m     <!--<script src="script.js"></script>-->
[1;34m*[0m </head>
[1;34m*[0m <body>
[1;34m*[0m <div>
[1;34m*[0m     <a href="/"><img class="logo-img" src="/quiz.svg" alt="Quiz logo"></a>
[1;34m*[0m </div>
[1;34m*[0m <div class="main">
[1;34m*[0m     
[1;34m*[0m <h1>Error 404, Page not Found!</h1>
[1;34m*[0m
...
[1;34m*[0m </body>
[1;34m*[0m </html>
[1;34m*[0m
[1;34m*[0m
...
```

### Debugging a specific entry

If you have a lot of entries (request / response pairs) in your Hurl file, using [`--verbose`] or [`--very-verbose`]
can produce a lot of logs and can be difficult to analyse. Instead of passing options to the command line, you can
use an `[Options]` section that will activate logs only for the specified entry:

```hurl
# Checking our home page:
# ...

# Check that we have a 404 response for broken links:
# ...

# Check our health API:
# ...

# Check question API:
GET http://localhost:8080/api/questions
# You can pass options to this entry only
[Options]
verbose: true
[QueryStringParams]
offset: 0
size: 20
sort: oldest

HTTP 200
# ...
```

And run it without [`--verbose`] option:

```shell
$ hurl --no-output basic.hurl
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 4[0m
[1;34m*[0m
[1;34m*[0m [1mEntry options:[0m
[1;34m*[0m verbose: true
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m #HttpOnly_localhost	FALSE	/	FALSE	0	JSESSIONID	31818147FB20A7085AC54C372318BAF1
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:8080/api/questions
[1;34m*[0m [QueryStringParams]
[1;34m*[0m offset: 0
[1;34m*[0m size: 20
[1;34m*[0m sort: oldest
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:8080/api/questions?offset=0&size=20&sort=oldest' --cookie 'JSESSIONID=31818147FB20A7085AC54C372318BAF1'
[1;34m*[0m
> [1;35mGET /api/questions?offset=0&size=20&sort=oldest HTTP/1.1[0m
> [1;36mHost[0m: localhost:8080
> [1;36mAccept[0m: */*
> [1;36mCookie[0m: JSESSIONID=31818147FB20A7085AC54C372318BAF1
> [1;36mUser-Agent[0m: hurl/1.7.0-snapshot
>
[1;34m*[0m [1mResponse:[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200[0m
< [1;36mX-Content-Type-Options[0m: nosniff
< [1;36mX-XSS-Protection[0m: 1; mode=block
< [1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
< [1;36mPragma[0m: no-cache
< [1;36mExpires[0m: 0
< [1;36mX-Frame-Options[0m: DENY
< [1;36mContent-Type[0m: application/json
< [1;36mTransfer-Encoding[0m: chunked
< [1;36mDate[0m: Wed, 17 Aug 2022 08:11:50 GMT
<
[1;34m*[0m
```


## Interactive Mode

We can run the whole Hurl file request by request, with the [`--interactive` option]:

```shell
[1;34m*[0m [1mOptions:[0m
[1;34m*[0m     fail fast: true
[1;34m*[0m     insecure: false
[1;34m*[0m     follow redirect: false
[1;34m*[0m     max redirect: 50

Interactive mode:

Next request:

GET http://localhost:8080

Press Q (Quit) or C (Continue)

[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 1[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:8080
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:8080'
[1;34m*[0m
> [1;35mGET / HTTP/1.1[0m
> [1;36mHost[0m: localhost:8080
> [1;36mAccept[0m: */*
> [1;36mUser-Agent[0m: hurl/1.7.0-snapshot
>
[1;34m*[0m [1mResponse:[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200[0m
< [1;36mSet-Cookie[0m: JSESSIONID=B08BF0F6F83E91750A76E97713A5C144; Path=/; HttpOnly
< [1;36mX-Content-Type-Options[0m: nosniff
< [1;36mX-XSS-Protection[0m: 1; mode=block
< [1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
< [1;36mPragma[0m: no-cache
< [1;36mExpires[0m: 0
< [1;36mX-Frame-Options[0m: DENY
< [1;36mContent-Type[0m: text/html;charset=UTF-8
< [1;36mContent-Language[0m: en-FR
< [1;36mTransfer-Encoding[0m: chunked
< [1;36mDate[0m: Wed, 17 Aug 2022 08:18:36 GMT
<
[1;34m*[0m

Interactive mode:

Next request:

GET http://localhost:8080/not-found

Press Q (Quit) or C (Continue)
```

## Include Headers Like curl

We can also run our file to only output HTTP headers, with [`-i/--include` option].
In this mode, headers of the last entry are displayed:

```shell
$ hurl -i basic.hurl
[1;32mHTTP/1.1 200
[0m[1;36mSet-Cookie[0m: JSESSIONID=76984131F0D0821C4A8D5CB3FC27CD3B; Path=/; HttpOnly
[1;36mX-Content-Type-Options[0m: nosniff
[1;36mX-XSS-Protection[0m: 1; mode=block
[1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
[1;36mPragma[0m: no-cache
[1;36mExpires[0m: 0
[1;36mX-Frame-Options[0m: DENY
[1;36mContent-Type[0m: application/json
[1;36mTransfer-Encoding[0m: chunked
[1;36mDate[0m: Fri, 13 Jan 2023 12:49:47 GMT

{"status":"RUNNING","reportedDate":"2023-01-13T13:49:47+01:00","healthy":true,"operationId":3183000623}
```

If you want to inspect any entry other than the last one, you can run your test to a
given entry with the [`--to-entry` option], starting at index 1:

```shell
$ hurl -i --to-entry 2 basic.hurl
[1;32mHTTP/1.1 404
[0m[1;36mVary[0m: Origin
[1;36mVary[0m: Access-Control-Request-Method
[1;36mVary[0m: Access-Control-Request-Headers
[1;36mX-Content-Type-Options[0m: nosniff
[1;36mX-XSS-Protection[0m: 1; mode=block
[1;36mCache-Control[0m: no-cache, no-store, max-age=0, must-revalidate
[1;36mPragma[0m: no-cache
[1;36mExpires[0m: 0
[1;36mX-Frame-Options[0m: DENY
[1;36mContent-Type[0m: text/html;charset=UTF-8
[1;36mContent-Language[0m: en-FR
[1;36mTransfer-Encoding[0m: chunked
[1;36mDate[0m: Fri, 13 Jan 2023 12:50:52 GMT

<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Error 404 - Quiz</title>
    <link rel="stylesheet" href="/style.css">
    <!--<script src="script.js"></script>-->
</head>
<body>
<div>
    <a href="/"><img class="logo-img" src="/quiz.svg" alt="Quiz logo"></a>
</div>
<div class="main">
    
<h1>Error 404, Page not Found!</h1>

<a href="/">Quiz Home</a>


</div>
<footer>
    <div class="footer">
        <div class="footer-body">a game by <a href="https://hurl.dev">Hurl&RightArrowLeftArrow; Team</a></div>
    </div>
</footer>
</body>
</html>

```

## Using a Proxy

Finally, you can use a proxy between Hurl and your server to inspect requests and responses.

For instance, with [squid]:

1. First, launch proxy listenning on port 3128

    ```shell
$ squid_conf="http_access allow all\nhttp_port 3128\nrequest_header_add From-Proxy Hello\nreply_header_add From-Proxy Hello"
$ (echo "${squid_conf}" | squid -d 2 -N -f /dev/stdin > proxy.log 2>&1) &
2023/04/06 07:58:23| Starting Squid
    ```

2. Then, run Hurl with [`-x/--proxy` option]

    ```shell
$ hurl --proxy localhost:8888 basic.hurl
    ```

[`-v/--verbose` option]: /docs/manual.md#verbose
[`--very-verbose`]: /docs/manual.md#very-verbose
[`--verbose`]: /docs/manual.md#verbose
[`--interactive` option]: /docs/manual.md#interactive
[`-i/--include` option]: /docs/manual.md#include
[`--to-entry` option]: /docs/manual.md#to-entry
[squid]: https://github.com/squid-cache/squid
[`-x/--proxy` option]: /docs/manual.md#proxy
