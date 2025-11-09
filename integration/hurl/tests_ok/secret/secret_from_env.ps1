Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_SECRET_a = 'secret1'
$env:HURL_SECRET_b = 'secret2'
$env:HURL_SECRET_c = '12345678'

hurl \
    --very-verbose \
    tests_ok/secret/secret_from_env.hurl 2>build/secret_from_env.err

$secrets = @("secret1", "secret2", "secret3", "12345678")

$file = "build/secret_from_env.err"

foreach ($secret in $secrets) {
    if (Get-Content $file | Select-String -CaseSensitive $secret) {
        echo "Secret <$secret> have leaked in $file"
        Get-Content $file
        exit 1
    }
}
