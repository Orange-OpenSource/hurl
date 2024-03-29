Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/result.tap) {
    Remove-Item build/parallel.tap
}

# We use TAP report in parallel because the reports (Junit, HTML, TAP)
# are in the same order as in the command line (which can be different
# from the real execution time).
hurl --parallel --no-output --report-tap build/parallel.tap `
  tests_ok/parallel_tap_a.hurl `
  tests_ok/parallel_tap_b.hurl `
  tests_ok/parallel_tap_c.hurl `
  tests_ok/parallel_tap_d.hurl `
  tests_ok/parallel_tap_e.hurl `
  tests_ok/parallel_tap_f.hurl `
  tests_ok/parallel_tap_g.hurl

Write-Host (Get-Content build/parallel.tap -Raw) -NoNewLine
