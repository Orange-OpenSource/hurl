Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- adhoc tests -----"

Write-Output 'GET https://google.com' | hurl --very-verbose --http2
Write-Output 'HEAD https://unpkg.com/vue@3.4.27/dist/vue.global.prod.js' | hurl --very-verbose

write-host -foregroundcolor Cyan "----- integration tests -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# hurl infos
(Get-Command hurl).Path
(Get-Command hurlfmt).Path
hurl --version
if ($LASTEXITCODE) { Throw }
hurlfmt --version
if ($LASTEXITCODE) { Throw }

# run integration tests
cd $project_root_path\integration\hurl
python integration.py
if ($LASTEXITCODE) { Throw }

cd $actual_dir

