Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/template/template_variable_not_found.hurl
