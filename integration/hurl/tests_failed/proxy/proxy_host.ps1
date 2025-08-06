Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --proxy unknown tests_failed/proxy/proxy.hurl
