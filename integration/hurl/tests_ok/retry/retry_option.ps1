Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --json tests_ok/retry/retry_option.hurl
