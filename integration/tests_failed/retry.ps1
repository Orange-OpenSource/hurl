Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/retry.hurl --retry --retry-max-count 5 --retry-interval 100 --verbose
