Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --location --verbose tests_ok/follow_redirect.hurl
