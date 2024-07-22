Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --variable name=Bruce tests_ok/cookies.hurl
