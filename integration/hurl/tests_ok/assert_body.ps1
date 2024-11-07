Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose tests_ok/assert_body.hurl
