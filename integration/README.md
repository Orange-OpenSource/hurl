# Hurl Integration Tests Suite

## Introduction

In the Hurl project, there are three type of tests:

- Rust unit tests: run at the root of the project with `cargo test --lib`
- Rust integration tests: run at the root of the project with `cargo test`. You will also run unit test with this command. 
To run Rust integration tests, you need to launch a local test server (see below).

These tests are "classic" Rust tests and should not surprise a Rust developer.

Along with tests, we have an extensive integration test suite. These tests launch scripts to run `hurl` and
`hurlfmt` and test various options and Hurl files. To run these tests you have to set up a local server (see below)
All these tests will be performed automatically in Hurl CI/CD, on various OS, for every pull request. 


## Set up Test Local Server

### Python 3.9+

The local test server is a Flask application, so you will need Python 3.9+ on your machine. You can create a Python
virtual environment and install required dependencies:

```shell
$ python3 -m venv .venv
$ source .venv/bin/activate
$ pip install --requirements bin/requirements-fozen.txt
```

### Proxy

Some integration tests need a proxy. You can use [mitmproxy] or [squid].

### Start local server

You can use the scripts [`bin/test/test_prerequisites.sh`] / [`bin/test/test_prerequisites.ps1`] depending on your OS to start the 
local test server and proxy. Once launch, there is:

- a Flask server instance listening on <http://localhost:8000>
- a Flask server instance listening on <https://localhost:8001>
- a Flask server instance listening on <https://localhost:8002>
- a HTTP proxy listening on <http://localhost:8888>

Now, everything is ready to run the integration tests!

## Integration Tests

### Organisation

Integration tests are under `integration` directory :

- [`tests_ok`]: every test there must be successful (exit code 0). The Hurl files in this folder are formatted with `hurlfmt`.
- [`tests_ok_not_linted`]: every test here must be successful but are not necessary formatted with `hurlfmt`. This way we can 
ensure that there is no regression even if a Hurl file doesn't follow a stricter format.
- [`tests_failed`]: every test must fail (exit code different from 0). Tests are syntactically correct, so the error
raised by the test is a runtime error.
- [`tests_error_parser`]: every test is not a syntactically correct Hurl file. We test here the parsing error message.
- [`tests_error_lint`]: every test is syntactically correct, but is not formatted through `hurlfmt`. We test here the linting
error message.

### Files Description

An integration test consists of:

- two runnable scripts: one for Linux, macOS (`foo.sh`) and one for Windows (`foo.ps1`). These are the 
integration tests that we want to execute.
- a Hurl file (`foo.hurl`)
- a Flask endpoint (`foo.py`). This is the server side used by the Hurl file. You can add as many assert as you want 
to test that our Hurl client conforms to what is expected. Generally, each integration test has its own Flask endpoint, 
even if there is some duplication between tests.
- an expected stdout file (`foo.out`). This file is the expected value for stdout. This file is not dependent from the OS, as we
want a Hurl file to have the same stdout on any OS. If the stdout have some variant data (like timestamp for instance), one 
can use a patterned expected file, with `~~~` for wildcard matching (`foo.out.pattern`)
- an expected stderr file (`foo.err`). This file is the expected value for stderr. This file is not dependent from the OS, as we
  want a Hurl file to have the same stderr on any OS. Like stdout, stderr expected file can be patterned (`foo.err.pattern`)
- an expected exit code (`foo.exit`). This file is the expected value of the script. If absent, the default exit code is 0.
- an expected HTML export of the Hurl source file (`foo.html`)
- an expected JSON export of the Hurl source file (`foo.json`). Note: this is not the stdout output of Hurl using [`--json`]. This
is a JSON view of the Hurl source file and can serve to convert from/to Hurl format.
- an expected list of `curl` commands. This list is the curl command equivalent to each request in the Hurl file
(which is logged in [`--verbose`]/[`--very-verbose`] mode). Each curl command is run against the server.

To run all integration tests:

```shell
$ cd integration
$ python3 integration.py
```

To run a particular integration test without any check:

```shell
$ cd integration
$ tests_ok/hello.sh
```

To run a particular integration test with all check (stdout, stderr, HTML/JSON export etc...):

```shell
$ cd integration
$ python3 test_script.py tests_ok/hello.sh
```

### Sample

`include.sh`:

```shell
#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/include.hurl --include --verbose
```

`include.ps1`:

```powershell
Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/include.hurl --include --verbose
```

`include.hurl`:

```hurl
GET http://localhost:8000/include

HTTP 200
`Hello`
```

`include.py`:

```python
from app import app
from flask import Response


@app.route("/include")
def include():
    return Response("Hello")
```

`include.out.pattern`:

```
HTTP/1.1 200
Server: Werkzeug/~~~ Python/~~~
Date: ~~~
Content-Type: text/html; charset=utf-8
Content-Length: 5
Server: Flask Server
Connection: close

Hello
```

`include.html`: 

```html
<pre><code class="language-hurl"><span class="hurl-entry"><span class="request"><span class="line"><span class="method">GET</span> <span class="url">http://localhost:8000/include</span></span>
</span><span class="response"><span class="line"></span>
<span class="line"><span class="version">HTTP</span> <span class="number">200</span></span>
<span class="line"><span class="string">`Hello`</span></span>
</span></span><span class="line"></span>
</code></pre>
```

`include.json`:

```json
{"entries":[{"request":{"method":"GET","url":"http://localhost:8000/include"},"response":{"status":200,"body":{"type":"text","value":"Hello"}}}]}
```

`include.curl`:

```
curl 'http://localhost:8000/include'
```

[mitmproxy]: https://mitmproxy.org
[squid]: http://www.squid-cache.org
[`--json`]: /docs/manual.md#json
[`bin/test/test_prerequisites.sh`]: /bin/test/test_prerequisites.sh
[`bin/test/test_prerequisites.ps1`]: /bin/test/test_prerequisites.ps1
[`tests_ok`]: /integration/tests_ok
[`tests_ok_not_linted`]: /integration/tests_ok_not_linted
[`tests_failed`]: /integration/tests_failed
[`tests_error_parser`]: /integration/tests_error_parser
[`tests_error_lint`]: /integration/tests_error_lint
[`--verbose`]: /docs/manual.md#verbose
[`--very-verbose`]: /docs/manual.md#very-verbose