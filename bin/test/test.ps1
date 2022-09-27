powershell write-host -foregroundcolor Cyan "----- tests -----"

& $PSScriptRoot\test_prerequisites.ps1
& $PSScriptRoot\test_unit.ps1
& $PSScriptRoot\test_integ.ps1
