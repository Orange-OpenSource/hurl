Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
hurl --version | grep Features | grep -q HTTP2
if ($LASTEXITCODE -eq 1) {
  exit 255
}
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config_2"

hurl tests_ok/http_version/http_version_2_option.hurl
