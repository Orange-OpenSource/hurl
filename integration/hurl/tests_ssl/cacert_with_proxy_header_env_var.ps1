Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Does not work without --ssl-no-revoke
# $env:HURL_PROXY_HEADER = 'X-TO-PROXY-1: to-proxy-1|X-TO-PROXY-2: to-proxy-2'
# hurl --verbosity brief `
#      --proxy http://127.0.0.1:3128 `
#      --header X-TO-SERVER:to-server `
#      --cacert tests_ssl/certs/server/cert.pem `
#      tests_ssl/cacert_with_proxy.hurl
exit 255
