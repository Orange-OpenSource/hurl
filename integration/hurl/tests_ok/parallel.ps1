Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We run 4 Hurl files in parallel, each one has a ~5s duration.
# On usual hardware, this should be executed in ~5s.

$Start = [DateTimeOffset]::Now.ToUnixTimeSeconds()

hurl --parallel --jobs 4 --verbose --variable name=Bob `
  tests_ok/parallel.hurl `
  tests_ok/parallel.hurl `
  tests_ok/parallel.hurl `
  tests_ok/parallel.hurl

$End = [DateTimeOffset]::Now.ToUnixTimeSeconds()

$Duration = $End - $Start
if ($Duration -gt 6) {
    Write-Host "Parallel execution duration failed ${Duration}s (limit 6s)"
    exit 1
}
