Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/result.xml) {
    Remove-Item build/result.xml
}
hurl --test --report-junit build/result.xml tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-junit build/result.xml tests_ok/test.3.hurl
Write-Host (Get-Content build/result.xml) -NoNewLine
