Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --proxy unknown tests_failed/proxy/proxy.hurl
