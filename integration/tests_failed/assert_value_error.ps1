Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/assert_value_error.hurl --json
