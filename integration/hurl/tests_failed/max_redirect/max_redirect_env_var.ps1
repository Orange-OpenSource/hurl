Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_MAX_REDIRS = '5'
hurl --location tests_failed/max_redirect/max_redirect.hurl
