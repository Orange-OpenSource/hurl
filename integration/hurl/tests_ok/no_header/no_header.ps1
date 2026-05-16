Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --no-header user-agent --no-header Accept tests_ok/no_header/no_header.hurl
