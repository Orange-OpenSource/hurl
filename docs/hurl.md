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



## HURL FILE FORMAT

The Hurl file format is fully documented in [https://hurl.dev/docs/hurl-file.html](https://hurl.dev/docs/hurl-file.html)

It consists of one or several HTTP requests

```hurl
GET http:/example.net/endpoint1
GET http:/example.net/endpoint2
```


### Capturing values

A value from an HTTP response can be-reused for successive HTTP requests.

A typical example occurs with csrf tokens.

```hurl
GET https://example.net
HTTP/1.1 200
# Capture the CSRF token value from html body.
[Captures]
csrf_token: xpath "normalize-space(//meta[@name='_csrf_token']/@content)"

# Do the login !
POST https://example.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
```

### Asserts

The HTTP response defined in the Hurl session are used to make asserts.

At the minimum, the response includes the asserts on the HTTP version and status code.

```hurl
GET http:/google.com
HTTP/1.1 302
```

It can also include asserts on the response headers

```hurl
GET http:/google.com
HTTP/1.1 302
Location: http://www.google.com
```

You can also include explicit asserts combining query and predicate

```hurl
GET http:/google.com
HTTP/1.1 302
[Asserts]
xpath "//title" == "301 Moved"
```

Thanks to asserts, Hurl can be used as a testing tool to run scenarii.




## OPTIONS

Options that exist in curl have exactly the same semantic.


### --color {#color}

Colorize Output



### -b, --cookie <file> {#cookie}

Read cookies from file (using the Netscape cookie file format).

Combined with [-c, --cookie-jar](#cookie-jar), you can simulate a cookie storage between successive Hurl runs.


### --compressed {#compressed}

Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.


### --connect-timeout <seconds> {#connect-timeout}

Maximum time in seconds that you allow Hurl's connection to take.

See also [-m, --max-time](#max-time) option.


### -c, --cookie-jar <file> {#cookie-jar}

Write cookies to FILE after running the session (only for one session).
The file will be written using the Netscape cookie file format.

Combined with [-b, --cookie](#cookie),you can simulate a cookie storage between successive Hurl runs.



### --fail-at-end {#fail-at-end}

Continue executing requests to the end of the Hurl file even when an assert error occurs.
By default, Hurl exits after an assert error in the HTTP response.

Note that this option does not affect the behavior with multiple input Hurl files.

All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.


### --file-root <dir> {#file-root}

Set root filesystem to import files in Hurl. This is used for both files in multipart form data and request body.
When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.




### -h, --help {#help}

Usage help. This lists all current command line options with a short description.



### --html <dir> {#html}

Generate html report in dir.

If the html report already exists, it will be updated with the new test results.


### --ignore-asserts {#ignore-asserts}

Ignore all asserts defined in the Hurl file.


### -i, --include {#include}

Include the HTTP headers in the output (last entry).


### --interactive {#interactive}

Stop between requests.
This is similar to a break point, You can then continue (Press C) or quit (Press Q).


### --json <file> {#json}

Write full session(s) to a json file. The format is very closed to HAR format.

If the json file already exists, the file will be updated with the new test results.


### -k, --insecure {#insecure}

This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.



### -L, --location {#location}

Follow redirect.  You can limit the amount of redirects to follow by using the [--max-redirs](#max-redirs) option.


### -m, --max-time <seconds> {#max-time}

Maximum time in seconds that you allow a request/response to take. This is the standard timeout.

See also [--connect-timeout](#connect-timeout) option.


### --max-redirs <num> {#max-redirs}

Set maximum number of redirection-followings allowed
By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.


### --no-color {#color}

Do not colorize Output



### --noproxy <no-proxy-list> {#noproxy}

Comma-separated list of hosts which do not use a proxy.
Override value from Environment variable no_proxy.



### --to-entry <entry-number> {#to-entry}

Execute Hurl file to ENTRY_NUMBER (starting at 1).
Ignore the remaining of the file. It is useful for debugging a session.



### -o, --output <file> {#output}

Write output to <file> instead of stdout.


### --progress {#progress}

Print filename and status for each test


### --summary {#summary}

Print test metrics at the end of the run

### --test {#test}

Activate test mode; equals --output /dev/null --progress --summary


### -x, --proxy [protocol://]host[:port] {#proxy}

Use the specified proxy.

### -u, --user <user:password> {#user}

Add basic Authentication header to each request.


### --variable <name=value> {#variable}

Define variable (name/value) to be used in Hurl templates.
Only string values can be defined.


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


### -V, --version {#version}

Prints version information



## ENVIRONMENT

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using
the [-x, --proxy](#proxy) option.

### http_proxy [protocol://]<host>[:port]

Sets the proxy server to use for HTTP.


### https_proxy [protocol://]<host>[:port]

Sets the proxy server to use for HTTPS.


### all_proxy [protocol://]<host>[:port]

Sets the proxy server to use if no protocol-specific proxy is set.

### no_proxy <comma-separated list of hosts>

list of host names that shouldn't go through any proxy.


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
