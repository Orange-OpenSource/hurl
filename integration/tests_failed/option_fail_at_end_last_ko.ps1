Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --fail-at-end tests_failed/option_fail_at_end_last_ko.hurl
