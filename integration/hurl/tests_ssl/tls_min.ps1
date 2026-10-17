Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --tls-min 1.2 --cacert tests_ssl/certs/server/cert.selfsigned.pem tests_ssl/tls_min.hurl
