Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --no-color tests_failed/multiline.hurl
