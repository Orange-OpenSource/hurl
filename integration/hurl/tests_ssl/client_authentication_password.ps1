Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Hurl file not supported on Windows
#hurl tests_ssl/client_authentication_password.hurl
$ErrorActionPreference = 'Continue'
exit 255
