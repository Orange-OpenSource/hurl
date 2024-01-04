Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --json --verbose tests_ok/json_output.hurl
