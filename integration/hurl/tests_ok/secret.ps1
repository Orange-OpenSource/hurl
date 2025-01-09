Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

if (Test-Path -Path build/secret) {
    Remove-Item -Recurse build/secret
}

hurl --very-verbose `
    --secret a=secret1 `
    --secret b=secret2 `
    --secret c=12345678 `
    --report-html build/secret/report-html `
    --report-json build/secret/report-json `
    tests_ok/secret.hurl

$secrets = @("secret1", "secret2", "secret3", "12345678")

$files = @(Get-ChildItem -Filter *.html -Recurse build/secret/report-html)
$files += @(Get-ChildItem -Filter *.json build/secret/report-json)
$files += @(Get-ChildItem tests_ok/secret.err.pattern)

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
