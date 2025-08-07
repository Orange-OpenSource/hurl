Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --retry 5 --retry-interval 100 --verbose tests_failed/retry/retry.hurl
