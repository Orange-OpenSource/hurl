Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_ssl/error_options_pinnedpubkey.hurl
