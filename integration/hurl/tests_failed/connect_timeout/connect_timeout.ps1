Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --connect-timeout 1 tests_failed/connect_timeout/connect_timeout.hurl
