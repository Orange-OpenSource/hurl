Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --color tests_failed/multiline/multiline.hurl
