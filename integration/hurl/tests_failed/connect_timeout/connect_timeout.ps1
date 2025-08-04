Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --connect-timeout 1 tests_failed/connect_timeout/connect_timeout.hurl
