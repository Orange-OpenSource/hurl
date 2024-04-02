Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- context -----"

write-host "# os"
Get-ComputerInfo -Property WindowsProductName,WindowsVersion,OsHardwareAbstractionLayer

write-host "# powershell"
(Get-Variable PSVersionTable -ValueOnly).PSVersion

write-host "# vcpkg"
(Get-Command -Name vcpkg -CommandType Application).Source
vcpkg --version
if ($LASTEXITCODE) { Throw }

write-host "# python"
(Get-Command -Name python -CommandType Application).Source
python -V
if ($LASTEXITCODE) { Throw }
(Get-Command -Name pip -CommandType Application).Source
pip --version
if ($LASTEXITCODE) { Throw }

write-host "# curl"
(Get-Command -Name curl -CommandType Application).Source
curl --version
if ($LASTEXITCODE) { Throw }

write-host "# rust"
(Get-Command -Name rustc -CommandType Application).Source
rustc --version
(Get-Command -Name cargo -CommandType Application).Source
cargo --version
if ($LASTEXITCODE) { Throw }

