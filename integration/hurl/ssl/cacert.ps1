Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl ssl/cacert.hurl --cacert ssl/ca/cert.pem --ssl-no-revoke --verbose
