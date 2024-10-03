Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --cacert tests_ssl/certs/ca/cert.pem --ssl-no-revoke tests_ssl/cacert.hurl
