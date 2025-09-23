Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- create windows64 installer -----"

$actual_dir=(Get-Location).Path

# install NSIS
if (Get-Command makensis -ErrorAction SilentlyContinue) {
    echo "makensis already installed"
} else {
    choco install --confirm --no-progress nsis
    $env:Path = 'C:\Program Files (x86)\NSIS' + ";" + "$env:Path"
}
makensis /VERSION
$nsis_dir=(Get-Command makensis).path | Split-Path -Parent
echo "nsis_dir=$nsis_dir"
Expand-Archive -Path "$PSScriptRoot\..\..\bin\windows\EnVar_plugin.zip" -DestinationPath "$nsis_dir" -Force -Verbose

# create win64 installer
cd $PSScriptRoot\..\..\target\win-package
makensis.exe /NOCD /V4 ..\..\bin\windows\hurl.nsi
if ($LASTEXITCODE) { Throw }

cd $actual_dir
