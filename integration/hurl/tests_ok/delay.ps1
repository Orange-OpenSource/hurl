Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/delay.hurl --delay 1000 --verbose
