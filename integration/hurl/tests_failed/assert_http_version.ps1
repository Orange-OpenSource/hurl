Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --continue-on-error tests_failed/assert_http_version.hurl
