Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --json tests_ok/json_output.hurl
