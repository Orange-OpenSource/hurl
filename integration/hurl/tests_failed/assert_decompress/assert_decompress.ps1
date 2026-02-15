Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --compressed tests_failed/assert_decompress/assert_decompress.hurl
