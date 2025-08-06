Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --json tests_failed/query/query_header_not_found.hurl
