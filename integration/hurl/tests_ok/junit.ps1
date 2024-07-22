Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/junit/result.xml) {
    Remove-Item build/junit/result.xml
}

# test.2.hurl is KO but we want the script to continue until the end
$ErrorActionPreference = 'Continue'
# We use --jobs 1 to force the standard error order to be test1 then test2.
hurl --test --jobs 1 --report-junit build/junit/result.xml tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-junit build/junit/result.xml tests_ok/test.3.hurl
# TBC: using || true versus $ErrorActionPreference = 'Continue'
hurl --test --report-junit build/junit/result.xml tests_ok/test.4.hurl || true
$ErrorActionPreference = 'Stop'

Write-Host (Get-Content build/junit/result.xml -Raw) -NoNewLine
