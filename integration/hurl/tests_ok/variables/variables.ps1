Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variables-file tests_ok/variables/variables0.env `
     --variables-file tests_ok/variables/variables1.env `
     --variable female=true `
     tests_ok/variables/variables.hurl
