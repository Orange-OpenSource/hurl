Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_error_parser/duplicate_response_section.hurl
