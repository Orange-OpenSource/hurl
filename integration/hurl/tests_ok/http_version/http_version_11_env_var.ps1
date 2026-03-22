Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_HTTP11 = '1'
hurl tests_ok/http_version/http_version_11.hurl
