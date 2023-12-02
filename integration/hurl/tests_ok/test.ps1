Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --test --glob "tests_ok/test.*.hurl"
