Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable name=Bruce tests_ok/cookie/cookies.hurl
