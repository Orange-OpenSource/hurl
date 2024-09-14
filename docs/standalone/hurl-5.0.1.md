# Hurl Documentation

## Version 5.0.1 - 18/09/2024

# Table of Contents

* [Introduction](#introduction)
    * [What's Hurl?](#introduction-home-whats-hurl)
    * [Also an HTTP Test Tool](#introduction-home-also-an-http-test-tool)
    * [Why Hurl?](#introduction-home-why-hurl)
    * [Powered by curl](#introduction-home-powered-by-curl)
    * [Feedbacks](#introduction-home-feedbacks)
    * [Resources](#introduction-home-resources)
* [Getting Started](#getting-started)
    * [Installation](#getting-started-installation-installation)
        * [Binaries Installation](#getting-started-installation-binaries-installation)
            * [Linux](#getting-started-installation-linux)
                * [Debian / Ubuntu](#getting-started-installation-debian--ubuntu)
                * [Alpine](#getting-started-installation-alpine)
                * [Arch Linux / Manjaro](#getting-started-installation-arch-linux--manjaro)
                * [NixOS / Nix](#getting-started-installation-nixos--nix)
            * [macOS](#getting-started-installation-macos)
                * [Homebrew](#getting-started-installation-homebrew)
                * [MacPorts](#getting-started-installation-macports)
            * [FreeBSD](#getting-started-installation-freebsd)
            * [Windows](#getting-started-installation-windows)
                * [Zip File](#getting-started-installation-zip-file)
                * [Installer](#getting-started-installation-installer)
                * [Chocolatey](#getting-started-installation-chocolatey)
                * [Scoop](#getting-started-installation-scoop)
                * [Windows Package Manager](#getting-started-installation-windows-package-manager)
            * [Cargo](#getting-started-installation-cargo)
            * [conda-forge](#getting-started-installation-conda-forge)
            * [Docker](#getting-started-installation-docker)
            * [npm](#getting-started-installation-npm)
        * [Building From Sources](#getting-started-installation-building-from-sources)
            * [Build on Linux](#getting-started-installation-build-on-linux)
                * [Debian based distributions](#getting-started-installation-debian-based-distributions)
                * [Fedora based distributions](#getting-started-installation-fedora-based-distributions)
                * [Red Hat based distributions](#getting-started-installation-red-hat-based-distributions)
                * [Arch based distributions](#getting-started-installation-arch-based-distributions)
                * [Alpine based distributions](#getting-started-installation-alpine-based-distributions)
            * [Build on macOS](#getting-started-installation-build-on-macos)
            * [Build on Windows](#getting-started-installation-build-on-windows)
    * [Manual](#getting-started-manual-manual)
        * [Name](#getting-started-manual-name)
        * [Synopsis](#getting-started-manual-synopsis)
        * [Description](#getting-started-manual-description)
        * [Hurl File Format](#getting-started-manual-hurl-file-format)
            * [Capturing values](#getting-started-manual-capturing-values)
            * [Asserts](#getting-started-manual-asserts)
        * [Options](#getting-started-manual-options)
        * [Environment](#getting-started-manual-environment)
        * [Exit Codes](#getting-started-manual-exit-codes)
        * [WWW](#getting-started-manual-www)
        * [See Also](#getting-started-manual-see-also)
    * [Samples](#getting-started-samples-samples)
        * [Getting Data](#getting-started-samples-getting-data)
            * [HTTP Headers](#getting-started-samples-http-headers)
            * [Query Params](#getting-started-samples-query-params)
            * [Basic Authentication](#getting-started-samples-basic-authentication)
            * [Passing Data between Requests ](#getting-started-samples-passing-data-between-requests)
        * [Sending Data](#getting-started-samples-sending-data)
            * [Sending HTML Form Data](#getting-started-samples-sending-html-form-data)
            * [Sending Multipart Form Data](#getting-started-samples-sending-multipart-form-data)
            * [Posting a JSON Body](#getting-started-samples-posting-a-json-body)
            * [Templating a JSON Body](#getting-started-samples-templating-a-json-body)
            * [Templating a XML Body](#getting-started-samples-templating-a-xml-body)
            * [Using GraphQL Query](#getting-started-samples-using-graphql-query)
        * [Testing Response](#getting-started-samples-testing-response)
            * [Testing Status Code](#getting-started-samples-testing-status-code)
            * [Testing Response Headers](#getting-started-samples-testing-response-headers)
            * [Testing REST APIs](#getting-started-samples-testing-rest-apis)
            * [Testing HTML Response](#getting-started-samples-testing-html-response)
            * [Testing Set-Cookie Attributes](#getting-started-samples-testing-set-cookie-attributes)
            * [Testing Bytes Content](#getting-started-samples-testing-bytes-content)
            * [SSL Certificate](#getting-started-samples-ssl-certificate)
            * [Checking Full Body](#getting-started-samples-checking-full-body)
        * [Reports](#getting-started-samples-reports)
            * [HTML Report](#getting-started-samples-html-report)
            * [JSON Report](#getting-started-samples-json-report)
            * [JUnit Report](#getting-started-samples-junit-report)
            * [TAP Report](#getting-started-samples-tap-report)
            * [JSON Output](#getting-started-samples-json-output)
        * [Others](#getting-started-samples-others)
            * [HTTP Version](#getting-started-samples-http-version)
            * [Polling and Retry](#getting-started-samples-polling-and-retry)
            * [Delaying Requests](#getting-started-samples-delaying-requests)
            * [Skipping Requests](#getting-started-samples-skipping-requests)
            * [Testing Endpoint Performance](#getting-started-samples-testing-endpoint-performance)
            * [Using SOAP APIs](#getting-started-samples-using-soap-apis)
            * [Capturing and Using a CSRF Token](#getting-started-samples-capturing-and-using-a-csrf-token)
            * [Checking Byte Order Mark (BOM) in Response Body](#getting-started-samples-checking-byte-order-mark-bom-in-response-body)
            * [AWS Signature Version 4 Requests](#getting-started-samples-aws-signature-version-4-requests)
            * [Using curl Options](#getting-started-samples-using-curl-options)
    * [Running Tests](#getting-started-running-tests-running-tests)
        * [Use --test Option](#getting-started-running-tests-use-test-option)
            * [Selecting Tests](#getting-started-running-tests-selecting-tests)
        * [Debugging](#getting-started-running-tests-debugging)
            * [Debug Logs](#getting-started-running-tests-debug-logs)
            * [HTTP Responses](#getting-started-running-tests-http-responses)
        * [Generating Report](#getting-started-running-tests-generating-report)
            * [HTML Report](#getting-started-running-tests-html-report)
            * [JSON Report](#getting-started-running-tests-json-report)
            * [JUnit Report](#getting-started-running-tests-junit-report)
            * [TAP Report](#getting-started-running-tests-tap-report)
        * [Use Variables in Tests](#getting-started-running-tests-use-variables-in-tests)
    * [Frequently Asked Questions](#getting-started-frequently-asked-questions-frequently-asked-questions)
        * [General](#getting-started-frequently-asked-questions-general)
            * [Why "Hurl"?](#getting-started-frequently-asked-questions-why-hurl)
            * [Yet Another Tool, I already use X](#getting-started-frequently-asked-questions-yet-another-tool-i-already-use-x)
            * [Hurl is build on top of libcurl, but what is added?](#getting-started-frequently-asked-questions-hurl-is-build-on-top-of-libcurl-but-what-is-added)
            * [Why shouldn't I use Hurl?](#getting-started-frequently-asked-questions-why-shouldnt-i-use-hurl)
            * [I have a large numbers of tests, how to run just specific tests?](#getting-started-frequently-asked-questions-i-have-a-large-numbers-of-tests-how-to-run-just-specific-tests)
            * [How can I use my Hurl files outside Hurl?](#getting-started-frequently-asked-questions-how-can-i-use-my-hurl-files-outside-hurl)
            * [Can I do calculation within a Hurl file?](#getting-started-frequently-asked-questions-can-i-do-calculation-within-a-hurl-file)
        * [macOS](#getting-started-frequently-asked-questions-macos)
            * [How can I use a custom libcurl (from Homebrew by instance)?](#getting-started-frequently-asked-questions-how-can-i-use-a-custom-libcurl-from-homebrew-by-instance)
* [File Format](#file-format)
    * [Hurl File](#file-format-hurl-file-hurl-file)
        * [Character Encoding](#file-format-hurl-file-character-encoding)
        * [File Extension](#file-format-hurl-file-file-extension)
        * [Comments](#file-format-hurl-file-comments)
        * [Special Characters in Strings](#file-format-hurl-file-special-characters-in-strings)
    * [Entry](#file-format-entry-entry)
        * [Definition](#file-format-entry-definition)
        * [Example](#file-format-entry-example)
        * [Description](#file-format-entry-description)
            * [Options](#file-format-entry-options)
            * [Cookie storage](#file-format-entry-cookie-storage)
            * [Redirects](#file-format-entry-redirects)
            * [Retry](#file-format-entry-retry)
            * [Control flow](#file-format-entry-control-flow)
    * [Request](#file-format-request-request)
        * [Definition](#file-format-request-definition)
        * [Example](#file-format-request-example)
        * [Structure](#file-format-request-structure)
        * [Description](#file-format-request-description)
            * [Method](#file-format-request-method)
            * [URL](#file-format-request-url)
            * [Headers](#file-format-request-headers)
            * [Query parameters](#file-format-request-query-parameters)
            * [Form parameters](#file-format-request-form-parameters)
            * [Multipart Form Data](#file-format-request-multipart-form-data)
            * [Cookies](#file-format-request-cookies)
            * [Basic Authentication](#file-format-request-basic-authentication)
            * [Body](#file-format-request-body)
                * [JSON body](#file-format-request-json-body)
                * [XML body](#file-format-request-xml-body)
                * [GraphQL query](#file-format-request-graphql-query)
                * [Multiline string body](#file-format-request-multiline-string-body)
                * [Oneline string body](#file-format-request-oneline-string-body)
                * [Base64 body](#file-format-request-base64-body)
                * [Hex body](#file-format-request-hex-body)
                * [File body](#file-format-request-file-body)
            * [Options](#file-format-request-options)
    * [Response](#file-format-response-response)
        * [Definition](#file-format-response-definition)
        * [Example](#file-format-response-example)
        * [Structure](#file-format-response-structure)
        * [Capture and Assertion](#file-format-response-capture-and-assertion)
            * [Body compression](#file-format-response-body-compression)
        * [Timings](#file-format-response-timings)
    * [Capturing Response](#file-format-capturing-response-capturing-response)
        * [Captures](#file-format-capturing-response-captures)
            * [Query](#file-format-capturing-response-query)
            * [Status capture](#file-format-capturing-response-status-capture)
            * [Header capture](#file-format-capturing-response-header-capture)
            * [URL capture](#file-format-capturing-response-url-capture)
            * [Cookie capture](#file-format-capturing-response-cookie-capture)
            * [Body capture](#file-format-capturing-response-body-capture)
            * [Bytes capture](#file-format-capturing-response-bytes-capture)
            * [XPath capture](#file-format-capturing-response-xpath-capture)
            * [JSONPath capture](#file-format-capturing-response-jsonpath-capture)
            * [Regex capture](#file-format-capturing-response-regex-capture)
            * [Variable capture](#file-format-capturing-response-variable-capture)
            * [Duration capture](#file-format-capturing-response-duration-capture)
            * [SSL certificate capture](#file-format-capturing-response-ssl-certificate-capture)
    * [Asserting Response](#file-format-asserting-response-asserting-response)
        * [Asserts](#file-format-asserting-response-asserts)
        * [Implicit asserts](#file-format-asserting-response-implicit-asserts)
            * [Version - Status](#file-format-asserting-response-version-status)
            * [Headers](#file-format-asserting-response-headers)
        * [Explicit asserts](#file-format-asserting-response-explicit-asserts)
            * [Predicates](#file-format-asserting-response-predicates)
            * [Status assert](#file-format-asserting-response-status-assert)
            * [Header assert](#file-format-asserting-response-header-assert)
            * [URL assert](#file-format-asserting-response-url-assert)
            * [Cookie assert](#file-format-asserting-response-cookie-assert)
            * [Body assert](#file-format-asserting-response-body-assert)
            * [Bytes assert](#file-format-asserting-response-bytes-assert)
            * [XPath assert](#file-format-asserting-response-xpath-assert)
            * [JSONPath assert](#file-format-asserting-response-jsonpath-assert)
            * [Regex assert](#file-format-asserting-response-regex-assert)
            * [SHA-256 assert](#file-format-asserting-response-sha-256-assert)
            * [MD5 assert](#file-format-asserting-response-md5-assert)
            * [Variable assert](#file-format-asserting-response-variable-assert)
            * [Duration assert](#file-format-asserting-response-duration-assert)
            * [SSL certificate assert](#file-format-asserting-response-ssl-certificate-assert)
        * [Body](#file-format-asserting-response-body)
            * [JSON body](#file-format-asserting-response-json-body)
            * [XML body](#file-format-asserting-response-xml-body)
            * [Multiline string body](#file-format-asserting-response-multiline-string-body)
                * [Oneline string body](#file-format-asserting-response-oneline-string-body)
            * [Base64 body](#file-format-asserting-response-base64-body)
            * [File body](#file-format-asserting-response-file-body)
    * [Filters](#file-format-filters-filters)
        * [Definition](#file-format-filters-definition)
        * [Example](#file-format-filters-example)
        * [Description](#file-format-filters-description)
            * [count](#file-format-filters-count)
            * [daysAfterNow](#file-format-filters-daysafternow)
            * [daysBeforeNow](#file-format-filters-daysbeforenow)
            * [decode](#file-format-filters-decode)
            * [format](#file-format-filters-format)
            * [htmlEscape](#file-format-filters-htmlescape)
            * [htmlUnescape](#file-format-filters-htmlunescape)
            * [jsonpath ](#file-format-filters-jsonpath)
            * [nth](#file-format-filters-nth)
            * [regex](#file-format-filters-regex)
            * [replace](#file-format-filters-replace)
            * [split](#file-format-filters-split)
            * [toDate](#file-format-filters-todate)
            * [toFloat](#file-format-filters-tofloat)
            * [toInt](#file-format-filters-toint)
            * [urlDecode](#file-format-filters-urldecode)
            * [urlEncode](#file-format-filters-urlencode)
            * [xpath](#file-format-filters-xpath)
    * [Templates](#file-format-templates-templates)
        * [Variables](#file-format-templates-variables)
        * [Types](#file-format-templates-types)
        * [Injecting Variables](#file-format-templates-injecting-variables)
            * [`variable` option](#file-format-templates-variable-option)
            * [`variables-file` option](#file-format-templates-variables-file-option)
            * [Environment variable](#file-format-templates-environment-variable)
            * [Options sections](#file-format-templates-options-sections)
        * [Templating Body](#file-format-templates-templating-body)
    * [Grammar](#file-format-grammar-grammar)
        * [Definitions](#file-format-grammar-definitions)
        * [Syntax Grammar](#file-format-grammar-syntax-grammar)
* [Resources](#resources)
    * [License](#resources-license-license)

# Introduction {#introduction}

<div class="home-logo">
    <img class="u-theme-light" src="https://hurl.dev/assets/img/logo-light.svg" width="277px" height="72px" alt="Hurl logo"/>
    <img class="u-theme-dark" src="https://hurl.dev/assets/img/logo-dark.svg" width="277px" height="72px" alt="Hurl logo"/>
</div>

## What's Hurl? {#introduction-home-whats-hurl}

Hurl is a command line tool that runs <b>HTTP requests</b> defined in a simple <b>plain text format</b>.

It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very
versatile: it can be used for both <b>fetching data</b> and <b>testing HTTP</b> sessions.

Hurl makes it easy to work with <b>HTML</b> content, <b>REST / SOAP / GraphQL</b> APIs, or any other <b>XML / JSON</b> based APIs. 

```hurl
# Get home:
GET https://example.org
HTTP 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"


# Do login!
POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

Chaining multiple requests is easy:

```hurl
GET https://example.org/api/health
GET https://example.org/api/step1
GET https://example.org/api/step2
GET https://example.org/api/step3
```

## Also an HTTP Test Tool {#introduction-home-also-an-http-test-tool}

Hurl can run HTTP requests but can also be used to <b>test HTTP responses</b>.
Different types of queries and predicates are supported, from [XPath](https://en.wikipedia.org/wiki/XPath) and [JSONPath](https://goessner.net/articles/JsonPath/) on body response,
to assert on status code and response headers.



It is well adapted for <b>REST / JSON APIs</b>

```hurl
POST https://example.org/api/tests
{
    "id": "4568",
    "evaluate": true
}
HTTP 200
[Asserts]
header "X-Frame-Options" == "SAMEORIGIN"
jsonpath "$.status" == "RUNNING"    # Check the status code
jsonpath "$.tests" count == 25      # Check the number of items
jsonpath "$.id" matches /\d{4}/     # Check the format of the id
```

<b>HTML content</b>

```hurl
GET https://example.org
HTTP 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"
```

<b>GraphQL</b> 

~~~hurl
POST https://example.org/graphql
```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```
HTTP 200
~~~

and even <b>SOAP APIs</b>

```hurl
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="https://example.org">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
HTTP 200
```

Hurl can also be used to test the <b>performance</b> of HTTP endpoints

```hurl
GET https://example.org/api/v1/pets
HTTP 200
[Asserts]
duration < 1000  # Duration in ms
```

And check response bytes

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

Finally, Hurl is easy to <b>integrate in CI/CD</b>, with text, JUnit, TAP and HTML reports

<div class="picture">
    <picture>
        <source srcset="https://hurl.dev/assets/img/home-waterfall-light.avif" type="image/avif">
        <source srcset="https://hurl.dev/assets/img/home-waterfall-light.webp" type="image/webp">
        <source srcset="https://hurl.dev/assets/img/home-waterfall-light.png" type="image/png">
        <img class="u-theme-light u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/home-waterfall-light.png" width="480" alt="HTML report"/>
    </picture>
    <picture>
        <source srcset="https://hurl.dev/assets/img/home-waterfall-dark.avif" type="image/avif">
        <source srcset="https://hurl.dev/assets/img/home-waterfall-dark.webp" type="image/webp">
        <source srcset="https://hurl.dev/assets/img/home-waterfall-dark.png" type="image/png">
        <img class="u-theme-dark u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/home-waterfall-dark.png" width="480" alt="HTML report"/>
    </picture>
</div>

## Why Hurl? {#introduction-home-why-hurl}

<ul class="showcase-container">
    <li class="showcase-item"><h2 class="showcase-item-title">Text Format</h2>For both devops and developers</li>
    <li class="showcase-item"><h2 class="showcase-item-title">Fast CLI</h2>A command line for local dev and continuous integration</li>
    <li class="showcase-item"><h2 class="showcase-item-title">Single Binary</h2>Easy to install, with no runtime required</li>
</ul>

## Powered by curl {#introduction-home-powered-by-curl}

Hurl is a lightweight binary written in [Rust](https://www.rust-lang.org). Under the hood, Hurl HTTP engine is
powered by [libcurl](https://curl.se/libcurl/), one of the most powerful and reliable file transfer libraries.
With its text file format, Hurl adds syntactic sugar to run and test HTTP requests,
but it's still the [curl](https://curl.se) that we love: __fast__, __efficient__ and __HTTP/3 ready__.

## Feedbacks {#introduction-home-feedbacks}

To support its development, [star Hurl on GitHub](https://github.com/Orange-OpenSource/hurl/stargazers)!

[Feedback, suggestion, bugs or improvements](https://github.com/Orange-OpenSource/hurl/issues) are welcome.

```hurl
POST https://hurl.dev/api/feedback
{
  "name": "John Doe",
  "feedback": "Hurl is awesome!"
}
HTTP 200
```

## Resources {#introduction-home-resources}

[License](#resources-license)

[Blog](https://hurl.dev/blog)

[Tutorial](https://hurl.dev/docs/tutorial/your-first-hurl-file.html)

[Documentation](https://hurl.dev) 

[GitHub](https://github.com/Orange-OpenSource/hurl)



<hr>

# Getting Started {#getting-started}

## Installation {#getting-started-installation-installation}

### Binaries Installation {#getting-started-installation-binaries-installation}

#### Linux {#getting-started-installation-linux}

Precompiled binary is available at [Hurl latest GitHub release](https://github.com/Orange-OpenSource/hurl/releases/latest):

```shell
$ INSTALL_DIR=/tmp
$ VERSION=5.0.1
$ curl --silent --location https://github.com/Orange-OpenSource/hurl/releases/download/$VERSION/hurl-$VERSION-x86_64-unknown-linux-gnu.tar.gz | tar xvz -C $INSTALL_DIR
$ export PATH=$INSTALL_DIR/hurl-$VERSION-x86_64-unknown-linux-gnu/bin:$PATH
```

##### Debian / Ubuntu {#getting-started-installation-debian--ubuntu}

For Debian / Ubuntu, Hurl can be installed using a binary .deb file provided in each Hurl release.

```shell
$ VERSION=5.0.1
$ curl --location --remote-name https://github.com/Orange-OpenSource/hurl/releases/download/$VERSION/hurl_${VERSION}_amd64.deb
$ sudo apt update && sudo apt install ./hurl_${VERSION}_amd64.deb
```

##### Alpine {#getting-started-installation-alpine}

Hurl is available on `testing` channel.

```shell
$ apk add --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing hurl
```

##### Arch Linux / Manjaro {#getting-started-installation-arch-linux--manjaro}

Hurl is available on [extra](https://archlinux.org/packages/extra/x86_64/hurl/) channel.

```shell
$ pacman -Sy hurl
```

##### NixOS / Nix {#getting-started-installation-nixos--nix}

[NixOS / Nix package](https://search.nixos.org/packages?from=0&size=1&sort=relevance&type=packages&query=hurl) is available on stable channel.

#### macOS {#getting-started-installation-macos}

Precompiled binaries for Intel and ARM CPUs are available at [Hurl latest GitHub release](https://github.com/Orange-OpenSource/hurl/releases/latest).

##### Homebrew {#getting-started-installation-homebrew}

```shell
$ brew install hurl
```

##### MacPorts {#getting-started-installation-macports}

```shell
$ sudo port install hurl
```

#### FreeBSD {#getting-started-installation-freebsd}

```shell
$ sudo pkg install hurl
```

#### Windows {#getting-started-installation-windows}

Windows requires the [Visual C++ Redistributable Package](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-microsoft-visual-c-redistributable-version) to be installed manually, as this is not included in the installer.

##### Zip File {#getting-started-installation-zip-file}

Hurl can be installed from a standalone zip file at [Hurl latest GitHub release](https://github.com/Orange-OpenSource/hurl/releases/latest). You will need to update your `PATH` variable.

##### Installer {#getting-started-installation-installer}

An executable installer is also available at [Hurl latest GitHub release](https://github.com/Orange-OpenSource/hurl/releases/latest).

##### Chocolatey {#getting-started-installation-chocolatey}

```shell
$ choco install hurl
```

##### Scoop {#getting-started-installation-scoop}

```shell
$ scoop install hurl
```

##### Windows Package Manager {#getting-started-installation-windows-package-manager}

```shell
$ winget install hurl
```

#### Cargo {#getting-started-installation-cargo}

If you're a Rust programmer, Hurl can be installed with cargo.

```shell
$ cargo install hurl
```

#### conda-forge {#getting-started-installation-conda-forge}

```shell
$ conda install -c conda-forge hurl
```

Hurl can also be installed with [`conda-forge`](https://conda-forge.org) powered package manager like [`pixi`](https://prefix.dev).

#### Docker {#getting-started-installation-docker}

```shell
$ docker pull ghcr.io/orange-opensource/hurl:latest
```

#### npm {#getting-started-installation-npm}

```shell
$ npm install --save-dev @orangeopensource/hurl
```

### Building From Sources {#getting-started-installation-building-from-sources}

Hurl sources are available in [GitHub](https://github.com/Orange-OpenSource/hurl).

#### Build on Linux {#getting-started-installation-build-on-linux}

Hurl depends on libssl, libcurl and libxml2 native libraries. You will need their development files in your platform.

##### Debian based distributions {#getting-started-installation-debian-based-distributions}

```shell
$ apt install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev libxml2-dev
```

##### Fedora based distributions {#getting-started-installation-fedora-based-distributions}

```shell
$ dnf install -y pkgconf-pkg-config gcc openssl-devel libxml2-devel
```

##### Red Hat based distributions {#getting-started-installation-red-hat-based-distributions}

```shell
$ yum install -y pkg-config gcc openssl-devel libxml2-devel
```

##### Arch based distributions {#getting-started-installation-arch-based-distributions}

```shell
$ pacman -S --noconfirm pkgconf gcc glibc openssl libxml2
```

##### Alpine based distributions {#getting-started-installation-alpine-based-distributions}

```shell
$ apk add curl-dev gcc libxml2-dev musl-dev openssl-dev
```

#### Build on macOS {#getting-started-installation-build-on-macos}

```shell
$ xcode-select --install
$ brew install pkg-config
```

Hurl is written in [Rust](https://www.rust-lang.org). You should [install](https://www.rust-lang.org/tools/install) the latest stable release.

```shell
$ curl https://sh.rustup.rs -sSf | sh -s -- -y
$ source $HOME/.cargo/env
$ rustc --version
$ cargo --version
```

Then build hurl:

```shell
$ git clone https://github.com/Orange-OpenSource/hurl
$ cd hurl
$ cargo build --release
$ ./target/release/hurl --version
```

#### Build on Windows {#getting-started-installation-build-on-windows}

Please follow the [contrib on Windows section](https://github.com/Orange-OpenSource/hurl/blob/master/contrib/windows/README.md).



<hr>

## Manual {#getting-started-manual-manual}

### Name {#getting-started-manual-name}

hurl - run and test HTTP requests.


### Synopsis {#getting-started-manual-synopsis}

**hurl** [options] [FILE...]


### Description {#getting-started-manual-description}

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

Hurl can take files as input, or directories. In the latter case, Hurl will search files with `.hurl` extension recursively.

Output goes to stdout by default. To have output go to a file, use the [`-o, --output`](#getting-started-manual-output) option:

```shell
$ hurl -o output input.hurl
```

By default, Hurl executes all HTTP requests and outputs the response body of the last HTTP call.

To have a test oriented output, you can use [`--test`](#getting-started-manual-test) option:

```shell
$ hurl --test *.hurl
```


### Hurl File Format {#getting-started-manual-hurl-file-format}

The Hurl file format is fully documented in [https://hurl.dev/docs/hurl-file.html](https://hurl.dev/docs/hurl-file.html)

It consists of one or several HTTP requests

```hurl
GET http://example.org/endpoint1
GET http://example.org/endpoint2
```


#### Capturing values {#getting-started-manual-capturing-values}

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

#### Asserts {#getting-started-manual-asserts}

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

### Options {#getting-started-manual-options}

Options that exist in curl have exactly the same semantics.

Options specified on the command line are defined for every Hurl file's entry,
except if they are tagged as cli-only (can not be defined in the Hurl request [Options] entry)

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

| Option                                                                                                                                                          | Description                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <a href="#getting-started-manual-aws-sigv4" id="getting-started-manual-aws-sigv4"><code>--aws-sigv4 &lt;PROVIDER1[:PROVIDER2[:REGION[:SERVICE]]]&gt;</code></a> | Generate an `Authorization` header with an AWS SigV4 signature.<br><br>Use [`-u, --user`](#getting-started-manual-user) to specify Access Key Id (username) and Secret Key (password).<br><br>To use temporary session credentials (e.g. for an AWS IAM Role), add the `X-Amz-Security-Token` header containing the session token.<br>                                                                                                      |
| <a href="#getting-started-manual-cacert" id="getting-started-manual-cacert"><code>--cacert &lt;FILE&gt;</code></a>                                              | Specifies the certificate file for peer verification. The file may contain multiple CA certificates and must be in PEM format.<br>Normally Hurl is built to use a default file for this, so this option is typically used to alter that default file.<br>                                                                                                                                                                                   |
| <a href="#getting-started-manual-cert" id="getting-started-manual-cert"><code>-E, --cert &lt;CERTIFICATE[:PASSWORD]&gt;</code></a>                              | Client certificate file and password.<br><br>See also [`--key`](#getting-started-manual-key).<br>                                                                                                                                                                                                                                                                                                                                           |
| <a href="#getting-started-manual-color" id="getting-started-manual-color"><code>--color</code></a>                                                              | Colorize debug output (the HTTP response output is not colorized).<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                    |
| <a href="#getting-started-manual-compressed" id="getting-started-manual-compressed"><code>--compressed</code></a>                                               | Request a compressed response using one of the algorithms br, gzip, deflate and automatically decompress the content.<br>                                                                                                                                                                                                                                                                                                                   |
| <a href="#getting-started-manual-connect-timeout" id="getting-started-manual-connect-timeout"><code>--connect-timeout &lt;SECONDS&gt;</code></a>                | Maximum time in seconds that you allow Hurl's connection to take.<br><br>You can specify time units in the connect timeout expression. Set Hurl to use a connect timeout of 20 seconds with `--connect-timeout 20s` or set it to 35,000 milliseconds with `--connect-timeout 35000ms`. No spaces allowed.<br><br>See also [`-m, --max-time`](#getting-started-manual-max-time).<br><br>This is a cli-only option.<br>                       |
| <a href="#getting-started-manual-connect-to" id="getting-started-manual-connect-to"><code>--connect-to &lt;HOST1:PORT1:HOST2:PORT2&gt;</code></a>               | For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead. This option can be used several times in a command line.<br><br>See also [`--resolve`](#getting-started-manual-resolve).<br>                                                                                                                                                                                                                                   |
| <a href="#getting-started-manual-continue-on-error" id="getting-started-manual-continue-on-error"><code>--continue-on-error</code></a>                          | Continue executing requests to the end of the Hurl file even when an assert error occurs.<br>By default, Hurl exits after an assert error in the HTTP response.<br><br>Note that this option does not affect the behavior with multiple input Hurl files.<br><br>All the input files are executed independently. The result of one file does not affect the execution of the other Hurl files.<br><br>This is a cli-only option.<br>        |
| <a href="#getting-started-manual-cookie" id="getting-started-manual-cookie"><code>-b, --cookie &lt;FILE&gt;</code></a>                                          | Read cookies from FILE (using the Netscape cookie file format).<br><br>Combined with [`-c, --cookie-jar`](#getting-started-manual-cookie-jar), you can simulate a cookie storage between successive Hurl runs.<br><br>This is a cli-only option.<br>                                                                                                                                                                                        |
| <a href="#getting-started-manual-cookie-jar" id="getting-started-manual-cookie-jar"><code>-c, --cookie-jar &lt;FILE&gt;</code></a>                              | Write cookies to FILE after running the session (only for one session).<br>The file will be written using the Netscape cookie file format.<br><br>Combined with [`-b, --cookie`](#getting-started-manual-cookie), you can simulate a cookie storage between successive Hurl runs.<br><br>This is a cli-only option.<br>                                                                                                                     |
| <a href="#getting-started-manual-delay" id="getting-started-manual-delay"><code>--delay &lt;MILLISECONDS&gt;</code></a>                                         | Sets delay before each request. The delay is not applied to requests that have been retried because of [`--retry`](#getting-started-manual-retry). See [`--retry-interval`](#getting-started-manual-retry-interval) to space retried requests.<br><br>You can specify time units in the delay expression. Set Hurl to use a delay of 2 seconds with `--delay 2s` or set it to 500 milliseconds with `--delay 500ms`. No spaces allowed.<br> |
| <a href="#getting-started-manual-error-format" id="getting-started-manual-error-format"><code>--error-format &lt;FORMAT&gt;</code></a>                          | Control the format of error message (short by default or long)<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                        |
| <a href="#getting-started-manual-file-root" id="getting-started-manual-file-root"><code>--file-root &lt;DIR&gt;</code></a>                                      | Set root directory to import files in Hurl. This is used for files in multipart form data, request body and response output.<br>When it is not explicitly defined, files are relative to the Hurl file's directory.<br><br>This is a cli-only option.<br>                                                                                                                                                                                   |
| <a href="#getting-started-manual-from-entry" id="getting-started-manual-from-entry"><code>--from-entry &lt;ENTRY_NUMBER&gt;</code></a>                          | Execute Hurl file from ENTRY_NUMBER (starting at 1).<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                                  |
| <a href="#getting-started-manual-glob" id="getting-started-manual-glob"><code>--glob &lt;GLOB&gt;</code></a>                                                    | Specify input files that match the given glob pattern.<br><br>Multiple glob flags may be used. This flag supports common Unix glob patterns like *, ? and [].<br>However, to avoid your shell accidentally expanding glob patterns before Hurl handles them, you must use single quotes or double quotes around each pattern.<br><br>This is a cli-only option.<br>                                                                         |
| <a href="#getting-started-manual-http10" id="getting-started-manual-http10"><code>-0, --http1.0</code></a>                                                      | Tells Hurl to use HTTP version 1.0 instead of using its internally preferred HTTP version.<br>                                                                                                                                                                                                                                                                                                                                              |
| <a href="#getting-started-manual-http11" id="getting-started-manual-http11"><code>--http1.1</code></a>                                                          | Tells Hurl to use HTTP version 1.1.<br>                                                                                                                                                                                                                                                                                                                                                                                                     |
| <a href="#getting-started-manual-http2" id="getting-started-manual-http2"><code>--http2</code></a>                                                              | Tells Hurl to use HTTP version 2.<br>For HTTPS, this means Hurl negotiates HTTP/2 in the TLS handshake. Hurl does this by default.<br>For HTTP, this means Hurl attempts to upgrade the request to HTTP/2 using the Upgrade: request header.<br>                                                                                                                                                                                            |
| <a href="#getting-started-manual-http3" id="getting-started-manual-http3"><code>--http3</code></a>                                                              | Tells Hurl to try HTTP/3 to the host in the URL, but fallback to earlier HTTP versions if the HTTP/3 connection establishment fails. HTTP/3 is only available for HTTPS and not for HTTP URLs.<br>                                                                                                                                                                                                                                          |
| <a href="#getting-started-manual-ignore-asserts" id="getting-started-manual-ignore-asserts"><code>--ignore-asserts</code></a>                                   | Ignore all asserts defined in the Hurl file.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                                          |
| <a href="#getting-started-manual-include" id="getting-started-manual-include"><code>-i, --include</code></a>                                                    | Include the HTTP headers in the output<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                                                |
| <a href="#getting-started-manual-insecure" id="getting-started-manual-insecure"><code>-k, --insecure</code></a>                                                 | This option explicitly allows Hurl to perform "insecure" SSL connections and transfers.<br>                                                                                                                                                                                                                                                                                                                                                 |
| <a href="#getting-started-manual-interactive" id="getting-started-manual-interactive"><code>--interactive</code></a>                                            | Stop between requests.<br><br>This is similar to a break point, You can then continue (Press C) or quit (Press Q).<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                    |
| <a href="#getting-started-manual-ipv4" id="getting-started-manual-ipv4"><code>-4, --ipv4</code></a>                                                             | This option tells Hurl to use IPv4 addresses only when resolving host names, and not for example try IPv6.<br>                                                                                                                                                                                                                                                                                                                              |
| <a href="#getting-started-manual-ipv6" id="getting-started-manual-ipv6"><code>-6, --ipv6</code></a>                                                             | This option tells Hurl to use IPv6 addresses only when resolving host names, and not for example try IPv4.<br>                                                                                                                                                                                                                                                                                                                              |
| <a href="#getting-started-manual-jobs" id="getting-started-manual-jobs"><code>--jobs &lt;NUM&gt;</code></a>                                                     | Maximum number of parallel jobs in parallel mode. Default value corresponds (in most cases) to the<br>current amount of CPUs.<br><br>See also [`--parallel`](#getting-started-manual-parallel).<br><br>This is a cli-only option.<br>                                                                                                                                                                                                       |
| <a href="#getting-started-manual-json" id="getting-started-manual-json"><code>--json</code></a>                                                                 | Output each Hurl file result to JSON. The format is very closed to HAR format.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                        |
| <a href="#getting-started-manual-key" id="getting-started-manual-key"><code>--key &lt;KEY&gt;</code></a>                                                        | Private key file name.<br>                                                                                                                                                                                                                                                                                                                                                                                                                  |
| <a href="#getting-started-manual-location" id="getting-started-manual-location"><code>-L, --location</code></a>                                                 | Follow redirect. To limit the amount of redirects to follow use the [`--max-redirs`](#getting-started-manual-max-redirs) option<br>                                                                                                                                                                                                                                                                                                         |
| <a href="#getting-started-manual-location-trusted" id="getting-started-manual-location-trusted"><code>--location-trusted</code></a>                             | Like [`-L, --location`](#getting-started-manual-location), but allows sending the name + password to all hosts that the site may redirect to.<br>This may or may not introduce a security breach if the site redirects you to a site to which you send your authentication info (which is plaintext in the case of HTTP Basic authentication).<br>                                                                                          |
| <a href="#getting-started-manual-max-filesize" id="getting-started-manual-max-filesize"><code>--max-filesize &lt;BYTES&gt;</code></a>                           | Specify the maximum size (in bytes) of a file to download. If the file requested is larger than this value, the transfer does not start.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                              |
| <a href="#getting-started-manual-max-redirs" id="getting-started-manual-max-redirs"><code>--max-redirs &lt;NUM&gt;</code></a>                                   | Set maximum number of redirection-followings allowed<br><br>By default, the limit is set to 50 redirections. Set this option to -1 to make it unlimited.<br>                                                                                                                                                                                                                                                                                |
| <a href="#getting-started-manual-max-time" id="getting-started-manual-max-time"><code>-m, --max-time &lt;SECONDS&gt;</code></a>                                 | Maximum time in seconds that you allow a request/response to take. This is the standard timeout.<br><br>You can specify time units in the maximum time expression. Set Hurl to use a maximum time of 20 seconds with `--max-time 20s` or set it to 35,000 milliseconds with `--max-time 35000ms`. No spaces allowed.<br><br>See also [`--connect-timeout`](#getting-started-manual-connect-timeout).<br><br>This is a cli-only option.<br>  |
| <a href="#getting-started-manual-netrc" id="getting-started-manual-netrc"><code>-n, --netrc</code></a>                                                          | Scan the .netrc file in the user's home directory for the username and password.<br><br>See also [`--netrc-file`](#getting-started-manual-netrc-file) and [`--netrc-optional`](#getting-started-manual-netrc-optional).<br>                                                                                                                                                                                                                 |
| <a href="#getting-started-manual-netrc-file" id="getting-started-manual-netrc-file"><code>--netrc-file &lt;FILE&gt;</code></a>                                  | Like [`--netrc`](#getting-started-manual-netrc), but provide the path to the netrc file.<br><br>See also [`--netrc-optional`](#getting-started-manual-netrc-optional).<br>                                                                                                                                                                                                                                                                  |
| <a href="#getting-started-manual-netrc-optional" id="getting-started-manual-netrc-optional"><code>--netrc-optional</code></a>                                   | Similar to [`--netrc`](#getting-started-manual-netrc), but make the .netrc usage optional.<br><br>See also [`--netrc-file`](#getting-started-manual-netrc-file).<br>                                                                                                                                                                                                                                                                        |
| <a href="#getting-started-manual-no-color" id="getting-started-manual-no-color"><code>--no-color</code></a>                                                     | Do not colorize output.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                                                               |
| <a href="#getting-started-manual-no-output" id="getting-started-manual-no-output"><code>--no-output</code></a>                                                  | Suppress output. By default, Hurl outputs the body of the last response.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                              |
| <a href="#getting-started-manual-noproxy" id="getting-started-manual-noproxy"><code>--noproxy &lt;HOST(S)&gt;</code></a>                                        | Comma-separated list of hosts which do not use a proxy.<br><br>Override value from Environment variable no_proxy.<br>                                                                                                                                                                                                                                                                                                                       |
| <a href="#getting-started-manual-output" id="getting-started-manual-output"><code>-o, --output &lt;FILE&gt;</code></a>                                          | Write output to FILE instead of stdout.<br>                                                                                                                                                                                                                                                                                                                                                                                                 |
| <a href="#getting-started-manual-parallel" id="getting-started-manual-parallel"><code>--parallel</code></a>                                                     | Run files in parallel.<br><br>Each Hurl file is executed in its own worker thread, without sharing anything with the other workers. The default run mode is sequential. Parallel execution is by default in [`--test`](#getting-started-manual-test) mode.<br><br>See also [`--jobs`](#getting-started-manual-jobs).<br><br>This is a cli-only option.<br>                                                                                  |
| <a href="#getting-started-manual-path-as-is" id="getting-started-manual-path-as-is"><code>--path-as-is</code></a>                                               | Tell Hurl to not handle sequences of /../ or /./ in the given URL path. Normally Hurl will squash or merge them according to standards but with this option set you tell it not to do that.<br>                                                                                                                                                                                                                                             |
| <a href="#getting-started-manual-proxy" id="getting-started-manual-proxy"><code>-x, --proxy &lt;[PROTOCOL://]HOST[:PORT]&gt;</code></a>                         | Use the specified proxy.<br>                                                                                                                                                                                                                                                                                                                                                                                                                |
| <a href="#getting-started-manual-repeat" id="getting-started-manual-repeat"><code>--repeat &lt;NUM&gt;</code></a>                                               | Repeat the input files sequence NUM times, -1 for infinite loop. Given a.hurl, b.hurl, c.hurl as input, repeat two<br>times will run a.hurl, b.hurl, c.hurl, a.hurl, b.hurl, c.hurl.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                  |
| <a href="#getting-started-manual-report-html" id="getting-started-manual-report-html"><code>--report-html &lt;DIR&gt;</code></a>                                | Generate HTML report in DIR.<br><br>If the HTML report already exists, it will be updated with the new test results.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                  |
| <a href="#getting-started-manual-report-json" id="getting-started-manual-report-json"><code>--report-json &lt;DIR&gt;</code></a>                                | Generate JSON report in DIR.<br><br>If the JSON report already exists, it will be updated with the new test results.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                  |
| <a href="#getting-started-manual-report-junit" id="getting-started-manual-report-junit"><code>--report-junit &lt;FILE&gt;</code></a>                            | Generate JUnit File.<br><br>If the FILE report already exists, it will be updated with the new test results.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                          |
| <a href="#getting-started-manual-report-tap" id="getting-started-manual-report-tap"><code>--report-tap &lt;FILE&gt;</code></a>                                  | Generate TAP report.<br><br>If the FILE report already exists, it will be updated with the new test results.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                          |
| <a href="#getting-started-manual-resolve" id="getting-started-manual-resolve"><code>--resolve &lt;HOST:PORT:ADDR&gt;</code></a>                                 | Provide a custom address for a specific host and port pair. Using this, you can make the Hurl requests(s) use a specified address and prevent the otherwise normally resolved address to be used. Consider it a sort of /etc/hosts alternative provided on the command line.<br>                                                                                                                                                            |
| <a href="#getting-started-manual-retry" id="getting-started-manual-retry"><code>--retry &lt;NUM&gt;</code></a>                                                  | Maximum number of retries, 0 for no retries, -1 for unlimited retries. Retry happens if any error occurs (asserts, captures, runtimes etc...).<br>                                                                                                                                                                                                                                                                                          |
| <a href="#getting-started-manual-retry-interval" id="getting-started-manual-retry-interval"><code>--retry-interval &lt;MILLISECONDS&gt;</code></a>              | Duration in milliseconds between each retry. Default is 1000 ms.<br><br>You can specify time units in the retry interval expression. Set Hurl to use a retry interval of 2 seconds with `--retry-interval 2s` or set it to 500 milliseconds with `--retry-interval 500ms`. No spaces allowed.<br>                                                                                                                                           |
| <a href="#getting-started-manual-ssl-no-revoke" id="getting-started-manual-ssl-no-revoke"><code>--ssl-no-revoke</code></a>                                      | (Windows) This option tells Hurl to disable certificate revocation checks. WARNING: this option loosens the SSL security, and by using this flag you ask for exactly that.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                            |
| <a href="#getting-started-manual-test" id="getting-started-manual-test"><code>--test</code></a>                                                                 | Activate test mode: with this, the HTTP response is not outputted anymore, progress is reported for each Hurl file tested, and a text summary is displayed when all files have been run.<br><br>In test mode, files are executed in parallel. To run test in a sequential way use `--job 1`.<br><br>See also [`--jobs`](#getting-started-manual-jobs).<br><br>This is a cli-only option.<br>                                                |
| <a href="#getting-started-manual-to-entry" id="getting-started-manual-to-entry"><code>--to-entry &lt;ENTRY_NUMBER&gt;</code></a>                                | Execute Hurl file to ENTRY_NUMBER (starting at 1).<br>Ignore the remaining of the file. It is useful for debugging a session.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                         |
| <a href="#getting-started-manual-unix-socket" id="getting-started-manual-unix-socket"><code>--unix-socket &lt;PATH&gt;</code></a>                               | (HTTP) Connect through this Unix domain socket, instead of using the network.<br>                                                                                                                                                                                                                                                                                                                                                           |
| <a href="#getting-started-manual-user" id="getting-started-manual-user"><code>-u, --user &lt;USER:PASSWORD&gt;</code></a>                                       | Add basic Authentication header to each request.<br>                                                                                                                                                                                                                                                                                                                                                                                        |
| <a href="#getting-started-manual-user-agent" id="getting-started-manual-user-agent"><code>-A, --user-agent &lt;NAME&gt;</code></a>                              | Specify the User-Agent string to send to the HTTP server.<br><br>This is a cli-only option.<br>                                                                                                                                                                                                                                                                                                                                             |
| <a href="#getting-started-manual-variable" id="getting-started-manual-variable"><code>--variable &lt;NAME=VALUE&gt;</code></a>                                  | Define variable (name/value) to be used in Hurl templates.<br>                                                                                                                                                                                                                                                                                                                                                                              |
| <a href="#getting-started-manual-variables-file" id="getting-started-manual-variables-file"><code>--variables-file &lt;FILE&gt;</code></a>                      | Set properties file in which your define your variables.<br><br>Each variable is defined as name=value exactly as with [`--variable`](#getting-started-manual-variable) option.<br><br>Note that defining a variable twice produces an error.<br><br>This is a cli-only option.<br>                                                                                                                                                         |
| <a href="#getting-started-manual-verbose" id="getting-started-manual-verbose"><code>-v, --verbose</code></a>                                                    | Turn on verbose output on standard error stream.<br>Useful for debugging.<br><br>A line starting with '>' means data sent by Hurl.<br>A line staring with '<' means data received by Hurl.<br>A line starting with '*' means additional info provided by Hurl.<br><br>If you only want HTTP headers in the output, [`-i, --include`](#getting-started-manual-include) might be the option you're looking for.<br>                           |
| <a href="#getting-started-manual-very-verbose" id="getting-started-manual-very-verbose"><code>--very-verbose</code></a>                                         | Turn on more verbose output on standard error stream.<br><br>In contrast to  [`--verbose`](#getting-started-manual-verbose) option, this option outputs the full HTTP body request and response on standard error. In addition, lines starting with '**' are libcurl debug logs.<br>                                                                                                                                                        |
| <a href="#getting-started-manual-help" id="getting-started-manual-help"><code>-h, --help</code></a>                                                             | Usage help. This lists all current command line options with a short description.<br>                                                                                                                                                                                                                                                                                                                                                       |
| <a href="#getting-started-manual-version" id="getting-started-manual-version"><code>-V, --version</code></a>                                                    | Prints version information<br>                                                                                                                                                                                                                                                                                                                                                                                                              |

### Environment {#getting-started-manual-environment}

Environment variables can only be specified in lowercase.

Using an environment variable to set the proxy has the same effect as using the [`-x, --proxy`](#getting-started-manual-proxy) option.

| Variable                                     | Description                                                                                                                                                                                                    |
|----------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `http_proxy [PROTOCOL://]<HOST>[:PORT]`      | Sets the proxy server to use for HTTP.<br>                                                                                                                                                                     |
| `https_proxy [PROTOCOL://]<HOST>[:PORT]`     | Sets the proxy server to use for HTTPS.<br>                                                                                                                                                                    |
| `all_proxy [PROTOCOL://]<HOST>[:PORT]`       | Sets the proxy server to use if no protocol-specific proxy is set.<br>                                                                                                                                         |
| `no_proxy <comma-separated list of hosts>`   | List of host names that shouldn't go through any proxy.<br>                                                                                                                                                    |
| `HURL_name value`                            | Define variable (name/value) to be used in Hurl templates. This is similar than [`--variable`](#getting-started-manual-variable) and [`--variables-file`](#getting-started-manual-variables-file) options.<br> |
| `NO_COLOR`                                   | When set to a non-empty string, do not colorize output (see [`--no-color`](#getting-started-manual-no-color) option).<br>                                                                                      |

### Exit Codes {#getting-started-manual-exit-codes}

| Value   | Description                                               |
|---------|-----------------------------------------------------------|
| `0`     | Success.<br>                                              |
| `1`     | Failed to parse command-line options.<br>                 |
| `2`     | Input File Parsing Error.<br>                             |
| `3`     | Runtime error (such as failure to connect to host).<br>   |
| `4`     | Assert Error.<br>                                         |

### WWW {#getting-started-manual-www}

[https://hurl.dev](https://hurl.dev)


### See Also {#getting-started-manual-see-also}

curl(1)  hurlfmt(1)



<hr>

## Samples {#getting-started-samples-samples}

To run a sample, edit a file with the sample content, and run Hurl:

```shell
$ vi sample.hurl

GET https://example.org

$ hurl sample.hurl
```

By default, Hurl behaves like [curl](https://curl.se) and outputs the last HTTP response's [entry](#file-format-entry). To have a test
oriented output, you can use [`--test` option](#getting-started-manual-test):

```shell
$ hurl --test sample.hurl
```

A particular response can be saved with [`[Options] section`](#file-format-request-options):

```hurl
GET https://example.ord/cats/123
[Options]
output: cat123.txt    # use - to output to stdout
HTTP 200

GET https://example.ord/dogs/567
HTTP 200
```

Finally, Hurl can take files as input, or directories. In the latter case, Hurl will search files with `.hurl` extension recursively.

```shell
$ hurl --test integration/*.hurl
$ hurl --test .
```

You can check [Hurl tests suite](https://github.com/Orange-OpenSource/hurl/tree/master/integration/hurl/tests_ok) for more samples.

### Getting Data {#getting-started-samples-getting-data}

A simple GET:

```hurl
GET https://example.org
```

Requests can be chained:

```hurl
GET https://example.org/a
GET https://example.org/b
HEAD https://example.org/c
GET https://example.org/c
```

[Doc](#file-format-request-method)

#### HTTP Headers {#getting-started-samples-http-headers}

A simple GET with headers:

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 
Accept: */*
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
```

[Doc](#file-format-request-headers)

#### Query Params {#getting-started-samples-query-params}

```hurl
GET https://example.org/news
[QueryStringParams]
order: newest
search: something to search
count: 100
```

Or:

```hurl
GET https://example.org/news?order=newest&search=something%20to%20search&count=100
```

> With `[QueryStringParams]` section, params don't need to be URL escaped.

[Doc](#file-format-request-query-parameters)

#### Basic Authentication {#getting-started-samples-basic-authentication}

```hurl
GET https://example.org/protected
[BasicAuth]
bob: secret
```

[Doc](#file-format-request-basic-authentication)

This is equivalent to construct the request with a [Authorization](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization) header:

```hurl
# Authorization header value can be computed with `echo -n 'bob:secret' | base64`
GET https://example.org/protected
Authorization: Basic Ym9iOnNlY3JldA== 
```

Basic authentication section allows per request authentication. If you want to add basic authentication to all the
requests of a Hurl file you could use [`-u/--user` option](#getting-started-manual-user):

```shell
$ hurl --user bob:secret login.hurl
```

[`--user`](#getting-started-manual-user) option can also be set per request:

```hurl
GET https://example.org/login
[Options]
user: bob:secret
HTTP 200

GET https://example.org/login
[Options]
user: alice:secret
HTTP 200
```

#### Passing Data between Requests  {#getting-started-samples-passing-data-between-requests}

[Captures](#file-format-capturing-response) can be used to pass data from one request to another:

```hurl
POST https://sample.org/orders
HTTP 201
[Captures]
order_id: jsonpath "$.order.id"

GET https://sample.org/orders/{{order_id}}
HTTP 200
```

[Doc](#file-format-capturing-response)

### Sending Data {#getting-started-samples-sending-data}

#### Sending HTML Form Data {#getting-started-samples-sending-html-form-data}

```hurl
POST https://example.org/contact
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

[Doc](#file-format-request-form-parameters)

#### Sending Multipart Form Data {#getting-started-samples-sending-multipart-form-data}

```hurl
POST https://example.org/upload
[MultipartFormData]
field1: value1
field2: file,example.txt;
# One can specify the file content type:
field3: file,example.zip; application/zip
```

[Doc](#file-format-request-multipart-form-data)

Multipart forms can also be sent with a [multiline string body](#file-format-request-multiline-string-body):

~~~hurl
POST https://example.org/upload
Content-Type: multipart/form-data; boundary="boundary"
```
--boundary
Content-Disposition: form-data; name="key1"

value1
--boundary
Content-Disposition: form-data; name="upload1"; filename="data.txt"
Content-Type: text/plain

Hello World!
--boundary
Content-Disposition: form-data; name="upload2"; filename="data.html"
Content-Type: text/html

<div>Hello <b>World</b>!</div>
--boundary--
```
~~~

In that case, files have to be inlined in the Hurl file.

[Doc](#file-format-request-multiline-string-body)



#### Posting a JSON Body {#getting-started-samples-posting-a-json-body}

With an inline JSON:

```hurl
POST https://example.org/api/tests
{
    "id": "456",
    "evaluate": true
}
```

[Doc](#file-format-request-json-body)

With a local file:

```hurl
POST https://example.org/api/tests
Content-Type: application/json
file,data.json;
```

[Doc](#file-format-request-file-body)

#### Templating a JSON Body {#getting-started-samples-templating-a-json-body}

```hurl
PUT https://example.org/api/hits
Content-Type: application/json
{
    "key0": "{{a_string}}",
    "key1": {{a_bool}},
    "key2": {{a_null}},
    "key3": {{a_number}}
}
```

Variables can be initialized via command line:

```shell
$ hurl --variable a_string=apple \
       --variable a_bool=true \
       --variable a_null=null \
       --variable a_number=42 \
       test.hurl
```

Resulting in a PUT request with the following JSON body:

```
{
    "key0": "apple",
    "key1": true,
    "key2": null,
    "key3": 42
}
```

[Doc](#file-format-templates)

#### Templating a XML Body {#getting-started-samples-templating-a-xml-body}

Using templates with [XML body](#file-format-request-xml-body) is not currently supported in Hurl. You can use templates in
[XML multiline string body](#file-format-request-multiline-string-body) with variables to send a variable XML body:

~~~hurl
POST https://example.org/echo/post/xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<Request>
    <Login>{{login}}</Login>
    <Password>{{password}}</Password>
</Request>
```
~~~

[Doc](#file-format-request-multiline-string-body)

#### Using GraphQL Query {#getting-started-samples-using-graphql-query}

A simple GraphQL query:

~~~hurl
POST https://example.org/starwars/graphql
```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```
~~~

A GraphQL query with variables:

~~~hurl
POST https://example.org/starwars/graphql
```graphql
query Hero($episode: Episode, $withFriends: Boolean!) {
  hero(episode: $episode) {
    name
    friends @include(if: $withFriends) {
      name
    }
  }
}

variables {
  "episode": "JEDI",
  "withFriends": false
}
```
~~~

GraphQL queries can also use [Hurl templates](#file-format-templates).

[Doc](#file-format-request-graphql-body)

### Testing Response {#getting-started-samples-testing-response}

Responses are optional, everything after `HTTP` is part of the response asserts.

```hurl
# A request with (almost) no check:
GET https://foo.com

# A status code check:
GET https://foo.com
HTTP 200

# A test on response body
GET https://foo.com
HTTP 200
[Asserts]
jsonpath "$.state" == "running"
```

#### Testing Status Code {#getting-started-samples-testing-status-code}

```hurl
GET https://example.org/order/435
HTTP 200
```

[Doc](#file-format-asserting-response-version-status)

```hurl
GET https://example.org/order/435
# Testing status code is in a 200-300 range
HTTP *
[Asserts]
status >= 200
status < 300
```

[Doc](#file-format-asserting-response-status-assert)


#### Testing Response Headers {#getting-started-samples-testing-response-headers}

Use implicit response asserts to test header values:

```hurl
GET https://example.org/index.html
HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

[Doc](#file-format-asserting-response-headers)


Or use explicit response asserts with [predicates](#file-format-asserting-response-predicates):

```hurl
GET https://example.org
HTTP 302
[Asserts]
header "Location" contains "www.example.net"
```

[Doc](#file-format-asserting-response-header-assert)

Implicit and explicit asserts can be combined:

```hurl
GET https://example.org/index.html
HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
[Asserts]
header "Location" contains "www.example.net"
```

#### Testing REST APIs {#getting-started-samples-testing-rest-apis}

Asserting JSON body response (node values, collection count etc...) with [JSONPath](https://goessner.net/articles/JsonPath/):

```hurl
GET https://example.org/order
screencapability: low
HTTP 200
[Asserts]
jsonpath "$.validated" == true
jsonpath "$.userInfo.firstName" == "Franck"
jsonpath "$.userInfo.lastName" == "Herbert"
jsonpath "$.hasDevice" == false
jsonpath "$.links" count == 12
jsonpath "$.state" != null
jsonpath "$.order" matches "^order-\\d{8}$"
jsonpath "$.order" matches /^order-\d{8}$/     # Alternative syntax with regex literal
jsonpath "$.created" isIsoDate
```

[Doc](#file-format-asserting-response-jsonpath-assert)


#### Testing HTML Response {#getting-started-samples-testing-html-response}

```hurl
GET https://example.org
HTTP 200
Content-Type: text/html; charset=UTF-8
[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" == 2  # Check the number of p
xpath "//p" count == 2  # Similar assert for p
xpath "boolean(count(//h2))" == false  # Check there is no h2  
xpath "//h2" not exists  # Similar assert for h2
xpath "string(//div[1])" matches /Hello.*/
```

[Doc](#file-format-asserting-response-xpath-assert)

#### Testing Set-Cookie Attributes {#getting-started-samples-testing-set-cookie-attributes}

```hurl
GET https://example.org/home
HTTP 200
[Asserts]
cookie "JSESSIONID" == "8400BAFE2F66443613DC38AE3D9D6239"
cookie "JSESSIONID[Value]" == "8400BAFE2F66443613DC38AE3D9D6239"
cookie "JSESSIONID[Expires]" contains "Wed, 13 Jan 2021"
cookie "JSESSIONID[Secure]" exists
cookie "JSESSIONID[HttpOnly]" exists
cookie "JSESSIONID[SameSite]" == "Lax"
```

[Doc](#file-format-asserting-response-cookie-assert)

#### Testing Bytes Content {#getting-started-samples-testing-bytes-content}

Check the SHA-256 response body hash:

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

[Doc](#file-format-asserting-response-sha-256-assert)

#### SSL Certificate {#getting-started-samples-ssl-certificate}

Check the properties of a SSL certificate:

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Subject" == "CN=example.org"
certificate "Issuer" == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches /[\da-f]+/
```

[Doc](#file-format-asserting-response-ssl-certificate-assert)

#### Checking Full Body {#getting-started-samples-checking-full-body}

Use implicit body to test an exact JSON body match:

```hurl
GET https://example.org/api/cats/123
HTTP 200
{
  "name" : "Purrsloud",
  "species" : "Cat",
  "favFoods" : ["wet food", "dry food", "<strong>any</strong> food"],
  "birthYear" : 2016,
  "photo" : "https://learnwebcode.github.io/json-example/images/cat-2.jpg"
}
```

[Doc](#file-format-asserting-response-json-body)

Or an explicit assert file:

```hurl
GET https://example.org/index.html
HTTP 200
[Asserts]
body == file,cat.json;
```

[Doc](#file-format-asserting-response-body-assert)

Implicit asserts supports XML body:

```hurl
GET https://example.org/api/catalog
HTTP 200
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <book id="bk101">
      <author>Gambardella, Matthew</author>
      <title>XML Developer's Guide</title>
      <genre>Computer</genre>
      <price>44.95</price>
      <publish_date>2000-10-01</publish_date>
      <description>An in-depth look at creating applications with XML.</description>
   </book>
</catalog>
```

[Doc](#file-format-asserting-response-xml-body)

Plain text:

~~~hurl
GET https://example.org/models
HTTP 200
```
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL! air, moon roof, loaded",4799.00
```
~~~

[Doc](#file-format-asserting-response-multiline-string-body)


One line:

```hurl
POST https://example.org/helloworld
HTTP 200
`Hello world!`
```

[Doc](#file-format-asserting-response-oneline-string-body)

File:

```hurl
GET https://example.org
HTTP 200
file,data.bin;
```

[Doc](#file-format-asserting-response-file-body)


### Reports {#getting-started-samples-reports}

#### HTML Report {#getting-started-samples-html-report}

```shell
$ hurl --test --report-html build/report/ *.hurl
```

[Doc](#getting-started-running-tests-generating-report)

#### JSON Report {#getting-started-samples-json-report}

```shell
$ hurl --test --report-json build/report/ *.hurl
```

[Doc](#getting-started-running-tests-generating-report)


#### JUnit Report {#getting-started-samples-junit-report}

```shell
$ hurl --test --report-junit build/report.xml *.hurl
```

[Doc](#getting-started-running-tests-generating-report)

#### TAP Report {#getting-started-samples-tap-report}

```shell
$ hurl --test --report-tap build/report.txt *.hurl
```

[Doc](#getting-started-running-tests-generating-report)

#### JSON Output {#getting-started-samples-json-output}

A structured output of running Hurl files can be obtained with [`--json` option](#getting-started-manual-json). Each file will produce a JSON export of the run.


```shell
$ hurl --json *.hurl
```


### Others {#getting-started-samples-others}

#### HTTP Version {#getting-started-samples-http-version}

Testing HTTP version (HTTP/1.0, HTTP/1.1, HTTP/2 or HTTP/3):

```hurl
GET https://foo.com
HTTP/3 200

GET https://bar.com
HTTP/2 200
```

[Doc](#file-format-asserting-response-version-status)

#### Polling and Retry {#getting-started-samples-polling-and-retry}

Retry request on any errors (asserts, captures, status code, runtime etc...):

```hurl
# Create a new job
POST https://api.example.org/jobs
HTTP 201
[Captures]
job_id: jsonpath "$.id"
[Asserts]
jsonpath "$.state" == "RUNNING"


# Pull job status until it is completed
GET https://api.example.org/jobs/{{job_id}}
[Options]
retry: 10   # maximum number of retry, -1 for unlimited
retry-interval: 500ms
HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"
```

[Doc](#file-format-entry-retry)

#### Delaying Requests {#getting-started-samples-delaying-requests}

Add delay for every request, or a particular request:

```hurl
# Delaying this request by 5 seconds
GET https://example.org/turtle
[Options]
delay: 5s
HTTP 200

# No delay!
GET https://example.org/turtle
HTTP 200
```

[Doc](#getting-started-manual-delay)

#### Skipping Requests {#getting-started-samples-skipping-requests}

```hurl
# a, c, d are run, b is skipped
GET https://example.org/a

GET https://example.org/b
[Options]
skip: true

GET https://example.org/c

GET https://example.org/d
```

[Doc](#getting-started-manual-skip)


#### Testing Endpoint Performance {#getting-started-samples-testing-endpoint-performance}

```hurl
GET https://sample.org/helloworld
HTTP *
[Asserts]
duration < 1000   # Check that response time is less than one second
```

[Doc](#file-format-asserting-response-duration-assert)

#### Using SOAP APIs {#getting-started-samples-using-soap-apis}

```hurl
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="https://example.org">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
HTTP 200
```

[Doc](#file-format-request-xml-body)

#### Capturing and Using a CSRF Token {#getting-started-samples-capturing-and-using-a-csrf-token}

```hurl
GET https://example.org
HTTP 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"


POST https://example.org/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

[Doc](#file-format-capturing-response-xpath-capture)

#### Checking Byte Order Mark (BOM) in Response Body {#getting-started-samples-checking-byte-order-mark-bom-in-response-body}

```hurl
GET https://example.org/data.bin
HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
```

[Doc](#file-format-asserting-response-bytes-assert)

#### AWS Signature Version 4 Requests {#getting-started-samples-aws-signature-version-4-requests}

Generate signed API requests with [AWS Signature Version 4](https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html), as used by several cloud providers.

```hurl
POST https://sts.eu-central-1.amazonaws.com/
[Options]
aws-sigv4: aws:amz:eu-central-1:sts
[FormParams]
Action: GetCallerIdentity
Version: 2011-06-15
```

The Access Key is given per [`--user`](#getting-started-manual-user), either with command line option or within the [`[Options]`](#file-format-request-options) section:

```hurl
POST https://sts.eu-central-1.amazonaws.com/
[Options]
aws-sigv4: aws:amz:eu-central-1:sts
user: bob=secret
[FormParams]
Action: GetCallerIdentity
Version: 2011-06-15
```

[Doc](#getting-started-manual-aws-sigv4)

#### Using curl Options {#getting-started-samples-using-curl-options}

curl options (for instance [`--resolve`](#getting-started-manual-resolve) or [`--connect-to`](#getting-started-manual-connect-to)) can be used as CLI argument. In this case, they're applicable
to each request of an Hurl file.

```shell
$ hurl --resolve foo.com:8000:127.0.0.1 foo.hurl
```

Use  [`[Options]` section](#file-format-request-options) to configure a specific request:

```hurl
GET http://bar.com
HTTP 200


GET http://foo.com:8000/resolve
[Options]
resolve: foo.com:8000:127.0.0.1
HTTP 200
`Hello World!`
```

[Doc](#file-format-request-options)




<hr>

## Running Tests {#getting-started-running-tests-running-tests}

### Use --test Option {#getting-started-running-tests-use-test-option}

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
test tool with an adapted output, you can use [`--test` option](#getting-started-manual-test):

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

In test mode, files are executed in parallel to speed-ud the execution. If a sequential run is needed, you can use
[`--jobs 1`](#getting-started-manual-jobs) option to execute tests one by one.

```shell
$ hurl --test --jobs 1 *.hurl
```

[`--repeat` option](#getting-started-manual-repeat) can be used to repeat run files and do performance check. For instance, this call will run 1000 tests
in parallel:

```shell
$ hurl --test --repeat 1000 stress.hurl
```


#### Selecting Tests {#getting-started-running-tests-selecting-tests}

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

Finally, you can use [`--glob` option](#getting-started-manual-glob) to test files that match a given pattern:

```shell
$ hurl --test --glob "test/integration/**/*.hurl"
```

### Debugging {#getting-started-running-tests-debugging}

#### Debug Logs {#getting-started-running-tests-debug-logs}

If you need more error context, you can use [`--error-format long` option](#getting-started-manual-error-format) to print HTTP bodies for failed asserts:

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

Individual requests can be modified with [`[Options]` section][options](#file-format-request-options) to turn on logs for a particular request, using
[`verbose`](#getting-started-manual-verbose) and [`very-verbose`](#getting-started-manual-very-verbose) option. 

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

[`--verbose`](#getting-started-manual-verbose) / [`--very-verbose`](#getting-started-manual-very-verbose) can also be enabled globally, for every requests of every tested files:

```shell
$ hurl --test --very-verbose .
```

#### HTTP Responses {#getting-started-running-tests-http-responses}

In test mode, HTTP responses are not displayed. One way to get HTTP responses even in test mode is to use 
[`--output` option](#getting-started-manual-output) of `[Options]` section: `--output file` allows to save a particular response to a file, while 
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



### Generating Report {#getting-started-running-tests-generating-report}

In the different reports, files are always referenced in the input order (which, as tests are executed in parallel, can 
be different from the execution order).

#### HTML Report {#getting-started-running-tests-html-report}

Hurl can generate an HTML report by using the [`--report-html DIR`](#getting-started-manual-report-html) option.

If the HTML report already exists, the test results will be appended to it.

<div class="picture">
    <img class="u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/hurl-html-report.png" width="670" alt="Hurl HTML Report">
</div>

The input Hurl files (HTML version) are also included and are easily accessed from the main page.

<div class="picture">
    <img class="u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/hurl-html-file.png" width="380" alt="Hurl HTML file">
</div>

#### JSON Report {#getting-started-running-tests-json-report}

A JSON report can be produced by using the [`--report-json DIR`](#getting-started-manual-report-json). The report directory will contain a `report.json` 
file, including each test file executed with [`--json`](#getting-started-manual-json) option and a reference to each HTTP response of the run dumped 
to disk.

If the JSON report already exists, it will be updated with the new test results.

#### JUnit Report {#getting-started-running-tests-junit-report}

A JUnit report can be produced by using the [`--report-junit FILE`](#getting-started-manual-report-junit) option.

If the JUnit report already exists, it will be updated with the new test results.

#### TAP Report {#getting-started-running-tests-tap-report}

A TAP report ([Test Anything Protocol](https://testanything.org)) can be produced by using the [`--report-tap FILE`](#getting-started-manual-report-tap) option.

If the TAP report already exists, it will be updated with the new test results.

### Use Variables in Tests {#getting-started-running-tests-use-variables-in-tests}

To use variables in your tests, you can:

- use [`--variable` option](#getting-started-manual-variable)
- use [`--variables-file` option](#getting-started-manual-variables-file)
- define environment variables, for instance `HURL_foo=bar`

You will find a detailed description in the [Injecting Variables](#file-format-templates-injecting-variables) section of the docs.




<hr>

## Frequently Asked Questions {#getting-started-frequently-asked-questions-frequently-asked-questions}

### General {#getting-started-frequently-asked-questions-general}

#### Why "Hurl"? {#getting-started-frequently-asked-questions-why-hurl}

The name Hurl is a tribute to the awesome [curl](https://curl.haxx.se), with a focus on the HTTP protocol.
While it may have an informal meaning not particularly elegant, [other eminent tools](https://git.wiki.kernel.org/index.php/GitFaq#Why_the_.27Git.27_name.3F) have set a precedent in naming.

#### Yet Another Tool, I already use X {#getting-started-frequently-asked-questions-yet-another-tool-i-already-use-x}

We think that Hurl has some advantages compared to similar tools.

Hurl is foremost a command line tool and should be easy to use on a local computer, or in a CI/CD pipeline. Some
tools in the same space as Hurl ([Postman](https://www.postman.com) for instance), are GUI oriented, and we find it
less attractive than CLI. As a command line tool, Hurl can be used to get HTTP data (like [curl](https://curl.haxx.se)),
but also as a test tool for HTTP sessions, or even as documentation.

Having a text based [file format](#file-format-hurl-file) is another advantage. The Hurl format is simple,
focused on the HTTP domain, can serve as documentation and can be read or written by non-technical people.

For instance posting JSON data with Hurl can be done with this simple file:

``` 
POST http://localhost:3000/api/login
{
  "username": "xyz",
  "password": "xyz"
}
```

With [curl](https://curl.haxx.se):

```
curl --header "Content-Type: application/json" \
     --request POST \
     --data '{"username": "xyz","password": "xyz"}' \
     http://localhost:3000/api/login
``` 


[Karate](https://github.com/intuit/karate), a tool combining API test automation, mocking, performance-testing, has
similar features but offers also much more at a cost of an increased complexity.

Comparing Karate file format:

```
Scenario: create and retrieve a cat

Given url 'http://myhost.com/v1/cats'
And request { name: 'Billie' }
When method post
Then status 201
And match response == { id: '#notnull', name: 'Billie }

Given path response.id
When method get
Then status 200
``` 

And Hurl:

```
# Scenario: create and retrieve a cat

POST http://myhost.com/v1/cats
{ "name": "Billie" }
HTTP 201
[Captures]
cat_id: jsonpath "$.id"
[Asserts]
jsonpath "$.name" == "Billie"

GET http://myshost.com/v1/cats/{{cat_id}}
HTTP 200
```

A key point of Hurl is to work on the HTTP domain. In particular, there is no JavaScript runtime, Hurl works on the
raw HTTP requests/responses, and not on a DOM managed by a HTML engine. For security, this can be seen as a feature:
let's say you want to test backend validation, you want to be able to bypass the browser or javascript validations and
directly test a backend endpoint.

Finally, with no headless browser and working on the raw HTTP data, Hurl is also
really reliable with a very small probability of false positives. Integration tests with tools like
[Selenium](https://www.selenium.dev) can, in this regard, be challenging to maintain.

Just use what is convenient for you. In our case, it's Hurl!

#### Hurl is build on top of libcurl, but what is added? {#getting-started-frequently-asked-questions-hurl-is-build-on-top-of-libcurl-but-what-is-added}

Hurl has two main functionalities on top of [curl](https://curl.haxx.se):

1. Chain several requests:

   With its [captures](#file-format-capturing-response), it enables to inject data received from a response into
   following requests. [CSRF tokens](https://en.wikipedia.org/wiki/Cross-site_request_forgery)
   are typical examples in a standard web session.

2. Test HTTP responses:

   With its [asserts](#file-format-asserting-response), responses can be easily tested.

Hurl benefits from the features of the `libcurl` against it is linked. You can check `libcurl` version with `hurl --version`.

For instance on macOS:

```shell
$ hurl --version
hurl 2.0.0 libcurl/7.79.1 (SecureTransport) LibreSSL/3.3.6 zlib/1.2.11 nghttp2/1.45.1
Features (libcurl):  alt-svc AsynchDNS HSTS HTTP2 IPv6 Largefile libz NTLM NTLM_WB SPNEGO SSL UnixSockets
Features (built-in): brotli
```

You can also check which `libcurl` is used.

On macOS:

```shell
$ which hurl
/opt/homebrew/bin/hurl
$ otool -L /opt/homebrew/bin/hurl:
	/usr/lib/libxml2.2.dylib (compatibility version 10.0.0, current version 10.9.0)
	/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation (compatibility version 150.0.0, current version 1858.112.0)
	/usr/lib/libcurl.4.dylib (compatibility version 7.0.0, current version 9.0.0)
	/usr/lib/libiconv.2.dylib (compatibility version 7.0.0, current version 7.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1311.100.3)
```

On Linux:

```shell
$ which hurl
/root/.cargo/bin/hurl
$ ldd /root/.cargo/bin/hurl
ldd /root/.cargo/bin/hurl
	linux-vdso.so.1 (0x0000ffff8656a000)
	libxml2.so.2 => /usr/lib/aarch64-linux-gnu/libxml2.so.2 (0x0000ffff85fe8000)
	libcurl.so.4 => /usr/lib/aarch64-linux-gnu/libcurl.so.4 (0x0000ffff85f45000)
	libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x0000ffff85f21000)
	...
	libkeyutils.so.1 => /lib/aarch64-linux-gnu/libkeyutils.so.1 (0x0000ffff82ed5000)
	libffi.so.7 => /usr/lib/aarch64-linux-gnu/libffi.so.7 (0x0000ffff82ebc000)
```

Note that some Hurl features are dependent on `libcurl` capacities: for instance, if your `libcurl` doesn't support
HTTP/2 Hurl won't be able to send HTTP/2 request. 


#### Why shouldn't I use Hurl? {#getting-started-frequently-asked-questions-why-shouldnt-i-use-hurl}

If you need a GUI. Currently, Hurl does not offer a GUI version (like [Postman](https://www.postman.com)). While we
think that it can be useful, we prefer to focus for the time-being on the core, keeping something simple and fast.
Contributions to build a GUI are welcome.


#### I have a large numbers of tests, how to run just specific tests? {#getting-started-frequently-asked-questions-i-have-a-large-numbers-of-tests-how-to-run-just-specific-tests}

By convention, you can organize Hurl files into different folders or prefix them.

For example, you can split your tests into two folders critical and additional.

```
critical/test1.hurl
critical/test2.hurl
additional/test1.hurl
additional/test2.hurl
```

You can simply run your critical tests with

```shell
$ hurl --test critical/*.hurl
```

#### How can I use my Hurl files outside Hurl? {#getting-started-frequently-asked-questions-how-can-i-use-my-hurl-files-outside-hurl}

Hurl file can be exported to a JSON file with `hurlfmt`.
This JSON file can then be easily parsed for converting a different format, getting ad-hoc information,...

For example, the Hurl file

```hurl
GET https://example.org/api/users/1
User-Agent: Custom
HTTP 200
[Asserts]
jsonpath "$.name" == "Bob"
```

will be converted to JSON with the following command:

```shell
$ hurlfmt test.hurl --out json | jq
{
  "entries": [
    {
      "request": {
        "method": "GET",
        "url": "https://example.org/api/users/1",
        "headers": [
          {
            "name": "User-Agent",
            "value": "Custom"
          }
        ]
      },
      "response": {
        "version": "HTTP",
        "status": 200,
        "asserts": [
          {
            "query": {
              "type": "jsonpath",
              "expr": "$.name"
            },
            "predicate": {
              "type": "==",
              "value": "Bob"
            }
          }
        ]
      }
    }
  ]
}
```


#### Can I do calculation within a Hurl file? {#getting-started-frequently-asked-questions-can-i-do-calculation-within-a-hurl-file}

Currently, the templating is very simple, only accessing variables.
Calculations can be done beforehand, before running the Hurl File.

For example, with date calculations, variables `now` and `tomorrow` can be used as param or expected value.

```shell
$ TODAY=$(date '+%y%m%d')
$ TOMORROW=$(date '+%y%m%d' -d"+1days")
$ hurl --variable "today=$TODAY" --variable "tomorrow=$TOMORROW" test.hurl
```

You can also use environment variables that begins with `HURL_` to inject data in an Hurl file.
For instance, to inject `today` and `tomorrow` variables:

```shell
$ export HURL_today=$(date '+%y%m%d')
$ export HURL_tomorrow=$(date '+%y%m%d' -d"+1days")
$ hurl test.hurl
```

You can also use [filters](#file-format-filters) to process HTTP responses in asserts and captures.

### macOS {#getting-started-frequently-asked-questions-macos}

#### How can I use a custom libcurl (from Homebrew by instance)? {#getting-started-frequently-asked-questions-how-can-i-use-a-custom-libcurl-from-homebrew-by-instance}

No matter how you've installed Hurl (using the precompiled binary for macOS or with [Homebrew](https://brew.sh))
Hurl is linked against the built-in system libcurl. If you want to use another libcurl (for instance,
if you've installed curl with Homebrew and want Hurl to use Homebrew's libcurl), you can patch Hurl with
the following command:

```shell
$ sudo install_name_tool -change /usr/lib/libcurl.4.dylib PATH_TO_CUSTOM_LIBCURL PATH_TO_HURL_BIN
```

For instance:

```shell
# /usr/local/opt/curl/lib/libcurl.4.dylib is installed by `brew install curl`
$ sudo install_name_tool -change /usr/lib/libcurl.4.dylib /usr/local/opt/curl/lib/libcurl.4.dylib /usr/local/bin/hurl
```



<hr>

# File Format {#file-format}

## Hurl File {#file-format-hurl-file-hurl-file}

### Character Encoding {#file-format-hurl-file-character-encoding}

Hurl file should be encoded in UTF-8, without a byte order mark at the beginning
(while Hurl ignores the presence of a byte order mark rather than treating it as an error)

### File Extension {#file-format-hurl-file-file-extension}

Hurl file extension is `.hurl`

### Comments {#file-format-hurl-file-comments}

Comments begin with `#` and continue until the end of line. Hurl file can serve as
a documentation for HTTP based workflows so it can be useful to be very descriptive.

```hurl
# A very simple Hurl file
# with tasty comments...
GET https://www.sample.net
x-app: MY_APP  # Add a dummy header
HTTP 302       # Check that we have a redirection
[Asserts]
header "Location" exists
header "Location" contains "login"  # Check that we are redirected to the login page
```

### Special Characters in Strings {#file-format-hurl-file-special-characters-in-strings}

String can include the following special characters:

- The escaped special characters \" (double quotation mark), \\ (backslash), \b (backspace), \f (form feed),
 \n (line feed), \r (carriage return), and \t (horizontal tab)
- An arbitrary Unicode scalar value, written as \u{n}, where n is a 1–8 digit hexadecimal number

```hurl
GET https://example.org/api
HTTP 200
# The following assert are equivalent:
[Asserts]
jsonpath "$.slideshow.title" == "A beautiful ✈!"
jsonpath "$.slideshow.title" == "A beautiful \u{2708}!"
```

In some case, (in headers value, etc..), you will also need to escape # to distinguish it from a comment.
In the following example:

```hurl
GET https://example.org/api
x-token: BEEF \#STEACK # Some comment
HTTP 200
```

We're sending a header `x-token` with value `BEEF #STEACK`



<hr>

## Entry {#file-format-entry-entry}

### Definition {#file-format-entry-definition}

A Hurl file is a list of entries, each entry being a mandatory [request](#file-format-request), optionally followed by a [response](#file-format-response).

Responses are not mandatory, a Hurl file consisting only of requests is perfectly valid. To sum up, responses can be used
to [capture values](#file-format-capturing-response) to perform subsequent requests, or [add asserts to HTTP responses](#file-format-asserting-response).

### Example {#file-format-entry-example}

```hurl
# First, test home title.
GET https://acmecorp.net
HTTP 200
[Asserts]
xpath "normalize-space(//head/title)" == "Hello world!"

# Get some news, response description is optional
GET https://acmecorp.net/news

# Do a POST request without CSRF token and check
# that status code is Forbidden 403
POST https://acmecorp.net/contact
[FormParams]
default: false
email: john.doe@rookie.org
number: 33611223344
HTTP 403
```

### Description {#file-format-entry-description}

#### Options {#file-format-entry-options}

[Options](#getting-started-manual-options) specified on the command line apply to every entry in an Hurl file. For instance, with [`--location` option](#getting-started-manual-location),
every entry of a given file will follow redirection:

```shell
$ hurl --location foo.hurl
```

You can use an [`[Options]` section][options](#file-format-request-options) to set option only for a specified request. For instance, in this Hurl file,
the second entry will follow location (so we can test the status code to be 200 instead of 301).

```hurl
GET https://google.fr
HTTP 301

GET https://google.fr
[Options]
location: true
HTTP 200

GET https://google.fr
HTTP 301
```

You can use the `[Options](#getting-started-manual-options)` section to log a specific entry:

```hurl
# ... previous entries

GET https://api.example.org
[Options]
very-verbose: true
HTTP 200

# ... next entries
```

#### Cookie storage {#file-format-entry-cookie-storage}

Requests in the same Hurl file share the cookie storage, enabling, for example, session based scenario.

#### Redirects {#file-format-entry-redirects}

By default, Hurl doesn't follow redirection. To effectively run a redirection, entries should describe each step
of the redirection, allowing insertion of asserts in each response.

```hurl
# First entry, test the redirection (status code and 'Location' header)
GET https://google.fr
HTTP 301
Location: https://www.google.fr/

# Second entry, the 200 OK response
GET https://www.google.fr
HTTP 200
```

Alternatively, one can use [`--location`](#getting-started-manual-location) / [`--location-trusted`](#getting-started-manual-location-trusted) options to force redirection
to be followed. In this case, asserts are executed on the last received response. Optionally, the number of
redirections can be limited with [`--max-redirs`](#getting-started-manual-max-redirs).

```hurl
# Running hurl --location google.hurl
GET https://google.fr
HTTP 200
```

Finally, you can force redirection on a particular request with an [`[Options]` section][options](#file-format-request-options) and the[`--location`](#getting-started-manual-location) 
/ [`--location-trusted`](#getting-started-manual-location-trusted) options:

```hurl
GET https://google.fr
[Options]
location-trusted: true
HTTP 200
```

#### Retry {#file-format-entry-retry}

Every entry can be retried upon asserts, captures or runtime errors. Retries allow polling scenarios and effective runs 
under flaky conditions. Asserts can be explicit (with an [`[Asserts]` section][asserts](#file-format-response-asserts)), or implicit (like [headers](#file-format-response-headers) or [status code](#file-format-response-version-status)).

Retries can be set globally for every request (see [`--retry`](#getting-started-manual-retry) and [`--retry-interval`](#getting-started-manual-retry-interval)), 
or activated on a particular request with an [`[Options]` section][options](#file-format-request-options).

For example, in this Hurl file, first we create a new job then we poll the new job until it's completed:

```hurl
# Create a new job
POST http://api.example.org/jobs
HTTP 201
[Captures]
job_id: jsonpath "$.id"
[Asserts]
jsonpath "$.state" == "RUNNING"


# Pull job status until it is completed
GET http://api.example.org/jobs/{{job_id}}
[Options]
retry: 10   # maximum number of retry, -1 for unlimited
retry-interval: 300ms
HTTP 200
[Asserts]
jsonpath "$.state" == "COMPLETED"
```

#### Control flow {#file-format-entry-control-flow}

In `[Options](#getting-started-manual-options)` section, `skip` and `repeat` can be used to control flow of execution:

- `skip: true/false` skip this request and execute the next one unconditionally,
- `repeat: N` loop the request N times. If there are assert or runtime errors, the requests execution is stopped.

```hurl
# This request will be played exactly 3 times
GET https://example.org/foo
[Options]
repeat: 3
HTTP 200

# This request is skipped
GET https://example.org/foo
[Options]
skip: true
HTTP 200
```

Additionally, a `delay` can be inserted between requests, to add a delay before execution of a request.

```hurl
# A 5 seconds delayed request 
GET https://example.org/foo
[Options]
delay: 5s
HTTP 200
```

[`delay`](#getting-started-manual-retry) and [`repeat`](#getting-started-manual-repeat) can also be used globally as command line options:

```shell
$ hurl --delay 500ms --repeat 3 foo.hurl
```



For complete reference, below is a diagram for the executed entries.

<div class="picture">
    <img class="u-theme-light u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/run-cycle-light.svg" alt="Run cycle explanation"/>
    <img class="u-theme-dark u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/run-cycle-dark.svg" alt="Run cycle explanation"/>
</div>





<hr>

## Request {#file-format-request-request}

### Definition {#file-format-request-definition}

Request describes an HTTP request: a mandatory [method](#file-format-request-method) and [URL](#file-format-request-url), followed by optional [headers](#file-format-request-headers).

Then, [query parameters](#file-format-request-query-parameters), [form parameters](#file-format-request-form-parameters), [multipart form data](#file-format-request-multipart-form-data), [cookies](#file-format-request-cookies), [basic authentication](#file-format-request-basic-authentication) and [options](#file-format-request-options)
can be used to configure the HTTP request.

Finally, an optional [body](#file-format-request-body) can be used to configure the HTTP request body.

### Example {#file-format-request-example}

```hurl
GET https://example.org/api/dogs?id=4567
User-Agent: My User Agent
Content-Type: application/json
[BasicAuth]
alice: secret
```

### Structure {#file-format-request-structure}

<div class="hurl-structure-schema">
  <div class="hurl-structure">
    <div class="hurl-structure-col-0">
        <div class="hurl-part-0">
            PUT https://sample.net
        </div>
        <div class="hurl-part-1">
            accept: */*<br>x-powered-by: Express<br>user-agent: Test
        </div>
        <div class="hurl-part-2">
            [QueryStringParams]<br>...
        </div>
        <div class="hurl-part-2">
            [FormParams]<br>...
        </div>
        <div class="hurl-part-2">
            [BasicAuth]<br>...
        </div>
        <div class="hurl-part-2">
            [Cookies]<br>...
        </div>
        <div class="hurl-part-2">
            ...
        </div>
        <div class="hurl-part-2">
            ...
        </div>
        <div class="hurl-part-3">
            {<br>
            &nbsp;&nbsp;"type": "FOO",<br>
            &nbsp;&nbsp;"value": 356789,<br>
            &nbsp;&nbsp;"ordered": true,<br>
            &nbsp;&nbsp;"index": 10<br>
            }
        </div>
    </div>
    <div class="hurl-structure-col-1">
        <div class="hurl-request-explanation-part-0">
            <a href="#file-format-request-method">Method</a> and <a href="#file-format-request-url">URL</a> (mandatory)
        </div>
        <div class="hurl-request-explanation-part-1">
            <br><a href="#file-format-request-headers">HTTP request headers</a> (optional)
        </div>
        <div class="hurl-request-explanation-part-2">
            <br>
            <br>
            <br>
            <br>
            <br>
        </div>
        <div class="hurl-request-explanation-part-2">
            <a href="#file-format-request-query-parameters">Query strings</a>, <a href="#file-format-request-form-parameters">form params</a>, <a href="#file-format-request-cookies">cookies</a>, <a href="#file-format-request-basic-authentication">authentication</a> ...<br>(optional sections, unordered)
        </div>
        <div class="hurl-request-explanation-part-2">
            <br>
            <br>
            <br>
            <br>
        </div>
        <div class="hurl-request-explanation-part-3">
            <br>
        </div>
        <div class="hurl-request-explanation-part-3">
            <a href="#file-format-request-body">HTTP request body</a> (optional)
        </div>
    </div>
</div>
</div>


[Headers](#file-format-request-headers), if present, follow directly after the [method](#file-format-request-method) and [URL](#file-format-request-url). This allows Hurl format to 'look like' the real HTTP format.
Contrary to HTTP headers, other parameters are defined in sections (`[Cookies]`, `[QueryStringParams]`, `[FormParams]` etc...)
These sections are not ordered and can be mixed in any way:

```hurl
GET https://example.org/api/dogs
User-Agent: My User Agent
[QueryStringParams]
id: 4567
order: newest
[BasicAuth]
alice: secret
```

```hurl
GET https://example.org/api/dogs
User-Agent: My User Agent
[BasicAuth]
alice: secret
[QueryStringParams]
id: 4567
order: newest
```

The last optional part of a request configuration is the request [body](#file-format-request-body). Request body must be the last parameter of a request
(after [headers](#file-format-request-headers) and request sections). Like headers, body have no explicit marker:

```hurl
POST https://example.org/api/dogs?id=4567
User-Agent: My User Agent
{
 "name": "Ralphy"
}
```

### Description {#file-format-request-description}

#### Method {#file-format-request-method}

Mandatory HTTP request method, usually one of `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`,
`TRACE` and `PATCH`. 

> Other methods can be used like `QUERY` with the constraint of using only uppercase chars.

#### URL {#file-format-request-url}

Mandatory HTTP request URL.

URL can contain query parameters, even if using a [query parameters section](#file-format-request-query-parameters) is preferred.

```hurl
# A request with URL containing query parameters.
GET https://example.org/forum/questions/?search=Install%20Linux&order=newest

# A request with query parameters section, equivalent to the first request.
GET https://example.org/forum/questions/
[QueryStringParams]
search: Install Linux
order: newest
```

> Query parameters in query parameter section are not URL encoded.

When query parameters are present in the URL and in a query parameters section, the resulting request will
have both parameters.

#### Headers {#file-format-request-headers}

Optional list of HTTP request headers.

A header consists of a name, followed by a `:` and a value.

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
Accept: */*
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
```

> Headers directly follow URL, without any section name, contrary to query parameters, form parameters
> or cookies

Note that a header usually doesn't start with double quotes. If a header value starts with double quotes, double
quotes will be part of the header value:

```hurl
PATCH https://example.org/file.txt
If-Match: "e0023aa4e"
```

`If-Match` request header will be sent will the following value `"e0023aa4e"` (started and ended with double quotes).

Headers must follow directly after the [method](#file-format-request-method) and [URL](#file-format-request-url).

#### Query parameters {#file-format-request-query-parameters}

Optional list of query parameters.

A query parameter consists of a field, followed by a `:` and a value. The query parameters section starts with
`[QueryStringParams]`. Contrary to query parameters in the URL, each value in the query parameters section is not
URL encoded.

```hurl
GET https://example.org/news
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
[QueryStringParams]
order: newest
search: {{custom-search}}
count: 100
```

If there are any parameters in the URL, the resulted request will have both parameters.

#### Form parameters {#file-format-request-form-parameters}

A form parameters section can be used to send data, like [HTML form](https://developer.mozilla.org/en-US/docs/Learn/Forms).

This section contains an optional list of key values, each key followed by a `:` and a value. Key values will be
encoded in key-value tuple separated by '&', with a '=' between the key and the value, and sent in the body request.
The content type of the request is `application/x-www-form-urlencoded`. The form parameters section starts
with `[FormParams]`.

```hurl
POST https://example.org/contact
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
```

Form parameters section can be seen as syntactic sugar over body section (values in form parameters section
are not URL encoded.). A [oneline string body](#file-format-request-oneline-string-body) could be used instead of a forms parameters section.

~~~hurl
# Run a POST request with form parameters section:
POST https://example.org/test
[FormParams]
name: John Doe
key1: value1

# Run the same POST request with a body section:
POST https://example.org/test
Content-Type: application/x-www-form-urlencoded
`name=John%20Doe&key1=value1`
~~~

When both [body section](#file-format-request-body) and form parameters section are present, only the body section is taken into account.

#### Multipart Form Data {#file-format-request-multipart-form-data}

A multipart form data section can be used to send data, with key / value and file content
(see [multipart/form-data on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)).

The form parameters section starts with `[MultipartFormData]`.

```hurl
POST https://example.org/upload
[MultipartFormData]
field1: value1
field2: file,example.txt;
# One can specify the file content type:
field3: file,example.zip; application/zip
```

Files are relative to the input Hurl file, and cannot contain implicit parent directory (`..`). You can use  
[`--file-root` option](#getting-started-manual-file-root) to specify the root directory of all file nodes.

Content type can be specified or inferred based on the filename extension:

- `.gif`: `image/gif`,
- `.jpg`: `image/jpeg`,
- `.jpeg`: `image/jpeg`,
- `.png`: `image/png`,
- `.svg`: `image/svg+xml`,
- `.txt`: `text/plain`,
- `.htm`: `text/html`,
- `.html`: `text/html`,
- `.pdf`: `application/pdf`,
- `.xml`: `application/xml`

By default, content type is `application/octet-stream`.

As an alternative to a `[MultipartFormData]` section, multipart forms can also be sent with a [multiline string body](#file-format-request-multiline-string-body):

~~~hurl
POST https://example.org/upload
Content-Type: multipart/form-data; boundary="boundary"
```
--boundary
Content-Disposition: form-data; name="key1"

value1
--boundary
Content-Disposition: form-data; name="upload1"; filename="data.txt"
Content-Type: text/plain

Hello World!
--boundary
Content-Disposition: form-data; name="upload2"; filename="data.html"
Content-Type: text/html

<div>Hello <b>World</b>!</div>
--boundary--
```
~~~

> When using a multiline string body to send a multipart form data, files content must be inlined in the Hurl file.


#### Cookies {#file-format-request-cookies}

Optional list of session cookies for this request.

A cookie consists of a name, followed by a `:` and a value. Cookies are sent per request, and are not added to
the cookie storage session, contrary to a cookie set in a header response. (for instance `Set-Cookie: theme=light`). The
cookies section starts with `[Cookies]`.

```hurl
GET https://example.org/index.html
[Cookies]
theme: light
sessionToken: abc123
```

Cookies section can be seen as syntactic sugar over corresponding request header.

```hurl
# Run a GET request with cookies section:
GET https://example.org/index.html
[Cookies]
theme: light
sessionToken: abc123

# Run the same GET request with a header:
GET https://example.org/index.html
Cookie: theme=light; sessionToken=abc123
```

#### Basic Authentication {#file-format-request-basic-authentication}

A basic authentication section can be used to perform [basic authentication](#file-format-request-basic-authentication).

Username is followed by a `:` and a password. The basic authentication section starts with
`[BasicAuth]`. Username and password are _not_ base64 encoded.


```hurl
# Perform basic authentication with login `bob` and password `secret`.
GET https://example.org/protected
[BasicAuth]
bob: secret
```

> Spaces surrounded username and password are trimmed. If you
> really want a space in your password (!!), you could use [Hurl unicode literals \u{20}](#file-format-hurl-file-special-characters-in-strings).

This is equivalent (but simpler) to construct the request with a [Authorization](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization) header:

```hurl
# Authorization header value can be computed with `echo -n 'bob:secret' | base64`
GET https://example.org/protected
Authorization: Basic Ym9iOnNlY3JldA== 
```

Basic authentication allows per request authentication.
If you want to add basic authentication to all the requests of a Hurl file
you can use [`-u/--user` option](#getting-started-manual-user).

#### Body {#file-format-request-body}

Optional HTTP body request.

If the body of the request is a [JSON](https://www.json.org) string or a [XML](https://en.wikipedia.org/wiki/XML) string, the value can be
directly inserted without any modification. For a text based body that is neither JSON nor XML,
one can use [multiline string body](#file-format-request-multiline-string-body) that starts with <code>&#96;&#96;&#96;</code> and ends
with <code>&#96;&#96;&#96;</code>. Multiline string body support "language hint" and can be used
to create [GraphQL queries](#file-format-request-graphql-query).

For a precise byte control of the request body, [Base64](https://en.wikipedia.org/wiki/Base64) encoded string, [hexadecimal string](#file-format-request-hex-body)
or [included file](#file-format-request-file-body) can be used to describe exactly the body byte content.

> You can set a body request even with a `GET` body, even if this is not a common practice.

The body section must be the last section of the request configuration.

##### JSON body {#file-format-request-json-body}

JSON request body is used to set a literal JSON as the request body.

```hurl
# Create a new doggy thing with JSON body:
POST https://example.org/api/dogs
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```

JSON request body can be [templatized with variables](#file-format-templates-templating-body):

```hurl
# Create a new catty thing with JSON body:
POST https://example.org/api/cats
{
    "id": 42,
    "lives": {{ lives_count }},
    "name": "{{ name }}"
}
```


When using JSON request body, the content type `application/json` is automatically set.

JSON request body can be seen as syntactic sugar of [multiline string body](#file-format-request-multiline-string-body) with `json` identifier:

~~~hurl
# Create a new doggy thing with JSON body:
POST https://example.org/api/dogs
```json
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```
~~~



##### XML body {#file-format-request-xml-body}

XML request body is used to set a literal XML as the request body.

~~~hurl
# Create a new soapy thing XML body:
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
Content-Length: 299
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="http://example.net">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
~~~

XML request body can be seen as syntactic sugar of [multiline string body](#file-format-request-multiline-string-body) with `xml` identifier:

~~~hurl
# Create a new soapy thing XML body:
POST https://example.org/InStock
Content-Type: application/soap+xml; charset=utf-8
Content-Length: 299
SOAPAction: "http://www.w3.org/2003/05/soap-envelope"
```xml
<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:m="http://example.net">
  <soap:Header></soap:Header>
  <soap:Body>
    <m:GetStockPrice>
      <m:StockName>GOOG</m:StockName>
    </m:GetStockPrice>
  </soap:Body>
</soap:Envelope>
```
~~~

> Contrary to JSON body, the succinct syntax of XML body can not use variables. If you need to use variables in your
> XML body, use a simple [multiline string body](#file-format-request-multiline-string-body) with variables.

##### GraphQL query {#file-format-request-graphql-query}

GraphQL query uses [multiline string body](#file-format-request-multiline-string-body) with `graphql` identifier:


~~~hurl
POST https://example.org/starwars/graphql
```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```
~~~

GraphQL query body can use [GraphQL variables](https://graphql.org/learn/queries/#variables):

~~~hurl
POST https://example.org/starwars/graphql
```graphql
query Hero($episode: Episode, $withFriends: Boolean!) {
  hero(episode: $episode) {
    name
    friends @include(if: $withFriends) {
      name
    }
  }
}

variables {
  "episode": "JEDI",
  "withFriends": false
}
```
~~~

GraphQL query, as every multiline string body, can use Hurl variables.

~~~hurl
POST https://example.org/starwars/graphql
```graphql
{
  human(id: "{{human_id}}") {
    name
    height(unit: FOOT)
  }
}
```
~~~

> Hurl variables and GraphQL variables can be mixed in the same body.


##### Multiline string body {#file-format-request-multiline-string-body}

For text based body that are neither JSON nor XML, one can use multiline string, started and ending with
<code>&#96;&#96;&#96;</code>.

~~~hurl
POST https://example.org/models
```
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL! air, moon roof, loaded",4799.00
```
~~~

The standard usage of a multiline string is:

~~~
```
line1
line2
line3
```
~~~

is evaluated as "line1\nline2\nline3\n".

Multiline string body can use language identifier, like `json`, `xml` or `graphql`. Depending on the language identifier,
an additional 'Content-Type' request header is sent, and the real body (bytes sent over the wire) can be different from the 
raw multiline text.

~~~hurl
POST https://example.org/api/dogs
```json
{
    "id": 0,
    "name": "Frieda",
}
```
~~~

##### Oneline string body {#file-format-request-oneline-string-body}

For text based body that do not contain newlines, one can use oneline string, started and ending with <code>&#96;</code>.

~~~hurl
POST https://example.org/helloworld
`Hello world!`
~~~


##### Base64 body {#file-format-request-base64-body}

Base64 body is used to set binary data as the request body.

Base64 body starts with `base64,` and end with `;`. MIME's Base64 encoding is supported (newlines and white spaces may be
present anywhere but are to be ignored on decoding), and `=` padding characters might be added.

```hurl
POST https://example.org
# Some random comments before body
base64,TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIG
FkaXBpc2NpbmcgZWxpdC4gSW4gbWFsZXN1YWRhLCBuaXNsIHZlbCBkaWN0dW0g
aGVuZHJlcml0LCBlc3QganVzdG8gYmliZW5kdW0gbWV0dXMsIG5lYyBydXRydW
0gdG9ydG9yIG1hc3NhIGlkIG1ldHVzLiA=;
```

##### Hex body {#file-format-request-hex-body}

Hex body is used to set binary data as the request body.

Hex body starts with `hex,` and end with `;`.

```hurl
PUT https://example.org
# Send a café, encoded in UTF-8
hex,636166c3a90a;
```


##### File body {#file-format-request-file-body}

To use the binary content of a local file as the body request, file body can be used. File body starts with
`file,` and ends with `;``

```hurl
POST https://example.org
# Some random comments before body
file,data.bin;
```

File are relative to the input Hurl file, and cannot contain implicit parent directory (`..`). You can use  
[`--file-root` option](#getting-started-manual-file-root) to specify the root directory of all file nodes.

#### Options {#file-format-request-options}

Options used to execute this request. 

Options such as [`--location`](#getting-started-manual-location), [`--verbose`](#getting-started-manual-verbose), [`--insecure`](#getting-started-manual-insecure) can be used at the command line and applied to every 
request of an Hurl file. An `[Options]` section can be used to apply option to only one request (without passing options 
to the command line), while other requests are unaffected.

```hurl
GET https://example.org
# An options section, each option is optional and applied only to this request...
[Options]
aws-sigv4: aws:amz:sts  # generate AWS SigV4 Authorization header
cacert: /etc/cert.pem   # custom certificate file
compressed: true        # request a compressed response
delay: 3s               # delay for this request
http3: true             # use HTTP/3 protocol version
insecure: true          # allow insecure SSL connections and transfers
ipv6: true              # use IPv6 addresses
location: true          # follow redirection for this request
max-redirs: 10          # maximum number of redirections
output: out.html        # dump the response to this file
path-as-is: true        # do not handle sequences of /../ or /./ in URL path
retry: 10               # number of retry if HTTP/asserts errors
retry-interval: 500ms   # interval between retry
skip: false             # skip this request
unix-socket: sock       # use Unix socket for transfer
user: bob:secret        # use basic authentication
proxy: my.proxy:8012    # define proxy (host:port where host can be an IP address)
variable: country=Italy # define variable country
variable: planet=Earth  # define variable planet
verbose: true           # allow verbose output
very-verbose: true      # allow more verbose output    
```

> Variable defined in an `[Options]` section are defined also for the next entries. This is 
> the exception, all other options are defined only for the current request.




<hr>

## Response {#file-format-response-response}

### Definition {#file-format-response-definition}

Responses can be used to capture values to perform subsequent requests, or add asserts to HTTP responses. Response on
requests are optional, a Hurl file can just consist of a sequence of [requests](#file-format-request).

A response describes the expected HTTP response, with mandatory [version and status](#file-format-asserting-response-version-status), followed by optional [headers](#file-format-asserting-response-headers),
[captures](#file-format-capturing-response-captures), [asserts](#file-format-asserting-response-asserts) and [body](#file-format-asserting-response-body). Assertions in the expected HTTP response describe values of the received HTTP response.
Captures capture values from the received HTTP response and populate a set of named variables that can be used
in the following entries.

### Example {#file-format-response-example}

```hurl
GET https://example.org
HTTP 200
Last-Modified: Wed, 21 Oct 2015 07:28:00 GMT
[Asserts]
xpath "normalize-space(//head/title)" startsWith "Welcome"
xpath "//li" count == 18
```

### Structure {#file-format-response-structure}

<div class="hurl-structure-schema">
  <div class="hurl-structure">
    <div class="hurl-structure-col-0">
        <div class="hurl-part-0">
            HTTP 200
        </div>
        <div class=" hurl-part-1">
            content-length: 206<br>accept-ranges: bytes<br>user-agent: Test
        </div>
        <div class="hurl-part-2">
            [Captures]<br>...
        </div>
        <div class="hurl-part-2">
            [Asserts]<br>...
        </div>
        <div class="hurl-part-3">
            {<br>
            &nbsp;&nbsp;"type": "FOO",<br>
            &nbsp;&nbsp;"value": 356789,<br>
            &nbsp;&nbsp;"ordered": true,<br>
            &nbsp;&nbsp;"index": 10<br>
            }
        </div>
    </div>
    <div class="hurl-structure-col-1">
        <div class="hurl-request-explanation-part-0">
            <a href="#file-format-asserting-response-version-status">Version and status (mandatory if response present)</a>
        </div>
        <div class="hurl-request-explanation-part-1">
            <br><a href="#file-format-asserting-response-headers">HTTP response headers</a> (optional)
        </div>
        <div class="hurl-request-explanation-part-2">
            <br>
            <br>
        </div>
        <div class="hurl-request-explanation-part-2">
            <a href="#file-format-capturing-response-capturing-response">Captures</a> and <a href="#file-format-asserting-response-asserts">asserts</a> (optional sections, unordered)
        </div>
        <div class="hurl-request-explanation-part-2">
          <br>
          <br>
          <br>
          <br>
        </div>
        <div class="hurl-request-explanation-part-3">
            <a href="#file-format-asserting-response-body">HTTP response body</a> (optional)
        </div>
    </div>
</div>
</div>


### Capture and Assertion {#file-format-response-capture-and-assertion}

With the response section, one can optionally [capture value from headers, body](#file-format-capturing-response), or [add assert on status code, body or headers](#file-format-asserting-response).

#### Body compression {#file-format-response-body-compression}

Hurl outputs the raw HTTP body to stdout by default. If response body is compressed (using [br, gzip, deflate](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Encoding)),
the binary stream is output, without any modification. One can use [`--compressed` option](#getting-started-manual-compressed)
to request a compressed response and automatically get the decompressed body.

Captures and asserts work automatically on the decompressed body, so you can request compressed data (using [`Accept-Encoding`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Encoding)
header by example) and add assert and captures on the decoded body as if there weren't any compression.

### Timings {#file-format-response-timings}

HTTP response timings are exposed through Hurl structured output (see [`--json`](#getting-started-manual-json)), HTML report (see [`--report-html`](#getting-started-manual-report-html))
and JSON report (see [`--report-json`](#getting-started-manual-report-json)).

On each response, libcurl response timings are available:

- __time_namelookup__: the time it took from the start until the name resolving was completed. You can use
  [`--resolve`](#getting-started-manual-resolve) to exclude DNS performance from the measure.
- __time_connect__:  The time it took from the start until the TCP connect to the remote host (or proxy) was completed.
- __time_appconnect__: The time it took from the start until the SSL/SSH/etc connect/handshake to the remote host was
  completed. The client is then ready to send its HTTP GET request.
- __time_starttransfer__: The time it took from the start until the first byte was just about to be transferred
  (just before Hurl reads the first byte from the network). This includes time_pretransfer and also the time the server
  needed to calculate the result.
- __time_total__: The total time that the full operation lasted.

All timings are in microsecond.

<div class="picture">
    <img class="u-theme-light u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/timings-light.svg" alt="Response timings explanation"/>
    <img class="u-theme-dark u-drop-shadow u-border u-max-width-100" src="https://hurl.dev/assets/img/timings-dark.svg" alt="Response timings explanation"/>
    <a href="https://blog.cloudflare.com/a-question-of-timing/"><small>Courtesy of CloudFlare</small></a>
</div>





<hr>

## Capturing Response {#file-format-capturing-response-capturing-response}

### Captures {#file-format-capturing-response-captures}

Captures are optional values that are __extracted from the HTTP response__ and stored in a named variable.
These captures may be the response status code, part of or the entire the body, and response headers.

Captured variables can be accessed through a run session; each new value of a given variable overrides the last value.

Captures can be useful for using data from one request in another request, such as when working with [CSRF tokens](https://en.wikipedia.org/wiki/Cross-site_request_forgery).
Variables in a Hurl file can be created from captures or [injected into the session](#file-format-templates-injecting-variables).

```hurl
# An example to show how to pass a CSRF token
# from one request to another:

# First GET request to get CSRF token value:
GET https://example.org
HTTP 200
# Capture the CSRF token value from html body.
[Captures]
csrf_token: xpath "normalize-space(//meta[@name='_csrf_token']/@content)"

# Do the login !
POST https://acmecorp.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

Structure of a capture:

<div class="schema-container schema-container u-font-size-2 u-font-size-3-sm">
 <div class="schema">
   <span class="schema-token schema-color-1">my_var<span class="schema-label">variable</span></span>
   <span> : </span>
   <span class="schema-token schema-color-2">xpath "string(//h1)"<span class="schema-label">query</span></span>
 </div>
</div>

A capture consists of a variable name, followed by `:` and a query. Captures
section starts with `[Captures]`.


#### Query {#file-format-capturing-response-query}

Queries are used to extract data from an HTTP response.

A query can be of the following type:

- [`status`](#file-format-capturing-response-status-capture)
- [`header`](#file-format-capturing-response-header-capture)
- [`url`](#file-format-capturing-response-url-capture)
- [`cookie`](#file-format-capturing-response-cookie-capture)
- [`body`](#file-format-capturing-response-body-capture)
- [`bytes`](#file-format-capturing-response-bytes-capture)
- [`xpath`](#file-format-capturing-response-xpath-capture)
- [`jsonpath`](#file-format-capturing-response-jsonpath-capture)
- [`regex`](#file-format-capturing-response-regex-capture)
- [`variable`](#file-format-capturing-response-variable-capture)
- [`duration`](#file-format-capturing-response-duration-capture)
- [`certificate`](#file-format-capturing-response-certificate-capture)

Extracted data can then be further refined using [filters](#file-format-filters).

#### Status capture {#file-format-capturing-response-status-capture}

Capture the received HTTP response status code. Status capture consists of a variable name, followed by a `:`, and the
keyword `status`.

```hurl
GET https://example.org
HTTP 200
[Captures]
my_status: status
```

#### Header capture {#file-format-capturing-response-header-capture}

Capture a header from the received HTTP response headers. Header capture consists of a variable name, followed by a `:`,
then the keyword `header` and a header name.

```hurl
POST https://example.org/login
[FormParams]
user: toto
password: 12345678
HTTP 302
[Captures]
next_url: header "Location"
```

#### URL capture {#file-format-capturing-response-url-capture}

Capture the last fetched URL. This is most meaningful if you have told Hurl to follow redirection (see [`[Options]` section][options](#file-format-request-options) or
[`--location` option](#getting-started-manual-location)). URL capture consists of a variable name, followed by a `:`, and the keyword `url`.

```hurl
GET https://example.org/redirecting
[Options]
location: true
HTTP 200
[Captures]
landing_url: url
```

#### Cookie capture {#file-format-capturing-response-cookie-capture}

Capture a [`Set-Cookie`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie) header from the received HTTP response headers. Cookie
capture consists of a variable name, followed by a `:`, then the keyword `cookie`
and a cookie name.

```hurl
GET https://example.org/cookies/set
HTTP 200
[Captures]
session-id: cookie "LSID"
```

Cookie attributes value can also be captured by using the following format:
`<cookie-name>[cookie-attribute]`. The following attributes are supported:
`Value`, `Expires`, `Max-Age`, `Domain`, `Path`, `Secure`, `HttpOnly` and `SameSite`.

```hurl
GET https://example.org/cookies/set
HTTP 200
[Captures]
value1: cookie "LSID"
value2: cookie "LSID[Value]"     # Equivalent to the previous capture
expires: cookie "LSID[Expires]"
max-age: cookie "LSID[Max-Age]"
domain: cookie "LSID[Domain]"
path: cookie "LSID[Path]"
secure: cookie "LSID[Secure]"
http-only: cookie "LSID[HttpOnly]"
same-site: cookie "LSID[SameSite]"
```


#### Body capture {#file-format-capturing-response-body-capture}

Capture the entire body (decoded as text) from the received HTTP response. The encoding used to decode the body 
is based on the `charset` value in the `Content-Type` header response.

```hurl
GET https://example.org/home
HTTP 200
[Captures]
my_body: body
```

If the `Content-Type` doesn't include any encoding hint, a [`decode` filter](#file-format-filters-decode) can be used to explicitly decode the body response
bytes.

```hurl
# Our HTML response is encoded using GB 2312.
# But, the 'Content-Type' HTTP response header doesn't precise any charset,
# so we decode explicitly the bytes.
GET https://example.org/cn
HTTP 200
[Captures]
my_body: bytes decode "gb2312"
```


#### Bytes capture {#file-format-capturing-response-bytes-capture}

Capture the entire body (as a raw bytestream) from the received HTTP response

```hurl
GET https://example.org/data.bin
HTTP 200
[Captures]
my_data: bytes
```


#### XPath capture {#file-format-capturing-response-xpath-capture}

Capture a [XPath](https://en.wikipedia.org/wiki/XPath) query from the received HTTP body decoded as a string.
Currently, only XPath 1.0 expression can be used.

```hurl
GET https://example.org/home
# Capture the identifier from the dom node <div id="pet0">5646eaf23</div
HTTP 200
[Captures]
ped-id: xpath "normalize-space(//div[@id='pet0'])"

# Open the captured page.
GET https://example.org/home/pets/{{pet-id}}
HTTP 200
```

XPath captures are not limited to node values (like string, or boolean); any
valid XPath can be captured and asserted with variable asserts.

```hurl
# Test that the XML endpoint return 200 pets
GET https://example.org/api/pets
HTTP 200
[Captures]
pets: xpath "//pets"
[Asserts]
variable "pets" count == 200
```

XPath expression can also be evaluated against part of the body with a [`xpath` filter](#file-format-filters-xpath):

```hurl
GET https://example.org/home_cn
HTTP 200
[Captures]
ped-id: bytes decode "gb2312" xpath "normalize-space(//div[@id='pet0'])"
```


#### JSONPath capture {#file-format-capturing-response-jsonpath-capture}

Capture a [JSONPath](https://goessner.net/articles/JsonPath/) query from the received HTTP body.

```hurl
POST https://example.org/api/contact
[FormParams]
token: {{token}}
email: toto@rookie.net
HTTP 200
[Captures]
contact-id: jsonpath "$['id']"
```

> Explain that the value selected by the JSONPath is coerced to a string when only one node is selected.

As with [XPath captures](#file-format-capturing-response-xpath-capture), JSONPath captures can be anything from string, number, to object and collections.
For instance, if we have a JSON endpoint that returns the following JSON:

```
{
  "a_null": null,
  "an_object": {
    "id": "123"
  },
  "a_list": [
    1,
    2,
    3
  ],
  "an_integer": 1,
  "a float": 1.1,
  "a_bool": true,
  "a_string": "hello"
}
```

We can capture the following paths:

```hurl
GET https://example.org/captures-json
HTTP 200
[Captures]
an_object:  jsonpath "$['an_object']"
a_list:     jsonpath "$['a_list']"
a_null:     jsonpath "$['a_null']"
an_integer: jsonpath "$['an_integer']"
a_float:    jsonpath "$['a_float']"
a_bool:     jsonpath "$['a_bool']"
a_string:   jsonpath "$['a_string']"
all:        jsonpath "$"
```


#### Regex capture {#file-format-capturing-response-regex-capture}

Capture a regex pattern from the HTTP received body, decoded as text.

```hurl
GET https://example.org/helloworld
HTTP 200
[Captures]
id_a: regex "id_a:([0-9]+)"
id_b: regex "id_b:(\\d+)"   # pattern using double quote 
id_c: regex /id_c:(\d+)/    # pattern using forward slash
name: regex "Hello ([a-zA-Z]+)"
```

The regex pattern must have at least one capture group, otherwise the
capture will fail. When the pattern is a double-quoted string, metacharacters beginning with a backslash in the pattern
(like `\d`, `\s`) must be escaped; literal pattern enclosed by `/` can also be used to avoid metacharacters escaping. 


#### Variable capture {#file-format-capturing-response-variable-capture}

Capture the value of a variable into another.

```hurl
GET https://example.org/helloworld
HTTP 200
[Captures]
in: body
name: variable "in"
```

#### Duration capture {#file-format-capturing-response-duration-capture}

Capture the response time of the request in ms.

```hurl
GET https://example.org/helloworld
HTTP 200
[Captures]
duration_in_ms: duration
```

#### SSL certificate capture {#file-format-capturing-response-ssl-certificate-capture}

Capture the SSL certificate properties. Certificate capture consists of the keyword `certificate`, followed by the certificate attribute value.

The following attributes are supported: `Subject`, `Issuer`, `Start-Date`, `Expire-Date` and `Serial-Number`.

```hurl
GET https://example.org
HTTP 200
[Captures]
cert_subject: certificate "Subject"
cert_issuer: certificate "Issuer"
cert_expire_date: certificate "Expire-Date"
cert_serial_number: certificate "Serial-Number"
```




<hr>

## Asserting Response {#file-format-asserting-response-asserting-response}

### Asserts {#file-format-asserting-response-asserts}

Asserts are used to test various properties of an HTTP response. Asserts can be implicits (such as version, status, 
headers) or explicit within an `[Asserts]` section. The delimiter of the request / response is `HTTP <STATUS-CODE>`: 
after this delimiter, you'll find the implicit asserts, then an `[Asserts]` section with all the explicit checks.


```hurl
GET https://api/example.org/cats
HTTP 200
Content-Type: application/json; charset=utf-8      # Implicit assert on Content-Type Header
[Asserts]                                          # Explicit asserts section 
bytes count == 120
header "Content-Type" contains "utf-8"
jsonpath "$.cats" count == 49
jsonpath "$.cats[0].name" == "Felix"
jsonpath "$.cats[0].lives" == 9
```

### Implicit asserts {#file-format-asserting-response-implicit-asserts}

#### Version - Status {#file-format-asserting-response-version-status}

Expected protocol version and status code of the HTTP response.

Protocol version is one of `HTTP/1.0`, `HTTP/1.1`, `HTTP/2`, `HTTP/3` or
`HTTP`; `HTTP` describes any version. Note that there are no status text following the status code.

```hurl
GET https://example.org/404.html
HTTP 404
```

Wildcard keywords `HTTP` and `*` can be used to disable tests on protocol version and status:

```hurl
GET https://example.org/api/pets
HTTP *
# Check that response status code is > 400 and <= 500
[Asserts]
status > 400
status <= 500
```

While `HTTP/1.0`, `HTTP/1.1`, `HTTP/2` and `HTTP/3` explicitly check HTTP version:

```hurl
# Check that our server responds with HTTP/2
GET https://example.org/api/pets
HTTP/2 200 
```


#### Headers {#file-format-asserting-response-headers}

Optional list of the expected HTTP response headers that must be in the received response.

A header consists of a name, followed by a `:` and a value.

For each expected header, the received response headers are checked. If the received header is not equal to the 
expected, or not present, an error is raised. The comparison is case-insensitive for the name: expecting a 
`Content-Type` header is equivalent to a `content-type` one. Note that the expected headers list is not fully 
descriptive: headers present in the response and not in the expected list doesn't raise error.

```hurl
# Check that user toto is redirected to home after login.
POST https://example.org/login
[FormParams]
user: toto
password: 12345678
HTTP 302
Location: https://example.org/home
```

> Quotes in the header value are part of the value itself.
>
> This is used by the [ETag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) Header
> ```
> ETag: W/"<etag_value>"
> ETag: "<etag_value>"
> ```


Testing duplicated headers is also possible.

For example with the `Set-Cookie` header:

```
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

You can either test the two header values:

```hurl
GET https://example.org/index.html
Host: example.net
HTTP 200
Set-Cookie: theme=light
Set-Cookie: sessionToken=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT
```

Or only one:

```hurl
GET https://example.org/index.html 
Host: example.net
HTTP 200
Set-Cookie: theme=light
```

If you want to test specifically the number of headers returned for a given header name, or
if you want to test header value with [predicates](#file-format-asserting-response-predicates) (like `startsWith`, `contains`, `exists`)
you can use the explicit [header assert](#file-format-asserting-response-header-assert).


### Explicit asserts {#file-format-asserting-response-explicit-asserts}

Optional list of assertions on the HTTP response within an `[Asserts]` section. Assertions can describe checks
on status code, on the received body (or part of it) and on response headers.

Structure of an assert:

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">jsonpath "$.book"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">contains<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">"Dune"<span class="schema-label">predicate value</span></span>
 </div>
</div>

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">body<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">matches<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">/\d{4}-\d{2}-\d{2}/<span class="schema-label">predicate value</span></span>
 </div>
</div>


An assert consists of a query followed by a predicate. The format of the query
is shared with [captures](#file-format-capturing-response-query), and can be one of :

- [`status`](#file-format-asserting-response-status-assert)
- [`header`](#file-format-asserting-response-header-assert)
- [`url`](#file-format-asserting-response-url-assert)
- [`cookie`](#file-format-asserting-response-cookie-assert)
- [`body`](#file-format-asserting-response-body-assert)
- [`bytes`](#file-format-asserting-response-bytes-assert)
- [`xpath`](#file-format-asserting-response-xpath-assert)
- [`jsonpath`](#file-format-asserting-response-jsonpath-assert)
- [`regex`](#file-format-asserting-response-regex-assert)
- [`sha256`](#file-format-asserting-response-sha-256-assert)
- [`md5`](#file-format-asserting-response-md5-assert)
- [`variable`](#file-format-asserting-response-variable-assert)
- [`duration`](#file-format-asserting-response-duration-assert)
- [`certificate`](#file-format-asserting-response-ssl-certificate-assert)

Queries are used to extract data from the HTTP response. Queries, in asserts and in captures, can be refined with [filters](#file-format-filters), like 
[`count`][count](#file-format-filters-count) to add tests on collections sizes.


#### Predicates {#file-format-asserting-response-predicates}

Predicates consist of a predicate function and a predicate value. Predicate functions are:

| Predicate            | Description                                                                           | Example                                                                                 |
|----------------------|---------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------|
| __`==`__             | Query and predicate value are equal                                                   | `jsonpath "$.book" == "Dune"`                                                           |
| __`!=`__             | Query and predicate value are different                                               | `jsonpath "$.color" != "red"`                                                           |
| __`>`__              | Query number is greater than predicate value                                          | `jsonpath "$.year" > 1978`                                                              |
| __`>=`__             | Query number is greater than or equal to the predicate value                          | `jsonpath "$.year" >= 1978`                                                             |
| __`<`__              | Query number is less than that predicate value                                        | `jsonpath "$.year" < 1978`                                                              |
| __`<=`__             | Query number is less than or equal to the predicate value                             | `jsonpath "$.year" <= 1978`                                                             |
| __`startsWith`__     | Query starts with the predicate value<br>Value is string or a binary content          | `jsonpath "$.movie" startsWith "The"`<br><br>`bytes startsWith hex,efbbbf;`             |
| __`endsWith`__       | Query ends with the predicate value<br>Value is string or a binary content            | `jsonpath "$.movie" endsWith "Back"`<br><br>`bytes endsWith hex,ab23456;`               |
| __`contains`__       | Query contains the predicate value<br>Value is string or a binary content             | `jsonpath "$.movie" contains "Empire"`<br><br>`bytes contains hex,beef;`                |
| __`includes`__       | Query collections includes the predicate value                                        | `jsonpath "$.nooks" includes "Dune"`                                                    |
| __`matches`__        | Part of the query string matches the regex pattern described by the predicate value   | `jsonpath "$.release" matches "\\d{4}"`<br><br>`jsonpath "$.release" matches /\d{4}/`   |
| __`exists`__         | Query returns a value                                                                 | `jsonpath "$.book" exists`                                                              |
| __`isBoolean`__      | Query returns a boolean                                                               | `jsonpath "$.succeeded" isBoolean`                                                      |
| __`isCollection`__   | Query returns a collection                                                            | `jsonpath "$.books" isCollection`                                                       |
| __`isEmpty`__        | Query returns an empty collection                                                     | `jsonpath "$.movies" isEmpty`                                                           |
| __`isFloat`__        | Query returns a float                                                                 | `jsonpath "$.height" isFloat`                                                           |
| __`isInteger`__      | Query returns an integer                                                              | `jsonpath "$.count" isInteger`                                                          |
| __`isIsoDate`__      | Query string returns a [RFC 3339] date (`YYYY-MM-DDTHH:mm:ss.sssZ`)                   | `jsonpath "$.publication_date" isIsoDate`                                               |
| __`isNumber`__       | Query returns an integer or a float                                                   | `jsonpath "$.count" isNumber`                                                           |
| __`isString`__       | Query returns a string                                                                | `jsonpath "$.name" isString`                                                            |


Each predicate can be negated by prefixing it with `not` (for instance, `not contains` or `not exists`)

<div class="schema-container schema-container u-font-size-1 u-font-size-2-sm u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">jsonpath "$.book"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-1">not contains<span class="schema-label">predicate type</span></span>
   <span class="schema-token schema-color-3">"Dune"<span class="schema-label">predicate value</span></span>
 </div>
</div>


A predicate value is typed, and can be a string, a boolean, a number, a bytestream, `null` or a collection. Note that
`"true"` is a string, whereas `true` is a boolean.

For instance, to test the presence of a h1 node in an HTML response, the following assert can be used:

```hurl
GET https://example.org/home
HTTP 200
[Asserts]
xpath "boolean(count(//h1))" == true
xpath "//h1" exists # Equivalent but simpler
```

As the XPath query `boolean(count(//h1))` returns a boolean, the predicate value in the assert must be either
`true` or `false` without double quotes. On the other side, say you have an article node and you want to check the value of some
[data attributes](https://developer.mozilla.org/en-US/docs/Learn/HTML/Howto/Use_data_attributes):

```xml
<article
  id="electric-cars"
  data-visible="true"
...
</article>
```

The following assert will check the value of the `data-visible` attribute:

```hurl
GET https://example.org/home
HTTP 200
[Asserts]
xpath "string(//article/@data-visible)" == "true"
```

In this case, the XPath query `string(//article/@data-visible)` returns a string, so the predicate value must be a
string.

The predicate function `==` can be used with string, numbers or booleans; `startWith` and `contains` can only
be used with strings and bytes, while `matches` only works on string. If a query returns a number, using a `matches` predicate will cause a runner error.

```hurl
# A really well tested web page...
GET https://example.org/home
HTTP 200
[Asserts]
header "Content-Type" contains "text/html"
header "Last-Modified" == "Wed, 21 Oct 2015 07:28:00 GMT"
xpath "//h1" exists  # Check we've at least one h1
xpath "normalize-space(//h1)" contains "Welcome"
xpath "//h2" count == 13
xpath "string(//article/@data-id)" startsWith "electric"
```

#### Status assert {#file-format-asserting-response-status-assert}

Check the received HTTP response status code. Status assert consists of the keyword `status` followed by a predicate
function and value.

```hurl
GET https://example.org
HTTP *
[Asserts]
status < 300
```

#### Header assert {#file-format-asserting-response-header-assert}

Check the value of a received HTTP response header. Header assert consists of the keyword `header` followed by the value
of the header, a predicate function and a predicate value. Like [headers implicit asserts](#file-format-asserting-response-headers), the check is 
case-insensitive for the name: comparing a `Content-Type` header is equivalent to a `content-type` one.

```hurl
GET https://example.org
HTTP 302
[Asserts]
header "Location" contains "www.example.net"
header "Last-Modified" matches /\d{2} [a-z-A-Z]{3} \d{4}/
```

If there are multiple headers with the same name, the header assert returns a collection, so `count`, `includes` can be
used in this case to test the header list.

Let's say we have this request and response:

```
> GET /hello HTTP/1.1
> Host: example.org
> Accept: */*
> User-Agent: hurl/2.0.0-SNAPSHOT
>
* Response: (received 12 bytes in 11 ms)
*
< HTTP/1.0 200 OK
< Vary: Content-Type
< Vary: User-Agent
< Content-Type: text/html; charset=utf-8
< Content-Length: 12
< Server: Flask Server
< Date: Fri, 07 Oct 2022 20:53:35 GMT
```

One can use explicit header asserts:

```hurl
GET https://example.org/hello
HTTP 200
[Asserts]
header "Vary" count == 2
header "Vary" includes "User-Agent"
header "Vary" includes "Content-Type"
```

Or implicit header asserts:

```hurl
GET https://example.org/hello
HTTP 200
Vary: User-Agent
Vary: Content-Type
```

#### URL assert {#file-format-asserting-response-url-assert}

Check the last fetched URL. This is most meaningful if you have told Hurl to follow redirection (see [`[Options]`section][options](#file-format-request-options) or
[`--location` option](#getting-started-manual-location)). URL assert consists of the keyword `url` followed by a predicate function and value.

```hurl
GET https://example.org/redirecting
[Options]
location: true
HTTP 200
[Asserts]
url == "https://example.org/redirected"
```


#### Cookie assert {#file-format-asserting-response-cookie-assert}

Check value or attributes of a [`Set-Cookie`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie) response header. Cookie assert
consists of the keyword `cookie`, followed by the cookie name (and optionally a
cookie attribute), a predicate function and value.

Cookie attributes value can be checked by using the following format:
`<cookie-name>[cookie-attribute]`. The following attributes are supported: `Value`,
`Expires`, `Max-Age`, `Domain`, `Path`, `Secure`, `HttpOnly` and `SameSite`.

```hurl
GET http://localhost:8000/cookies/set
HTTP 200

# Explicit check of Set-Cookie header value. If the attributes are
# not in this exact order, this assert will fail. 
Set-Cookie: LSID=DQAAAKEaem_vYg; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/accounts; SameSite=Lax;
Set-Cookie: HSID=AYQEVnDKrdst; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly; Path=/
Set-Cookie: SSID=Ap4PGTEq; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/

# Using cookie assert, one can check cookie value and various attributes.
[Asserts]
cookie "LSID" == "DQAAAKEaem_vYg"
cookie "LSID[Value]" == "DQAAAKEaem_vYg"
cookie "LSID[Expires]" exists
cookie "LSID[Expires]" contains "Wed, 13 Jan 2021"
cookie "LSID[Max-Age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[Secure]" exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[SameSite]" == "Lax"
```

> `Secure` and `HttpOnly` attributes can only be tested with `exists` or `not exists` predicates
> to reflect the [Set-Cookie header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie) semantics (in other words, queries `<cookie-name>[HttpOnly]`
> and `<cookie-name>[Secure]` don't return boolean).

#### Body assert {#file-format-asserting-response-body-assert}

Check the value of the received HTTP response body when decoded as a string.
Body assert consists of the keyword `body` followed by a predicate function and
value. The encoding used to decode the body is based on the `charset` value in the
`Content-Type` header response.

```hurl
GET https://example.org
HTTP 200
[Asserts]
body contains "<h1>Welcome!</h1>"
```

```hurl
# Our HTML response is encoded with GB 2312 (see https://en.wikipedia.org/wiki/GB_2312)
GET https://example.org/cn
HTTP 200
[Asserts]
header "Content-Type" == "text/html; charset=gb2312"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB 2312
body contains "你好世界"
```

If the `Content-Type` doesn't include any encoding hint, a [`decode` filter](#file-format-filters-decode) can be used to explicitly decode the body response
bytes.

```hurl
# Our HTML response is encoded using GB 2312.
# But, the 'Content-Type' HTTP response header doesn't precise any charset,
# so we decode explicitly the bytes.
GET https://example.org/cn
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB2312
bytes decode "gb2312" contains "你好世界"
```

#### Bytes assert {#file-format-asserting-response-bytes-assert}

Check the value of the received HTTP response body as a bytestream. Body assert
consists of the keyword `bytes` followed by a predicate function and value.

```hurl
GET https://example.org/data.bin
HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
bytes count == 12424
header "Content-Length" == "12424"
```

#### XPath assert {#file-format-asserting-response-xpath-assert}

Check the value of a [XPath](https://en.wikipedia.org/wiki/XPath) query on the received HTTP body decoded as a string (using the `charset` value in the
`Content-Type` header response). Currently, only XPath 1.0 expression can be used. Body assert consists of the
keyword `xpath` followed by a predicate function and value. Values can be string,
boolean or number depending on the XPath query.

Let's say we want to check this HTML response:

```plain
$ curl -v https://example.org

< HTTP/1.1 200 OK
< Content-Type: text/html; charset=UTF-8
...
<!doctype html>
<html>
  <head>
    <title>Example Domain</title>
    ...
  </head>

  <body>
    <div>
      <h1>Example</h1>
      <p>This domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.</p>
      <p><a href="https://www.iana.org/domains/example">More information...</a></p>
    </div>
  </body>
</html>
```

With Hurl, we can write multiple XPath asserts describing the DOM content:

```hurl
GET https://example.org
HTTP 200
Content-Type: text/html; charset=UTF-8
[Asserts]
xpath "string(/html/head/title)" contains "Example" # Check title
xpath "count(//p)" == 2                             # Check the number of <p>
xpath "//p" count == 2                              # Similar assert for <p>
xpath "boolean(count(//h2))" == false               # Check there is no <h2>  
xpath "//h2" not exists                             # Similar assert for <h2> 
```

XML Namespaces are also supported. Let's say you want to check this XML response:

```xml
<?xml version="1.0"?>
<!-- both namespace prefixes are available throughout -->
<bk:book xmlns:bk='urn:loc.gov:books'
         xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <bk:title>Cheaper by the Dozen</bk:title>
    <isbn:number>1568491379</isbn:number>
</bk:book>
```

This XML response can be tested with the following Hurl file:

```hurl
GET http://localhost:8000/assert-xpath
HTTP 200
[Asserts]

xpath "string(//bk:book/bk:title)" == "Cheaper by the Dozen"
xpath "string(//*[name()='bk:book']/*[name()='bk:title'])" == "Cheaper by the Dozen"
xpath "string(//*[local-name()='book']/*[local-name()='title'])" == "Cheaper by the Dozen"

xpath "string(//bk:book/isbn:number)" == "1568491379"
xpath "string(//*[name()='bk:book']/*[name()='isbn:number'])" == "1568491379"
xpath "string(//*[local-name()='book']/*[local-name()='number'])" == "1568491379"
```

The XPath expressions `string(//bk:book/bk:title)` and `string(//bk:book/isbn:number)` are written with `bk` and `isbn`
namespaces.

> For convenience, the first default namespace can be used with `_`


#### JSONPath assert {#file-format-asserting-response-jsonpath-assert}

Check the value of a [JSONPath](https://goessner.net/articles/JsonPath/) query on the received HTTP body decoded as a JSON
document. JSONPath assert consists of the keyword `jsonpath` followed by a predicate
function and value.

Let's say we want to check this JSON response:

```plain
curl -v http://httpbin.org/json

< HTTP/1.1 200 OK
< Content-Type: application/json
...

{
  "slideshow": {
    "author": "Yours Truly",
    "date": "date of publication",
    "slides": [
      {
        "title": "Wake up to WonderWidgets!",
        "type": "all"
      },
       ...
    ],
    "title": "Sample Slide Show"
  }
}
```

With Hurl, we can write multiple JSONPath asserts describing the DOM content:


```hurl
GET http://httpbin.org/json
HTTP 200
[Asserts]
jsonpath "$.slideshow.author" == "Yours Truly"
jsonpath "$.slideshow.slides[0].title" contains "Wonder"
jsonpath "$.slideshow.slides" count == 2
jsonpath "$.slideshow.date" != null
jsonpath "$.slideshow.slides[*].title" includes "Mind Blowing!"
```

> Explain that the value selected by the JSONPath is coerced to a string when only
> one node is selected.

In `matches` predicates, metacharacters beginning with a backslash (like `\d`, `\s`) must be escaped.
Alternatively, `matches` predicate support [JavaScript-like Regular expression syntax](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions) to enhance
the readability:

```hurl
GET https://sample.org/hello
HTTP 200
[Asserts]

# Predicate value with matches predicate:
jsonpath "$.date" matches "^\\d{4}-\\d{2}-\\d{2}$"
jsonpath "$.name" matches "Hello [a-zA-Z]+!"

# Equivalent syntax:
jsonpath "$.date" matches /^\d{4}-\d{2}-\d{2}$/
jsonpath "$.name" matches /Hello [a-zA-Z]+!/
```

#### Regex assert {#file-format-asserting-response-regex-assert}

Check that the HTTP received body, decoded as text, matches a regex pattern.

```hurl
GET https://sample.org/hello
HTTP 200
[Asserts]
regex "^(\\d{4}-\\d{2}-\\d{2})$" == "2018-12-31"
# Same assert as previous using regex literals
regex /^(\d{4}-\d{2}-\d{2})$/ == "2018-12-31"
```

The regex pattern must have at least one capture group, otherwise the
assert will fail. The assertion is done on the captured group value. When the regex pattern is a double-quoted string, 
metacharacters beginning with a backslash in the pattern (like `\d`, `\s`) must be escaped; literal pattern enclosed by
`/` can also be used to avoid metacharacters escaping.


#### SHA-256 assert {#file-format-asserting-response-sha-256-assert}

Check response body [SHA-256](https://en.wikipedia.org/wiki/SHA-2) hash.

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
```

#### MD5 assert {#file-format-asserting-response-md5-assert}

Check response body [MD5](https://en.wikipedia.org/wiki/MD5) hash.

```hurl
GET https://example.org/data.tar.gz
HTTP 200
[Asserts]
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```


#### Variable assert {#file-format-asserting-response-variable-assert}

```hurl
# Test that the XML endpoint return 200 pets 
GET https://example.org/api/pets
HTTP 200
[Captures]
pets: xpath "//pets"
[Asserts]
variable "pets" count == 200
```

#### Duration assert {#file-format-asserting-response-duration-assert}

Check the total duration (sending plus receiving time) of the HTTP transaction.

```hurl
GET https://sample.org/helloworld
HTTP 200
[Asserts]
duration < 1000   # Check that response time is less than one second
```

#### SSL certificate assert {#file-format-asserting-response-ssl-certificate-assert}

Check the SSL certificate properties. Certificate assert consists of the keyword `certificate`, followed by the certificate attribute value.

The following attributes are supported: `Subject`, `Issuer`, `Start-Date`, `Expire-Date` and `Serial-Number`.

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Subject" == "CN=example.org"
certificate "Issuer" == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches "[0-9af]+"
```

### Body {#file-format-asserting-response-body}

Optional assertion on the received HTTP response body. Body section can be seen
as syntactic sugar over [body asserts](#file-format-asserting-response-body-assert) (with `==` predicate). If the
body of the response is a [JSON](https://www.json.org) string or a [XML](https://en.wikipedia.org/wiki/XML) string, the body assertion can
be directly inserted without any modification. For a text based body that is neither JSON nor XML,
one can use multiline string that starts with <code>&#96;&#96;&#96;</code> and ends
with <code>&#96;&#96;&#96;</code>. For a precise byte control of the response body,
a [Base64](https://en.wikipedia.org/wiki/Base64) encoded string or an input file can be used to describe exactly
the body byte content to check.

#### JSON body {#file-format-asserting-response-json-body}

```hurl
# Get a doggy thing:
GET https://example.org/api/dogs/{{dog-id}}
HTTP 200
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```

JSON response body can be seen as syntactic sugar of [multiline string body](#file-format-asserting-response-multiline-string-body) with `json` identifier:

~~~hurl
# Get a doggy thing:
GET https://example.org/api/dogs/{{dog-id}}
HTTP 200
```json
{
    "id": 0,
    "name": "Frieda",
    "picture": "images/scottish-terrier.jpeg",
    "age": 3,
    "breed": "Scottish Terrier",
    "location": "Lisco, Alabama"
}
```
~~~


#### XML body {#file-format-asserting-response-xml-body}

~~~hurl
GET https://example.org/api/catalog
HTTP 200
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <book id="bk101">
      <author>Gambardella, Matthew</author>
      <title>XML Developer's Guide</title>
      <genre>Computer</genre>
      <price>44.95</price>
      <publish_date>2000-10-01</publish_date>
      <description>An in-depth look at creating applications with XML.</description>
   </book>
</catalog>
~~~

XML response body can be seen as syntactic sugar of [multiline string body](#file-format-asserting-response-multiline-string-body) with `xml` identifier:

~~~hurl
GET https://example.org/api/catalog
HTTP 200
```xml
<?xml version="1.0" encoding="UTF-8"?>
<catalog>
   <book id="bk101">
      <author>Gambardella, Matthew</author>
      <title>XML Developer's Guide</title>
      <genre>Computer</genre>
      <price>44.95</price>
      <publish_date>2000-10-01</publish_date>
      <description>An in-depth look at creating applications with XML.</description>
   </book>
</catalog>
```
~~~


#### Multiline string body {#file-format-asserting-response-multiline-string-body}

~~~hurl
GET https://example.org/models
HTTP 200
```
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL! air, moon roof, loaded",4799.00
```
~~~

The standard usage of a multiline string is :

~~~
```
line1
line2
line3
```
~~~

##### Oneline string body {#file-format-asserting-response-oneline-string-body}

For text based response body that do not contain newlines, one can use oneline string, started and ending with <code>&#96;</code>.

~~~hurl
POST https://example.org/helloworld
HTTP 200
`Hello world!`
~~~


#### Base64 body {#file-format-asserting-response-base64-body}

Base64 response body assert starts with `base64,` and end with `;`. MIME's Base64 encoding
is supported (newlines and white spaces may be present anywhere but are to be
ignored on decoding), and `=` padding characters might be added.

```hurl
GET https://example.org
HTTP 200
base64,TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIG
FkaXBpc2NpbmcgZWxpdC4gSW4gbWFsZXN1YWRhLCBuaXNsIHZlbCBkaWN0dW0g
aGVuZHJlcml0LCBlc3QganVzdG8gYmliZW5kdW0gbWV0dXMsIG5lYyBydXRydW
0gdG9ydG9yIG1hc3NhIGlkIG1ldHVzLiA=;
```

#### File body {#file-format-asserting-response-file-body}

To use the binary content of a local file as the body response assert, file body
can be used. File body starts with `file,` and ends with `;``

```hurl
GET https://example.org
HTTP 200
file,data.bin;
```

File are relative to the input Hurl file, and cannot contain implicit parent
directory (`..`). You can use [`--file-root` option](#getting-started-manual-file-root) to specify the root directory
of all file nodes.




<hr>

## Filters {#file-format-filters-filters}

### Definition {#file-format-filters-definition}

[Captures](#file-format-capturing-response) and [asserts](#file-format-asserting-response) share a common structure: query. A query is used to extract data from an HTTP response; this data 
can come from the HTTP response body, the HTTP response headers or from the HTTP meta-information (like `duration` for instance)...

In this example, the query __`jsonpath "$.books[0].name"`__ is used in a capture to save data and in an assert to test 
the HTTP response body.

__Capture__:

<div class="schema-container schema-container u-font-size-2 u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-1">name<span class="schema-label">variable</span></span>
   <span> : </span>
   <span class="schema-token schema-color-2">jsonpath "$.books[0].name"<span class="schema-label">query</span></span>
 </div>
</div>

__Assert__:

<div class="schema-container schema-container u-font-size-2 u-font-size-3-md">
 <div class="schema">
   <span class="schema-token schema-color-2">jsonpath "$.books[0].name"<span class="schema-label">query</span></span>
   <span class="schema-token schema-color-3">== "Dune"<span class="schema-label">predicate</span></span>
 </div>
</div>

In both case, the query is exactly the same: queries are the core structure of asserts and captures. Sometimes, you want
to process data extracted by queries: that's the purpose of __filters__.

Filters are used to transform value extracted by a query and can be used in asserts and captures to refine data. Filters 
__can be chained__, allowing for fine-grained data extraction. 


<div class="schema-container schema-container u-font-size-2 u-font-size-3-md">
 <div class="schema">
    <span class="schema-token schema-color-2">jsonpath "$.name"<span class="schema-label">query</span></span>
    <span class="schema-token schema-color-1">split "," nth 0<span class="schema-label">2 filters</span></span>
    <span class="schema-token schema-color-3">== "Herbert"<span class="schema-label">predicate</span></span>
 </div>
</div>


### Example {#file-format-filters-example}

```hurl
GET https://example.org/api
HTTP 200
[Captures]
name: jsonpath "$user.id" replace /\d/ "x"
[Asserts]
header "x-servers" split "," count == 2
header "x-servers" split "," nth 0 == "rec1"
header "x-servers" split "," nth 1 == "rec3"
jsonpath "$.books" count == 12
```

### Description {#file-format-filters-description}

#### count {#file-format-filters-count}

Counts the number of items in a collection.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.books" count == 12
```

#### daysAfterNow {#file-format-filters-daysafternow}

Returns the number of days between now and a date in the future.

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Expire-Date" daysAfterNow > 15
```

#### daysBeforeNow {#file-format-filters-daysbeforenow}

Returns the number of days between now and a date in the past.

```hurl
GET https://example.org
HTTP 200
[Asserts]
certificate "Start-Date" daysBeforeNow < 100
```

#### decode {#file-format-filters-decode}

Decode bytes to string using encoding.

```hurl
# The 'Content-Type' HTTP response header does not precise the charset 'gb2312'
# so body must be decoded explicitly by Hurl before processing any text based assert
GET https://exapple.org/hello_china
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
# Content-Type has no encoding clue, we must decode ourselves the body response.
bytes decode "gb2312" xpath "string(//body)" == "你好世界"
```

#### format {#file-format-filters-format}

Formats a date to a string given [a specification format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

```hurl
GET https://example.org
HTTP 200
[Asserts]
cookie "LSID[Expires]" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
```

#### htmlEscape {#file-format-filters-htmlescape}

Converts the characters `&`, `<` and `>` to HTML-safe sequence.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.text" htmlEscape == "a &gt; b"
```

#### htmlUnescape {#file-format-filters-htmlunescape}

Converts all named and numeric character references (e.g. `&gt;`, `&#62;`, `&#x3e;`) to the corresponding Unicode characters.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.escaped_html[1]" htmlUnescape == "Foo © bar 𝌆"
```

#### jsonpath  {#file-format-filters-jsonpath}

Evaluates a [JSONPath](https://goessner.net/articles/JsonPath/) expression.

```hurl
GET https://example.org/api
HTTP 200
[Captures]
books: xpath "string(//body/@data-books)" 
[Asserts]
variable "books" jsonpath "$[0].name" == "Dune"
variable "books" jsonpath "$[0].author" == "Franck Herbert"
```


#### nth {#file-format-filters-nth}

Returns the element from a collection at a zero-based index.

```hurl
GET https://example.org/api
HTTP 200
[Asserts]
jsonpath "$.books" nth 2 == "Children of Dune"
```

#### regex {#file-format-filters-regex}

Extracts regex capture group. Pattern must have at least one capture group.

```hurl
GET https://example.org/foo
HTTP 200
[Captures]
param1: header "header1"
param2: header "header2" regex "Hello (.*)!"
param3: header "header2" regex /Hello (.*)!/
```

#### replace {#file-format-filters-replace}

Replaces all occurrences of old string with new string.

```hurl
GET https://example.org/foo
HTTP 200
[Captures]
url: jsonpath "$.url" replace "http://" "https://"
[Asserts]
jsonpath "$.ips" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"
```

#### split {#file-format-filters-split}

Splits to a list of strings around occurrences of the specified delimiter.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.ips" split ", " count == 3
```

#### toDate {#file-format-filters-todate}

Converts a string to a date given [a specification format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

```hurl
GET https:///example.org
HTTP 200
[Asserts]
header "Expires" toDate "%a, %d %b %Y %H:%M:%S GMT" daysBeforeNow > 1000
```


ISO 8601 / RFC 3339 date and time format have shorthand format `%+`:

```hurl
GET https://example.org/api/books
HTTP 200
[Asserts]
jsonpath "$.published" == "2023-01-23T18:25:43.511Z"
jsonpath "$.published" toDate "%Y-%m-%dT%H:%M:%S%.fZ" format "%A" == "Monday"
jsonpath "$.published" toDate "%+" format "%A" == "Monday" # %+ can be used to parse ISO 8601 / RFC 3339
```

#### toFloat {#file-format-filters-tofloat}

Converts to float number.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.pi" toFloat == 3.14
```

#### toInt {#file-format-filters-toint}

Converts to integer number.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.id" toInt == 123
```

#### urlDecode {#file-format-filters-urldecode}

Replaces %xx escapes with their single-character equivalent.

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.encoded_url" urlDecode == "https://mozilla.org/?x=шеллы"
```

#### urlEncode {#file-format-filters-urlencode}

Percent-encodes all the characters which are not included in unreserved chars (see [RFC3986](https://www.rfc-editor.org/rfc/rfc3986)) with the exception of forward slash (/).

```hurl
GET https://example.org/foo
HTTP 200
[Asserts]
jsonpath "$.url" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
```

#### xpath {#file-format-filters-xpath}

Evaluates a [XPath](https://en.wikipedia.org/wiki/XPath) expression.

```hurl
GET https://example.org/hello_gb2312
HTTP 200
[Asserts]
bytes decode "gb2312" xpath "string(//body)" == "你好世界"
```




<hr>

## Templates {#file-format-templates-templates}

### Variables {#file-format-templates-variables}

In Hurl file, you can generate value using two curly braces, i.e `{{my_variable}}`. For instance, if you want to reuse a
value from an HTTP response in the next entries, you can capture this value in a variable and reuse it in a template.

```hurl
GET https://example.org

HTTP 200
[Captures]
csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"


# Do the login !
POST https://acmecorp.net/login?user=toto&password=1234
X-CSRF-TOKEN: {{csrf_token}}
HTTP 302
```

In this example, we capture the value of the [CSRF token](https://en.wikipedia.org/wiki/Cross-site_request_forgery) from the body of the first response, and inject it
as a header in the next POST request.

```hurl
GET https://example.org/api/index

HTTP 200
[Captures]
index: body


GET https://example.org/api/status

HTTP 200
[Asserts]
jsonpath "$.errors[{{index}}].id" == "error"
```

In this second example, we capture the body in a variable `index`, and reuse this value in the query
`jsonpath "$.errors[{{index}}].id"`.

### Types {#file-format-templates-types}

Variables are typed, and can be either string, bool, number, `null` or collections. Depending on the variable type,
templates can be rendered differently. Let's say we have captured an integer value into a variable named
`count`:

```hurl
GET https://sample/counter

HTTP 200
[Captures]
count: jsonpath "$.results[0]"
```

The following entry:

```hurl
GET https://sample/counter/{{count}} 

HTTP 200
[Asserts]
jsonpath "$.id" == "{{count}}"
```

will be rendered at runtime to:

```hurl
GET https://sample/counter/458
 
HTTP 200
[Asserts]
jsonpath "$.id" == "458"
```

resulting in a comparison between the [JSONPath](#file-format-asserting-response-jsonpath-assert) expression and a string value.

On the other hand, the following assert:

```hurl
GET https://sample/counter/{{count}} 

HTTP 200
[Asserts]
jsonpath "$.index" == {{count}}
```

will be rendered at runtime to:

```hurl
GET https://sample/counter/458 

HTTP 200
[Asserts]
jsonpath "$.index" == 458
```

resulting in a comparison between the [JSONPath](#file-format-asserting-response-jsonpath-assert) expression and an integer value.

So if you want to use typed values (in asserts for instances), you can use `{{my_var}}`.
If you're interested in the string representation of a variable, you can surround the variable with double quotes
, as in `"{{my_var}}"`.

> When there is no possible ambiguities, like using a variable in an URL, or
> in a header, you can omit the double quotes. The value will always be rendered
> as a string.

### Injecting Variables {#file-format-templates-injecting-variables}

Variables can also be injected in a Hurl file:

- by using [`--variable` option](#getting-started-manual-variable)
- by using [`--variables-file` option](#getting-started-manual-variables-file)
- by defining environment variables, for instance `HURL_foo=bar`
- by defining variables in an [`[Options]` section][options](#file-format-request-options)

Lets' see how to inject variables, given this `test.hurl`:

```hurl
GET https://{{host}}/{{id}}/status
HTTP 304

GET https://{{host}}/health
HTTP 200
```

#### `variable` option {#file-format-templates-variable-option}

Variable can be defined with command line option:

```shell
$ hurl --variable host=example.net --variable id=1234 test.hurl
``` 


#### `variables-file` option {#file-format-templates-variables-file-option}

We can also define all injected variables in a file:

```shell
$ hurl --variables-file vars.env test.hurl
``` 

where `vars.env` is

```
host=example.net
id=1234
```

#### Environment variable {#file-format-templates-environment-variable}

We can use environment variables in the form of `HURL_name=value`:

```shell
$ export HURL_host=example.net
$ export HURL_id=1234 
$ hurl test.hurl
```

#### Options sections {#file-format-templates-options-sections}

We can define variables in `[Options]` section. Variables defined in a section are available for the next requests.

```hurl
GET https://{{host}}/{{id}}/status
[Options]
variable: host=example.net
variable: id=1234
HTTP 304

GET https://{{host}}/health
HTTP 200
```


### Templating Body {#file-format-templates-templating-body}

Variables can be used in [JSON body](#file-format-request-json-body):

~~~hurl
PUT https://example.org/api/hits
{
    "key0": "{{a_string}}",
    "key1": {{a_bool}},
    "key2": {{a_null}},
    "key3": {{a_number}}
}
~~~

Note that [XML body](#file-format-request-xml-body) can't use variables directly, for the moment. In order to templatize a XML body, you can use 
[multiline string body](#file-format-request-multiline-string-body) with variables. The multiline string body allows to templatize any text based body (JSON, XML, 
CSV etc...):

~~~hurl
PUT https://example.org/api/hits
Content-Type: application/json
```
{
    "key0": "{{a_string}}",
    "key1": {{a_bool}},
    "key2": {{a_null}},
    "key3": {{a_number}}
}
```
~~~

Variables can be initialized via command line:

```shell
$ hurl --variable a_string=apple --variable a_bool=true --variable a_null=null --variable a_number=42 test.hurl
```

Resulting in a PUT request with the following JSON body:

```
{
    "key0": "apple",
    "key1": true,
    "key2": null,
    "key3": 42
}
```



<hr>

## Grammar {#file-format-grammar-grammar}

### Definitions {#file-format-grammar-definitions}

Short description:

- operator &#124; denotes alternative,
- operator * denotes iteration (zero or more),
- operator + denotes iteration (one or more),

### Syntax Grammar {#file-format-grammar-syntax-grammar}

<div class="grammar-ruleset"><h3 id="general">General</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="hurl-file">hurl-file</span></div><div class="grammar-rule-expression"><a href="#entry">entry</a><span class="grammar-symbol">*</span><br>
<a href="#lt">lt</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="entry">entry</span><span class="grammar-usedby">(used by <a href="#hurl-file">hurl-file</a>)</span></div><div class="grammar-rule-expression"><a href="#request">request</a><br>
<a href="#response">response</a><span class="grammar-symbol">?</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="request">request</span><span class="grammar-usedby">(used by <a href="#entry">entry</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#method">method</a>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a><br>
<a href="#header">header</a><span class="grammar-symbol">*</span><br>
<a href="#request-section">request-section</a><span class="grammar-symbol">*</span><br>
<a href="#body">body</a><span class="grammar-symbol">?</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="response">response</span><span class="grammar-usedby">(used by <a href="#entry">entry</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#version">version</a>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#status">status</a>&nbsp;<a href="#lt">lt</a><br>
<a href="#header">header</a><span class="grammar-symbol">*</span><br>
<a href="#response-section">response-section</a><span class="grammar-symbol">*</span><br>
<a href="#body">body</a><span class="grammar-symbol">?</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="method">method</span><span class="grammar-usedby">(used by <a href="#request">request</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[A-Z]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="version">version</span><span class="grammar-usedby">(used by <a href="#response">response</a>)</span></div><div class="grammar-rule-expression">&nbsp;<span class="grammar-literal">HTTP/1.0</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">HTTP/1.1</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">HTTP/2</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">HTTP</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="status">status</span><span class="grammar-usedby">(used by <a href="#response">response</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[0-9]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="header">header</span><span class="grammar-usedby">(used by <a href="#request">request</a>,&nbsp;<a href="#response">response</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#key-value">key-value</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="body">body</span><span class="grammar-usedby">(used by <a href="#request">request</a>,&nbsp;<a href="#response">response</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#bytes">bytes</a>&nbsp;<a href="#lt">lt</a></div></div>
</div><div class="grammar-ruleset"><h3 id="sections">Sections</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="request-section">request-section</span><span class="grammar-usedby">(used by <a href="#request">request</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#basic-auth-section">basic-auth-section</a><br>
<span class="grammar-symbol">|</span><a href="#query-string-params-section">query-string-params-section</a><br>
<span class="grammar-symbol">|</span><a href="#form-params-section">form-params-section</a><br>
<span class="grammar-symbol">|</span><a href="#multipart-form-data-section">multipart-form-data-section</a><br>
<span class="grammar-symbol">|</span><a href="#cookies-section">cookies-section</a><br>
<span class="grammar-symbol">|</span><a href="#options-section">options-section</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="response-section">response-section</span><span class="grammar-usedby">(used by <a href="#response">response</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#captures-section">captures-section</a><br>
<span class="grammar-symbol">|</span><a href="#asserts-section">asserts-section</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="query-string-params-section">query-string-params-section</span><span class="grammar-usedby">(used by <a href="#request-section">request-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[QueryStringParams]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#key-value">key-value</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="form-params-section">form-params-section</span><span class="grammar-usedby">(used by <a href="#request-section">request-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[FormParams]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#key-value">key-value</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multipart-form-data-section">multipart-form-data-section</span><span class="grammar-usedby">(used by <a href="#request-section">request-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[MultipartFormData]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#multipart-form-data-param">multipart-form-data-param</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="cookies-section">cookies-section</span><span class="grammar-usedby">(used by <a href="#request-section">request-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[Cookies]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#key-value">key-value</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="captures-section">captures-section</span><span class="grammar-usedby">(used by <a href="#response-section">response-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[Captures]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#capture">capture</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="asserts-section">asserts-section</span><span class="grammar-usedby">(used by <a href="#response-section">response-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[Asserts]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#assert">assert</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="basic-auth-section">basic-auth-section</span><span class="grammar-usedby">(used by <a href="#request-section">request-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[BasicAuth]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#key-value">key-value</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="options-section">options-section</span><span class="grammar-usedby">(used by <a href="#request-section">request-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-literal">[Options]</span>&nbsp;<a href="#lt">lt</a><br>
<a href="#option">option</a><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="key-value">key-value</span><span class="grammar-usedby">(used by <a href="#header">header</a>,&nbsp;<a href="#query-string-params-section">query-string-params-section</a>,&nbsp;<a href="#form-params-section">form-params-section</a>,&nbsp;<a href="#cookies-section">cookies-section</a>,&nbsp;<a href="#basic-auth-section">basic-auth-section</a>,&nbsp;<a href="#multipart-form-data-param">multipart-form-data-param</a>)</span></div><div class="grammar-rule-expression"><a href="#key-string">key-string</a>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multipart-form-data-param">multipart-form-data-param</span><span class="grammar-usedby">(used by <a href="#multipart-form-data-section">multipart-form-data-section</a>)</span></div><div class="grammar-rule-expression"><a href="#file-param">file-param</a><span class="grammar-symbol">|</span><a href="#key-value">key-value</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="file-param">file-param</span><span class="grammar-usedby">(used by <a href="#multipart-form-data-param">multipart-form-data-param</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#key-string">key-string</a>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#file-value">file-value</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="file-value">file-value</span><span class="grammar-usedby">(used by <a href="#file-param">file-param</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">file,</span>&nbsp;<a href="#filename">filename</a>&nbsp;<span class="grammar-literal">;</span>&nbsp;<span class="grammar-symbol">(</span><a href="#file-contenttype">file-contenttype</a><span class="grammar-symbol">)</span><span class="grammar-symbol">?</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="file-contenttype">file-contenttype</span><span class="grammar-usedby">(used by <a href="#file-value">file-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[a-zA-Z0-9/+-]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="capture">capture</span><span class="grammar-usedby">(used by <a href="#captures-section">captures-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#key-string">key-string</a>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#query">query</a>&nbsp;<span class="grammar-symbol">(</span><a href="#sp">sp</a>&nbsp;<a href="#filter">filter</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="assert">assert</span><span class="grammar-usedby">(used by <a href="#asserts-section">asserts-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<a href="#query">query</a>&nbsp;<span class="grammar-symbol">(</span><a href="#sp">sp</a>&nbsp;<a href="#filter">filter</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#predicate">predicate</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="option">option</span><span class="grammar-usedby">(used by <a href="#options-section">options-section</a>)</span></div><div class="grammar-rule-expression"><a href="#lt">lt</a><span class="grammar-symbol">*</span><br>
<span class="grammar-symbol">(</span><a href="#aws-sigv4-option">aws-sigv4-option</a><span class="grammar-symbol">|</span><a href="#ca-certificate-option">ca-certificate-option</a><span class="grammar-symbol">|</span><a href="#client-certificate-option">client-certificate-option</a><span class="grammar-symbol">|</span><a href="#client-key-option">client-key-option</a><span class="grammar-symbol">|</span><a href="#compressed-option">compressed-option</a><span class="grammar-symbol">|</span><a href="#connect-to-option">connect-to-option</a><span class="grammar-symbol">|</span><a href="#delay-option">delay-option</a><span class="grammar-symbol">|</span><a href="#follow-redirect-option">follow-redirect-option</a><span class="grammar-symbol">|</span><a href="#follow-redirect-trusted-option">follow-redirect-trusted-option</a><span class="grammar-symbol">|</span><a href="#http10-option">http10-option</a><span class="grammar-symbol">|</span><a href="#http11-option">http11-option</a><span class="grammar-symbol">|</span><a href="#http2-option">http2-option</a><span class="grammar-symbol">|</span><a href="#http3-option">http3-option</a><span class="grammar-symbol">|</span><a href="#insecure-option">insecure-option</a><span class="grammar-symbol">|</span><a href="#ipv4-option">ipv4-option</a><span class="grammar-symbol">|</span><a href="#ipv6-option">ipv6-option</a><span class="grammar-symbol">|</span><a href="#max-redirs-option">max-redirs-option</a><span class="grammar-symbol">|</span><a href="#netrc-option">netrc-option</a><span class="grammar-symbol">|</span><a href="#netrc-file-option">netrc-file-option</a><span class="grammar-symbol">|</span><a href="#netrc-optional-option">netrc-optional-option</a><span class="grammar-symbol">|</span><a href="#output-option">output-option</a><span class="grammar-symbol">|</span><a href="#path-as-is-option">path-as-is-option</a><span class="grammar-symbol">|</span><a href="#proxy-option">proxy-option</a><span class="grammar-symbol">|</span><a href="#repeat-option">repeat-option</a><span class="grammar-symbol">|</span><a href="#resolve-option">resolve-option</a><span class="grammar-symbol">|</span><a href="#retry-option">retry-option</a><span class="grammar-symbol">|</span><a href="#retry-interval-option">retry-interval-option</a><span class="grammar-symbol">|</span><a href="#skip-option">skip-option</a><span class="grammar-symbol">|</span><a href="#unix-socket-option">unix-socket-option</a><span class="grammar-symbol">|</span><a href="#user-option">user-option</a><span class="grammar-symbol">|</span><a href="#variable-option">variable-option</a><span class="grammar-symbol">|</span><a href="#verbose-option">verbose-option</a><span class="grammar-symbol">|</span><a href="#very-verbose-option">very-verbose-option</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="aws-sigv4-option">aws-sigv4-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">aws-sigv4</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="ca-certificate-option">ca-certificate-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">cacert</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#filename">filename</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="client-certificate-option">client-certificate-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">cert</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#filename-password">filename-password</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="client-key-option">client-key-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">key</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="compressed-option">compressed-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">compressed</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="connect-to-option">connect-to-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">connect-to</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="delay-option">delay-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">delay</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#duration-option">duration-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="follow-redirect-option">follow-redirect-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">location</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="follow-redirect-trusted-option">follow-redirect-trusted-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">location-trusted</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="http10-option">http10-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">http1.0</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="http11-option">http11-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">http1.1</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="http2-option">http2-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">http2</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="http3-option">http3-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">http3</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="insecure-option">insecure-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">insecure</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="ipv4-option">ipv4-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">ipv4</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="ipv6-option">ipv6-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">ipv6</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="max-redirs-option">max-redirs-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">max-redirs</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#integer-option">integer-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="netrc-option">netrc-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">netrc</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="netrc-file-option">netrc-file-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">netrc-file</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="netrc-optional-option">netrc-optional-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">netrc-optional</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="output-option">output-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">output</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="path-as-is-option">path-as-is-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">path-as-is</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="proxy-option">proxy-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">proxy</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="resolve-option">resolve-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">resolve</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="repeat-option">repeat-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">repeat</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#integer-option">integer-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="retry-option">retry-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">retry</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#integer-option">integer-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="retry-interval-option">retry-interval-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">retry-interval</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#duration-option">duration-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="skip-option">skip-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">skip</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="unix-socket-option">unix-socket-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">unix-socket</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="user-option">user-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">user</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#value-string">value-string</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="variable-option">variable-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">variable</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#variable-definition">variable-definition</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="verbose-option">verbose-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">verbose</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="very-verbose-option">very-verbose-option</span><span class="grammar-usedby">(used by <a href="#option">option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">very-verbose</span>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#boolean-option">boolean-option</a>&nbsp;<a href="#lt">lt</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="variable-definition">variable-definition</span><span class="grammar-usedby">(used by <a href="#variable-option">variable-option</a>)</span></div><div class="grammar-rule-expression"><a href="#variable-name">variable-name</a>&nbsp;<span class="grammar-literal">=</span>&nbsp;<a href="#variable-value">variable-value</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="boolean-option">boolean-option</span><span class="grammar-usedby">(used by <a href="#compressed-option">compressed-option</a>,&nbsp;<a href="#follow-redirect-option">follow-redirect-option</a>,&nbsp;<a href="#follow-redirect-trusted-option">follow-redirect-trusted-option</a>,&nbsp;<a href="#http10-option">http10-option</a>,&nbsp;<a href="#http11-option">http11-option</a>,&nbsp;<a href="#http2-option">http2-option</a>,&nbsp;<a href="#http3-option">http3-option</a>,&nbsp;<a href="#insecure-option">insecure-option</a>,&nbsp;<a href="#ipv4-option">ipv4-option</a>,&nbsp;<a href="#ipv6-option">ipv6-option</a>,&nbsp;<a href="#netrc-option">netrc-option</a>,&nbsp;<a href="#netrc-optional-option">netrc-optional-option</a>,&nbsp;<a href="#path-as-is-option">path-as-is-option</a>,&nbsp;<a href="#skip-option">skip-option</a>,&nbsp;<a href="#verbose-option">verbose-option</a>,&nbsp;<a href="#very-verbose-option">very-verbose-option</a>)</span></div><div class="grammar-rule-expression"><a href="#boolean">boolean</a><span class="grammar-symbol">|</span><a href="#template">template</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="integer-option">integer-option</span><span class="grammar-usedby">(used by <a href="#max-redirs-option">max-redirs-option</a>,&nbsp;<a href="#repeat-option">repeat-option</a>,&nbsp;<a href="#retry-option">retry-option</a>)</span></div><div class="grammar-rule-expression"><a href="#integer">integer</a><span class="grammar-symbol">|</span><a href="#template">template</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="duration-option">duration-option</span><span class="grammar-usedby">(used by <a href="#delay-option">delay-option</a>,&nbsp;<a href="#retry-interval-option">retry-interval-option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#integer">integer</a>&nbsp;<a href="#duration-unit">duration-unit</a><span class="grammar-symbol">?</span><span class="grammar-symbol">)</span><span class="grammar-symbol">|</span><a href="#template">template</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="duration-unit">duration-unit</span><span class="grammar-usedby">(used by <a href="#duration-option">duration-option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">ms</span><span class="grammar-symbol">|</span><span class="grammar-literal">s</span><span class="grammar-symbol">|</span><span class="grammar-literal">m</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="variable-value">variable-value</span><span class="grammar-usedby">(used by <a href="#variable-definition">variable-definition</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#null">null</a><br>
<span class="grammar-symbol">|</span><a href="#boolean">boolean</a><br>
<span class="grammar-symbol">|</span><a href="#integer">integer</a><br>
<span class="grammar-symbol">|</span><a href="#float">float</a><br>
<span class="grammar-symbol">|</span><a href="#key-string">key-string</a><br>
<span class="grammar-symbol">|</span><a href="#quoted-string">quoted-string</a></div></div>
</div><div class="grammar-ruleset"><h3 id="query">Query</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="query">query</span><span class="grammar-usedby">(used by <a href="#capture">capture</a>,&nbsp;<a href="#assert">assert</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#status-query">status-query</a><br>
<span class="grammar-symbol">|</span><a href="#url-query">url-query</a><br>
<span class="grammar-symbol">|</span><a href="#header-query">header-query</a><br>
<span class="grammar-symbol">|</span><a href="#certificate-query">certificate-query</a><br>
<span class="grammar-symbol">|</span><a href="#cookie-query">cookie-query</a><br>
<span class="grammar-symbol">|</span><a href="#body-query">body-query</a><br>
<span class="grammar-symbol">|</span><a href="#xpath-query">xpath-query</a><br>
<span class="grammar-symbol">|</span><a href="#jsonpath-query">jsonpath-query</a><br>
<span class="grammar-symbol">|</span><a href="#regex-query">regex-query</a><br>
<span class="grammar-symbol">|</span><a href="#variable-query">variable-query</a><br>
<span class="grammar-symbol">|</span><a href="#duration-query">duration-query</a><br>
<span class="grammar-symbol">|</span><a href="#bytes-query">bytes-query</a><br>
<span class="grammar-symbol">|</span><a href="#sha256-query">sha256-query</a><br>
<span class="grammar-symbol">|</span><a href="#md5-query">md5-query</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="status-query">status-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">status</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="url-query">url-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">url</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="header-query">header-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">header</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="certificate-query">certificate-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">certificate</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">Subject</span><span class="grammar-symbol">|</span><span class="grammar-literal">Issuer</span><span class="grammar-symbol">|</span><span class="grammar-literal">Start-Date</span><span class="grammar-symbol">|</span><span class="grammar-literal">Expire-Date</span><span class="grammar-symbol">|</span><span class="grammar-literal">Serial-Number</span><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="cookie-query">cookie-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">cookie</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="body-query">body-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">body</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="xpath-query">xpath-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">xpath</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="jsonpath-query">jsonpath-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">jsonpath</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="regex-query">regex-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">regex</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">|</span><a href="#regex">regex</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="variable-query">variable-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">variable</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="duration-query">duration-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">duration</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="sha256-query">sha256-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">sha256</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="md5-query">md5-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">md5</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="bytes-query">bytes-query</span><span class="grammar-usedby">(used by <a href="#query">query</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">bytes</span></div></div>
</div><div class="grammar-ruleset"><h3 id="predicates">Predicates</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="predicate">predicate</span><span class="grammar-usedby">(used by <a href="#assert">assert</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><span class="grammar-literal">not</span>&nbsp;<a href="#sp">sp</a><span class="grammar-symbol">)</span><span class="grammar-symbol">?</span>&nbsp;<a href="#predicate-func">predicate-func</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="predicate-func">predicate-func</span><span class="grammar-usedby">(used by <a href="#predicate">predicate</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#equal-predicate">equal-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#not-equal-predicate">not-equal-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#greater-predicate">greater-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#greater-or-equal-predicate">greater-or-equal-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#less-predicate">less-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#less-or-equal-predicate">less-or-equal-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#start-with-predicate">start-with-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#end-with-predicate">end-with-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#contain-predicate">contain-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#match-predicate">match-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#exist-predicate">exist-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#is-empty-predicate">is-empty-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#include-predicate">include-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#integer-predicate">integer-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#float-predicate">float-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#boolean-predicate">boolean-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#string-predicate">string-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#collection-predicate">collection-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#date-predicate">date-predicate</a><br>
<span class="grammar-symbol">|</span><a href="#iso-date-predicate">iso-date-predicate</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="equal-predicate">equal-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">==</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#predicate-value">predicate-value</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="not-equal-predicate">not-equal-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">!=</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#predicate-value">predicate-value</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="greater-predicate">greater-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">&gt;</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#number">number</a><span class="grammar-symbol">|</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="greater-or-equal-predicate">greater-or-equal-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">&gt;=</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#sp">sp</a><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-symbol">(</span><a href="#number">number</a><span class="grammar-symbol">|</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="less-predicate">less-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">&lt;</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#number">number</a><span class="grammar-symbol">|</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="less-or-equal-predicate">less-or-equal-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">&lt;=</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#number">number</a><span class="grammar-symbol">|</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="start-with-predicate">start-with-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">startsWith</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">|</span><a href="#oneline-hex">oneline-hex</a><span class="grammar-symbol">|</span><a href="#oneline-base64">oneline-base64</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="end-with-predicate">end-with-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">endsWith</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">|</span><a href="#oneline-hex">oneline-hex</a><span class="grammar-symbol">|</span><a href="#oneline-base64">oneline-base64</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="contain-predicate">contain-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">contains</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="match-predicate">match-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">matches</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">|</span><a href="#regex">regex</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="exist-predicate">exist-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">exists</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="is-empty-predicate">is-empty-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isEmpty</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="include-predicate">include-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">includes</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#predicate-value">predicate-value</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="integer-predicate">integer-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isInteger</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="float-predicate">float-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isFloat</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="boolean-predicate">boolean-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isBoolean</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="string-predicate">string-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isString</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="collection-predicate">collection-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isCollection</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="date-predicate">date-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isDate</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="iso-date-predicate">iso-date-predicate</span><span class="grammar-usedby">(used by <a href="#predicate-func">predicate-func</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">isIsoDate</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="predicate-value">predicate-value</span><span class="grammar-usedby">(used by <a href="#equal-predicate">equal-predicate</a>,&nbsp;<a href="#not-equal-predicate">not-equal-predicate</a>,&nbsp;<a href="#include-predicate">include-predicate</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#boolean">boolean</a><br>
<span class="grammar-symbol">|</span><a href="#multiline-string">multiline-string</a><br>
<span class="grammar-symbol">|</span><a href="#null">null</a><br>
<span class="grammar-symbol">|</span><a href="#number">number</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-base64">oneline-base64</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-file">oneline-file</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-hex">oneline-hex</a><br>
<span class="grammar-symbol">|</span><a href="#quoted-string">quoted-string</a><br>
<span class="grammar-symbol">|</span><a href="#template">template</a></div></div>
</div><div class="grammar-ruleset"><h3 id="bytes">Bytes</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="bytes">bytes</span><span class="grammar-usedby">(used by <a href="#body">body</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#json-value">json-value</a><br>
<span class="grammar-symbol">|</span><a href="#xml">xml</a><br>
<span class="grammar-symbol">|</span><a href="#multiline-string">multiline-string</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-string">oneline-string</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-base64">oneline-base64</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-file">oneline-file</a><br>
<span class="grammar-symbol">|</span><a href="#oneline-hex">oneline-hex</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="xml">xml</span><span class="grammar-usedby">(used by <a href="#bytes">bytes</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">&lt;</span>&nbsp;<span class="grammar-literal">To Be Defined</span>&nbsp;<span class="grammar-literal">&gt;</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-base64">oneline-base64</span><span class="grammar-usedby">(used by <a href="#start-with-predicate">start-with-predicate</a>,&nbsp;<a href="#end-with-predicate">end-with-predicate</a>,&nbsp;<a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#bytes">bytes</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">base64,</span>&nbsp;<span class="grammar-regex">[A-Z0-9+-= \n]+</span>&nbsp;<span class="grammar-literal">;</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-file">oneline-file</span><span class="grammar-usedby">(used by <a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#bytes">bytes</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">file,</span>&nbsp;<a href="#filename">filename</a>&nbsp;<span class="grammar-literal">;</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-hex">oneline-hex</span><span class="grammar-usedby">(used by <a href="#start-with-predicate">start-with-predicate</a>,&nbsp;<a href="#end-with-predicate">end-with-predicate</a>,&nbsp;<a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#bytes">bytes</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">hex,</span>&nbsp;<a href="#hexdigit">hexdigit</a><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-literal">;</span></div></div>
</div><div class="grammar-ruleset"><h3 id="strings">Strings</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="quoted-string">quoted-string</span><span class="grammar-usedby">(used by <a href="#variable-value">variable-value</a>,&nbsp;<a href="#header-query">header-query</a>,&nbsp;<a href="#cookie-query">cookie-query</a>,&nbsp;<a href="#xpath-query">xpath-query</a>,&nbsp;<a href="#jsonpath-query">jsonpath-query</a>,&nbsp;<a href="#regex-query">regex-query</a>,&nbsp;<a href="#variable-query">variable-query</a>,&nbsp;<a href="#greater-predicate">greater-predicate</a>,&nbsp;<a href="#greater-or-equal-predicate">greater-or-equal-predicate</a>,&nbsp;<a href="#less-predicate">less-predicate</a>,&nbsp;<a href="#less-or-equal-predicate">less-or-equal-predicate</a>,&nbsp;<a href="#start-with-predicate">start-with-predicate</a>,&nbsp;<a href="#end-with-predicate">end-with-predicate</a>,&nbsp;<a href="#contain-predicate">contain-predicate</a>,&nbsp;<a href="#match-predicate">match-predicate</a>,&nbsp;<a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#jsonpath-filter">jsonpath-filter</a>,&nbsp;<a href="#regex-filter">regex-filter</a>,&nbsp;<a href="#replace-filter">replace-filter</a>,&nbsp;<a href="#split-filter">split-filter</a>,&nbsp;<a href="#xpath-filter">xpath-filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">"</span>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string-content">quoted-string-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-literal">"</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="quoted-string-content">quoted-string-content</span><span class="grammar-usedby">(used by <a href="#quoted-string">quoted-string</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#quoted-string-text">quoted-string-text</a><span class="grammar-symbol">|</span><a href="#quoted-string-escaped-char">quoted-string-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="quoted-string-text">quoted-string-text</span><span class="grammar-usedby">(used by <a href="#quoted-string-content">quoted-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~["\\]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="quoted-string-escaped-char">quoted-string-escaped-char</span><span class="grammar-usedby">(used by <a href="#quoted-string-content">quoted-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">"</span><span class="grammar-symbol">|</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">\b</span><span class="grammar-symbol">|</span><span class="grammar-literal">\f</span><span class="grammar-symbol">|</span><span class="grammar-literal">\n</span><span class="grammar-symbol">|</span><span class="grammar-literal">\r</span><span class="grammar-symbol">|</span><span class="grammar-literal">\t</span><span class="grammar-symbol">|</span><span class="grammar-literal">\u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="key-string">key-string</span><span class="grammar-usedby">(used by <a href="#key-value">key-value</a>,&nbsp;<a href="#file-param">file-param</a>,&nbsp;<a href="#capture">capture</a>,&nbsp;<a href="#variable-value">variable-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#key-string-content">key-string-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="key-string-content">key-string-content</span><span class="grammar-usedby">(used by <a href="#key-string">key-string</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#key-string-text">key-string-text</a><span class="grammar-symbol">|</span><a href="#key-string-escaped-char">key-string-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="key-string-text">key-string-text</span><span class="grammar-usedby">(used by <a href="#key-string-content">key-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#alphanum">alphanum</a><span class="grammar-symbol">|</span><span class="grammar-literal">_</span><span class="grammar-symbol">|</span><span class="grammar-literal">-</span><span class="grammar-symbol">|</span><span class="grammar-literal">.</span><span class="grammar-symbol">|</span><span class="grammar-literal">[</span><span class="grammar-symbol">|</span><span class="grammar-literal">]</span><span class="grammar-symbol">|</span><span class="grammar-literal">@</span><span class="grammar-symbol">|</span><span class="grammar-literal">$</span><span class="grammar-symbol">)</span><span class="grammar-symbol">+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="key-string-escaped-char">key-string-escaped-char</span><span class="grammar-usedby">(used by <a href="#key-string-content">key-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">#</span><span class="grammar-symbol">|</span><span class="grammar-literal">:</span><span class="grammar-symbol">|</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">\b</span><span class="grammar-symbol">|</span><span class="grammar-literal">\f</span><span class="grammar-symbol">|</span><span class="grammar-literal">\n</span><span class="grammar-symbol">|</span><span class="grammar-literal">\r</span><span class="grammar-symbol">|</span><span class="grammar-literal">\t</span><span class="grammar-symbol">|</span><span class="grammar-literal">\u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="value-string">value-string</span><span class="grammar-usedby">(used by <a href="#request">request</a>,&nbsp;<a href="#key-value">key-value</a>,&nbsp;<a href="#aws-sigv4-option">aws-sigv4-option</a>,&nbsp;<a href="#client-key-option">client-key-option</a>,&nbsp;<a href="#connect-to-option">connect-to-option</a>,&nbsp;<a href="#netrc-file-option">netrc-file-option</a>,&nbsp;<a href="#output-option">output-option</a>,&nbsp;<a href="#proxy-option">proxy-option</a>,&nbsp;<a href="#resolve-option">resolve-option</a>,&nbsp;<a href="#unix-socket-option">unix-socket-option</a>,&nbsp;<a href="#user-option">user-option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#value-string-content">value-string-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="value-string-content">value-string-content</span><span class="grammar-usedby">(used by <a href="#value-string">value-string</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#value-string-text">value-string-text</a><span class="grammar-symbol">|</span><a href="#value-string-escaped-char">value-string-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="value-string-text">value-string-text</span><span class="grammar-usedby">(used by <a href="#value-string-content">value-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~[#\n\\]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="value-string-escaped-char">value-string-escaped-char</span><span class="grammar-usedby">(used by <a href="#value-string-content">value-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">#</span><span class="grammar-symbol">|</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">\b</span><span class="grammar-symbol">|</span><span class="grammar-literal">\f</span><span class="grammar-symbol">|</span><span class="grammar-literal">\n</span><span class="grammar-symbol">|</span><span class="grammar-literal">\r</span><span class="grammar-symbol">|</span><span class="grammar-literal">\t</span><span class="grammar-symbol">|</span><span class="grammar-literal">\u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-string">oneline-string</span><span class="grammar-usedby">(used by <a href="#bytes">bytes</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">`</span>&nbsp;<span class="grammar-symbol">(</span><a href="#oneline-string-content">oneline-string-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-literal">`</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-string-content">oneline-string-content</span><span class="grammar-usedby">(used by <a href="#oneline-string">oneline-string</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#oneline-string-text">oneline-string-text</a><span class="grammar-symbol">|</span><a href="#oneline-string-escaped-char">oneline-string-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-string-text">oneline-string-text</span><span class="grammar-usedby">(used by <a href="#oneline-string-content">oneline-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~[#\n\\]</span>&nbsp;<span class="grammar-symbol">~</span><span class="grammar-literal">`</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="oneline-string-escaped-char">oneline-string-escaped-char</span><span class="grammar-usedby">(used by <a href="#oneline-string-content">oneline-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">`</span><span class="grammar-symbol">|</span><span class="grammar-literal">#</span><span class="grammar-symbol">|</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">b</span><span class="grammar-symbol">|</span><span class="grammar-literal">f</span><span class="grammar-symbol">|</span><span class="grammar-literal">u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multiline-string">multiline-string</span><span class="grammar-usedby">(used by <a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#bytes">bytes</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">```</span>&nbsp;<a href="#multiline-string-type">multiline-string-type</a><span class="grammar-symbol">?</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">,</span>&nbsp;<a href="#multiline-string-attribute">multiline-string-attribute</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<a href="#lt">lt</a><br>
<span class="grammar-symbol">(</span><a href="#multiline-string-content">multiline-string-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<a href="#lt">lt</a><br>
<span class="grammar-literal">```</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multiline-string-type">multiline-string-type</span><span class="grammar-usedby">(used by <a href="#multiline-string">multiline-string</a>)</span></div><div class="grammar-rule-expression">&nbsp;<span class="grammar-literal">base64</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">hex</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">json</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">xml</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">graphql</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multiline-string-attribute">multiline-string-attribute</span><span class="grammar-usedby">(used by <a href="#multiline-string">multiline-string</a>)</span></div><div class="grammar-rule-expression">&nbsp;<span class="grammar-literal">escape</span><br>
<span class="grammar-symbol">|</span><span class="grammar-literal">novariable</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multiline-string-content">multiline-string-content</span><span class="grammar-usedby">(used by <a href="#multiline-string">multiline-string</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#multiline-string-text">multiline-string-text</a><span class="grammar-symbol">|</span><a href="#multiline-string-escaped-char">multiline-string-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multiline-string-text">multiline-string-text</span><span class="grammar-usedby">(used by <a href="#multiline-string-content">multiline-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~[\\]+</span>&nbsp;<span class="grammar-symbol">~</span><span class="grammar-literal">```</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="multiline-string-escaped-char">multiline-string-escaped-char</span><span class="grammar-usedby">(used by <a href="#multiline-string-content">multiline-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">b</span><span class="grammar-symbol">|</span><span class="grammar-literal">f</span><span class="grammar-symbol">|</span><span class="grammar-literal">n</span><span class="grammar-symbol">|</span><span class="grammar-literal">r</span><span class="grammar-symbol">|</span><span class="grammar-literal">t</span><span class="grammar-symbol">|</span><span class="grammar-literal">`</span><span class="grammar-symbol">|</span><span class="grammar-literal">u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename">filename</span><span class="grammar-usedby">(used by <a href="#file-value">file-value</a>,&nbsp;<a href="#ca-certificate-option">ca-certificate-option</a>,&nbsp;<a href="#oneline-file">oneline-file</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#filename-content">filename-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-content">filename-content</span><span class="grammar-usedby">(used by <a href="#filename">filename</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#filename-text">filename-text</a><span class="grammar-symbol">|</span><a href="#filename-escaped-char">filename-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-text">filename-text</span><span class="grammar-usedby">(used by <a href="#filename-content">filename-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~[#;{} \n\\]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-escaped-char">filename-escaped-char</span><span class="grammar-usedby">(used by <a href="#filename-content">filename-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">b</span><span class="grammar-symbol">|</span><span class="grammar-literal">f</span><span class="grammar-symbol">|</span><span class="grammar-literal">n</span><span class="grammar-symbol">|</span><span class="grammar-literal">r</span><span class="grammar-symbol">|</span><span class="grammar-literal">t</span><span class="grammar-symbol">|</span><span class="grammar-literal">#</span><span class="grammar-symbol">|</span><span class="grammar-literal">;</span><span class="grammar-symbol">|</span><span class="grammar-literal"> </span><span class="grammar-symbol">|</span><span class="grammar-literal">{</span><span class="grammar-symbol">|</span><span class="grammar-literal">}</span><span class="grammar-symbol">|</span><span class="grammar-literal">u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-password">filename-password</span><span class="grammar-usedby">(used by <a href="#client-certificate-option">client-certificate-option</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#filename-password-content">filename-password-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-password-content">filename-password-content</span><span class="grammar-usedby">(used by <a href="#filename-password">filename-password</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#filename-password-text">filename-password-text</a><span class="grammar-symbol">|</span><a href="#filename-password-escaped-char">filename-password-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-password-text">filename-password-text</span><span class="grammar-usedby">(used by <a href="#filename-password-content">filename-password-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~[#;{} \n\\]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filename-password-escaped-char">filename-password-escaped-char</span><span class="grammar-usedby">(used by <a href="#filename-password-content">filename-password-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">b</span><span class="grammar-symbol">|</span><span class="grammar-literal">f</span><span class="grammar-symbol">|</span><span class="grammar-literal">n</span><span class="grammar-symbol">|</span><span class="grammar-literal">r</span><span class="grammar-symbol">|</span><span class="grammar-literal">t</span><span class="grammar-symbol">|</span><span class="grammar-literal">#</span><span class="grammar-symbol">|</span><span class="grammar-literal">;</span><span class="grammar-symbol">|</span><span class="grammar-literal"> </span><span class="grammar-symbol">|</span><span class="grammar-literal">{</span><span class="grammar-symbol">|</span><span class="grammar-literal">}</span><span class="grammar-symbol">|</span><span class="grammar-literal">:</span><span class="grammar-symbol">|</span><span class="grammar-literal">u</span>&nbsp;<a href="#unicode-char">unicode-char</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="unicode-char">unicode-char</span><span class="grammar-usedby">(used by <a href="#quoted-string-escaped-char">quoted-string-escaped-char</a>,&nbsp;<a href="#key-string-escaped-char">key-string-escaped-char</a>,&nbsp;<a href="#value-string-escaped-char">value-string-escaped-char</a>,&nbsp;<a href="#oneline-string-escaped-char">oneline-string-escaped-char</a>,&nbsp;<a href="#multiline-string-escaped-char">multiline-string-escaped-char</a>,&nbsp;<a href="#filename-escaped-char">filename-escaped-char</a>,&nbsp;<a href="#filename-password-escaped-char">filename-password-escaped-char</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">{</span>&nbsp;<a href="#hexdigit">hexdigit</a><span class="grammar-symbol">+</span>&nbsp;<span class="grammar-literal">}</span></div></div>
</div><div class="grammar-ruleset"><h3 id="json">JSON</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-value">json-value</span><span class="grammar-usedby">(used by <a href="#bytes">bytes</a>,&nbsp;<a href="#json-key-value">json-key-value</a>,&nbsp;<a href="#json-array">json-array</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#template">template</a><br>
<span class="grammar-symbol">|</span><a href="#json-object">json-object</a><br>
<span class="grammar-symbol">|</span><a href="#json-array">json-array</a><br>
<span class="grammar-symbol">|</span><a href="#json-string">json-string</a><br>
<span class="grammar-symbol">|</span><a href="#json-number">json-number</a><br>
<span class="grammar-symbol">|</span><a href="#boolean">boolean</a><br>
<span class="grammar-symbol">|</span><a href="#null">null</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-object">json-object</span><span class="grammar-usedby">(used by <a href="#json-value">json-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">{</span>&nbsp;<a href="#json-key-value">json-key-value</a>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">,</span>&nbsp;<a href="#json-key-value">json-key-value</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-literal">}</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-key-value">json-key-value</span><span class="grammar-usedby">(used by <a href="#json-object">json-object</a>)</span></div><div class="grammar-rule-expression"><a href="#json-string">json-string</a>&nbsp;<span class="grammar-literal">:</span>&nbsp;<a href="#json-value">json-value</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-array">json-array</span><span class="grammar-usedby">(used by <a href="#json-value">json-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">[</span>&nbsp;<a href="#json-value">json-value</a>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">,</span>&nbsp;<a href="#json-value">json-value</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-literal">]</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-string">json-string</span><span class="grammar-usedby">(used by <a href="#json-value">json-value</a>,&nbsp;<a href="#json-key-value">json-key-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">"</span>&nbsp;<span class="grammar-symbol">(</span><a href="#json-string-content">json-string-content</a><span class="grammar-symbol">|</span><a href="#template">template</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span>&nbsp;<span class="grammar-literal">"</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-string-content">json-string-content</span><span class="grammar-usedby">(used by <a href="#json-string">json-string</a>)</span></div><div class="grammar-rule-expression"><a href="#json-string-text">json-string-text</a><span class="grammar-symbol">|</span><a href="#json-string-escaped-char">json-string-escaped-char</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-string-text">json-string-text</span><span class="grammar-usedby">(used by <a href="#json-string-content">json-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~["\\]</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-string-escaped-char">json-string-escaped-char</span><span class="grammar-usedby">(used by <a href="#json-string-content">json-string-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">"</span><span class="grammar-symbol">|</span><span class="grammar-literal">\</span><span class="grammar-symbol">|</span><span class="grammar-literal">b</span><span class="grammar-symbol">|</span><span class="grammar-literal">f</span><span class="grammar-symbol">|</span><span class="grammar-literal">n</span><span class="grammar-symbol">|</span><span class="grammar-literal">r</span><span class="grammar-symbol">|</span><span class="grammar-literal">t</span><span class="grammar-symbol">|</span><span class="grammar-literal">u</span>&nbsp;<a href="#hexdigit">hexdigit</a>&nbsp;<a href="#hexdigit">hexdigit</a>&nbsp;<a href="#hexdigit">hexdigit</a>&nbsp;<a href="#hexdigit">hexdigit</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="json-number">json-number</span><span class="grammar-usedby">(used by <a href="#json-value">json-value</a>)</span></div><div class="grammar-rule-expression"><a href="#integer">integer</a>&nbsp;<a href="#fraction">fraction</a><span class="grammar-symbol">?</span>&nbsp;<a href="#exponent">exponent</a><span class="grammar-symbol">?</span></div></div>
</div><div class="grammar-ruleset"><h3 id="template-expression">Template / Expression</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="template">template</span><span class="grammar-usedby">(used by <a href="#boolean-option">boolean-option</a>,&nbsp;<a href="#integer-option">integer-option</a>,&nbsp;<a href="#duration-option">duration-option</a>,&nbsp;<a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#quoted-string">quoted-string</a>,&nbsp;<a href="#key-string">key-string</a>,&nbsp;<a href="#value-string">value-string</a>,&nbsp;<a href="#oneline-string">oneline-string</a>,&nbsp;<a href="#multiline-string">multiline-string</a>,&nbsp;<a href="#filename">filename</a>,&nbsp;<a href="#filename-password">filename-password</a>,&nbsp;<a href="#json-value">json-value</a>,&nbsp;<a href="#json-string">json-string</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">{{</span>&nbsp;<a href="#expr">expr</a>&nbsp;<span class="grammar-literal">}}</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="expr">expr</span><span class="grammar-usedby">(used by <a href="#template">template</a>)</span></div><div class="grammar-rule-expression"><a href="#variable-name">variable-name</a>&nbsp;<span class="grammar-symbol">(</span><a href="#sp">sp</a>&nbsp;<a href="#filter">filter</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="variable-name">variable-name</span><span class="grammar-usedby">(used by <a href="#variable-definition">variable-definition</a>,&nbsp;<a href="#expr">expr</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[A-Za-z]</span>&nbsp;<span class="grammar-regex">[A-Za-z_-0-9]*</span></div></div>
</div><div class="grammar-ruleset"><h3 id="filter">Filter</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="filter">filter</span><span class="grammar-usedby">(used by <a href="#capture">capture</a>,&nbsp;<a href="#assert">assert</a>,&nbsp;<a href="#expr">expr</a>)</span></div><div class="grammar-rule-expression">&nbsp;<a href="#count-filter">count-filter</a><br>
<span class="grammar-symbol">|</span><a href="#days-after-now-filter">days-after-now-filter</a><br>
<span class="grammar-symbol">|</span><a href="#days-before-now-filter">days-before-now-filter</a><br>
<span class="grammar-symbol">|</span><a href="#decode-filter">decode-filter</a><br>
<span class="grammar-symbol">|</span><a href="#format-filter">format-filter</a><br>
<span class="grammar-symbol">|</span><a href="#html-escape-filter">html-escape-filter</a><br>
<span class="grammar-symbol">|</span><a href="#html-unescape-filter">html-unescape-filter</a><br>
<span class="grammar-symbol">|</span><a href="#jsonpath-filter">jsonpath-filter</a><br>
<span class="grammar-symbol">|</span><a href="#nth-filter">nth-filter</a><br>
<span class="grammar-symbol">|</span><a href="#regex-filter">regex-filter</a><br>
<span class="grammar-symbol">|</span><a href="#replace-filter">replace-filter</a><br>
<span class="grammar-symbol">|</span><a href="#split-filter">split-filter</a><br>
<span class="grammar-symbol">|</span><a href="#to-date-filter">to-date-filter</a><br>
<span class="grammar-symbol">|</span><a href="#to-float-filter">to-float-filter</a><br>
<span class="grammar-symbol">|</span><a href="#to-int-filter">to-int-filter</a><br>
<span class="grammar-symbol">|</span><a href="#url-decode-filter">url-decode-filter</a><br>
<span class="grammar-symbol">|</span><a href="#url-encode-filter">url-encode-filter</a><br>
<span class="grammar-symbol">|</span><a href="#xpath-filter">xpath-filter</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="count-filter">count-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">count</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="days-after-now-filter">days-after-now-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">daysAfterNow</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="days-before-now-filter">days-before-now-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">daysBeforeNow</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="decode-filter">decode-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">decode</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="format-filter">format-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">format</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="html-escape-filter">html-escape-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">htmlEscape</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="html-unescape-filter">html-unescape-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">htmlUnescape</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="jsonpath-filter">jsonpath-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">jsonpath</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="nth-filter">nth-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">nth</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#integer">integer</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="regex-filter">regex-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">regex</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">|</span><a href="#regex">regex</a><span class="grammar-symbol">)</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="replace-filter">replace-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">replace</span>&nbsp;<a href="#sp">sp</a>&nbsp;<span class="grammar-symbol">(</span><a href="#quoted-string">quoted-string</a><span class="grammar-symbol">|</span><a href="#regex">regex</a><span class="grammar-symbol">)</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="split-filter">split-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">split</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="to-date-filter">to-date-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">toDate</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="to-float-filter">to-float-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">toFloat</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="to-int-filter">to-int-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">toInt</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="url-decode-filter">url-decode-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">urlDecode</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="url-encode-filter">url-encode-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">urlEncode</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="xpath-filter">xpath-filter</span><span class="grammar-usedby">(used by <a href="#filter">filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">xpath</span>&nbsp;<a href="#sp">sp</a>&nbsp;<a href="#quoted-string">quoted-string</a></div></div>
</div><div class="grammar-ruleset"><h3 id="lexical-grammar">Lexical Grammar</h3><div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="boolean">boolean</span><span class="grammar-usedby">(used by <a href="#boolean-option">boolean-option</a>,&nbsp;<a href="#variable-value">variable-value</a>,&nbsp;<a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#json-value">json-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">true</span><span class="grammar-symbol">|</span><span class="grammar-literal">false</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="null">null</span><span class="grammar-usedby">(used by <a href="#variable-value">variable-value</a>,&nbsp;<a href="#predicate-value">predicate-value</a>,&nbsp;<a href="#json-value">json-value</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">null</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="alphanum">alphanum</span><span class="grammar-usedby">(used by <a href="#key-string-text">key-string-text</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[A-Za-z0-9]</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="integer">integer</span><span class="grammar-usedby">(used by <a href="#integer-option">integer-option</a>,&nbsp;<a href="#duration-option">duration-option</a>,&nbsp;<a href="#variable-value">variable-value</a>,&nbsp;<a href="#json-number">json-number</a>,&nbsp;<a href="#nth-filter">nth-filter</a>,&nbsp;<a href="#float">float</a>,&nbsp;<a href="#number">number</a>)</span></div><div class="grammar-rule-expression"><a href="#digit">digit</a><span class="grammar-symbol">+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="float">float</span><span class="grammar-usedby">(used by <a href="#variable-value">variable-value</a>,&nbsp;<a href="#number">number</a>)</span></div><div class="grammar-rule-expression"><a href="#integer">integer</a>&nbsp;<a href="#fraction">fraction</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="number">number</span><span class="grammar-usedby">(used by <a href="#greater-predicate">greater-predicate</a>,&nbsp;<a href="#greater-or-equal-predicate">greater-or-equal-predicate</a>,&nbsp;<a href="#less-predicate">less-predicate</a>,&nbsp;<a href="#less-or-equal-predicate">less-or-equal-predicate</a>,&nbsp;<a href="#predicate-value">predicate-value</a>)</span></div><div class="grammar-rule-expression"><a href="#integer">integer</a><span class="grammar-symbol">|</span><a href="#float">float</a></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="digit">digit</span><span class="grammar-usedby">(used by <a href="#integer">integer</a>,&nbsp;<a href="#fraction">fraction</a>,&nbsp;<a href="#exponent">exponent</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[0-9]</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="hexdigit">hexdigit</span><span class="grammar-usedby">(used by <a href="#oneline-hex">oneline-hex</a>,&nbsp;<a href="#unicode-char">unicode-char</a>,&nbsp;<a href="#json-string-escaped-char">json-string-escaped-char</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[0-9A-Fa-f]</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="fraction">fraction</span><span class="grammar-usedby">(used by <a href="#json-number">json-number</a>,&nbsp;<a href="#float">float</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">.</span>&nbsp;<a href="#digit">digit</a><span class="grammar-symbol">+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="exponent">exponent</span><span class="grammar-usedby">(used by <a href="#json-number">json-number</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><span class="grammar-literal">e</span><span class="grammar-symbol">|</span><span class="grammar-literal">E</span><span class="grammar-symbol">)</span>&nbsp;<span class="grammar-symbol">(</span><span class="grammar-literal">+</span><span class="grammar-symbol">|</span><span class="grammar-literal">-</span><span class="grammar-symbol">)</span><span class="grammar-symbol">?</span>&nbsp;<a href="#digit">digit</a><span class="grammar-symbol">+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="sp">sp</span><span class="grammar-usedby">(used by <a href="#request">request</a>,&nbsp;<a href="#response">response</a>,&nbsp;<a href="#capture">capture</a>,&nbsp;<a href="#assert">assert</a>,&nbsp;<a href="#header-query">header-query</a>,&nbsp;<a href="#certificate-query">certificate-query</a>,&nbsp;<a href="#cookie-query">cookie-query</a>,&nbsp;<a href="#xpath-query">xpath-query</a>,&nbsp;<a href="#jsonpath-query">jsonpath-query</a>,&nbsp;<a href="#regex-query">regex-query</a>,&nbsp;<a href="#variable-query">variable-query</a>,&nbsp;<a href="#predicate">predicate</a>,&nbsp;<a href="#equal-predicate">equal-predicate</a>,&nbsp;<a href="#not-equal-predicate">not-equal-predicate</a>,&nbsp;<a href="#greater-predicate">greater-predicate</a>,&nbsp;<a href="#greater-or-equal-predicate">greater-or-equal-predicate</a>,&nbsp;<a href="#less-predicate">less-predicate</a>,&nbsp;<a href="#less-or-equal-predicate">less-or-equal-predicate</a>,&nbsp;<a href="#start-with-predicate">start-with-predicate</a>,&nbsp;<a href="#end-with-predicate">end-with-predicate</a>,&nbsp;<a href="#contain-predicate">contain-predicate</a>,&nbsp;<a href="#match-predicate">match-predicate</a>,&nbsp;<a href="#include-predicate">include-predicate</a>,&nbsp;<a href="#expr">expr</a>,&nbsp;<a href="#jsonpath-filter">jsonpath-filter</a>,&nbsp;<a href="#nth-filter">nth-filter</a>,&nbsp;<a href="#regex-filter">regex-filter</a>,&nbsp;<a href="#replace-filter">replace-filter</a>,&nbsp;<a href="#split-filter">split-filter</a>,&nbsp;<a href="#xpath-filter">xpath-filter</a>,&nbsp;<a href="#lt">lt</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">[ \t]</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="lt">lt</span><span class="grammar-usedby">(used by <a href="#hurl-file">hurl-file</a>,&nbsp;<a href="#request">request</a>,&nbsp;<a href="#response">response</a>,&nbsp;<a href="#header">header</a>,&nbsp;<a href="#body">body</a>,&nbsp;<a href="#query-string-params-section">query-string-params-section</a>,&nbsp;<a href="#form-params-section">form-params-section</a>,&nbsp;<a href="#multipart-form-data-section">multipart-form-data-section</a>,&nbsp;<a href="#cookies-section">cookies-section</a>,&nbsp;<a href="#captures-section">captures-section</a>,&nbsp;<a href="#asserts-section">asserts-section</a>,&nbsp;<a href="#basic-auth-section">basic-auth-section</a>,&nbsp;<a href="#options-section">options-section</a>,&nbsp;<a href="#file-param">file-param</a>,&nbsp;<a href="#capture">capture</a>,&nbsp;<a href="#assert">assert</a>,&nbsp;<a href="#option">option</a>,&nbsp;<a href="#aws-sigv4-option">aws-sigv4-option</a>,&nbsp;<a href="#ca-certificate-option">ca-certificate-option</a>,&nbsp;<a href="#client-certificate-option">client-certificate-option</a>,&nbsp;<a href="#client-key-option">client-key-option</a>,&nbsp;<a href="#compressed-option">compressed-option</a>,&nbsp;<a href="#connect-to-option">connect-to-option</a>,&nbsp;<a href="#delay-option">delay-option</a>,&nbsp;<a href="#follow-redirect-option">follow-redirect-option</a>,&nbsp;<a href="#follow-redirect-trusted-option">follow-redirect-trusted-option</a>,&nbsp;<a href="#http10-option">http10-option</a>,&nbsp;<a href="#http11-option">http11-option</a>,&nbsp;<a href="#http2-option">http2-option</a>,&nbsp;<a href="#http3-option">http3-option</a>,&nbsp;<a href="#insecure-option">insecure-option</a>,&nbsp;<a href="#ipv4-option">ipv4-option</a>,&nbsp;<a href="#ipv6-option">ipv6-option</a>,&nbsp;<a href="#max-redirs-option">max-redirs-option</a>,&nbsp;<a href="#netrc-option">netrc-option</a>,&nbsp;<a href="#netrc-file-option">netrc-file-option</a>,&nbsp;<a href="#netrc-optional-option">netrc-optional-option</a>,&nbsp;<a href="#output-option">output-option</a>,&nbsp;<a href="#path-as-is-option">path-as-is-option</a>,&nbsp;<a href="#proxy-option">proxy-option</a>,&nbsp;<a href="#resolve-option">resolve-option</a>,&nbsp;<a href="#repeat-option">repeat-option</a>,&nbsp;<a href="#retry-option">retry-option</a>,&nbsp;<a href="#retry-interval-option">retry-interval-option</a>,&nbsp;<a href="#skip-option">skip-option</a>,&nbsp;<a href="#unix-socket-option">unix-socket-option</a>,&nbsp;<a href="#user-option">user-option</a>,&nbsp;<a href="#variable-option">variable-option</a>,&nbsp;<a href="#verbose-option">verbose-option</a>,&nbsp;<a href="#very-verbose-option">very-verbose-option</a>,&nbsp;<a href="#multiline-string">multiline-string</a>)</span></div><div class="grammar-rule-expression"><a href="#sp">sp</a><span class="grammar-symbol">*</span>&nbsp;<a href="#comment">comment</a><span class="grammar-symbol">?</span>&nbsp;<span class="grammar-regex">[\n]?</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="comment">comment</span><span class="grammar-usedby">(used by <a href="#lt">lt</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">#</span>&nbsp;<span class="grammar-regex">~[\n]*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="regex">regex</span><span class="grammar-usedby">(used by <a href="#regex-query">regex-query</a>,&nbsp;<a href="#match-predicate">match-predicate</a>,&nbsp;<a href="#regex-filter">regex-filter</a>,&nbsp;<a href="#replace-filter">replace-filter</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">/</span>&nbsp;<a href="#regex-content">regex-content</a>&nbsp;<span class="grammar-literal">/</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="regex-content">regex-content</span><span class="grammar-usedby">(used by <a href="#regex">regex</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-symbol">(</span><a href="#regex-text">regex-text</a><span class="grammar-symbol">|</span><a href="#regex-escaped-char">regex-escaped-char</a><span class="grammar-symbol">)</span><span class="grammar-symbol">*</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="regex-text">regex-text</span><span class="grammar-usedby">(used by <a href="#regex-content">regex-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-regex">~[\n\/]+</span></div></div>
<div class="grammar-rule"><div class="grammar-rule-declaration"><span class="grammar-rule-id" id="regex-escaped-char">regex-escaped-char</span><span class="grammar-usedby">(used by <a href="#regex-content">regex-content</a>)</span></div><div class="grammar-rule-expression"><span class="grammar-literal">\</span>&nbsp;<span class="grammar-regex">~[\n]</span></div></div>
</div>


<hr>

# Resources {#resources}

## License {#resources-license-license}

```

                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
   other entities that control, are controlled by, or are under common
   control with that entity. For the purposes of this definition,
   "control" means (i) the power, direct or indirect, to cause the
   direction or management of such entity, whether by contract or
   otherwise, or (ii) ownership of fifty percent (50%) or more of the
   outstanding shares, or (iii) beneficial ownership of such entity.

   "You" (or "Your") shall mean an individual or Legal Entity
   exercising permissions granted by this License.

   "Source" form shall mean the preferred form for making modifications,
   including but not limited to software source code, documentation
   source, and configuration files.

   "Object" form shall mean any form resulting from mechanical
   transformation or translation of a Source form, including but
   not limited to compiled object code, generated documentation,
   and conversions to other media types.

   "Work" shall mean the work of authorship, whether in Source or
   Object form, made available under the License, as indicated by a
   copyright notice that is included in or attached to the work
   (an example is provided in the Appendix below).

   "Derivative Works" shall mean any work, whether in Source or Object
   form, that is based on (or derived from) the Work and for which the
   editorial revisions, annotations, elaborations, or other modifications
   represent, as a whole, an original work of authorship. For the purposes
   of this License, Derivative Works shall not include works that remain
   separable from, or merely link (or bind by name) to the interfaces of,
   the Work and Derivative Works thereof.

   "Contribution" shall mean any work of authorship, including
   the original version of the Work and any modifications or additions
   to that Work or Derivative Works thereof, that is intentionally
   submitted to Licensor for inclusion in the Work by the copyright owner
   or by an individual or Legal Entity authorized to submit on behalf of
   the copyright owner. For the purposes of this definition, "submitted"
   means any form of electronic, verbal, or written communication sent
   to the Licensor or its representatives, including but not limited to
   communication on electronic mailing lists, source code control systems,
   and issue tracking systems that are managed by, or on behalf of, the
   Licensor for the purpose of discussing and improving the Work, but
   excluding communication that is conspicuously marked or otherwise
   designated in writing by the copyright owner as "Not a Contribution."

   "Contributor" shall mean Licensor and any individual or Legal Entity
   on behalf of whom a Contribution has been received by Licensor and
   subsequently incorporated within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   copyright license to reproduce, prepare Derivative Works of,
   publicly display, publicly perform, sublicense, and distribute the
   Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   (except as stated in this section) patent license to make, have made,
   use, offer to sell, sell, import, and otherwise transfer the Work,
   where such license applies only to those patent claims licensable
   by such Contributor that are necessarily infringed by their
   Contribution(s) alone or by combination of their Contribution(s)
   with the Work to which such Contribution(s) was submitted. If You
   institute patent litigation against any entity (including a
   cross-claim or counterclaim in a lawsuit) alleging that the Work
   or a Contribution incorporated within the Work constitutes direct
   or contributory patent infringement, then any patent licenses
   granted to You under this License for that Work shall terminate
   as of the date such litigation is filed.

4. Redistribution. You may reproduce and distribute copies of the
   Work or Derivative Works thereof in any medium, with or without
   modifications, and in Source or Object form, provided that You
   meet the following conditions:

   (a) You must give any other recipients of the Work or
   Derivative Works a copy of this License; and

   (b) You must cause any modified files to carry prominent notices
   stating that You changed the files; and

   (c) You must retain, in the Source form of any Derivative Works
   that You distribute, all copyright, patent, trademark, and
   attribution notices from the Source form of the Work,
   excluding those notices that do not pertain to any part of
   the Derivative Works; and

   (d) If the Work includes a "NOTICE" text file as part of its
   distribution, then any Derivative Works that You distribute must
   include a readable copy of the attribution notices contained
   within such NOTICE file, excluding those notices that do not
   pertain to any part of the Derivative Works, in at least one
   of the following places: within a NOTICE text file distributed
   as part of the Derivative Works; within the Source form or
   documentation, if provided along with the Derivative Works; or,
   within a display generated by the Derivative Works, if and
   wherever such third-party notices normally appear. The contents
   of the NOTICE file are for informational purposes only and
   do not modify the License. You may add Your own attribution
   notices within Derivative Works that You distribute, alongside
   or as an addendum to the NOTICE text from the Work, provided
   that such additional attribution notices cannot be construed
   as modifying the License.

   You may add Your own copyright statement to Your modifications and
   may provide additional or different license terms and conditions
   for use, reproduction, or distribution of Your modifications, or
   for any such Derivative Works as a whole, provided Your use,
   reproduction, and distribution of the Work otherwise complies with
   the conditions stated in this License.

5. Submission of Contributions. Unless You explicitly state otherwise,
   any Contribution intentionally submitted for inclusion in the Work
   by You to the Licensor shall be under the terms and conditions of
   this License, without any additional terms or conditions.
   Notwithstanding the above, nothing herein shall supersede or modify
   the terms of any separate license agreement you may have executed
   with Licensor regarding such Contributions.

6. Trademarks. This License does not grant permission to use the trade
   names, trademarks, service marks, or product names of the Licensor,
   except as required for reasonable and customary use in describing the
   origin of the Work and reproducing the content of the NOTICE file.

7. Disclaimer of Warranty. Unless required by applicable law or
   agreed to in writing, Licensor provides the Work (and each
   Contributor provides its Contributions) on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
   implied, including, without limitation, any warranties or conditions
   of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
   PARTICULAR PURPOSE. You are solely responsible for determining the
   appropriateness of using or redistributing the Work and assume any
   risks associated with Your exercise of permissions under this License.

8. Limitation of Liability. In no event and under no legal theory,
   whether in tort (including negligence), contract, or otherwise,
   unless required by applicable law (such as deliberate and grossly
   negligent acts) or agreed to in writing, shall any Contributor be
   liable to You for damages, including any direct, indirect, special,
   incidental, or consequential damages of any character arising as a
   result of this License or out of the use or inability to use the
   Work (including but not limited to damages for loss of goodwill,
   work stoppage, computer failure or malfunction, or any and all
   other commercial damages or losses), even if such Contributor
   has been advised of the possibility of such damages.

9. Accepting Warranty or Additional Liability. While redistributing
   the Work or Derivative Works thereof, You may choose to offer,
   and charge a fee for, acceptance of support, warranty, indemnity,
   or other liability obligations and/or rights consistent with this
   License. However, in accepting such obligations, You may act only
   on Your own behalf and on Your sole responsibility, not on behalf
   of any other Contributor, and only if You agree to indemnify,
   defend, and hold each Contributor harmless for any liability
   incurred by, or claims asserted against, such Contributor by reason
   of your accepting any such warranty or additional liability.

END OF TERMS AND CONDITIONS

APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

Copyright 2021 Hurl

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```


<hr>


