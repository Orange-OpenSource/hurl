Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --color --verbosity brief tests_ok/verbose/verbosity.hurl
