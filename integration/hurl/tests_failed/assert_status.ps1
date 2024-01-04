Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --json tests_failed/assert_status.hurl
