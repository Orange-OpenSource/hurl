Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/timeout.hurl --max-time 1
