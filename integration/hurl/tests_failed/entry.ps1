Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --from-entry 10 --to-entry 1 tests_failed/entry.hurl
