Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --ipv4 tests_ok/bench/bench.hurl
