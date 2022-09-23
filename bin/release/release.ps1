echo "----- build release -----"

# build
cargo build --release --verbose --locked

# create final package
New-Item -ItemType Directory -Name .\target\win-package
Get-ChildItem -Path '.\target\release' -Recurse -Include *.dll -File | Copy-Item -Destination '.\target\win-package'
Get-ChildItem -Path '.\target\release' -Recurse -Include hurl*.exe -File | Copy-Item -Destination '.\target\win-package'
((.\target\win-package\hurl.exe --version) -Split " ")[1] > .\target\win-package\version.txt
Get-Content .\target\win-package\version.txt

# add hurl to PATH
$hurl_dir=(Get-Location).path
$oldpath=(Get-ItemProperty -Path HKCU:\Environment -Name Path).Path
$newpath="$hurl_dir\target\win-package;$oldpath"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
(Get-Command hurl).Path

# test hurl execution
hurl --version
