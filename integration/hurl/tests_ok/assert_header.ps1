Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --json --verbose tests_ok/assert_header.hurl
