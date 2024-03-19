Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --from-entry 2 --to-entry 4 --no-output tests_ok/entry.hurl
