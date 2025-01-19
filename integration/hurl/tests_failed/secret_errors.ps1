Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --continue-on-error --secret name=a_secret_value tests_failed/secret_errors.hurl
