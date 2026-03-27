Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:HURL_MAX_FILESIZE = '255'
hurl --continue-on-error tests_failed/max_filesize/max_filesize.hurl
