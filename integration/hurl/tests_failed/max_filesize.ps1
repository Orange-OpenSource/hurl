Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --max-filesize 255 tests_failed/max_filesize.hurl
