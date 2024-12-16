Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --test --very-verbose --secret a=foofoofoo --secret b=barbar --secret c=baz tests_ok/secret.hurl 2>build/secret_test.err

$words=@("foofoofoo", "barbar", "baz")

foreach ($word in $words) {
    if (Get-Content build/secret_test.err | Select-String -CaseSensitive $word) {
        # Secrets have leaked!
        exit 1
    }
}

