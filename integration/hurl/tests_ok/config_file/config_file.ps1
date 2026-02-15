Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME=$PSScriptRoot
hurl --no-color tests_ok/config_file/config_file.hurl
