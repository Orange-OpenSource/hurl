Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_LOCATION = '1'
hurl tests_ok/follow_redirect/follow_redirect.hurl
