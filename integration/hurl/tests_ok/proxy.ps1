Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --proxy localhost:3128 --verbose tests_ok/proxy.hurl
hurl --proxy 127.0.0.1:3128 --verbose tests_ok/proxy.hurl
