Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-pretty tests_ok/assert/assert_match.hurl
