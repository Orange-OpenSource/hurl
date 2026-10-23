Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --verbose tests_failed/curl_injection/curl_injection.hurl
