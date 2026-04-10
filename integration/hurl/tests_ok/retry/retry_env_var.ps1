Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_RETRY = '10'
$env:HURL_RETRY_INTERVAL = '100ms'
hurl --verbose --json tests_ok/retry/retry.hurl
