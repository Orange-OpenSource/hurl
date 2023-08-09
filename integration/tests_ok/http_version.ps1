Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --http1.0 tests_ok/http_version.hurl
