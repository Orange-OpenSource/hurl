Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-color tests_failed/multipart_form_data/multipart_form_data.hurl
