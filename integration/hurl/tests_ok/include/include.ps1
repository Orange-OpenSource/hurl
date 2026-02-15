Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --include tests_ok/include/include.hurl
