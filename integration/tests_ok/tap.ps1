Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/result.tap) {
    Remove-Item build/result.tap
}
hurl --test --report-tap build/result.tap tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-tap build/result.tap tests_ok/test.3.hurl
Write-Host (Get-Content build/result.tap -Raw) -NoNewLine
