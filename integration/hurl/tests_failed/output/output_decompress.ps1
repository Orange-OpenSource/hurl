Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --compressed tests_failed/output/output_decompress.hurl
