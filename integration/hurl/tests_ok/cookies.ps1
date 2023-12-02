Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/cookies.hurl --variable name=Bruce --verbose
