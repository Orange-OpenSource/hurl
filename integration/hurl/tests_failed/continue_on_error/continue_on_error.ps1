Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --continue-on-error tests_failed/continue_on_error/continue_on_error.hurl
