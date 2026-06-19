Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

$env:XDG_CONFIG_HOME = "$PSScriptRoot/config_jsonpath_no_coercion"

hurl tests_ok/jsonpath/jsonpath_store_no_coercion.hurl
