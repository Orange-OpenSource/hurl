Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --color tests_ok/captures/captures_verbose.hurl
