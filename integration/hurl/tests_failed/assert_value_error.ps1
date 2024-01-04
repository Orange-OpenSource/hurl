Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --json tests_failed/assert_value_error.hurl
