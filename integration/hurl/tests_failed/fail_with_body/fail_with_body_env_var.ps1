Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_FAIL_WITH_BODY = '1'
hurl tests_failed/fail_with_body/fail_with_body.hurl
