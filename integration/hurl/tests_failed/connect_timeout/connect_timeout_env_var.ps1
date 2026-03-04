Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_CONNECT_TIMEOUT = '500ms'
hurl tests_failed/connect_timeout/connect_timeout.hurl
