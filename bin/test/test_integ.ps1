echo "----- integration tests -----"

$actual_dir=(Get-Location).Path

# run integration tests
cd $PSScriptRoot\..\..\integration
python ./integration.py

cd $actual_dir
