Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --proxy localhost:1111 tests_failed/proxy/proxy.hurl
