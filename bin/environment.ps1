Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- context -----"

# get windows infos
Get-ComputerInfo -Property WindowsProductName,WindowsVersion,OsHardwareAbstractionLayer

# get powershell infos
(Get-Variable PSVersionTable -ValueOnly).PSVersion

# get vcpkg infos
vcpkg --version
if ($LASTEXITCODE) { Throw }

# get python infos
python -V
if ($LASTEXITCODE) { Throw }

# get cargo info
cargo --version
if ($LASTEXITCODE) { Throw }
