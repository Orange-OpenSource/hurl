Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# FIXME: We simulate CI in order to disable progress bar (we don't have --no-progress-bar)
$env:CI = '1'

hurl --no-color --test --json --jobs 1 `
  tests_ok/parallel/parallel_a.hurl `
  tests_ok/parallel/parallel_b.hurl `
  tests_ok/parallel/parallel_c.hurl `
  tests_ok/parallel/parallel_d.hurl `
  tests_ok/parallel/parallel_e.hurl `
  tests_ok/parallel/parallel_f.hurl `
  tests_ok/parallel/parallel_g.hurl
