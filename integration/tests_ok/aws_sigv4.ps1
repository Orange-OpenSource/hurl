Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --user-agent hurl-test --user someAccessKeyId:someSecretKey tests_ok/aws_sigv4.hurl --verbose
