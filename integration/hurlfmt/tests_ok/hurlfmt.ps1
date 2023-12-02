Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
echo 'GET http://localhost:8000/hello' | hurlfmt --color
