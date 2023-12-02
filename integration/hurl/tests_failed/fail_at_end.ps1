Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --fail-at-end tests_failed/fail_at_end.hurl
