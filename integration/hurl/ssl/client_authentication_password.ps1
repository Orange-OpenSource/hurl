Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Hurl file not supported on Windows
#hurl ssl/client_authentication_password.hurl
$ErrorActionPreference = 'Continue'
exit 255
