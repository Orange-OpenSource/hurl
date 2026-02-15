Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --verbose tests_ok/content_type/content_type.hurl
