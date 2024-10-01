Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ssl/client_authentication.hurl --cacert tests_ssl/server/cert.selfsigned.pem --cert tests_ssl/client/cert.pem --key tests_ssl/client/key.pem --verbose || true
