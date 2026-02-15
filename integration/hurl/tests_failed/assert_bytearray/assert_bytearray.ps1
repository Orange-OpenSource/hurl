Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/assert_bytearray/assert_bytearray.hurl
