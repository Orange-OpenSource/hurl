Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

New-Item -Force -Name "build" -ItemType Directory
echo "GET     http://localhost:8000/hello" >build/test.hurl
hurlfmt --in-place build/test.hurl
Get-Content  build/test.hurl
