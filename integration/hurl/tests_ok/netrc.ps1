Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --verbose --netrc-file tests_ok/netrc_file.netrc tests_ok/netrc.hurl
