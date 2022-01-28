# What ?

This document describes the steps to build the `windows 64bits hurl binary` with `cmd.exe`.

All command have been launched with `admin` privileges on `c:\` root dir and executed sequentially. *(If you don't want to use `c:\` as installation path and git clone path, please replace all its references before executing commands)*

All the steps have been tested on a `blank` Windows 10 64bits and total operation time is about `30 minutes` with a xdsl connection (5mb/sec). 

You just have to follow each chapter sequentially until you get a windows installer allowing the native installation of hurl on your favorite windows 64bits computer.

# Win64 workspace installation

## Manual softwares installation :

- install `builds tools c++` and `english language` by executing https://visualstudio.microsoft.com/fr/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16
- install `choco` by executing https://docs.chocolatey.org/en-us/choco/setup#install-with-cmd.exe

## Command line softwares installation

```cmd
cd c:\
choco install --confirm --no-progress curl unxUtils git 7zip nsis python3 winlibs-llvm-free
refreshenv
%ChocolateyInstall%\bin\curl --output "c:\rustup-init.exe" "https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe"
c:\rustup-init.exe -y  --default-toolchain stable-x86_64-pc-windows-msvc
setx RUST_BACKTRACE "full" /M
refreshenv
git.exe clone https://github.com/microsoft/vcpkg
c:\vcpkg\bootstrap-vcpkg.bat
setx /M PATH "%PATH%;c:\vcpkg"
setx VCPKGRS_DYNAMIC "1" /M
refreshenv
vcpkg install libxml2:x64-windows
vcpkg integrate install
```

## Clone hurl project

```cmd
git.exe clone https://github.com/Orange-OpenSource/hurl
```

## Build win64 exe binary

```cmd
cd c:\hurl
cargo build --release --verbose
mkdir c:\hurl\target\win-package
%ChocolateyInstall%\bin\find c:\hurl\target\release -name "*.dll" | xargs -i cp -frp {} c:\hurl\target\win-package
%ChocolateyInstall%\bin\find c:\hurl\target\release -maxdepth 1 -name "hurl*.exe" | xargs -i cp -frp {} c:\hurl\target\win-package
setx /M PATH "%PATH%;c:\hurl\target\win-package"
refreshenv
```

## Test your app

launch proxy server

```cmd
pip3 install mitmproxy
start /B mitmdump -p 8888 --modify-header "/From-Proxy/Hello"
```

launch test server

```cmd
pip3 install flask
cd c:\hurl\integration
start /B server.py
```

launch the ssl server

```cmd
cd c:\hurl\integration
start /B ssl/server.py
```

launch hurl unit tests

```cmd
cd c:\hurl
cargo test --features strict --tests
```

launch hurl integration tests

```cmd
cd c:\hurl\integration
./integration.py
```

## Generate version.txt file

```cmd
hurl.exe --version | cut -d" " -f2 > c:\hurl\target\win-package\version.txt
```

## Create a simple zip package

```cmd
set /P hurl_package_version=<c:\hurl\target\win-package\version.txt
cd c:\hurl\target\win-package
7z.exe a -y hurl-%hurl_package_version%-win64.zip *.dll *hurl.exe *hurlfmt.exe *.txt ..\..\*.md -xr!hex_literal*.dll
```

## Create a real package installer for win64

```cmd
setx /M path "%path%;C:\Program Files (x86)\NSIS\Bin"
refreshenv
cd c:\hurl\target\win-package
makensis /NOCD /V4 c:\hurl\ci\windows\hurl.nsi
```
