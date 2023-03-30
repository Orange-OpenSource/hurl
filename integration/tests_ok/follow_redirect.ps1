Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/follow_redirect.hurl --location --verbose
