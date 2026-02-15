Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_error_parser/invalid_jsonpath.hurl
