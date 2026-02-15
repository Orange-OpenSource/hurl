Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/invalid_protocol/invalid_protocol.hurl
