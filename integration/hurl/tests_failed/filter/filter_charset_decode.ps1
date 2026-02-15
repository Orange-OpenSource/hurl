Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/filter/filter_charset_decode.hurl
