Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path -Path build/secret) {
    Remove-Item -Recurse build/secret
}

hurl --very-verbose `
    --secrets-file tests_ok/secret/secrets.env `
    --curl build/secret/curl.txt `
    --cookie-jar build/secret-cookies.txt `
    --report-html build/secret/report-html `
    --report-json build/secret/report-json `
    tests_ok/secret/secret.hurl

$secrets = @("secret1", "secret2", "secret3", "12345678", "secret-dynamic-0", "secret-dynamic-1", "secret-dynamic-2")

$files = @(Get-ChildItem -Filter *.html -Recurse build/secret/report-html)
$files += @(Get-ChildItem -Filter *.json build/secret/report-json)
$files += @(Get-ChildItem build/secret/curl.txt)
$files += @(Get-ChildItem build/secret-cookies.txt)
$files += @(Get-ChildItem tests_ok/secret/secret.err.pattern)

foreach ($secret in $secrets) {
    foreach ($file in $files) {
        # Don't search leaks in sources
        if ($file.name.EndsWith("source.html")) {
            continue
        }
        if (Get-Content $file | Select-String -CaseSensitive $secret) {
            echo "Secret <$secret> have leaked in $file"
            Get-Content $file
            exit 1
        }
    }
}
