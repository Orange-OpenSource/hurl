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
[1mhello.hurl[0m: [1;36mRunning[0m [1/2]
[1mhello.hurl[0m: [1;32mSuccess[0m (6 request(s) in 245 ms)
[1massert_json.hurl[0m: [1;36mRunning[0m [2/2]
[1massert_json.hurl[0m: [1;32mSuccess[0m (8 request(s) in 308 ms)
--------------------------------------------------------------------------------
Executed files:  2
Succeeded files: 2 (100.0%)
Failed files:    0 (0.0%)
Duration:        561 ms
```

Or, in case of errors:

```shell
$ hurl --test hello.hurl error_assert_status.hurl 
[1mhello.hurl[0m: [1;36mRunning[0m [1/2]
[1mhello.hurl[0m: [1;32mSuccess[0m (6 request(s) in 258 ms)
[1massert_json.hurl[0m: [1;36mRunning[0m [2/2]
[1;31merror[0m: [1mAssert status code[0m
  [1;34m-->[0m assert_json.hurl:6:8
   [1;34m|[0m
[1;34m 6[0m [1;34m|[0m HTTP/* 200
   [1;34m|[0m        [1;31m^^^[0m [1;31mactual value is <301>[0m
   [1;34m|[0m

[1massert_json.hurl[0m: [1;31mFailure[0m (5 request(s) in 230 ms)
--------------------------------------------------------------------------------
Executed files:  2
Succeeded files: 1 (50.0%)
Failed files:    1 (50.0%)
Duration:        499 ms
```

You can use [`--glob` option] to test files that match a given pattern:

```shell
$ hurl --test --glob "test/integration/**/*.hurl"
```

## Generating an HTML Report

Hurl can also generate an HTML report by using the [`--report-html HTML_DIR`] option.

If the HTML report already exists, the test results will be appended to it.

<img src="/docs/assets/img/hurl-html-report.png" width="320" height="258" alt="Hurl HTML Report">

The input Hurl files (HTML version) are also included and are easily accessed from the main page.

<img src="/docs/assets/img/hurl-html-file.png"  width="380" height="206" alt="Hurl HTML file">


## Use Variables in Tests

To use variables in your tests, you can:

- use [`--variable` option]
- use [`--variables-file` option]
- define environment variables, for instance `HURL_foo=bar`

You will find a detail description in the [Injecting Variables] section of the doc.

[`--output /dev/null`]: /docs/manual.md#output
[`--test`]: /docs/manual.md#test
[`--report-html HTML_DIR`]: /docs/manual.md#report-html
[`--test` option]: /docs/manual.md#test
[`--glob` option]: /docs/manual.md#glob
[`--variable` option]: /docs/manual.md#variable
[`--variables-file` option]: /docs/manual.md#variables-file
[Injecting Variables]: /docs/templates.md#injecting-variables