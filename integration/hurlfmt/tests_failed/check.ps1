Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurlfmt --check tests_failed/check_ok.hurl \
                tests_failed/check_error_io.hurl \
                tests_failed/check_error_parse.hurl \
                tests_failed/check_error_unformatted.hurl
