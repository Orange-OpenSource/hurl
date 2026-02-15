Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_error_parser/json_object_trailing_comma.hurl
