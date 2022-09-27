powershell write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install libxml and libcurl
$vcpkg_dir=(Get-command vcpkg).Source
if (Test-Path $vcpkg_dir\installed\x64-windows\lib\libcurl.lib) {echo "curl already installed"} else {vcpkg install curl:x64-windows}
if (Test-Path $vcpkg_dir\installed\x64-windows\lib\libxml2.lib) {echo "libxml2 already installed"} else {vcpkg install libxml2:x64-windows}
vcpkg update
vcpkg upgrade
vcpkg integrate install

# update pip
python -m pip install --upgrade pip --quiet
