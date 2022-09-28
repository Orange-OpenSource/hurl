powershell write-host -foregroundcolor Cyan "----- install windows64 installer -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# install windows64 installer
$package_dir="$project_root_path\target\win-package"
Start-Process powershell "$package_dir\*win64-installer.exe /S" -NoNewWindow -Wait -PassThru

# refresh env
$registry_user_path=(Get-ItemProperty -Path 'HKCU:\Environment').Path
$registry_machine_path=(Get-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment').Path
$env:Path = "$registry_user_path;$registry_machine_path"


# hurl infos
(Get-Command hurl).Path
(Get-Command hurlfmt).Path
hurl --version
hurlfmt --version

cd $actual_dir
