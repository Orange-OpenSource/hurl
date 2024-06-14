Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- install rust -----"

echo "==== get rust version"
$rust_version = ((Get-Content packages/hurl/Cargo.toml) -match '^rust-version')[0].Split('"')[1]
echo "rust_version=$rust_version"

echo "==== get rustup-init.exe installer"
Invoke-WebRequest -Uri "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" -OutFile "rustup-init.exe"

echo "==== install rust"
Start-Process powershell ".\rustup-init.exe --default-toolchain $rust_version -y" -NoNewWindow -Wait -PassThru

echo "==== refresh env"
$env:Path += ";$env:USERPROFILE\.cargo\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::Process)
rustc --version
cargo --version

echo "==== remove tmp files"
Remove-Item -Path .\rustup-init.exe

