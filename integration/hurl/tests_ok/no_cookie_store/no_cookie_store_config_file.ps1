Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME=$PSScriptRoot
hurl tests_ok/no_cookie_store/no_cookie_store.hurl
