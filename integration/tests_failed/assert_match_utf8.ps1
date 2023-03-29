Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/assert_match_utf8.hurl --json
