Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --verbose --no-output --curl build/headers.curl tests_ok/headers.hurl

Write-Host (Get-Content build/headers.curl -Raw) -NoNewLine
