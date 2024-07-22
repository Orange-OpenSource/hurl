Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/tap/result.tap) {
    Remove-Item build/tap/result.tap
}

# test.2.hurl is KO but we want the script to continue until the end
$ErrorActionPreference = 'Continue'
hurl --test --report-tap build/tap/result.tap tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-tap build/tap/result.tap tests_ok/test.3.hurl
$ErrorActionPreference = 'Stop'

Write-Host (Get-Content build/tap/result.tap -Raw) -NoNewLine
