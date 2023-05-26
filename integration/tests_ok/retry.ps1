Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/retry.hurl --retry 10 --retry-interval 100 --verbose --json
