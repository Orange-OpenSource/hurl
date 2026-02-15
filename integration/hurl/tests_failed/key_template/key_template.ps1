Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/key_template/key_template.hurl
