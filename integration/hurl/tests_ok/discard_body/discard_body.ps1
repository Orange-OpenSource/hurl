Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose --discard-body Accept tests_ok/discard_body/discard_body.hurl
