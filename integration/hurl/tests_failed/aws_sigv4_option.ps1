Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

try {
    curl --aws-sigv4
} finally {
    exit 255
}

hurl --user someAccessKeyId:someSecretKey tests_ok/aws_sigv4_option.hurl
