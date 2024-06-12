Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We test that --output truncates an existing file then appends it.
# In parallel, the order of the response can't be guaranteed, that's why we're using the same
# response here.

echo "Not a response" > build/output_parallel.bin

hurl --parallel --output build/output_parallel.bin tests_ok/output.hurl tests_ok/output.hurl
Write-Host (Get-Content build/output_parallel.bin -Raw) -NoNewLine
