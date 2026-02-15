Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl ---no-color --json tests_failed/assert_status/assert_status.hurl
