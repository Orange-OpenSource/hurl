Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# --interactive not supported on Windows
#spawn hurl --no-color --verbose --interactive tests_ok/interactive.hurl
exit 255
