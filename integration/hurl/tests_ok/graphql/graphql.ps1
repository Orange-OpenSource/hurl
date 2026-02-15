Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --no-pretty tests_ok/graphql/graphql.hurl
