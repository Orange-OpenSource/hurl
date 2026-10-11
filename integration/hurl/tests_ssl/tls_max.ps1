Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --variable ver=1.2 tests_ssl/tls_max.hurl --verbose
