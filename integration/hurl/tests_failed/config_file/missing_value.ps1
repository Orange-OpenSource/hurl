Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME="$PSScriptRoot/missing_value_config"
hurl tests_failed/config_file/config_file.hurl
