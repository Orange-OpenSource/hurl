Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- activate python venv -----"

$global:OriginalPath = $env:PATH
python -m venv "$env:TMP\venv"
. $env:TMP\venv\Scripts\activate.ps1
$env:PATH = "$env:PATH" + ";" + "$global:OriginalPath"
python -m pip install --upgrade pip --quiet
