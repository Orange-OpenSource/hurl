Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem tests_ssl/cacert_selfsigned.hurl
