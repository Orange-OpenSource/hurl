Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --continue-on-error tests_failed/body_binary.hurl
