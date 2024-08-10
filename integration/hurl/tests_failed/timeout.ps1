Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
hurl tests_failed/timeout.hurl --max-time 1     # Default unit for max-time in seconds
hurl tests_failed/timeout.hurl --max-time 1s
hurl tests_failed/timeout.hurl --max-time 500ms
