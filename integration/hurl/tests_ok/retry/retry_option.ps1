Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --verbose --json tests_ok/retry/retry_option.hurl
