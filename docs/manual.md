# Manual

## Name

hurl - run and test HTTP requests.


## Synopsis

**hurl** [options] [FILE...]


## Description

**Hurl** is a command line tool that runs HTTP requests defined in a simple plain text format.

It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very versatile, it can be used for fetching data and testing HTTP sessions: HTML content, REST / SOAP / GraphQL APIs, or any other XML / JSON based APIs.

```shell
$ hurl session.hurl
```

If no input files are specified, input is read from stdin.

```shell
$ echo GET http://httpbin.org/get | hurl
    {
      "args": {},
      "headers": {
        "Accept": "*/*",
        "Accept-Encoding": "gzip",
        "Content-Length": "0",
        "Host": "httpbin.org",
        "User-Agent": "hurl/0.99.10",
        "X-Amzn-Trace-Id": "Root=1-5eedf4c7-520814d64e2f9249ea44e0"
      },
      "origin": "1.2.3.4",
      "url": "http://httpbin.org/get"
    }
```


Output goes to stdout by default. To have output go to a file, use the [`-o, --output`](#output) option:

```shell
$ hurl -o output input.hurl
```

By default, Hurl executes all HTTP requests and outputs the response body of the last HTTP call.

To have a test oriented output, you can use [`--test`](#test) option:

```shell
$ hurl --test *.hurl
```


## Hurl File Format

The Hurl file format is fully documented in [https://hurl.dev/docs/hurl-file.html](https://hurl.dev/docs/hurl-file.html)

It consists of one or several HTTP requests

```hurl
GET http:/example.org/endpoint1
GET http:/example.org/endpoint2
```


### Capturing values

A value from an HTTP response can be-reused for successive HTTP requests.

A typical example occurs with CSRF tokens.

```hurl
GET https://example.org
HTTP 200
# Capture the CSRF token value from html body.
[Captures]
csrf_token: xpath "normalize-space(//meta[@name='_csrf_token']/@content)"

# Do the login !
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
```

More information on captures can be found here [https://hurl.dev/docs/capturing-response.html](https://hurl.dev/docs/capturing-response.html)

### Asserts

The HTTP response defined in the Hurl file are used to make asserts. Responses are optional.

At the minimum, response includes assert on the HTTP status code.

```hurl
GET http:/example.org
HTTP 301
```

It can also include asserts on the response headers

```hurl
GET http:/example.org
HTTP 301
Location: http://www.example.org
```

Explicit asserts can be included by combining a query and a predicate

```hurl
GET http:/example.org
HTTP 301
[Asserts]
xpath "string(//title)" == "301 Moved"
```

With the addition of asserts, Hurl can be used as a testing tool to run scenarios.

More information on asserts can be found here [https://hurl.dev/docs/asserting-response.html](https://hurl.dev/docs/asserting-response.html)

## Options

Options that exist in curl have exactly the same semantics.

Options specified on the command line are defined for every Hurl file's entry.

For instance:

```shell
$ hurl --location foo.hurl
```

will follow redirection for each entry in `foo.hurl`. You can also define an option only for a particular entry with an `[Options]` section. For instance, this Hurl file:

```hurl
GET https://example.org
HTTP 301

GET https://example.org
[Options]
location: true
HTTP 200
```

will follow a redirection only for the second entry.

| Option | Description |
| --- | --- |
| <a href="#cacert" id="cacert"><code>--cacert &lt;FILE&gt;</code></a> | Specifies the certificate file for peer verification. The file may contain multiple CA certificates and must be in PEM format.<br>Normally Hurl is built to use a default file for this, so this option is typically used to alter that default file.<br> |
| <a href="#cert" id="cert"><code>-E, --cert &lt;CERTIFICATE[:PASSWORD]&gt;</code></a> | Client certificate file and password.<br><br>See also [`--key`](#key).<br> |
| <a href="#color" id="color"><code>--color</code></a> | Colorize debug output (the HTTP response output is not colorized). <br> |
| <a href="#compressed" id="compressed"><code>--compressed</code></a> | Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.<br> |
| <a href="#connect-timeout" id="connect-timeout"><code>--connect-timeout &lt;SECONDS&gt;</code></a> | Maximum time in seconds that you allow Hurl's connection to take.<br><br>See also [`-m, --max-time`](#max-time).<br> |
| <a href="#connect-to" id="connect-to"><code>--connect-to &lt;HOST1:PORT1:HOST2:PORT2&gt;</code></a> | For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead. This option can be used several times in a command line.<br><br>See also [`--resolve`](#resolve).<br> |
| <a href="#cookie" id="cookie"><code>-b, --cookie &lt;FILE&gt;</code></a> | Read cookies from FILE (using the Netscape cookie file format).<br><br>Combined with [`-c, --cookie-jar`](#cookie-jar), you can simulate a cookie storage between successive Hurl runs.<br> |
| <a href="#cookie-jar" id="cookie-jar"><code>-c, --cookie-jar &lt;FILE&gt;</code></a> | Write cookies to FILE after running the session (only for one session).<br>The file will be written using the Netscape cookie file format.<br><br>Combined with [`-b, --cookie`](#cookie), you can simulate a cookie storage between successive Hurl runs.<br> |
| <a href="#fail-at-end" id="fail-at-end"><code>--fail-at-end</code></a> | Continue executing requests to the end of the Hurl file even when an assert error occurs.<br>By default, Hurl exits after an assert error in the HTTP response.<br><br>Note that this option does not affect the behavior with multiple input Hurl files.<br><br>All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.<br> |
| <a href="#file-root" id="file-root"><code>--file-root &lt;DIR&gt;</code></a> | Set root file system to import files in Hurl. This is used for both files in multipart form data and request body.<br>When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.<br> |
| <a href="#location" id="location"><code>-L, --location</code></a> | Follow redirect. To limit the amount of redirects to follow use the [`--max-redirs`](#max-redirs) option<br> |
| <a href="#glob" id="glob"><code>--glob &lt;GLOB&gt;</code></a> | Specify input files that match the given glob pattern.<br><br>Multiple glob flags may be used. This flag supports common Unix glob patterns like *, ? and []. <br>However, to avoid your shell accidentally expanding glob patterns before Hurl handles them, you must use single quotes or double quotes around each pattern.<br> |
| <a href="#include" id="include"><code>-i, --include</code></a> | Include the HTTP headers in the output (last entry).<br> |
| <a href="#ignore-asserts" id="ignore-asserts"><code>--ignore-asserts</code></a> | Ignore all asserts defined in the Hurl file.<br> |
| <a href="#insecure" id="insecure"><code>-k, --insecure</code></a> | This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.<br> |
| <a href="#interactive" id="interactive"><code>--interactive</code></a> | Stop between requests.<br>This is similar to a break point, You can then continue (Press C) or quit (Press Q).<br> |
| <a href="#json" id="json"><code>--json</code></a> | Output each hurl file result to JSON. The format is very closed to HAR format. <br> |
| <a href="#key" id="key"><code>--key &lt;KEY&gt;</code></a> | Private key file name.<br> |
| <a href="#max-redirs" id="max-redirs"><code>--max-redirs &lt;NUM&gt;</code></a> | Set maximum number of redirection-followings allowed<br>By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.<br> |
| <a href="#max-time" id="max-time"><code>-m, --max-time &lt;SECONDS&gt;</code></a> | Maximum time in seconds that you allow a request/response to take. This is the standard timeout.<br><br>See also [`--connect-timeout`](#connect-timeout).<br> |
| <a href="#no-color" id="no-color"><code>--no-color</code></a> | Do not colorize output.<br> |
| <a href="#no-output" id="no-output"><code>--no-output</code></a> | Suppress output. By default, Hurl outputs the body of the last response.<br> |
| <a href="#noproxy" id="noproxy"><code>--noproxy &lt;HOST(S)&gt;</code></a> | Comma-separated list of hosts which do not use a proxy.<br>Override value from Environment variable no_proxy.<br> |
| <a href="#output" id="output"><code>-o, --output &lt;FILE&gt;</code></a> | Write output to FILE instead of stdout.<br> |
| <a href="#path-as-is" id="path-as-is"><code>--path-as-is</code></a> | Tell curl to not handle sequences of /../ or /./ in the given URL path. Normally curl will squash or merge them according to standards but with this option set you tell it not to do that.<br> |
| <a href="#proxy" id="proxy"><code>-x, --proxy &lt;[PROTOCOL://]HOST[:PORT]&gt;</code></a> | Use the specified proxy.<br> |
| <a href="#report-junit" id="report-junit"><code>--report-junit &lt;FILE&gt;</code></a> | Generate JUnit File.<br><br>If the FILE report already exists, it will be updated with the new test results.<br> |
| <a href="#report-html" id="report-html"><code>--report-html &lt;DIR&gt;</code></a> | Generate HTML report in DIR.<br><br>If the HTML report already exists, it will be updated with the new test results.<br> |
| <a href="#resolve" id="resolve"><code>--resolve &lt;HOST:PORT:ADDR&gt;</code></a> | Provide a custom address for a specific host and port pair. Using this, you can make the Hurl requests(s) use a specified address and prevent the otherwise normally resolved address to be used. Consider it a sort of /etc/hosts alternative provided on the command line.<br> |
| <a href="#retry" id="retry"><code>--retry</code></a> | Retry requests if any error occurs (asserts, captures, runtimes etc...).<br> |
| <a href="#retry-interval" id="retry-interval"><code>--retry-interval &lt;MILLISECONDS&gt;</code></a> | Duration in milliseconds between each retry. Default is 1000 ms.<br> |
| <a href="#retry-max-count" id="retry-max-count"><code>--retry-max-count &lt;NUM&gt;</code></a> | Maximum number of retries. Set this option to -1 to make it unlimited. Default is 10.<br> |
| <a href="#ssl-no-revoke" id="ssl-no-revoke"><code>--ssl-no-revoke</code></a> | (Windows) This option tells Hurl to disable certificate revocation checks. WARNING: this option loosens the SSL security, and by using this flag you ask for exactly that.<br> |
| <a href="#test" id="test"><code>--test</code></a> | Activate test mode: with this, the HTTP response is not outputted anymore, progress is reported for each Hurl file tested, and a text summary is displayed when all files have been run.<br> |
| <a href="#to-entry" id="to-entry"><code>--to-entry &lt;ENTRY_NUMBER&gt;</code></a> | Execute Hurl file to ENTRY_NUMBER (starting at 1).<br>Ignore the remaining of the file. It is useful for debugging a session.<br> |
| <a href="#user" id="user"><code>-u, --user &lt;USER:PASSWORD&gt;</code></a> | Add basic Authentication header to each request.<br> |
| <a href="#user-agent" id="user-agent"><code>-A, --user-agent &lt;NAME&gt;</code></a> | Specify the User-Agent string to send to the HTTP server.<br> |
| <a href="#variable" id="variable"><code>--variable &lt;NAME=VALUE&gt;</code></a> | Define variable (name/value) to be used in Hurl templates.<br> |
| <a href="#variables-file" id="variables-file"><code>--variables-file &lt;FILE&gt;</code></a> | Set properties file in which your define your variables.<br><br>Each variable is defined as name=value exactly as with [`--variable`](#variable) option.<br><br>Note that defining a variable twice produces an error.<br> |
| <a href="#verbose" id="verbose"><code>-v, --verbose</code></a> | Turn on verbose output on standard error stream.<br>Useful for debugging.<br><br>A line starting with '>' means data sent by Hurl.<br>A line staring with '<' means data received by Hurl.<br>A line starting with '*' means additional info provided by Hurl.<br><br>If you only want HTTP headers in the output, [`-i, --include`](#include) might be the option you're looking for.<br> |
| <a href="#very-verbose" id="very-verbose"><code>--very-verbose</code></a> | Turn on more verbose output on standard error stream.<br><br>In contrast to  [`--verbose`](#verbose) option, this option outputs the full HTTP body request and response on standard error. In addition, lines starting with '**' are libcurl debug logs.<br> |
| <a href="#help" id="help"><code>-h, --help</code></a> | Usage help. This lists all current command line options with a short description.<br> |
| <a href="#version" id="version"><code>-V, --version</code></a> | Prints version information<br> |

## Environment

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using the [`-x, --proxy`](#proxy) option.

| Variable | Description |
| --- | --- |
| `http_proxy [PROTOCOL://]<HOST>[:PORT]` | Sets the proxy server to use for HTTP.<br> |
| `https_proxy [PROTOCOL://]<HOST>[:PORT]` | Sets the proxy server to use for HTTPS.<br> |
| `all_proxy [PROTOCOL://]<HOST>[:PORT]` | Sets the proxy server to use if no protocol-specific proxy is set.<br> |
| `no_proxy <comma-separated list of hosts>` | List of host names that shouldn't go through any proxy.<br> |
| `HURL_name value` | Define variable (name/value) to be used in Hurl templates. This is similar than [`--variable`](#variable) and [`--variables-file`](#variables-file) options.<br> |
| `NO_COLOR` | When set to a non-empty string, do not colorize output (see [`--no-color`](#no-color) option).<br> |

## Exit Codes

| Value | Description |
| --- | --- |
| `0` | Success.<br> |
| `1` | Failed to parse command-line options.<br> |
| `2` | Input File Parsing Error.<br> |
| `3` | Runtime error (such as failure to connect to host).<br> |
| `4` | Assert Error.<br> |

## WWW

[https://hurl.dev](https://hurl.dev)


## See Also

curl(1)  hurlfmt(1)


