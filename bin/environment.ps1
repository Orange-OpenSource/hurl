powershell write-host -foregroundcolor Cyan "----- context -----"

# get windows infos
Get-ComputerInfo -Property WindowsProductName,WindowsVersion,OsHardwareAbstractionLayer

# get vcpkg infos
vcpkg --version

# get python infos
python -V

# get cargo info
cargo --version
