Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --cacert tests_ssl/ca/cert.pem --ssl-no-revoke tests_ssl/cacert.hurl
