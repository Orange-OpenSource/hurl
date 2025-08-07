Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose tests_failed/retry/retry_option.hurl
