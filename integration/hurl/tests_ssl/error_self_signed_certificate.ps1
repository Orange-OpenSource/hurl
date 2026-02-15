Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_ssl/error_self_signed_certificate.hurl
