Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --header foo tests_failed/header/header.hurl
