Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --json --verbose tests_ok/assert/assert_header.hurl
