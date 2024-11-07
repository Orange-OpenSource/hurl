Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --no-output --curl build/multilines.curl tests_ok/multilines.hurl

Write-Host (Get-Content build/multilines.curl -Raw) -NoNewLine
