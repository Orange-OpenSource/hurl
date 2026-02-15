Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/assert_invalid_predicate_type/assert_invalid_predicate_type.hurl
