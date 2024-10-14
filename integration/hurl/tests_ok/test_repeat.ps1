Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --test --repeat 100 tests_ok/test_repeat.hurl

