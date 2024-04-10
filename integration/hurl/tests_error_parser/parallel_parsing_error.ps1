Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --parallel --color `
  tests_error_parser/parallel_parsing_error_a.hurl `
  tests_error_parser/parallel_parsing_error_b.hurl `
  tests_error_parser/parallel_parsing_error_c.hurl `
  tests_error_parser/parallel_parsing_error_d.hurl
