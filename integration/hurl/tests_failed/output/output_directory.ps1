Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

New-Item -Path build\tmp -Force -ItemType Directory
hurl --no-color --output build/tmp tests_ok/hello/hello.hurl
