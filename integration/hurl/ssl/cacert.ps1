Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --cacert ssl/ca/cert.pem --ssl-no-revoke ssl/cacert.hurl
