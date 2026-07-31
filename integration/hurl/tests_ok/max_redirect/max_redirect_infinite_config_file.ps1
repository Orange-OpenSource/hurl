Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"

hurl --ipv4 --location tests_ok/max_redirect/max_redirect_infinite.hurl
