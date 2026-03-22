Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_DELAY = '1000'
hurl tests_ok/delay/delay.hurl
$env:HURL_DELAY = '1s'
hurl tests_ok/delay/delay.hurl
