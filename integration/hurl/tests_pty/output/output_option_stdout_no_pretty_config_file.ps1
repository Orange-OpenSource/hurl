Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config"
hurl --no-output tests_pty/output/output_option_stdout.hurl
