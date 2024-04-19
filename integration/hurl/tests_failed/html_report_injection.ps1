Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path -Path build/injection/report) {
    Remove-Item -Recurse -Force build/injection/report
}


# We test a Hurl file that triggers a runtime error and want to check that any HTML files
# in the report has a plain "<script>" tag.
$ErrorActionPreference = 'Continue'
hurl --verbose --report-html build/injection/report tests_failed/html_report_injection.hurl

Select-String -Path build/injection/report/store -Pattern "<script>"

exit 1
