Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --curl build/assert_body.curl --no-output tests_ok/assert_body.hurl

Write-Host (Get-Content build/assert_body.curl -Raw) -NoNewLine
