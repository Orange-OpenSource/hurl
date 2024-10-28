Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-output tests_ok/limit_rate_option.hurl
