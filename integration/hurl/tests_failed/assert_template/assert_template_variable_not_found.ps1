Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --json tests_failed/assert_template/assert_template_variable_not_found.hurl
