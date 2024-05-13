Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --max-time 5 tests_failed/streaming.hurl
