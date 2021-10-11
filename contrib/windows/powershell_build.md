# What ?

This document describes the steps to build the `windows 64bits hurl binary` with `powershell.exe`.

All command have been launched with `admin` privileges on `c:\` root dir and executed sequentially. *(If you don't want to use `c:\` as installation path and git clone path, please replace all its references before executing commands )*

All the steps have been tested on a `blank` Windows 10 64bits and total operation time is about `30 minutes` with a xdsl connection (5mb/sec). 

You just have to follow each chapter sequentially until you get a windows installer allowing the native installation of hurl on your favorite windows 64bits computer.

# Win64 workspace installation

## Manual softwares installation :

- install `builds tools c++` and `english language` by executing https://visualstudio.microsoft.com/fr/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16
- install `choco` by executing https://docs.chocolatey.org/en-us/choco/setup#install-with-powershell.exe

## Command line softwares installation

```powershell
cd c:\
choco install --confirm --no-progress git nsis python3 winlibs-llvm-free nsis
Invoke-WebRequest -UseBasicParsing -OutFile "c:\rustup-init.exe" "https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe"
c:\rustup-init.exe -y  --default-toolchain stable-x86_64-pc-windows-msvc
Set-ItemProperty -Path HKCU:\Environment -Name RUST_BACKTRACE -Value "full"
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") 
git.exe clone https://github.com/microsoft/vcpkg
c:\vcpkg\bootstrap-vcpkg.bat
$oldpath = Get-ItemProperty -Path HKCU:\Environment -Name Path
$newpath = $oldpath.Path += ";c:\vcpkg"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") 
vcpkg install libxml2:x64-windows
vcpkg integrate install
Set-ItemProperty -Path HKCU:\Environment -Name VCPKGRS_DYNAMIC -Value "1"
$env:VCPKGRS_DYNAMIC = [System.Environment]::GetEnvironmentVariable("VCPKGRS_DYNAMIC","User")
```

## Clone hurl project

```powershell
git.exe config --global core.autocrlf false
git.exe clone https://github.com/Orange-OpenSource/hurl
```

## Fix currently known win64 hurl compilation bugs

```
(Get-Content c:\hurl\packages\hurl\src\runner\hurl_file.rs).replace('```', '') | Set-Content c:\hurl\packages\hurl\src\runner\hurl_file.rs
```

## Build win64 exe binary

```powershell
cd c:\hurl
cargo build --release --verbose
New-Item -ItemType "Directory" -Path "c:\hurl\target" -Name "win-package"
Get-ChildItem -Path "c:\hurl\target\release" -Recurse -Include *.dll -File | Copy-Item -Destination "c:\hurl\target\win-package"
Get-ChildItem -Path "c:\hurl\target\release" -Recurse -Include hurl*.exe -File | Copy-Item -Destination "c:\hurl\target\win-package"
((c:\hurl\target\win-package\hurl.exe --version) -Split " ")[1] > c:\hurl\target\win-package\version.txt
$oldpath = Get-ItemProperty -Path HKCU:\Environment -Name Path
$newpath = $oldpath.Path += ";c:\hurl\target\win-package"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

## Test your app

install proxy and server

```powershell
pip3 install mitmproxy flask
```

Keep original powershell prompt on background, and open one more separate powershell prompt to launch the server

```powershell
cd c:\hurl\integration
python server.py
```

Keep original powershell prompt on background, and open one more separate powershell prompt to launch the ssl server

```powershell
cd c:\hurl\integration
python ssl/server.py
```


Keep original powershell prompt on background, and open one more separate powershell prompt to launch the proxy

```powershell
mitmdump --listen-port 8888 --modify-header "/From-Proxy/Hello"
```

focus on original powershell prompt and launch hurl unit tests

```powershell
cd c:\hurl\integration
cargo test --verbose
```

launch hurl integration tests

```powershell
cd c:\hurl\integration
./integration.py
```

## Generate version.txt file

```powershell
((c:\hurl\target\win-package\hurl.exe --version) -Split " ")[1] > c:\hurl\target\win-package\version.txt
```

## Create a simple zip package

```powershell
$hurl_package_version = Get-Content c:\hurl\target\win-package\version.txt
cd c:\hurl\target\win-package
Get-ChildItem -Path *.dll, *hurl.exe, *hurlfmt.exe, *.txt, ../../*.md  -Exclude hex_literal* | Compress-Archive -DestinationPath hurl-${hurl_package_version}-win64.zip
```

## Create a real package installer for win64

```powershell
cd c:\hurl\target\win-package
$oldpath = Get-ItemProperty -Path HKCU:\Environment -Name Path
$newpath = $oldpath.Path += ";C:\Program Files (x86)\NSIS\Bin"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") 
makensis.exe /NOCD /V4 ..\..\ci\windows\hurl.nsi
```
