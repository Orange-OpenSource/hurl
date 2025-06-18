Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --jobs 1 `
    --progress-bar `
    --color `
    --test `
    tests_ok/progress_bar/progress_bar_a.hurl `
    tests_ok/progress_bar/progress_bar_b.hurl `
    tests_ok/progress_bar/progress_bar_c_with_a_very_long_long_long_long_name.hurl

