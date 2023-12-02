Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --file-root . tests_ok/import_curl.out > $null  # Validate expected file
hurlfmt --in curl tests_ok/import_curl.in
