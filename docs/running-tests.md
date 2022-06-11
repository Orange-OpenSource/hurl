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
hello.hurl: RUNNING [1/2]
hello.hurl: SUCCESS
assert_json.hurl: RUNNING [2/2]
assert_json.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  2
Succeeded: 2 (100.0%)
Failed:    0 (0.0%)
Duration:  427ms
```

Or, in case of errors:

```shell
$ hurl --test hello.hurl error_assert_status.hurl 
hello.hurl: RUNNING [1/2]
hello.hurl: SUCCESS
error_assert_status.hurl: RUNNING [2/2]
error: Assert Status
  --> error_assert_status.hurl:2:10
   |
 2 | HTTP/1.0 200
   |          ^^^ actual value is <404>
   |

error_assert_status.hurl: FAILURE
-------------------------------------------------------------
Executed:  2
Succeeded: 1 (50.0%)
Failed:    1 (50.0%)
Duration:  52ms
```

You can use [`--glob` option] to test files that match a given patten:

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

[`--output /dev/null`]: /docs/man-page.md#output
[`--test`]: /docs/man-page.md#test
[`--report-html HTML_DIR`]: /docs/man-page.md#report-html
[`--test` option]: /docs/man-page.md#test
[`--glob` option]: /docs/man-page.md#glob
[`--variable` option]: /docs/man-page.md#variable
[`--variables-file` option]: /docs/man-page.md#variables-file
[Injecting Variables]: /docs/templates.md#injecting-variables