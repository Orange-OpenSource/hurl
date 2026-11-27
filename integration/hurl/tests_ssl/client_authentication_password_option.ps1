Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# PEM client certificates are not supported by Schannel (curl TLS backend on Windows)
# TODO: add a Windows specific test using a PKCS#12 client certificate (.p12)
#hurl tests_ssl/client_authentication_password_option.hurl
$ErrorActionPreference = 'Continue'
exit 255
