Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- activate python venv -----"

python -m venv "$env:TMP\venv"
. $env:TMP\venv\Scripts\activate.ps1
python -m pip install --upgrade pip --quiet

