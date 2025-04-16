Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME  = Split-Path -Parent $MyInvocation.MyCommand.Path
hurl tests_ok/config_file.hurl
Write-Output ""
hurl --repeat 1 tests_ok/config_file.hurl
