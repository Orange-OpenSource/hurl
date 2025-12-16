Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbosity brief tests_ok/verbose/verbosity.hurl
