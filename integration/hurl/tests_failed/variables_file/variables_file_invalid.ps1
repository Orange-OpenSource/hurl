Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --continue-on-error tests_failed/variables_file/variables_file_invalid.hurl
