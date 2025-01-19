Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path build/report/json) {
    Remove-Item -Recurse build/report/json
}

# test.2.hurl is KO but we want the script to continue until the end
$ErrorActionPreference = 'Continue'
hurl --test --report-json build/report/json tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-json build/report/json tests_ok/test.3.hurl
$ErrorActionPreference = 'Stop'

Write-Host (Get-Content build/report/json/report.json -Raw) -NoNewLine
