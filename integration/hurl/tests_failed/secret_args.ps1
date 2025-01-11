Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --secret foo=a --secret foo=b tests_failed/secret_args.hurl
