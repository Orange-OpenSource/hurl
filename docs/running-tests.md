# Running Tests

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


For testing, we are only interested in the asserts results, we don't need body response.
Several options relating to testing can be used:

- do not output response body ([`--output /dev/null`])

    ```shell
$ hurl --output /dev/null hello.hurl assert_json.hurl
    ```

- show progress ([`--progress`])

    ```shell
$ hurl --progress /dev/null hello.hurl assert_json.hurl
hello.hurl: RUNNING [1/2]
hello.hurl: SUCCESS
assert_json.hurl: RUNNING [2/2]
assert_json.hurl: SUCCESS
Hello World![
{ "id": 1, "name": "Bob"},
{ "id": 2, "name": "Bill"}
]
    ```

- print summary ([`--summary`])

    ```shell
$ hurl --summary hello.hurl assert_json.hurl
Hello World![
{ "id": 1, "name": "Bob"},
{ "id": 2, "name": "Bill"}
]
--------------------------------------------------------------------------------
Executed:  2
Succeeded: 2 (100.0%)
Failed:    0 (0.0%)
Duration:  134ms
    ```

For convenience, all these options can also be set with the unique option [`--test`].

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


## Generating an HTML report

Hurl can also generates an HTML by using the [`--report-html HTML_DIR`] option.

If the HTML report already exists, the test results will be appended to it.

<img src="/docs/assets/img/hurl-html-report.png" width="320" height="258" alt="Hurl HTML Report">

The input Hurl files (HTML version) are also included and are easily accessed from the main page.

<img src="/docs/assets/img/hurl-html-file.png"  width="380" height="206" alt="Hurl HTML file">


[`--output /dev/null`]: /docs/man-page.md#output
[`--progress`]: /docs/man-page.md#progress
[`--summary`]: /docs/man-page.md#summary
[`--test`]: /docs/man-page.md#test
[`--report-html HTML_DIR`]: /docs/man-page.md#report-html
