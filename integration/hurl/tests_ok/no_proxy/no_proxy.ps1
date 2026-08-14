Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Disable proxy at the command-line
hurl --no-proxy 127.0.0.1 tests_ok/no_proxy/no_proxy.hurl

# Disable proxy from the config file
$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"
hurl tests_ok/no_proxy/no_proxy.hurl
Remove-Item Env:XDG_CONFIG_HOME

# Disable proxy from environment variable
$env:no_proxy = '127.0.0.1'
hurl --proxy localhost:3128 tests_ok/no_proxy/no_proxy.hurl
Remove-Item Env:no_proxy
