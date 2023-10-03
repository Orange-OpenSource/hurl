Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
curl --version | grep Features | grep --quiet HTTP2
if ($LASTEXITCODE -eq 1) {
  exit 0
}
$ErrorActionPreference = 'Stop'

hurl tests_ok/http_version_2_option.hurl
