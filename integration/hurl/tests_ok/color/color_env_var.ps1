Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_COLOR = '1'
hurl --include tests_ok/color/color.hurl
