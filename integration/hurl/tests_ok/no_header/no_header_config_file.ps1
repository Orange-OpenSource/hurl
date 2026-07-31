Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME=$PSScriptRoot
hurl tests_ok/no_header/no_header.hurl
