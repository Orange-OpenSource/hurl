Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
# We're using --jobs 1 to fix the standard error order.
hurl --test --jobs 1 --glob "tests_ok/test.*.hurl"
