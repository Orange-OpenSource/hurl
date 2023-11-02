## NAME

hurl - run and test HTTP requests.


## SYNOPSIS

**hurl** [options] [FILE...]


## DESCRIPTION

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


## HURL FILE FORMAT

The Hurl file format is fully documented in [https://hurl.dev/docs/hurl-file.html](https://hurl.dev/docs/hurl-file.html)

It consists of one or several HTTP requests

```hurl
GET http://example.org/endpoint1
GET http://example.org/endpoint2
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
GET http://example.org
HTTP 301
```

It can also include asserts on the response headers

```hurl
GET http://example.org
HTTP 301
Location: http://www.example.org
```

Explicit asserts can be included by combining a query and a predicate

```hurl
GET http://example.org
HTTP 301
[Asserts]
xpath "string(//title)" == "301 Moved"
```

With the addition of asserts, Hurl can be used as a testing tool to run scenarios.

More information on asserts can be found here [https://hurl.dev/docs/asserting-response.html](https://hurl.dev/docs/asserting-response.html)

## OPTIONS

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

### --aws-sigv4 <PROVIDER1[:PROVIDER2[:REGION[:SERVICE]]]> {#aws-sigv4}

Generate an `Authorization` header with an AWS SigV4 signature.

Use [`-u, --user`](#user) to specify Access Key Id (username) and Secret Key (password).

To use temporary session credentials (e.g. for an AWS IAM Role), add the `X-Amz-Security-Token` header containing the session token.

### --cacert <FILE> {#cacert}

Specifies the certificate file for peer verification. The file may contain multiple CA certificates and must be in PEM format.
Normally Hurl is built to use a default file for this, so this option is typically used to alter that default file.

### -E, --cert <CERTIFICATE[:PASSWORD]> {#cert}

Client certificate file and password.

See also [`--key`](#key).

### --color {#color}

Colorize debug output (the HTTP response output is not colorized).

### --compressed {#compressed}

Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.

### --connect-timeout <SECONDS> {#connect-timeout}

Maximum time in seconds that you allow Hurl's connection to take.

See also [`-m, --max-time`](#max-time).

### --connect-to <HOST1:PORT1:HOST2:PORT2> {#connect-to}

For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead. This option can be used several times in a command line.

See also [`--resolve`](#resolve).

### --continue-on-error {#continue-on-error}

Continue executing requests to the end of the Hurl file even when an assert error occurs.
By default, Hurl exits after an assert error in the HTTP response.

Note that this option does not affect the behavior with multiple input Hurl files.

All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.

### -b, --cookie <FILE> {#cookie}

Read cookies from FILE (using the Netscape cookie file format).

Combined with [`-c, --cookie-jar`](#cookie-jar), you can simulate a cookie storage between successive Hurl runs.

### -c, --cookie-jar <FILE> {#cookie-jar}

Write cookies to FILE after running the session (only for one session).
The file will be written using the Netscape cookie file format.

Combined with [`-b, --cookie`](#cookie), you can simulate a cookie storage between successive Hurl runs.

### --delay <MILLISECONDS> {#delay}

Sets delay before each request.

### --error-format <FORMAT> {#error-format}

Control the format of error message (short by default or long)

### --file-root <DIR> {#file-root}

Set root file system to import files in Hurl. This is used for both files in multipart form data and request body.
When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.

### --glob <GLOB> {#glob}

Specify input files that match the given glob pattern.

Multiple glob flags may be used. This flag supports common Unix glob patterns like *, ? and [].
However, to avoid your shell accidentally expanding glob patterns before Hurl handles them, you must use single quotes or double quotes around each pattern.

### -0, --http1.0 {#http10}

Tells Hurl to use HTTP version 1.0 instead of using its internally preferred HTTP version.

### --http1.1 {#http11}

Tells Hurl to use HTTP version 1.1.

### --http2 {#http2}

Tells Hurl to use HTTP version 2.
For HTTPS, this means Hurl negotiates HTTP/2 in the TLS handshake. Hurl does this by default.
For HTTP, this means Hurl attempts to upgrade the request to HTTP/2 using the Upgrade: request header.

### --http3 {#http3}

Tells Hurl to try HTTP/3 to the host in the URL, but fallback to earlier HTTP versions if the HTTP/3 connection establishment fails. HTTP/3 is only available for HTTPS and not for HTTP URLs.

### --ignore-asserts {#ignore-asserts}

Ignore all asserts defined in the Hurl file.

### -i, --include {#include}

Include the HTTP headers in the output

### -k, --insecure {#insecure}

This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.

### --interactive {#interactive}

Stop between requests.

This is similar to a break point, You can then continue (Press C) or quit (Press Q).

### -4, --ipv4 {#ipv4}

This option tells Hurl to use IPv4 addresses only when resolving host names, and not for example try IPv6.

### -6, --ipv6 {#ipv6}

This option tells Hurl to use IPv6 addresses only when resolving host names, and not for example try IPv4.

### --json {#json}

Output each hurl file result to JSON. The format is very closed to HAR format.

### --key <KEY> {#key}

Private key file name.

### -L, --location {#location}

Follow redirect. To limit the amount of redirects to follow use the [`--max-redirs`](#max-redirs) option

### --max-redirs <NUM> {#max-redirs}

Set maximum number of redirection-followings allowed

By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.

### -m, --max-time <SECONDS> {#max-time}

Maximum time in seconds that you allow a request/response to take. This is the standard timeout.

See also [`--connect-timeout`](#connect-timeout).

### --no-color {#no-color}

Do not colorize output.

### --no-output {#no-output}

Suppress output. By default, Hurl outputs the body of the last response.

### --noproxy <HOST(S)> {#noproxy}

Comma-separated list of hosts which do not use a proxy.

Override value from Environment variable no_proxy.

### -o, --output <FILE> {#output}

Write output to FILE instead of stdout.

### --path-as-is {#path-as-is}

Tell Hurl to not handle sequences of /../ or /./ in the given URL path. Normally Hurl will squash or merge them according to standards but with this option set you tell it not to do that.

### -x, --proxy <[PROTOCOL://]HOST[:PORT]> {#proxy}

Use the specified proxy.

### --report-html <DIR> {#report-html}

Generate HTML report in DIR.

If the HTML report already exists, it will be updated with the new test results.

### --report-junit <FILE> {#report-junit}

Generate JUnit File.

If the FILE report already exists, it will be updated with the new test results.

### --report-tap <FILE> {#report-tap}

Generate TAP report.

If the FILE report already exists, it will be updated with the new test results.

### --resolve <HOST:PORT:ADDR> {#resolve}

Provide a custom address for a specific host and port pair. Using this, you can make the Hurl requests(s) use a specified address and prevent the otherwise normally resolved address to be used. Consider it a sort of /etc/hosts alternative provided on the command line.

### --retry <NUM> {#retry}

Maximum number of retries, 0 for no retries, -1 for unlimited retries. Retry happens if any error occurs (asserts, captures, runtimes etc...).

### --retry-interval <MILLISECONDS> {#retry-interval}

Duration in milliseconds between each retry. Default is 1000 ms.

### --ssl-no-revoke {#ssl-no-revoke}

(Windows) This option tells Hurl to disable certificate revocation checks. WARNING: this option loosens the SSL security, and by using this flag you ask for exactly that.

### --test {#test}

Activate test mode: with this, the HTTP response is not outputted anymore, progress is reported for each Hurl file tested, and a text summary is displayed when all files have been run.

### --to-entry <ENTRY_NUMBER> {#to-entry}

Execute Hurl file to ENTRY_NUMBER (starting at 1).
Ignore the remaining of the file. It is useful for debugging a session.

### -u, --user <USER:PASSWORD> {#user}

Add basic Authentication header to each request.

### -A, --user-agent <NAME> {#user-agent}

Specify the User-Agent string to send to the HTTP server.

### --variable <NAME=VALUE> {#variable}

Define variable (name/value) to be used in Hurl templates.

### --variables-file <FILE> {#variables-file}

Set properties file in which your define your variables.

Each variable is defined as name=value exactly as with [`--variable`](#variable) option.

Note that defining a variable twice produces an error.

### --verbose {#verbose}

Turn on verbose output on standard error stream.
Useful for debugging.

A line starting with '>' means data sent by Hurl.
A line staring with '<' means data received by Hurl.
A line starting with '*' means additional info provided by Hurl.

If you only want HTTP headers in the output, [`-i, --include`](#include) might be the option you're looking for.

### --very-verbose {#very-verbose}

Turn on more verbose output on standard error stream.

In contrast to  [`--verbose`](#verbose) option, this option outputs the full HTTP body request and response on standard error. In addition, lines starting with '**' are libcurl debug logs.

### -h, --help {#help}

Usage help. This lists all current command line options with a short description.

### -V, --version {#version}

Prints version information

## ENVIRONMENT

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using the [`-x, --proxy`](#proxy) option.

### http_proxy [PROTOCOL://]<HOST>[:PORT]

Sets the proxy server to use for HTTP.

### https_proxy [PROTOCOL://]<HOST>[:PORT]

Sets the proxy server to use for HTTPS.

### all_proxy [PROTOCOL://]<HOST>[:PORT]

Sets the proxy server to use if no protocol-specific proxy is set.

### no_proxy <comma-separated list of hosts>

List of host names that shouldn't go through any proxy.

### HURL_name value

Define variable (name/value) to be used in Hurl templates. This is similar than [`--variable`](#variable) and [`--variables-file`](#variables-file) options.

### NO_COLOR

When set to a non-empty string, do not colorize output (see [`--no-color`](#no-color) option).

## EXIT CODES

### 0

Success.

### 1

Failed to parse command-line options.

### 2

Input File Parsing Error.

### 3

Runtime error (such as failure to connect to host).

### 4

Assert Error.

## WWW

[https://hurl.dev](https://hurl.dev)


## SEE ALSO

curl(1)  hurlfmt(1)
