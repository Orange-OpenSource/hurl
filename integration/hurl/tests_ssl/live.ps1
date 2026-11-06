Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# The tests for certificate reuse need a libcurl >= 8.2.0
$skip_cert_test = 'true'
$curl_version = [regex]::Match((curl.exe --version | Out-String), '^curl (\d+\.\d+\.\d+)').Groups[1].Value
if ([version]$curl_version -ge [version]'8.2.0') {
    $skip_cert_test = 'false'
}

hurl --variable skip_cert_test=$skip_cert_test tests_ssl/live.hurl
