powershell write-host -foregroundcolor Cyan "----- context -----"

# get windows infos
Get-ComputerInfo -Property WindowsProductName,WindowsVersion,OsHardwareAbstractionLayer

# get powershell infos
(Get-Variable PSVersionTable -ValueOnly).PSVersion

# get vcpkg infos
vcpkg --version

# get python infos
python -V

# get cargo info
cargo --version
