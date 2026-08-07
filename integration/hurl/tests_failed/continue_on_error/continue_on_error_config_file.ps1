Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"

hurl tests_failed/continue_on_error/continue_on_error.hurl
