Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'

$env:HURL_MAX_TIME = '1'
hurl tests_failed/timeout/timeout.hurl
$env:HURL_MAX_TIME = '1s'
hurl tests_failed/timeout/timeout.hurl
$env:HURL_MAX_TIME = '500ms'
hurl tests_failed/timeout/timeout.hurl
