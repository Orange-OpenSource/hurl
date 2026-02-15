Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --verbose tests_ok/hello/hello.hurl
