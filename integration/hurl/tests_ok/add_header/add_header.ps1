Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME=$PSScriptRoot/config
hurl --header 'header-b:baz' --header 'header-c:qux' tests_ok/add_header/add_header.hurl
