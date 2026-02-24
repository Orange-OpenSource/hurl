Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_VERBOSE = '0'
hurl tests_ok/verbosity/verbosity.hurl
Remove-Item Env:HURL_VERBOSE

$env:HURL_VERBOSE = '1'
hurl tests_ok/verbosity/verbosity.hurl
Remove-Item Env:HURL_VERBOSE

$env:HURL_VERY_VERBOSE = '1'
hurl tests_ok/verbosity/verbosity.hurl
Remove-Item Env:HURL_VERY_VERBOSE

$env:HURL_VERBOSITY = 'brief'
hurl tests_ok/verbosity/verbosity.hurl
Remove-Item Env:HURL_VERBOSITY

# Overrides env var

$env:HURL_VERBOSITY = 'brief'
hurl --verbose tests_ok/verbosity/verbosity.hurl
Remove-Item Env:HURL_VERBOSITY
