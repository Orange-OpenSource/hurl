Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --ipv4 tests_failed/ipv4/ipv4.hurl
