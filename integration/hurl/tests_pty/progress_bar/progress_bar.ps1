Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Progress bar is disabled in CI environment, so we unset it to check progress bar display.
$env:CI = $null

hurl --jobs 1 `
    --no-color `
    --test `
    tests_ok/progress_bar/progress_bar_a.hurl `
    tests_ok/progress_bar/progress_bar_b.hurl `
    tests_ok/progress_bar/progress_bar_c_with_a_very_long_long_long_long_name.hurl

