Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable ver=1.2 --verbose tests_ssl/tls_min_option.hurl
