Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --verbosity brief tests_ok/verbosity/verbosity.hurl
