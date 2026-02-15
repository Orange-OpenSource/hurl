Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --verbose tests_ok/delay/delay_option.hurl
