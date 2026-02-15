Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --max-time 5 tests_failed/streaming/streaming.hurl
