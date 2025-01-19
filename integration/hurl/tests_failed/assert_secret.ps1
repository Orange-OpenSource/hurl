Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --secret name=Alice tests_failed/assert_secret.hurl
