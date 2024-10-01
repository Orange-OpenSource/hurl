Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --cacert tests_ssl/ca/cert.pem --ssl-no-revoke --json tests_ssl/cacert.hurl
