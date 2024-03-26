Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --verbose tests_ok/basic_authentication_per_request.hurl
