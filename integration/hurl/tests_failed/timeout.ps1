Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
hurl --no-color tests_failed/timeout.hurl --max-time 1     # Default unit for max-time in seconds
hurl --no-color tests_failed/timeout.hurl --max-time 1s
hurl --no-color tests_failed/timeout.hurl --max-time 500ms
