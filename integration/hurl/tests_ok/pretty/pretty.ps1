Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --pretty tests_ok/pretty/pretty.hurl
