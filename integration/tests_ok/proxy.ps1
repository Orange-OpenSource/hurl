Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/proxy.hurl --proxy localhost:3128 --verbose
hurl tests_ok/proxy_option.hurl --verbose
