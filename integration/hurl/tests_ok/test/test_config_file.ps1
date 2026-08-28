Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"

# We're using --jobs 1 to fix the standard error order.
hurl --jobs 1 --glob "tests_ok/test/test.*.hurl"
