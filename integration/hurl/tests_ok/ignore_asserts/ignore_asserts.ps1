Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-assert tests_ok/ignore_asserts/ignore_asserts.hurl
