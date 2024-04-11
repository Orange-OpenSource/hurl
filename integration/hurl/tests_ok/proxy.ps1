Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --proxy localhost:3128 --verbose tests_ok/proxy.hurl
