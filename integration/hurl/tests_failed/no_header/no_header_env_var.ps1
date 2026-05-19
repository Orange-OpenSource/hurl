Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_NO_HEADER='foo|'
hurl tests_failed/no_header/no_header.hurl
