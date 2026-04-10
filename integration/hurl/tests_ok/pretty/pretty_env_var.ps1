Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_PRETTY = '1'
hurl tests_ok/pretty/pretty.hurl
