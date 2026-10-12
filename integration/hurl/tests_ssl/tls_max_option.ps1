Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable ver=1.2 --verbose tests_ssl/tls_max_option.hurl
