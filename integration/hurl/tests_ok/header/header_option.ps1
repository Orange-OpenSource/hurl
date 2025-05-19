Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --header 'key: from-cli' `
  --variable my_header='key: from-variable' `
  --variable my_key=key-from-variable `
  --variable my_value=value-from-variable `
  tests_ok/header/header_option.hurl
