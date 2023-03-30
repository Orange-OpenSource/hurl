Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/retry.hurl --retry --retry-interval 100 --verbose --json
