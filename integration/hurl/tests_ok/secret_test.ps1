Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --test \
    --very-verbose \
    --secret a=secret1 \
    --secret b=secret2 \
    --secret c=secret3 \
    tests_ok/secret.hurl 2>build/secret_test.err

$secrets = @("secret1", "secret2", "secret3")

$file = "build/secret_test.err"

foreach ($secret in $secrets) {
    if (Get-Content $file | Select-String -CaseSensitive $secret) {
        echo "Secret <$secret> have leaked in $file"
        exit 1
    }
}
