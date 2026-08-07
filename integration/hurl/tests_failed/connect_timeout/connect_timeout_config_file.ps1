Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"

hurl tests_failed/connect_timeout/connect_timeout.hurl
