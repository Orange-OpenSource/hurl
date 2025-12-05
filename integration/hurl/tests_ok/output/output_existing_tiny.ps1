Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We test that --output truncates an existing file then appends it.

echo @"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"@ > build/output_existing_tiny.bin

hurl --output build/output_existing_tiny.bin tests_ok/output/output_existing_tiny.hurl
Write-Host (Get-Content build/output_existing_tiny.bin -Raw) -NoNewLine
