# Running Tests

## Use --test Option

Hurl is run by default as an HTTP client, returning the body of the last HTTP response.

```shell
$ hurl hello.hurl
Hello World!
```

When multiple input files are provided, Hurl outputs the body of the last HTTP response of each file.

```shell
$ hurl hello.hurl assert_json.hurl
Hello World![
  { "id": 1, "name": "Bob"},
  { "id": 2, "name": "Bill"}
]
```

For testing, we are only interested in the asserts results, we don't need the HTTP body response. To use Hurl as a 
test tool with an adapted output, you can use [`--test` option]:

```shell
$ hurl --test hello.hurl assert_json.hurl
[1mhello.hurl[0m: [1;32mSuccess[0m (6 request(s) in 245 ms)
[1massert_json.hurl[0m: [1;32mSuccess[0m (8 request(s) in 308 ms)
--------------------------------------------------------------------------------
Executed files:    2
Executed requests: 10 (17.82/s)
Succeeded files:   2 (100.0%)
Failed files:      0 (0.0%)
Duration:          561 ms
```

Or, in case of errors:

```shell
$ hurl --test hello.hurl error_assert_status.hurl 
[1mhello.hurl[0m: [1;32mSuccess[0m (4 request(s) in 5 ms)
[1;31merror[0m: [1mAssert status code[0m
  [1;34m-->[0m error_assert_status.hurl:9:6
[1;34m   |[0m
[1;34m   |[0m [90mGET http://localhost:8000/not_found[0m
[1;34m   |[0m[90m ...[0m
[1;34m 9 |[0m HTTP 200
[1;34m   |[0m[1;31m      ^^^ actual value is <404>[0m
[1;34m   |[0m

[1merror_assert_status.hurl[0m: [1;31mFailure[0m (1 request(s) in 2 ms)
--------------------------------------------------------------------------------
Executed files:    2
Executed requests: 5 (500.0/s)
Succeeded files:   1 (50.0%)
Failed files:      1 (50.0%)
Duration:          10 ms
```

> With or without `--test`, all asserts are always executed. `--test` adds a run recap and disables the output of the 
> last response. To ignore asserts execution, you can use [`--ignore-asserts`].

In test mode, files are executed in parallel to speed-ud the execution. If a sequential run is needed, you can use
[`--jobs 1`] option to execute tests one by one.

```shell
$ hurl --test --jobs 1 *.hurl
```

[`--repeat` option] can be used to repeat run files and do [performance check]. For instance, this call will run 1000 tests
in parallel:

```shell
$ hurl --test --repeat 1000 stress.hurl
```


### Selecting Tests

Hurl can take multiple files into inputs:

```shell
$ hurl --test test/integration/a.hurl test/integration/b.hurl test/integration/c.hurl 
```

```shell
$ hurl --test test/integration/*.hurl 
```

Or you can simply give a directory and Hurl will find files with `.hurl` extension recursively:

```shell
$ hurl --test test/integration/
```

Finally, you can use [`--glob` option] to test files that match a given pattern:

```shell
$ hurl --test --glob "test/integration/**/*.hurl"
```

## Debugging

### Debug Logs

If you need more error context, you can use [`--error-format long` option] to print HTTP bodies for failed asserts:

```shell
$ hurl --test --error-format long hello.hurl error_assert_status.hurl 
[1mhello.hurl[0m: [1;32mSuccess[0m (4 request(s) in 6 ms)
[1;32mHTTP/1.1 404
[0m[1;36mServer[0m: Werkzeug/3.0.3 Python/3.12.4
[1;36mDate[0m: Wed, 10 Jul 2024 15:42:41 GMT
[1;36mContent-Type[0m: text/html; charset=utf-8
[1;36mContent-Length[0m: 207
[1;36mServer[0m: Flask Server
[1;36mConnection[0m: close

<!doctype html>
<html lang=en>
<title>404 Not Found</title>
<h1>Not Found</h1>
<p>The requested URL was not found on the server. If you entered the URL manually please check your spelling and try again.</p>


[1;31merror[0m: [1mAssert status code[0m
  [1;34m-->[0m error_assert_status.hurl:9:6
[1;34m   |[0m
[1;34m   |[0m [90mGET http://localhost:8000/not_found[0m
[1;34m   |[0m[90m ...[0m
[1;34m 9 |[0m HTTP 200
[1;34m   |[0m[1;31m      ^^^ actual value is <404>[0m
[1;34m   |[0m

[1merror_assert_status.hurl[0m: [1;31mFailure[0m (1 request(s) in 2 ms)
--------------------------------------------------------------------------------
Executed files:    2
Executed requests: 5 (454.5/s)
Succeeded files:   1 (50.0%)
Failed files:      1 (50.0%)
Duration:          11 ms
```

Individual requests can be modified with [`[Options]` section][options] to turn on logs for a particular request, using
[`verbose`] and [`very-verbose`] option. 

With this Hurl file:

```hurl
GET https://foo.com
HTTP 200

GET https://bar.com
[Options]
very-verbose: true
HTTP 200

GET https://baz.com
HTTP 200
```

Running `hurl --test .` will output debug logs for the request to `https://bar.com`.

[`--verbose`] / [`--very-verbose`] can also be enabled globally, for every requests of every tested files:

```shell
$ hurl --test --very-verbose .
```

### HTTP Responses

In test mode, HTTP responses are not displayed. One way to get HTTP responses even in test mode is to use 
[`--output` option] of `[Options]` section: `--output file` allows to save a particular response to a file, while 
`--output -` allows to redirect HTTP responses to standard output.

```hurl
GET http://foo.com
HTTP 200

GET https://bar.com
[Options]
output: -
HTTP 200
```

```shell
$ hurl --test .
<html><head><meta http-equiv="content-type" content="text/html;charset=utf-8">
<title>301 Moved</TITLE></head><body>
<h1>301 Moved</h1>
The document has moved
<a HREF="https://www.bar.com/">here</a>.
</body></html>
[1m/tmp/test.hurl[0m: [1;32mSuccess[0m (2 request(s) in 184 ms)
--------------------------------------------------------------------------------
Executed files:    1
Executed requests: 2 (10.7/s)
Succeeded files:   1 (100.0%)
Failed files:      0 (0.0%)
Duration:          187 ms
```

## Stress and Performance Tests

Hurl can be used to perform stress tests:

- with [query duration]:

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
duration < 1200
```

- [response metrics] with [`--very-verbose`] or [`--json`] to get structured timing data:

```shell
$ hurl --json foo.hurl | jq
...
          "timings": {
            "app_connect": 0,
            "begin_call": "2025-10-06T07:00:57.127794Z",
            "connect": 120915,
            "end_call": "2025-10-06T07:00:57.280724Z",
            "name_lookup": 92987,
            "pre_transfer": 121014,
            "start_transfer": 152721,
            "total": 152807
          }
...
```

In performance uses-cases, it's important to know how Hurl is working to get the best request per second load. Each Hurl file 
is processed by its own libcurl instance (roughly speaking a living HTTP connection). Let's say we want to stress our 
application with 100,000 HTTP requests and see how it's behaving. The simplest idea would be to make a single Hurl file 
repeating 100,000 requests:

```hurl
GET https://my-website/health
[Options]
repeat: 100000
```

and run it:

```shell
$ hurl --test perf.hurl
```

With a single Hurl file, we won't benefit from any parallel runs (files are run in parallel, requests _within_ a file are
run sequentially). To benefit from parallel run, we can use [`--repeat`] as a CLI argument to repeat file execution. It's 
as if we've written 100,000 identical Hurl files and run them:


```hurl
GET https://my-website/health
```

and run it:

```shell
$ hurl --test --repeat 100000 perf.hurl
```

In this case, we're running 100,000 Hurl files in parallel. Now, we have another problem: as each file is basically a
HTTP connection, we can run out of TCP port, a phenomenon known as [ephemeral ports exhaustion]. So, to recap:

- running 100,000 requests in a single file won't benefit from parallel run
- running 100,000 one request files can lead to TCP port resources issues

The solution is to combine the two approaches: running 10 files of 10,000 HTTP requests. With 10 files of 10,000
requests, we're going to execute those files in parallel in their own worker. Each worker will be working sequentially, 
maximizing the usage and not running into TCP ports exhaustion. We can force jobs count to be exactly 10 so 
all workers start at the same time and no one is waiting to get a job done:

```hurl
GET https://my-website/health
[Options]
repeat: 10000
```

```shell
$ hurl --test --repeat 10 --jobs 10 perf.hurl
```



## Generating Report

In the different reports, files are always referenced in the input order (which, as tests are executed in parallel, can 
be different from the execution order).

### HTML Report

Hurl can generate an HTML report by using the [`--report-html DIR`] option.

If the HTML report already exists, the test results will be appended to it.

<div class="picture">
    <img class="u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/hurl-html-report.png" width="670" alt="Hurl HTML Report">
</div>

The input Hurl files (HTML version) are also included and are easily accessed from the main page.

<div class="picture">
    <img class="u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/hurl-html-file.png" width="380" alt="Hurl HTML file">
</div>

### JSON Report

A JSON report can be produced by using the [`--report-json DIR`]. The report directory will contain a `report.json` 
file, including each test file executed with [`--json`] option and a reference to each HTTP response of the run dumped 
to disk.

If the JSON report already exists, it will be updated with the new test results.

### JUnit Report

A JUnit report can be produced by using the [`--report-junit FILE`] option.

If the JUnit report already exists, it will be updated with the new test results.

### TAP Report

A TAP report ([Test Anything Protocol]) can be produced by using the [`--report-tap FILE`] option.

If the TAP report already exists, it will be updated with the new test results.

## Use Variables in Tests

To use variables in your tests, you can:

- use [`--variable` option]
- use [`--variables-file` option]
- define environment variables, for instance `HURL_VARIABLE_foo=bar`

You will find a detailed description in the [Injecting Variables] section of the docs.

[`--output /dev/null`]: /docs/manual.md#output
[`--test`]: /docs/manual.md#test
[`--report-html DIR`]: /docs/manual.md#report-html
[`--report-json DIR`]: /docs/manual.md#report-json
[`--report-junit FILE`]: /docs/manual.md#report-junit
[`--report-tap FILE`]: /docs/manual.md#report-tap
[`--test` option]: /docs/manual.md#test
[`--glob` option]: /docs/manual.md#glob
[`--variable` option]: /docs/manual.md#variable
[`--variables-file` option]: /docs/manual.md#variables-file
[Injecting Variables]: /docs/templates.md#injecting-variables
[Test Anything Protocol]: https://testanything.org
[`--jobs 1`]: /docs/manual.md#jobs
[`--json`]: /docs/manual.md#json
[`--error-format long` option]: /docs/manual.md#error-format
[options]: /docs/request.md#options
[`--verbose`]: /docs/manual.md#verbose
[`--very-verbose`]: /docs/manual.md#very-verbose
[`verbose`]: /docs/manual.md#verbose
[`very-verbose`]: /docs/manual.md#very-verbose
[`--output` option]: /docs/manual.md#output
[`--repeat` option]: /docs/manual.md#repeat
[query duration]: /docs/asserting-response.md#duration-assert
[response metrics]: /docs/response.md#timings
[`--ignore-asserts`]: /docs/manual.md#ignore-asserts
[performance check]: /docs/running-tests.md#stress-and-performance-tests
[ephemeral ports exhaustion]: https://blog.cloudflare.com/how-to-stop-running-out-of-ephemeral-ports-and-start-to-love-long-lived-connections/
[`--repeat`]: /docs/manual.md#repeat