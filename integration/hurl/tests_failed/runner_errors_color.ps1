Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --continue-on-error --color tests_failed/runner_errors.hurl
