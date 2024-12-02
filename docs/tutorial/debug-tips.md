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
[1;34m*[0m     follow redirect: false
[1;34m*[0m     insecure: false
[1;34m*[0m     max redirect: 50
[1;34m*[0m     retry: 0
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 1[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:3000
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:3000'
[1;34m*[0m
> [1;35mGET / HTTP/1.1[0m
> [1;36mHost[0m: localhost:3000
> [1;36mAccept[0m: */*
> [1;36mUser-Agent[0m: hurl/4.0.0
>
[1;34m*[0m [1mResponse: (received 9564 bytes in 11 ms)[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200 OK[0m
< [1;36mContent-Type[0m: text/html; charset=utf-8
< [1;36mContent-Length[0m: 9564
< [1;36mSet-Cookie[0m: x-session-id=s%3AEE3wsnrgUPSyAkgJZGa3jMWk7xmOtv4E.kXQpkmNBXnFOqmeSssqXnecF4qqv1D7bKu3rpbEJxmQ; Path=/; HttpOnly; SameSite=Strict
< [1;36mDate[0m: Wed, 26 Jul 2023 13:16:39 GMT
< [1;36mConnection[0m: keep-alive
< [1;36mKeep-Alive[0m: timeout=5
<
[1;34m*[0m
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 2[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m #HttpOnly_localhost	FALSE	/	FALSE	0	x-session-id	s%3AEE3wsnrgUPSyAkgJZGa3jMWk7xmOtv4E.kXQpkmNBXnFOqmeSssqXnecF4qqv1D7bKu3rpbEJxmQ
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:3000/not-found
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl --cookie 'x-session-id=s%3AEE3wsnrgUPSyAkgJZGa3jMWk7xmOtv4E.kXQpkmNBXnFOqmeSssqXnecF4qqv1D7bKu3rpbEJxmQ' 'http://localhost:3000/not-found'
[1;34m*[0m
> [1;35mGET /not-found HTTP/1.1[0m
> [1;36mHost[0m: localhost:3000
> [1;36mAccept[0m: */*
> [1;36mCookie[0m: x-session-id=s%3AEE3wsnrgUPSyAkgJZGa3jMWk7xmOtv4E.kXQpkmNBXnFOqmeSssqXnecF4qqv1D7bKu3rpbEJxmQ
> [1;36mUser-Agent[0m: hurl/4.0.0
>
[1;34m*[0m [1mResponse: (received 2217 bytes in 3 ms)[0m
[1;34m*[0m
< [1;32mHTTP/1.1 404 Not Found[0m
< [1;36mContent-Type[0m: text/html; charset=utf-8
< [1;36mContent-Length[0m: 2217
< [1;36mDate[0m: Wed, 26 Jul 2023 13:16:39 GMT
< [1;36mConnection[0m: keep-alive
< [1;36mKeep-Alive[0m: timeout=5
<
...
```

Lines beginning with `*` are debug info, lines that begin with `>` are HTTP request headers and lines that begin with
`<` are HTTP response headers.

In each run request, we can also see a curl command line to replay this particular request:

```shell
...
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl --cookie 'x-session-id=s%3AEE3wsnrgUPSyAkgJZGa3jMWk7xmOtv4E.kXQpkmNBXnFOqmeSssqXnecF4qqv1D7bKu3rpbEJxmQ' 'http://localhost:3000/not-found'
...
```

In verbose mode, HTTP request and response bodies are not displayed in the debug logs. If you need to inspect the 
request or response body, you can display more logs with [`--very-verbose`] option:

```shell
$ hurl --very-verbose --no-output basic.hurl
[1;34m*[0m [1mOptions:[0m
[1;34m*[0m     fail fast: true
[1;34m*[0m     follow redirect: false
[1;34m*[0m     insecure: false
[1;34m*[0m     max redirect: 50
[1;34m*[0m     retry: 0
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 1[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:3000
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:3000'
[1;34m*[0m
[1;34m**[0m [32mWARNING: failed to open cookie file ""[0m
[1;34m**[0m [32m  Trying 127.0.0.1:3000...[0m
[1;34m**[0m [32mConnected to localhost (127.0.0.1) port 3000 (#0)[0m
> [1;35mGET / HTTP/1.1[0m
> [1;36mHost[0m: localhost:3000
> [1;36mAccept[0m: */*
> [1;36mUser-Agent[0m: hurl/4.0.0
>
[1;34m*[0m [1mRequest body:[0m
[1;34m*[0m
[1;34m**[0m [32mAdded cookie x-session-id="s%3A_l88C6GKbPeC5YuDLraWARY32NB3bP-l.T%2BViEW%2BqMrmLZDqwzDxtEbdtW67lCKt0jGvvlfqls%2FI" for domain localhost, path /, expire 0[0m
[1;34m**[0m [32mConnection #0 to host localhost left intact[0m
[1;34m*[0m [1mResponse: (received 9564 bytes in 9 ms)[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200 OK[0m
< [1;36mContent-Type[0m: text/html; charset=utf-8
< [1;36mContent-Length[0m: 9564
< [1;36mSet-Cookie[0m: x-session-id=s%3A_l88C6GKbPeC5YuDLraWARY32NB3bP-l.T%2BViEW%2BqMrmLZDqwzDxtEbdtW67lCKt0jGvvlfqls%2FI; Path=/; HttpOnly; SameSite=Strict
< [1;36mDate[0m: Wed, 26 Jul 2023 13:19:45 GMT
< [1;36mConnection[0m: keep-alive
< [1;36mKeep-Alive[0m: timeout=5
<
[1;34m*[0m [1mResponse body:[0m
[1;34m*[0m <!doctype html>
[1;34m*[0m <html lang="en">
[1;34m*[0m     <head>
[1;34m*[0m         <meta charset="UTF-8" />
[1;34m*[0m         <title>Movies Box</title>
[1;34m*[0m         <link rel="icon" type="image/png" href="/img/favicon.png" />
[1;34m*[0m         <link rel="stylesheet" href="/css/style.css" />
[1;34m*[0m     </head>
[1;34m*[0m     <body>
...
[1;34m*[0m     </body>
[1;34m*[0m </html>
[1;34m*[0m
[1;34m*[0m [1mTimings:[0m
[1;34m*[0m begin: 2023-07-26 13:19:45.378037 UTC
[1;34m*[0m end: 2023-07-26 13:19:45.387332 UTC
[1;34m*[0m namelookup: 4182 µs
[1;34m*[0m connect: 4798 µs
[1;34m*[0m app_connect: 0 µs
[1;34m*[0m pre_transfer: 4912 µs
[1;34m*[0m start_transfer: 9126 µs
[1;34m*[0m total: 9171 µs
[1;34m*[0m
[1;34m*[0m [1m------------------------------------------------------------------------------[0m
[1;34m*[0m [1mExecuting entry 2[0m
[1;34m*[0m
[1;34m*[0m [1mCookie store:[0m
[1;34m*[0m #HttpOnly_localhost	FALSE	/	FALSE	0	x-session-id	s%3A_l88C6GKbPeC5YuDLraWARY32NB3bP-l.T%2BViEW%2BqMrmLZDqwzDxtEbdtW67lCKt0jGvvlfqls%2FI
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:3000/not-found
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl --cookie 'x-session-id=s%3A_l88C6GKbPeC5YuDLraWARY32NB3bP-l.T%2BViEW%2BqMrmLZDqwzDxtEbdtW67lCKt0jGvvlfqls%2FI' 'http://localhost:3000/not-found'
[1;34m*[0m
[1;34m**[0m [32mFound bundle for host: 0x60000340c930 [serially][0m
[1;34m**[0m [32mCan not multiplex, even if we wanted to[0m
[1;34m**[0m [32mRe-using existing connection #0 with host localhost[0m
> [1;35mGET /not-found HTTP/1.1[0m
> [1;36mHost[0m: localhost:3000
> [1;36mAccept[0m: */*
> [1;36mCookie[0m: x-session-id=s%3A_l88C6GKbPeC5YuDLraWARY32NB3bP-l.T%2BViEW%2BqMrmLZDqwzDxtEbdtW67lCKt0jGvvlfqls%2FI
> [1;36mUser-Agent[0m: hurl/4.0.0
>
[1;34m*[0m [1mRequest body:[0m
[1;34m*[0m
[1;34m**[0m [32mConnection #0 to host localhost left intact[0m
[1;34m*[0m [1mResponse: (received 2217 bytes in 5 ms)[0m
[1;34m*[0m
< [1;32mHTTP/1.1 404 Not Found[0m
< [1;36mContent-Type[0m: text/html; charset=utf-8
< [1;36mContent-Length[0m: 2217
< [1;36mDate[0m: Wed, 26 Jul 2023 13:19:45 GMT
< [1;36mConnection[0m: keep-alive
< [1;36mKeep-Alive[0m: timeout=5
<
[1;34m*[0m [1mResponse body:[0m
[1;34m*[0m <!doctype html>
[1;34m*[0m <html lang="en">
...
[1;34m*[0m <h3>Not Found</h3>
[1;34m*[0m <h4>404</h4>
...
[1;34m*[0m </html>
[1;34m*[0m
[1;34m*[0m [1mTimings:[0m
[1;34m*[0m begin: 2023-07-26 13:19:45.390823 UTC
[1;34m*[0m end: 2023-07-26 13:19:45.395983 UTC
[1;34m*[0m namelookup: 44 µs
[1;34m*[0m connect: 0 µs
[1;34m*[0m app_connect: 0 µs
[1;34m*[0m pre_transfer: 126 µs
[1;34m*[0m start_transfer: 5100 µs
[1;34m*[0m total: 5124 µs
[1;34m*[0m
...
```

[`--very-verbose`] output is much more verbose; with body request and response, [`libcurl`] logs and [response timings] 
are displayed. 

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

# Check search API:
GET http://localhost:3000/api/search
[Options]
verbose: true
[QueryStringParams]
q: 1982
sort: name

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
[1;34m*[0m #HttpOnly_localhost	FALSE	/	FALSE	0	x-session-id	s%3Aq_5wf1l2wBQ_96y6kpLeR0J4zLJF34EZ.n%2Bu1UJPqK0Ih2tz3Dd6w2kXAuufueT6HQDekBPtHhbc
[1;34m*[0m
[1;34m*[0m [1mRequest:[0m
[1;34m*[0m GET http://localhost:3000/api/search
[1;34m*[0m [QueryStringParams]
[1;34m*[0m q: 1982
[1;34m*[0m sort: name
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl --cookie 'x-session-id=s%3Aq_5wf1l2wBQ_96y6kpLeR0J4zLJF34EZ.n%2Bu1UJPqK0Ih2tz3Dd6w2kXAuufueT6HQDekBPtHhbc' 'http://localhost:3000/api/search?q=1982&sort=name'
[1;34m*[0m
> [1;35mGET /api/search?q=1982&sort=name HTTP/1.1[0m
> [1;36mHost[0m: localhost:3000
> [1;36mAccept[0m: */*
> [1;36mCookie[0m: x-session-id=s%3Aq_5wf1l2wBQ_96y6kpLeR0J4zLJF34EZ.n%2Bu1UJPqK0Ih2tz3Dd6w2kXAuufueT6HQDekBPtHhbc
> [1;36mUser-Agent[0m: hurl/4.0.0
>
[1;34m*[0m [1mResponse: (received 1447 bytes in 0 ms)[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200 OK[0m
< [1;36mCache-control[0m: no-store
< [1;36mContent-Type[0m: application/json; charset=utf-8
< [1;36mContent-Length[0m: 1447
< [1;36mDate[0m: Wed, 26 Jul 2023 13:29:39 GMT
< [1;36mConnection[0m: keep-alive
< [1;36mKeep-Alive[0m: timeout=5
<
[1;34m*[0m
```

## Use Error Format

When you’ve asserts errors, the analysis can be difficult because you don’t have a lot of information apart 
of the expected values:

```shell
$ hurl --test basic.hurl
[1mbasic.hurl[0m: [1;36mRunning[0m [1/1]
[1;31merror[0m: [1mAssert failure[0m
  [1;34m-->[0m basic.hurl:47:0
   [1;34m|[0m
[1;34m47[0m [1;34m|[0m jsonpath "$[0].name" == "Robocop"
   [1;34m|[0m   [1;31mactual:   string <Blade Runner>[0m
   [1;34m|[0m   [1;31mexpected: string <Robocop>[0m
   [1;34m|[0m

[1mbasic.hurl[0m: [1;31mFailure[0m (4 request(s) in 16 ms)
--------------------------------------------------------------------------------
Executed files:  1
Succeeded files: 0 (0.0%)
Failed files:    1 (100.0%)
Duration:        17 ms
```

With [`--error-format`] option, you can opt in for a longer error description in case of error assert. On any error,
we'll get the response headers and body. This is useful to see the expected values, especially in CI/CD 
context when you've to analyse past executed tests.

```shell
$ hurl --error-format long --test basic.hurl
[1mbasic.hurl[0m: [1;36mRunning[0m [1/1]
[1;32mHTTP/1.1 200
[0m[1;36mCache-control[0m: no-store
[1;36mContent-Type[0m: application/json; charset=utf-8
[1;36mContent-Length[0m: 1447
[1;36mDate[0m: Wed, 26 Jul 2023 14:14:00 GMT
[1;36mConnection[0m: keep-alive
[1;36mKeep-Alive[0m: timeout=5

[{"name":"Blade Runner","url":"/movies/blade-runner","director":"Ridley Scott","release_date":"1982-06-25","actors":["Harrison Ford","Rutger Hauer","Sean Young","Edward James Olmos"],"artwork":"/img/blade-runner-800x1200.webp","artwork_128":"/img/blade-runner-128x192.webp"},{"name":"Conan the Barbarian","url":"/movies/conan-the-barbarian","director":"John Milius","release_date":"1982-05-14","actors":["Arnold Schwarzenegger","James Earl Jones","Sandahl Bergman","Ben Davidson","Cassandra Gaviola","Gerry Lopez","Mako","Valerie Quennessen","William Smith","Max von Sydow"],"artwork":"/img/conan-the-barbarian-800x1200.webp","artwork_128":"/img/conan-the-barbarian-128x192.webp"},{"name":"The Dark Crystal","url":"/movies/the-dark-crystal","director":"Jim Henson","release_date":"1982-12-17","actors":["Stephen Garlick","Lisa Maxwell","Billie Whitelaw","Percy Edwards"],"artwork":"/img/the-dark-crystal-800x1200.webp","artwork_128":"/img/the-dark-crystal-128x192.webp"},{"name":"The Thing","url":"/movies/the-thing","director":"John Carpenter","release_date":"1982-06-25","actors":["Kurt Russell"],"artwork":"/img/the-thing-800x1200.webp","artwork_128":"/img/the-thing-128x192.webp"},{"name":"Tron","url":"/movies/tron","director":"Steven Lisberger","release_date":"1982-07-09","actors":["Jeff Bridges","Bruce Boxleitner","David Warner","Cindy Morgan","Barnard Hughes"],"artwork":"/img/tron-800x1200.webp","artwork_128":"/img/tron-128x192.webp"}]

[1;31merror[0m: [1mAssert failure[0m
  [1;34m-->[0m basic.hurl:47:0
   [1;34m|[0m
[1;34m47[0m [1;34m|[0m jsonpath "$[0].name" == "Robocop"
   [1;34m|[0m   [1;31mactual:   string <Blade Runner>[0m
   [1;34m|[0m   [1;31mexpected: string <Robocop>[0m
   [1;34m|[0m

[1mbasic.hurl[0m: [1;31mFailure[0m (4 request(s) in 23 ms)
```

## Get Response Body

When there are errors (HTTP runtimes or asserts errors), Hurl doesn't output HTTP response body. But sometimes the response
body is necessary to explain failures. To do so, either:

- use [`--very-verbose`] globally or per-request to get the full body response

```hurl
GET https://foo.com/success
HTTP 200

GET https://foo.com/failure
[Options]
very-verbose: true
HTTP 200

GET https://foo.com/success
HTTP 200
```

- use [`--output`] per-request and [`--ignore-asserts`]: `--ignore-asserts` will disable any check, while `--output` can
be used to output any particular response body. With this file, the response of `https://foo.com/failure` will be outputted
on standard output:

```hurl
GET https://foo.com/success
HTTP 200

GET https://foo.com/failure
[Options]
# use - to output on standard output, foo.bin to save on disk 
output: -
HTTP 200

GET https://foo.com/success
HTTP 200
```

To get more information, one can also used a JSON report with [`--report-json`]. This option produces a structured export
of all run datas (request headers, response headers, response body, curl debug command line etc...)

```shell
$ hurl --report-json /tmp/report *.hurl
```


## Interactive Mode

We can run the whole Hurl file request by request, with the [`--interactive` option]:

```shell
$ hurl --verbose --interactive basic.hurl
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
[1;34m*[0m GET http://localhost:3000
[1;34m*[0m
[1;34m*[0m Request can be run with the following curl command:
[1;34m*[0m curl 'http://localhost:3000'
[1;34m*[0m
> [1;35mGET / HTTP/1.1[0m
> [1;36mHost[0m: localhost:3000
> [1;36mAccept[0m: */*
> [1;36mUser-Agent[0m: hurl/4.0.0
>
[1;34m*[0m [1mResponse: (received 9564 bytes in 11 ms)[0m
[1;34m*[0m
< [1;32mHTTP/1.1 200 OK[0m
< [1;36mContent-Type[0m: text/html; charset=utf-8
< [1;36mContent-Length[0m: 9564
< [1;36mSet-Cookie[0m: x-session-id=s%3AEE3wsnrgUPSyAkgJZGa3jMWk7xmOtv4E.kXQpkmNBXnFOqmeSssqXnecF4qqv1D7bKu3rpbEJxmQ; Path=/; HttpOnly; SameSite=Strict
< [1;36mDate[0m: Wed, 26 Jul 2023 13:16:39 GMT
< [1;36mConnection[0m: keep-alive
< [1;36mKeep-Alive[0m: timeout=5
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
[0m[1;36mCache-control[0m: no-store
[1;36mContent-Type[0m: application/json; charset=utf-8
[1;36mContent-Length[0m: 1447
[1;36mDate[0m: Wed, 26 Jul 2023 14:58:27 GMT
[1;36mConnection[0m: keep-alive
[1;36mKeep-Alive[0m: timeout=5

[{"name":"Blade Runner","url":"/movies/blade-runner","director":"Ridley Scott","release_date":"1982-06-25",...
```

If you want to inspect any entry other than the last one, you can run your test to a
given entry with the [`--to-entry` option], starting at index 1:

```shell
$ hurl -i --to-entry 2 basic.hurl
[1;32mHTTP/1.1 404
[0m[1;36mContent-Type[0m: text/html; charset=utf-8
[1;36mContent-Length[0m: 2217
[1;36mDate[0m: Wed, 26 Jul 2023 14:59:57 GMT
[1;36mConnection[0m: keep-alive
[1;36mKeep-Alive[0m: timeout=5

<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <title></title>
...
    </body>
</html>
```

## Export curl Commands

[`--curl`] command line option can be used to produce a file of curl commands of a run. This is equivalent of running 
Hurl in verbose and grepping the standard error for the debug curl command. The produced file is just a text list of 
curl debug commands, one line per entry (retry command are not written).

```shell
$ echo 'HEAD https://example.org' | hurl --repeat 3 --curl /tmp/curl.txt
$ cat /tmp/curl.txt
curl --head 'https://example.org'
curl --head 'https://example.org'
curl --head 'https://example.org'
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


[`-v/--verbose` option]: /docs/manual.md#verbose
[`--very-verbose`]: /docs/manual.md#very-verbose
[`--verbose`]: /docs/manual.md#verbose
[`--interactive` option]: /docs/manual.md#interactive
[`-i/--include` option]: /docs/manual.md#include
[`--to-entry` option]: /docs/manual.md#to-entry
[mitmproxy]: https://mitmproxy.org
[`-x/--proxy` option]: /docs/manual.md#proxy
[`--error-format`]: /docs/manual.md#error-format
[`libcurl`]: https://curl.se/libcurl/
[response timings]: /docs/response.md#timings
[`--ignore-asserts`]: /docs/manual.md#ignore-asserts
[`--output`]: /docs/manual.md#output
[`--curl`]: /docs/manual.md#curl
[`--report-json`]: /docs/manual.md#report-json
