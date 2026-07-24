Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config_10"

hurl tests_ok/http_version/http_version_10.hurl
