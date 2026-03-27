Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_LOCATION_TRUSTED = '1'
hurl tests_ok/follow_redirect/follow_redirect_trusted.hurl
