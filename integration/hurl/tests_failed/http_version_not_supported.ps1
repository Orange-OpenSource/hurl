Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
hurl --version | grep Features | grep -q HTTP3
if ($LASTEXITCODE -eq 0) {
  exit 255
}
$ErrorActionPreference = 'Stop'

hurl --http3 tests_failed/http_version_not_supported.hurl
