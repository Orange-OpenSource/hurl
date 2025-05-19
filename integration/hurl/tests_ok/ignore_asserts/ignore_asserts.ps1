Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl  --ignore-asserts tests_ok/ignore_asserts/ignore_asserts.hurl
