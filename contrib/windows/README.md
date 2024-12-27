# What ?

This document describes the steps to build the `windows 64bits hurl binary` with Powershell 7.

All command have been launched with `admin` privileges on `c:\` root dir and executed sequentially. *(If you don't want to use `c:\` as installation path and git clone path, please replace all its references before executing commands )*

All the steps have been tested on a `blank` Windows 10 64bits and total operation time is about `30 minutes` with a xdsl connection (5mb/sec). 

You just have to follow each chapter sequentially until you get a windows installer allowing the native installation of hurl on your favorite windows 64bits computer.

# Build requirements

Install vs_buildtools

```pwsh
cd c:\
Invoke-WebRequest -UseBasicParsing https://aka.ms/vs/17/release/vs_buildtools.exe  -Outfile vs_buildtools.exe
Start-Process -Wait -PassThru -FilePath .\vs_buildtools.exe -ArgumentList "--addProductLang", "En-us", "--add", "Microsoft.VisualStudio.Workload.VCTools", "--includeRecommended", "--passive", "--norestart", "--nocache", "--wait"
```

Install chocolatey

```pwsh
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

Install git, llvm, nsis and python3

```pwsh
choco install --confirm --no-progress git winlibs-llvm-free nsis
choco install --confirm --no-progress python311
python -m pip install --upgrade pip --quiet
```

Install rust

```pwsh
Invoke-WebRequest -UseBasicParsing -OutFile "c:\rustup-init.exe" "https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe"
c:\rustup-init.exe -y  --default-toolchain stable-x86_64-pc-windows-msvc
Set-ItemProperty -Path HKCU:\Environment -Name RUST_BACKTRACE -Value "full"
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") 
```

Install vcpkg

```pwsh
git.exe config --global core.autocrlf false
git.exe config --global core.eol lf
git.exe clone https://github.com/microsoft/vcpkg
c:\vcpkg\bootstrap-vcpkg.bat
$oldpath = Get-ItemProperty -Path HKCU:\Environment -Name Path
$newpath = $oldpath.Path += ";c:\vcpkg"
Set-ItemProperty -Path HKCU:\Environment -Name Path -Value $newpath
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") 
```

# Clone hurl project

```pwsh
cd c:\
git.exe clone https://github.com/Orange-OpenSource/hurl.git
```

# Build

```pwsh
cd c:\hurl
.\bin\install_prerequisites_windows.ps1
.\bin\release\release.ps1
```

# Create a simple zip package

```pwsh
cd c:\hurl
.\bin\release\create_windows64_zip_package.ps1
```

# Create a package installer for win64

```pwsh
cd c:\hurl
.\bin\release\create_windows64_installer.ps1
```

