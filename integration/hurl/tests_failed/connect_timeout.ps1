Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/connect_timeout.hurl --connect-timeout 1
