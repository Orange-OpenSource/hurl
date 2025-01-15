Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --very-verbose `
    --secret a=secret1 `
    --secret b=secret2 `
    --secret c=12345678 `
    --report-html build/secret `
    tests_ok/secret.hurl

$secrets = @("secret1", "secret2", "12345678")

$files = Get-ChildItem -Filter *.html -Recurse build/secret

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
