Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --continue-on-error tests_failed/max_redirect/max_redirect_option.hurl
