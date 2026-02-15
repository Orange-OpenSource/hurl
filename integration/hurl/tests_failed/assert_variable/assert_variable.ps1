Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --continue-on-error tests_failed/assert_variable/assert_variable.hurl
