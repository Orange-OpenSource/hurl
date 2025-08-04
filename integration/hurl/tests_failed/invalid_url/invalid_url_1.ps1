Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable host=localhost:8000 tests_failed/invalid_url/invalid_url_1.hurl
