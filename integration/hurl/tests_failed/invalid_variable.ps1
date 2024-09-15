Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
hurl --variable newFoo=Bar --variable newUuid=123 tests_failed/invalid_variable.hurl

