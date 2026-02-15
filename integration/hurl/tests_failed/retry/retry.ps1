Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --retry 5 --retry-interval 100 --verbose tests_failed/retry/retry.hurl
