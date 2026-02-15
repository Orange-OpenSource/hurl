Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/assert_content_encoding/assert_content_encoding.hurl
