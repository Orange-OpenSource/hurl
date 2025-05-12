Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose tests_ok/assert/assert_body.hurl
