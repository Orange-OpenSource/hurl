Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --continue-on-error --max-filesize 255 tests_failed/max_filesize.hurl
