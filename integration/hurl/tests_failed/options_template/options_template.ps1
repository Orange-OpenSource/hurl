Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --continue-on-error tests_failed/options_template/options_template.hurl
