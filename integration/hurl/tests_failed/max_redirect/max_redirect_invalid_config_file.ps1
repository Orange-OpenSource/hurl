Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/invalid_config"

hurl --location tests_failed/max_redirect/max_redirect.hurl
