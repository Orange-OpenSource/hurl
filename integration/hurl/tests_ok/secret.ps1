Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --very-verbose `
    --secret a=foofoofoo `
    --secret b=barbar `
    --secret c=baz `
    tests_ok/secret.hurl
