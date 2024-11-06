Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --json tests_failed/assert_template_variable_not_found.hurl
