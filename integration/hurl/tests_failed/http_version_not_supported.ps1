Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$ErrorActionPreference = 'Continue'
$features = hurl --version | Select-String -Pattern 'Features'
if ($features -match 'HTTP3') {
    exit 255
}
$ErrorActionPreference = 'Stop'

hurl --http3 tests_failed/http_version_not_supported.hurl
