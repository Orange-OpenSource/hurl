Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --user someAccessKeyId:someSecretKey tests_ok/aws_sigv4_option.hurl --verbose
