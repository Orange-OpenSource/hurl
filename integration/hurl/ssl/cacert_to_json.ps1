Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --cacert ssl/ca/cert.pem --ssl-no-revoke --json ssl/cacert.hurl
