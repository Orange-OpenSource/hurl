Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"

hurl --continue-on-error tests_failed/error_format_long/error_format_long.hurl
