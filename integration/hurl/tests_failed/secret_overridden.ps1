Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --secret name=a_secret_value tests_failed/secret_overridden.hurl
