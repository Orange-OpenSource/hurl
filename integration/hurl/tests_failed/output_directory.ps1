Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

New-Item -Path build\tmp -Force -ItemType Directory
hurl --output build/tmp tests_ok/hello.hurl
