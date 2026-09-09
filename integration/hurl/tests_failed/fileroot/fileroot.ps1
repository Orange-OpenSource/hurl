Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We test that a symlink cannot access outside the file-root (directory containing the Hurl file by default)
# Symlinks can be a file, or a directory containing a file that doesn't exist yet.

$unauthorized = Join-Path $env:TEMP 'unauthorized.bin'
$unauthorizedDir = Join-Path $env:TEMP 'unauthorized_dir'
Remove-Item -Path $unauthorizedDir -Recurse -Force -ErrorAction SilentlyContinue
New-Item -Path $unauthorized -Force -ItemType File | Out-Null
New-Item -Path $unauthorizedDir -Force -ItemType Directory | Out-Null
New-Item -Path tests_failed/fileroot/authorized.bin -Force -ItemType SymbolicLink -Target $unauthorized | Out-Null
New-Item -Path tests_failed/fileroot/authorized_dir -Force -ItemType SymbolicLink -Target $unauthorizedDir | Out-Null
hurl --continue-on-error tests_failed/fileroot/fileroot.hurl
