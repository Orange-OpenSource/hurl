Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --parallel tests_ok/stdout.hurl
