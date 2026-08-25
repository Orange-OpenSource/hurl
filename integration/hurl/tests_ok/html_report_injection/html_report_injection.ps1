Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path -Path build/tests_ok_injection/report) {
    Remove-Item -Recurse -Force build/tests_ok_injection/report
}

hurl --report-html build/tests_ok_injection/report tests_ok/html_report_injection/html_report_injection.hurl

$files = @(Get-ChildItem -File -Recurse build/tests_ok_injection/report/store)

foreach ($file in $files) {
    $found = Get-Content $file | Select-String -CaseSensitive "<script>"
    if ($found) {
        echo "Found unescaped <script> in HTML report"
        echo $found
        exit 1
    }
}
