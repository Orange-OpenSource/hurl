powershell write-host -foregroundcolor Cyan "----- integration tests -----"

$actual_dir=(Get-Location).Path
$project_root_path=(Resolve-Path -LiteralPath $PSScriptRoot\..\..).path

# hurl infos
(Get-Command hurl).Path
(Get-Command hurlfmt).Path
hurl --version
hurlfmt --version

# run integration tests
cd $project_root_path\integration
python integration.py

cd $actual_dir
