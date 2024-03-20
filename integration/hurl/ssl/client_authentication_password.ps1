Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Hurl file not supported on Windows
$ErrorActionPreference = 'Continue'
exit 255
