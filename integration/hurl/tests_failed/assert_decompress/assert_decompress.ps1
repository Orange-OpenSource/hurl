Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --compressed tests_failed/assert_decompress/assert_decompress.hurl
