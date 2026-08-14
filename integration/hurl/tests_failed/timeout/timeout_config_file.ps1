Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'

$XDG_CONFIG_HOME = (Join-Path (Split-Path -Parent $MyInvocation.MyCommand.Path) "config")
$env:XDG_CONFIG_HOME = $XDG_CONFIG_HOME
hurl tests_failed/timeout/timeout.hurl
exit $LASTEXITCODE
