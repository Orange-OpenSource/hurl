Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path -Path build/tests_failed_injection/report) {
    Remove-Item -Recurse -Force build/tests_failed_injection/report
}


# We test a Hurl file that triggers a runtime error and want to check that any HTML files
# in the report has a plain "<script>" tag.
$ErrorActionPreference = 'Continue'
hurl --verbose --report-html build/tests_failed_injection/report tests_failed/html_report_injection/html_report_injection.hurl

$files = @(Get-ChildItem -File -Recurse build/tests_failed_injection/report/store)

foreach ($file in $files) {
    Get-Content $file | Select-String -CaseSensitive "<script>"
}

exit 1
