Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_COMPRESSED = '1'
hurl tests_ok/compressed/compressed.hurl
