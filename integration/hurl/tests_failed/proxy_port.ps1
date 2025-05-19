Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --proxy localhost:1111 tests_ok/hello/hello.hurl
