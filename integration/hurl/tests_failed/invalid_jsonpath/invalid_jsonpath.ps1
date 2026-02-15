Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/invalid_jsonpath/invalid_jsonpath.hurl
