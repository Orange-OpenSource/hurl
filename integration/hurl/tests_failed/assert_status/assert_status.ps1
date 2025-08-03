Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --json tests_failed/assert_status/assert_status.hurl
