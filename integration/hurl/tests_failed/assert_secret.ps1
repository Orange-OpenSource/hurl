Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path -Path build/assert_secret) {
    Remove-Item -Recurse build/assert_secret
}

# We want to check leaks and do not stop at the first error
$ErrorActionPreference = 'Continue'

hurl --secret name1=Alice `
    --secret name2=Bob `
    --error-format long `
    --report-html build/assert_secret/report-html `
    --report-json build/assert_secret/report-json `
    --report-junit build/assert_secret/report-junit/junit.xml `
    tests_failed/assert_secret.hurl

$secrets = @("Alice", "Bob")

$files = @(Get-ChildItem -Filter *.html -Recurse build/assert_secret/report-html)
$files += @(Get-ChildItem -Filter *.json build/assert_secret/)
$files += @(Get-ChildItem build/assert_secret/report-junit/junit.xml)
$files += @(Get-ChildItem tests_failed/assert_secret.err.pattern)

foreach ($secret in $secrets) {
    foreach ($file in $files) {
        # Don't search leaks in sources
        if ($file.name.EndsWith("source.html")) {
            continue
        }
        if (Get-Content $file | Select-String -CaseSensitive $secret) {
            echo "Secret <$secret> have leaked in $file"
            exit 1
        }
    }
}
