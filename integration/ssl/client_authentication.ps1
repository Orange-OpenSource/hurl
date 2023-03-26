Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl ssl/client_authentication.hurl --cacert ssl/server/cert.selfsigned.pem --cert ssl/client/cert.pem --key ssl/client/key.pem --verbose || true
