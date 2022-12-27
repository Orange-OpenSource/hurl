powershell write-host -foregroundcolor Cyan "----- install system prerequisites -----"

# install libxml and libcurl[openssl]
$vcpkg_dir=(Get-command vcpkg).Source
if (Test-Path $vcpkg_dir\installed\x64-windows\bin\libssl-3-x64.dll) {echo "curl[openssl] already installed"} else {vcpkg install --recurse curl[openssl]:x64-windows}
if (Test-Path $vcpkg_dir\installed\x64-windows\bin\libxml2.dll) {echo "libxml2 already installed"} else {vcpkg install --recurse libxml2:x64-windows}
vcpkg update
vcpkg upgrade --no-dry-run
vcpkg integrate install

# update pip
python -m pip install --upgrade pip --quiet
