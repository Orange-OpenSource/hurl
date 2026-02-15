Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --verbose tests_failed/retry/retry_option.hurl
