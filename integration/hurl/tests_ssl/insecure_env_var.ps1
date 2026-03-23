Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_INSECURE = '1'
hurl tests_ssl/insecure.hurl
