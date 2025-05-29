Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --very-verbose tests_ok/utf8/utf8.hurl
