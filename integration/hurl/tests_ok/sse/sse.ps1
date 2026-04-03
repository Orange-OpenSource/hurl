Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose tests_ok/sse/sse.hurl
