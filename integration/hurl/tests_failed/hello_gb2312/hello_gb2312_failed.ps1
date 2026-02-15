Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/hello_gb2312/hello_gb2312_failed.hurl
