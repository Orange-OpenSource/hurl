# Man Page

## Name

hurl - run and test HTTP requests.


## Synopsis

**hurl** [options] [FILE...]


## Description

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


## Hurl File Format

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

## Options

Options that exist in curl have exactly the same semantic.

Option | Description
 --- | --- 
<a href="#cacert" id="cacert"><code>--cacert</code></a> | Tells curl to use the specified certificate file to verify the peer.<br/>The file may contain multiple CA certificates.<br/>The certificate(s) must be in PEM format.<br/>Normally curl is built to use a default file for this, so this option is typically used to alter that default file.<br/>
<a href="#color" id="color"><code>--color</code></a> | Colorize Output<br/>
<a href="#compressed" id="compressed"><code>--compressed</code></a> | Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.<br/>
<a href="#connect-timeout" id="connect-timeout"><code>--connect-timeout &lt;seconds&gt;</code></a> | Maximum time in seconds that you allow Hurl's connection to take.<br/><br/>See also [-m, --max-time](#max-time) option.<br/>
<a href="#cookie" id="cookie"><code>-b, --cookie &lt;file&gt;</code></a> | Read cookies from file (using the Netscape cookie file format).<br/><br/>Combined with [-c, --cookie-jar](#cookie-jar), you can simulate a cookie storage between successive Hurl runs.<br/>
<a href="#cookie-jar" id="cookie-jar"><code>-c, --cookie-jar &lt;file&gt;</code></a> | Write cookies to FILE after running the session (only for one session).<br/>The file will be written using the Netscape cookie file format.<br/><br/>Combined with [-b, --cookie](#cookie), you can simulate a cookie storage between successive Hurl runs.<br/>
<a href="#fail-at-end" id="fail-at-end"><code>--fail-at-end</code></a> | Continue executing requests to the end of the Hurl file even when an assert error occurs.<br/>By default, Hurl exits after an assert error in the HTTP response.<br/><br/>Note that this option does not affect the behavior with multiple input Hurl files.<br/><br/>All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.<br/>
<a href="#file-root" id="file-root"><code>--file-root &lt;dir&gt;</code></a> | Set root filesystem to import files in Hurl. This is used for both files in multipart form data and request body.<br/>When this is not explicitly defined, the files are relative to the current directory in which Hurl is running.<br/>
<a href="#location" id="location"><code>-L, --location</code></a> | Follow redirect.  You can limit the amount of redirects to follow by using the [--max-redirs](#max-redirs) option.<br/>
<a href="#glob" id="glob"><code>--glob &lt;glob&gt;</code></a> | Specify input files that match the given glob pattern.<br/><br/>Multiple glob flags may be used. This flag supports common Unix glob patterns like *, ? and []. <br/>However, to avoid your shell accidentally expanding glob patterns before Hurl handles them, you must use single quotes or double quotes around each pattern.<br/>
<a href="#include" id="include"><code>-i, --include</code></a> | Include the HTTP headers in the output (last entry).<br/>
<a href="#ignore-asserts" id="ignore-asserts"><code>--ignore-asserts</code></a> | Ignore all asserts defined in the Hurl file.<br/>
<a href="#insecure" id="insecure"><code>-k, --insecure</code></a> | This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.<br/>
<a href="#interactive" id="interactive"><code>--interactive</code></a> | Stop between requests.<br/>This is similar to a break point, You can then continue (Press C) or quit (Press Q).<br/>
<a href="#json" id="json"><code>--json</code></a> | Output each hurl file result to JSON. The format is very closed to HAR format. <br/>
<a href="#max-redirs" id="max-redirs"><code>--max-redirs &lt;num&gt;</code></a> | Set maximum number of redirection-followings allowed<br/>By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.<br/>
<a href="#max-time" id="max-time"><code>-m, --max-time &lt;seconds&gt;</code></a> | Maximum time in seconds that you allow a request/response to take. This is the standard timeout.<br/><br/>See also [--connect-timeout](#connect-timeout) option.<br/>
<a href="#no-color" id="no-color"><code>--no-color</code></a> | Do not colorize Output<br/>
<a href="#no-output" id="no-output"><code>--no-output</code></a> | Suppress output. By default, Hurl outputs the body of the last response.<br/>
<a href="#noproxy" id="noproxy"><code>--noproxy &lt;no-proxy-list&gt;</code></a> | Comma-separated list of hosts which do not use a proxy.<br/>Override value from Environment variable no_proxy.<br/>
<a href="#output" id="output"><code>-o, --output &lt;file&gt;</code></a> | Write output to <file> instead of stdout.<br/>
<a href="#progress" id="progress"><code>--progress</code></a> | Print filename and status for each test (on stderr)<br/>
<a href="#proxy" id="proxy"><code>-x, --proxy [protocol://]host[:port]</code></a> | Use the specified proxy.<br/>
<a href="#report-junit" id="report-junit"><code>--report-junit &lt;file&gt;</code></a> | Generate JUNIT <file>.<br/><br/>If the <file> report already exists, it will be updated with the new test results.<br/>
<a href="#report-html" id="report-html"><code>--report-html &lt;dir&gt;</code></a> | Generate HTML report in dir.<br/><br/>If the HTML report already exists, it will be updated with the new test results.<br/>
<a href="#summary" id="summary"><code>--summary</code></a> | Print test metrics at the end of the run (on stderr)<br/>
<a href="#test" id="test"><code>--test</code></a> | Activate test mode; equals [--no-output](#no-output) [--progress](#progress) [--summary](#summary)<br/>
<a href="#to-entry" id="to-entry"><code>--to-entry &lt;entry-number&gt;</code></a> | Execute Hurl file to ENTRY_NUMBER (starting at 1).<br/>Ignore the remaining of the file. It is useful for debugging a session.<br/>
<a href="#user" id="user"><code>-u, --user &lt;user:password&gt;</code></a> | Add basic Authentication header to each request.<br/>
<a href="#user-agent" id="user-agent"><code>-A, --user-agent &lt;name&gt;</code></a> | Specify the User-Agent string to send to the HTTP server.<br/>
<a href="#variable" id="variable"><code>--variable &lt;name=value&gt;</code></a> | Define variable (name/value) to be used in Hurl templates.<br/>
<a href="#variables-file" id="variables-file"><code>--variables-file &lt;file&gt;</code></a> | Set properties file in which your define your variables.<br/><br/>Each variable is defined as name=value exactly as with [--variable](#variable) option.<br/><br/>Note that defining a variable twice produces an error.<br/>
<a href="#verbose" id="verbose"><code>-v, --verbose</code></a> | Turn on verbose output on standard error stream<br/>Useful for debugging.<br/><br/>A line starting with '>' means data sent by Hurl.<br/>A line staring with '<' means data received by Hurl.<br/>A line starting with '*' means additional info provided by Hurl.<br/><br/>If you only want HTTP headers in the output, -i, --include might be the option you're looking for.<br/>
<a href="#help" id="help"><code>-h, --help</code></a> | Usage help. This lists all current command line options with a short description.<br/>
<a href="#version" id="version"><code>-V, --version</code></a> | Prints version information<br/>

## Environment

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using the [-x, --proxy](#proxy) option.

Variable | Description
 --- | --- 
`http_proxy [protocol://]<host>[:port]` | Sets the proxy server to use for HTTP.<br/>
`https_proxy [protocol://]<host>[:port]` | Sets the proxy server to use for HTTPS.<br/>
`all_proxy [protocol://]<host>[:port]` | Sets the proxy server to use if no protocol-specific proxy is set.<br/>
`no_proxy <comma-separated list of hosts>` | list of host names that shouldn't go through any proxy.<br/>
`HURL_name value` | Define variable (name/value) to be used in Hurl templates. This is similar than [--variable](#variable) and [--variables-file](#variables-file) options.<br/>

## Exit Codes

Value | Description
 --- | --- 
`1` | Failed to parse command-line options.<br/>
`2` | Input File Parsing Error.<br/>
`3` | Runtime error (such as failure to connect to host).<br/>
`4` | Assert Error.<br/>

## WWW

[https://hurl.dev](https://hurl.dev)


## See Also

curl(1)  hurlfmt(1)

