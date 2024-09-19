Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl --verbose tests_ok/multipart_form_data.hurl
