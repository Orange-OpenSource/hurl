Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --location --max-redirs -1 tests_ok/max_redirect_infinite.hurl
