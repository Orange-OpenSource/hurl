Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-output tests_ok/parse_cache.hurl
