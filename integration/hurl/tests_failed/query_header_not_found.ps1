Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/query_header_not_found.hurl --json
