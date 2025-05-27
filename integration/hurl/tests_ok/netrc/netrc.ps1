Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --netrc-file tests_ok/netrc/netrc_file.netrc tests_ok/netrc/netrc.hurl
