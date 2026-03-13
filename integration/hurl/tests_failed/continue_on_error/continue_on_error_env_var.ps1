Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_CONTINUE_ON_ERROR = '1'
hurl tests_failed/continue_on_error/continue_on_error.hurl
