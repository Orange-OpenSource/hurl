Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color --compressed tests_failed/output/output_decompress.hurl
