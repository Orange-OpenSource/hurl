Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --continue-on-error tests_failed/max_redirect_option.hurl
