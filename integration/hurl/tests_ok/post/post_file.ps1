Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable filename=data.bin --verbose tests_ok/post/post_file.hurl
