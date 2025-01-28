Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --test \
    --very-verbose \
    --secret a=secret1 \
    --secret b=secret2 \
    --secret c=12345678 \
    tests_ok/secret_test.hurl 2>build/secret_test.err

$secrets = @("secret1", "secret2", "secret3", "12345678")

$file = "build/secret_test.err"

foreach ($secret in $secrets) {
    if (Get-Content $file | Select-String -CaseSensitive $secret) {
        echo "Secret <$secret> have leaked in $file"
        Get-Content $file
        exit 1
    }
}
