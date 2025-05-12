Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --jobs 1 --progress-bar --test tests_ok/progress_bar_a.hurl tests_ok/progress_bar_b.hurl
