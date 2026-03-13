Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_ERROR_FORMAT = 'long'
hurl --continue-on-error tests_failed/error_format_long/error_format_long.hurl
