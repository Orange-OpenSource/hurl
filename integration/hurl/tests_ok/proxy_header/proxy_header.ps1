Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Send proxy header Foo:Bar at the command-line
hurl --proxy-header Foo:Bar tests_ok/proxy_header/proxy_header.hurl

# Send proxy header Foo:Bar from config file
$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"
hurl tests_ok/proxy_header/proxy_header.hurl
Remove-Item Env:XDG_CONFIG_HOME

# Send proxy header Foo:Bar from environment variable
$env:HURL_PROXY_HEADER = 'Foo:Bar'
hurl tests_ok/proxy_header/proxy_header.hurl
Remove-Item Env:HURL_PROXY_HEADER

