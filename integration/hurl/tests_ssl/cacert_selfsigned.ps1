Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ssl/cacert.hurl --cacert tests_ssl/ca/cert.pem --ssl-no-revoke --verbose
