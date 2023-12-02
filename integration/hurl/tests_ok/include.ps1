Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/include.hurl --include --verbose
