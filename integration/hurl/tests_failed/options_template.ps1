Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --continue-on-error tests_failed/options_template.hurl
