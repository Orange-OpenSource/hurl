Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# PEM client certificates are not supported by Schannel (curl TLS backend on Windows)
# TODO: add a Windows specific test using a PKCS#12 client certificate (.p12)
#hurl --cacert tests_ssl/certs/server/cert.selfsigned.pem `
#    --cert tests_ssl/certs/client/cert.pem:foobar `
#    --key tests_ssl/certs/client/encrypted.key.pem `
#    tests_ssl/client_authentication_password.hurl
$ErrorActionPreference = 'Continue'
exit 255
