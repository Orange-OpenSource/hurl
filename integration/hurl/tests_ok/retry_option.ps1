Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --json tests_ok/retry_option.hurl
