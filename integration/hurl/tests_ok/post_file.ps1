Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/post_file.hurl --variable filename=data.bin --verbose
