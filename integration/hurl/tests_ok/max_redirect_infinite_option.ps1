Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --ipv4 tests_ok/max_redirect_infinite_option.hurl
