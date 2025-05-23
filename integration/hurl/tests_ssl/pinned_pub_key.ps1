Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ssl/pinned_pub_key.hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem --pinnedpubkey tests_ssl/certs/server/key.pub.pem --verbose
