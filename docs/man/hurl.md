## NAME

hurl - run and test HTTP requests.


## SYNOPSIS

**hurl** [options] [FILE...]


## DESCRIPTION

**Hurl** is an HTTP client that performs HTTP requests defined in a simple plain text format.

Hurl is very versatile, it enables to chain HTTP requests, capture values from HTTP responses and make asserts.

```
$ hurl session.hurl
```

If no input-files are specified, input is read from stdin.

```
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


Output goes to stdout by default. For output to a file, use the -o option:

```
$ hurl -o output input.hurl
```

By default, Hurl executes all HTTP requests and outputs the response body of the last HTTP call.

To have a test oriented output, you can use --test option:

```
$ hurl --test *.hurl
```


## HURL FILE FORMAT

The Hurl file format is fully documented in <https://hurl.dev/docs/hurl-file.html>

It consists of one or several HTTP requests

```hurl
GET http:/example.org/endpoint1
GET http:/example.org/endpoint2
```


### Capturing values

A value from an HTTP response can be-reused for successive HTTP requests.

A typical example occurs with csrf tokens.

```hurl
GET https://example.org
HTTP/1.1 200
# Capture the CSRF token value from html body.
[Captures]
csrf_token: xpath "normalize-space(//meta[@name='_csrf_token']/@content)"

# Do the login !
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
```

More information on captures here <https://hurl.dev/docs/capturing-response.html>

### Asserts

The HTTP response defined in the Hurl session are used to make asserts.

At the minimum, the response includes the asserts on the HTTP version and status code.

```hurl
GET http:/google.com
HTTP/1.1 301
```

It can also include asserts on the response headers

```hurl
GET http:/google.com
HTTP/1.1 301
Location: http://www.google.com
```

You can also include explicit asserts combining query and predicate

```hurl
GET http:/google.com
HTTP/1.1 301
[Asserts]
xpath "string(//title)" == "301 Moved"
```

Thanks to asserts, Hurl can be used as a testing tool to run scenarii.

More information on asserts here <https://hurl.dev/docs/asserting-response.html>

## OPTIONS

Options that exist in curl have exactly the same semantic.

### --cacert {#cacert}

Tells curl to use the specified certificate file to verify the peer.
The file may contain multiple CA certificates.
The certificate(s) must be in PEM format.
Normally curl is built to use a default file for this, so this option is typically used to alter that default file.

### --color {#color}

Colorize Output

### --compressed {#compressed}

Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.

### --connect-timeout <seconds> {#connect-timeout}

Maximum time in seconds that you allow Hurl's connection to take.

See also [-m, --max-time](#max-time) option.

### -b, --cookie <file> {#cookie}

Read cookies from file (using the Netscape cookie file format).

Combined with [-c, --cookie-jar](#cookie-jar), you can simulate a cookie storage between successive Hurl runs.

### -c, --cookie-jar <file> {#cookie-jar}

Write cookies to FILE after running the session (only for one session).
The file will be written using the Netscape cookie file format.

Combined with [-b, --cookie](#cookie), you can simulate a cookie storage between successive Hurl runs.

### --fail-at-end {#fail-at-end}

Continue executing requests to the end of the Hurl file even when an assert error occurs.
By default, Hurl exits after an assert error in the HTTP response.

Note that this option does not affect the behavior with multiple input Hurl files.

All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.

### --file-root <dir> {#file-root}

Set root filesystem to import files in Hurl. This is used for both files in multipart form data and request body.
When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.

### -L, --location {#location}

Follow redirect.  You can limit the amount of redirects to follow by using the [--max-redirs](#max-redirs) option.

### --glob <glob> {#glob}

Specify input files that match the given glob pattern.

Multiple glob flags may be used. This flag supports common Unix glob patterns like *, ? and []. 
However, to avoid your shell accidentally expanding glob patterns before Hurl handles them, you must use single quotes or double quotes around each pattern.

### -i, --include {#include}

Include the HTTP headers in the output (last entry).

### --ignore-asserts {#ignore-asserts}

Ignore all asserts defined in the Hurl file.

### -k, --insecure {#insecure}

This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.

### --interactive {#interactive}

Stop between requests.
This is similar to a break point, You can then continue (Press C) or quit (Press Q).

### --json {#json}

Output each hurl file result to JSON. The format is very closed to HAR format. 

### --max-redirs <num> {#max-redirs}

Set maximum number of redirection-followings allowed
By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.

### -m, --max-time <seconds> {#max-time}

Maximum time in seconds that you allow a request/response to take. This is the standard timeout.

See also [--connect-timeout](#connect-timeout) option.

### --no-color {#no-color}

Do not colorize Output

### --no-output {#no-output}

Suppress output. By default, Hurl outputs the body of the last response.

### --noproxy <no-proxy-list> {#noproxy}

Comma-separated list of hosts which do not use a proxy.
Override value from Environment variable no_proxy.

### -o, --output <file> {#output}

Write output to <file> instead of stdout.

### --progress {#progress}

Print filename and status for each test (on stderr)

### -x, --proxy [protocol://]host[:port] {#proxy}

Use the specified proxy.

### --report-junit <file> {#report-junit}

Generate JUNIT <file>.

If the <file> report already exists, it will be updated with the new test results.

### --report-html <dir> {#report-html}

Generate HTML report in dir.

If the HTML report already exists, it will be updated with the new test results.

### --summary {#summary}

Print test metrics at the end of the run (on stderr)

### --test {#test}

Activate test mode; equals [--no-output](#no-output) [--progress](#progress) [--summary](#summary)

### --to-entry <entry-number> {#to-entry}

Execute Hurl file to ENTRY_NUMBER (starting at 1).
Ignore the remaining of the file. It is useful for debugging a session.

### -u, --user <user:password> {#user}

Add basic Authentication header to each request.

### -A, --user-agent <name> {#user-agent}

Specify the User-Agent string to send to the HTTP server.

### --variable <name=value> {#variable}

Define variable (name/value) to be used in Hurl templates.

### --variables-file <file> {#variables-file}

Set properties file in which your define your variables.

Each variable is defined as name=value exactly as with [--variable](#variable) option.

Note that defining a variable twice produces an error.

### -v, --verbose {#verbose}

Turn on verbose output on standard error stream
Useful for debugging.

A line starting with '>' means data sent by Hurl.
A line staring with '<' means data received by Hurl.
A line starting with '*' means additional info provided by Hurl.

If you only want HTTP headers in the output, -i, --include might be the option you're looking for.

### -h, --help {#help}

Usage help. This lists all current command line options with a short description.

### -V, --version {#version}

Prints version information

## ENVIRONMENT

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using the [-x, --proxy](#proxy) option.

### http_proxy [protocol://]<host>[:port]

Sets the proxy server to use for HTTP.

### https_proxy [protocol://]<host>[:port]

Sets the proxy server to use for HTTPS.

### all_proxy [protocol://]<host>[:port]

Sets the proxy server to use if no protocol-specific proxy is set.

### no_proxy <comma-separated list of hosts>

list of host names that shouldn't go through any proxy.

### HURL_name value

Define variable (name/value) to be used in Hurl templates. This is similar than [--variable](#variable) and [--variables-file](#variables-file) options.

## EXIT CODES

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
