Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config_trusted"

hurl tests_ok/follow_redirect/follow_redirect_trusted.hurl