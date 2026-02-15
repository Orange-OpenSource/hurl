Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Hurl file not supported on Windows
#hurl --no-color tests_ssl/error_client_authentication_password.hurl
$ErrorActionPreference = 'Continue'
exit 255
