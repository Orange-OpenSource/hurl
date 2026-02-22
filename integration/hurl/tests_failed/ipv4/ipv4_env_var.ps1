Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_IPV4 = '1'
hurl tests_failed/ipv4/ipv4.hurl
