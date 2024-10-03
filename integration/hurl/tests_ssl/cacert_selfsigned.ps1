Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ssl/cacert_selfsigned.hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem --verbose
