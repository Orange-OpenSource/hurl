Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --tls-max 1.2 --cacert tests_ssl/certs/server/cert.selfsigned.pem tests_ssl/tls_max.hurl
