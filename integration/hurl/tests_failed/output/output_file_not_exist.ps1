Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --output C:/foo/bar/baz tests_ok/hello/hello.hurl
