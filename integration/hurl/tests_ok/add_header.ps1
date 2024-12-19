Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --header 'header-b:baz' --header 'header-c:qux' tests_ok/add_header.hurl
