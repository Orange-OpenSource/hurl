Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --verbose tests_ok/follow_redirect_option.hurl
