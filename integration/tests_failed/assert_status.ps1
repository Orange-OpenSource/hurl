Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/assert_status.hurl --json
