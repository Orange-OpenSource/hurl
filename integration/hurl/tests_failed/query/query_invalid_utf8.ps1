Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl tests_failed/query/query_invalid_utf8.hurl
