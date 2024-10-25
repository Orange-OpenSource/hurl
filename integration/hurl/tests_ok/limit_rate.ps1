Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-output --limit-rate 2000000 tests_ok/limit_rate.hurl
