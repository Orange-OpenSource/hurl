Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- tests -----"

& $PSScriptRoot\test_prerequisites.ps1
if ($LASTEXITCODE) { Throw }
& $PSScriptRoot\test_unit.ps1
if ($LASTEXITCODE) { Throw }
& $PSScriptRoot\..\release\release.ps1
if ($LASTEXITCODE) { Throw }
& $PSScriptRoot\test_integ.ps1
if ($LASTEXITCODE) { Throw }

