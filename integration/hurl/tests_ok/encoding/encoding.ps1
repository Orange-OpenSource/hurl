Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --output - tests_ok/encoding/encoding.hurl
