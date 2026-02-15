Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/assert_file/assert_file.hurl
