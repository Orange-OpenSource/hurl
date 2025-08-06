Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --proxy localhost:1111 tests_failed/proxy/proxy.hurl
