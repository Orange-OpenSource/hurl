Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-header foo --no-header '' tests_failed/no_header/no_header.hurl
