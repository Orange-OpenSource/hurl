Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
# Does not work without --ssl-no-revoke
#hurl --cacert tests_ssl/ca/cert.pem tests_ssl/cacert.hurl
exit 255
